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
        return Ok(()); // No API configured yet
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

pub async fn push_clips(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, clips) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone(), c.clip_slots.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/clipboard/push_slots", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&serde_json::json!({ "slots": clips, "source": "desktop" }))
        .send()
        .await?;

    if resp.status().is_success() {
        info!("Pushed clipboard slots");
    } else {
        warn!("Push clipboard slots failed: {}", resp.status());
    }
    Ok(())
}

pub async fn pull_clips(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/clipboard/pull_slots", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await?;

    if resp.status().is_success() {
        let remote = resp.json::<Vec<crate::config::ClipSlot>>().await?;
        let mut lock = cfg.lock().unwrap();
        lock.clip_slots = remote;
        let _ = lock.save();
        info!("Pulled clipboard slots");
    } else {
        warn!("Pull clipboard slots failed: {}", resp.status());
    }
    Ok(())
}

pub async fn push_config(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, config) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone(), c.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/config/push", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&serde_json::json!({ "config": config, "source": "desktop" }))
        .send()
        .await?;

    if resp.status().is_success() {
        info!("Pushed config");
    } else {
        warn!("Push config failed: {}", resp.status());
    }
    Ok(())
}

pub async fn pull_config_full(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/config/pull", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await?;

    if resp.status().is_success() {
        let mut remote = resp.json::<crate::config::Config>().await?;
        let mut lock = cfg.lock().unwrap();
        remote.hotkey_map = lock.hotkey_map.clone();
        *lock = remote;
        lock.normalize();
        let _ = lock.save();
        info!("Pulled full config");
    } else {
        warn!("Pull config failed: {}", resp.status());
    }
    Ok(())
}

pub async fn push_prompts(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, prompts) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone(), c.prompts.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/prompts/push", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&serde_json::json!({ "prompts": prompts, "source": "desktop" }))
        .send()
        .await?;

    if resp.status().is_success() {
        info!("Pushed prompts");
    } else {
        warn!("Push prompts failed: {}", resp.status());
    }
    Ok(())
}

pub async fn pull_prompts(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/prompts/pull", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await?;

    if resp.status().is_success() {
        let remote = resp.json::<Vec<crate::config::Prompt>>().await?;
        let mut lock = cfg.lock().unwrap();
        lock.prompts = remote;
        let _ = lock.save();
        info!("Pulled prompts");
    } else {
        warn!("Pull prompts failed: {}", resp.status());
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

/// Pull updated config (hotstrings, prompts, etc.) from Cloudflare
pub async fn pull_config(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };

    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();

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
            }
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

// ── IPC-triggered sync operations ─────────────────────────────────────

/// Push all clipboard slots to Cloudflare
pub async fn push_clips(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, clips) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone(), c.clip_slots.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/clipboard/push-all", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&serde_json::json!({ "clips": clips, "source": "desktop" }))
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => info!("Clips pushed to remote"),
        Ok(r) => warn!("Push clips failed: {}", r.status()),
        Err(e) => warn!("Push clips error: {}", e),
    }
    Ok(())
}

/// Pull clipboard slots from Cloudflare
pub async fn pull_clips(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
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
                let _ = c.save();
                info!("Pulled {} clips from remote", c.clip_slots.len());
            }
        }
    }
    Ok(())
}

/// Push full config to Cloudflare
pub async fn push_config(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, config_json) = {
        let c = cfg.lock().unwrap();
        let json = serde_json::to_value(&*c).unwrap_or_default();
        (c.api_url.clone(), c.api_token.clone(), json)
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/config/push", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&config_json)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => info!("Config pushed to remote"),
        Ok(r) => warn!("Push config failed: {}", r.status()),
        Err(e) => warn!("Push config error: {}", e),
    }
    Ok(())
}

/// Pull prompts from Cloudflare
pub async fn pull_prompts(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/config/prompts", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            if let Ok(prompts) = r.json::<Vec<crate::config::Prompt>>().await {
                let mut c = cfg.lock().unwrap();
                info!("Pulled {} prompts from remote", prompts.len());
                c.prompts = prompts;
                let _ = c.save();
            }
        }
    }
    Ok(())
}

/// Push prompts to Cloudflare
pub async fn push_prompts(cfg: &Arc<Mutex<Config>>) -> Result<()> {
    let (api_url, api_token, prompts) = {
        let c = cfg.lock().unwrap();
        (c.api_url.clone(), c.api_token.clone(), c.prompts.clone())
    };
    if api_token.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/config/prompts", api_url))
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&prompts)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => info!("Prompts pushed to remote"),
        Ok(r) => warn!("Push prompts failed: {}", r.status()),
        Err(e) => warn!("Push prompts error: {}", e),
    }
    Ok(())
}
