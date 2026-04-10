use crate::config::Config;
use crate::AppEvent;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tao::event_loop::EventLoopProxy;
use tracing::{error, info, warn};

/// Push a clipboard entry to the Cloudflare API
pub async fn push_clip(cfg: &Arc<Mutex<Config>>, content: &str) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };

    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/clipboard/push", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&serde_json::json!({
            "content": content,
            "source": "desktop"
        }))
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            info!("Clip pushed to remote");
        }
        Ok(r) => {
            warn!("Push clip failed: {}", r.status());
        }
        Err(e) => {
            warn!("Push clip error: {}", e);
        }
    }

    Ok(())
}

/// Push the full config (hotkeys, hotstrings, prompts, tts, clips) to Cloudflare
pub async fn push_config(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, payload) = {
        let c = cfg.lock().unwrap();
        if c.api_token.is_empty() {
            return Ok(());
        }
        let payload = serde_json::json!({
            "hotkeys": c.hotkeys,
            "hotstrings": c.hotstrings,
            "prompts": c.prompts,
            "tts": c.tts,
            "clip_slots": c.clip_slots,
        });
        (c.api_url.clone(), c.api_token.clone(), payload)
    };

    let client = reqwest::Client::new();
    let resp = client
        .put(format!("{}/config/sync", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&payload)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            info!("Full config pushed to remote");
        }
        Ok(r) => {
            warn!("Config push failed: {}", r.status());
        }
        Err(e) => {
            warn!("Config push error: {}", e);
        }
    }

    Ok(())
}

/// Background sync loop — periodically pulls config updates from Cloudflare
pub async fn sync_loop(proxy: EventLoopProxy<AppEvent>, cfg: Arc<Mutex<Config>>) -> Result<()> {
    loop {
        let interval = {
            let c = cfg.lock().unwrap();
            c.sync_interval_secs
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

        if let Err(e) = pull_config(&cfg).await {
            error!("Config sync error: {}", e);
        } else {
            let _ = proxy.send_event(AppEvent::ConfigReloaded);
        }
    }
}

/// Pull updated config (hotkeys, hotstrings, clip history) from Cloudflare
async fn pull_config(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };

    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();

    // Pull hotkeys
    let resp = client
        .get(format!("{}/config/hotkeys", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            if let Ok(hotkeys) = r.json::<Vec<crate::config::HotkeyBinding>>().await {
                let mut c = cfg.lock().unwrap();
                c.hotkeys = hotkeys;
                info!("Synced {} hotkeys from remote", c.hotkeys.len());
            }
        } else {
            warn!("Hotkeys sync failed: {}", r.status());
        }
    }

    // Pull hotstrings
    let resp = client
        .get(format!("{}/config/hotstrings", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            if let Ok(hotstrings) = r.json::<Vec<crate::config::Hotstring>>().await {
                let mut c = cfg.lock().unwrap();
                c.hotstrings = hotstrings;
                info!("Synced {} hotstrings from remote", c.hotstrings.len());
            }
        } else {
            warn!("Hotstrings sync failed: {}", r.status());
        }
    }

    // Pull prompts
    let resp = client
        .get(format!("{}/config/prompts", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            if let Ok(prompts) = r.json::<Vec<crate::config::Prompt>>().await {
                let mut c = cfg.lock().unwrap();
                c.prompts = prompts;
                info!("Synced {} prompts from remote", c.prompts.len());
            }
        } else if r.status().as_u16() != 404 {
            warn!("Prompts sync failed: {}", r.status());
        }
    }

    // Pull clip history for slots
    let resp = client
        .get(format!("{}/clipboard/history?limit=10", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            if let Ok(clips) = r.json::<Vec<RemoteClip>>().await {
                let mut c = cfg.lock().unwrap();
                c.clip_slots = clips
                    .into_iter()
                    .map(|rc| crate::config::ClipSlot {
                        content: rc.content,
                        timestamp: rc.timestamp.unwrap_or_default(),
                    })
                    .collect();
                info!("Synced {} clipboard history entries from remote", c.clip_slots.len());
            }
        } else {
            warn!("Clipboard history sync failed: {}", r.status());
        }
    }

    // Save config locally
    {
        let c = cfg.lock().unwrap();
        if let Err(e) = c.save() {
            warn!("Failed to save config: {}", e);
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct RemoteClip {
    content: String,
    timestamp: Option<String>,
}
