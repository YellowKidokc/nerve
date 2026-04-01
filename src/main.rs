#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clipboard;
mod config;
mod hotkeys;
mod hotstrings;
mod panels;
mod sync_client;
mod tray;
mod tts;
#[allow(dead_code)]
mod window_mgmt;

use anyhow::Result;
use std::sync::{Arc, Mutex};
use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tracing::{error, info};

/// Custom events the system can send to the main event loop
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// Clipboard changed — content string
    ClipboardChanged(String),
    /// Hotkey triggered — hotkey id
    HotkeyTriggered(u32),
    /// Open a webview panel by name
    OpenPanel(String),
    /// Toggle panel visibility
    TogglePanel(String),
    /// Quit the application
    Quit,
    /// Config was reloaded from remote
    ConfigReloaded,
    /// IPC message from a webview panel
    IpcMessage { panel: String, body: String },
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("clipsync_agent=info".parse().unwrap()),
        )
        .init();

    info!("ClipSync Agent starting...");

    // Load config
    let cfg = config::Config::load()?;
    let cfg = Arc::new(Mutex::new(cfg));
    {
        let cfg_lock = cfg.lock().unwrap();
        clipboard::refresh_slots_from_config(&cfg_lock);
        // Load TTS settings from config
        tts::configure(
            &cfg_lock.tts.voice,
            cfg_lock.tts.speed,
            &cfg_lock.tts.engine,
            cfg_lock.tts.volume,
        );
    }

    // Build event loop with custom events
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // Start clipboard monitor thread
    let clip_proxy = proxy.clone();
    let clip_cfg = Arc::clone(&cfg);
    std::thread::spawn(move || {
        if let Err(e) = clipboard::monitor(clip_proxy, clip_cfg) {
            error!("Clipboard monitor error: {}", e);
        }
    });

    // Start hotstring engine thread (low-level keyboard hook)
    let hs_proxy = proxy.clone();
    let hs_cfg = Arc::clone(&cfg);
    std::thread::spawn(move || {
        if let Err(e) = hotstrings::engine(hs_proxy, hs_cfg) {
            error!("Hotstring engine error: {}", e);
        }
    });

    // Start background sync thread (pulls config from Cloudflare periodically)
    let sync_proxy = proxy.clone();
    let sync_cfg = Arc::clone(&cfg);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            if let Err(e) = sync_client::sync_loop(sync_proxy, sync_cfg).await {
                error!("Sync loop error: {}", e);
            }
        });
    });

    // Build hotkey map and register global hotkeys
    let mut hotkey_manager: Option<global_hotkey::GlobalHotKeyManager> = {
        let mut hk_cfg = cfg.lock().unwrap();
        hotkeys::build_hotkey_map(&mut hk_cfg);
        match hotkeys::register_all(&hk_cfg) {
            Ok(mgr) => Some(mgr),
            Err(e) => {
                error!("Failed to register hotkeys: {}", e);
                None
            }
        }
    };

    // Create system tray
    let _tray = tray::create_tray(&proxy)?;

    // Panel manager — holds open webview windows
    let mut panel_mgr = panels::PanelManager::new(proxy.clone());

    info!("ClipSync Agent ready.");

    // Main event loop
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check global hotkey events
        if let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().try_recv() {
            let cfg_lock = cfg.lock().unwrap();
            if let Some(action) = cfg_lock.hotkey_action(event.id()) {
                match action.as_str() {
                    "toggle_clipboard" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("clipboard".into()));
                    }
                    "toggle_prompts" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("prompts".into()));
                    }
                    "toggle_chat" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("chat".into()));
                    }
                    "toggle_links" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("links".into()));
                    }
                    "toggle_research" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("research".into()));
                    }
                    "toggle_dashboard" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("dashboard".into()));
                    }
                    "toggle_settings" => {
                        let _ = proxy.send_event(AppEvent::TogglePanel("settings".into()));
                    }
                    "tts_read_selection" => {
                        // Copy current selection (Ctrl+C), then read it aloud
                        info!("TTS: reading selection");
                        std::thread::spawn(|| {
                            tts::read_selection();
                        });
                    }
                    "paste_slot_1" => clipboard::paste_slot(0),
                    "paste_slot_2" => clipboard::paste_slot(1),
                    "paste_slot_3" => clipboard::paste_slot(2),
                    "paste_slot_4" => clipboard::paste_slot(3),
                    "paste_slot_5" => clipboard::paste_slot(4),
                    "paste_slot_6" => clipboard::paste_slot(5),
                    "paste_slot_7" => clipboard::paste_slot(6),
                    "paste_slot_8" => clipboard::paste_slot(7),
                    "paste_slot_9" => clipboard::paste_slot(8),
                    "paste_slot_10" => clipboard::paste_slot(9),
                    other => {
                        info!("Hotkey action: {}", other);
                    }
                }
            }
        }

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Event loop initialized");
            }

            Event::UserEvent(app_event) => match app_event {
                AppEvent::ClipboardChanged(content) => {
                    info!("Clipboard: {} chars", content.len());
                    // Push to sync client (fire and forget)
                    let sync_cfg = Arc::clone(&cfg);
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();
                        rt.block_on(async {
                            let _ = sync_client::push_clip(&sync_cfg, &content).await;
                        });
                    });
                }

                AppEvent::OpenPanel(name) => {
                    panel_mgr.open(&name, event_loop, &cfg);
                }

                AppEvent::TogglePanel(name) => {
                    panel_mgr.toggle(&name, event_loop, &cfg);
                }

                AppEvent::ConfigReloaded => {
                    info!("Config reloaded");
                    let mut cfg_lock = cfg.lock().unwrap();
                    cfg_lock.normalize();
                    hotkeys::build_hotkey_map(&mut cfg_lock);
                    clipboard::refresh_slots_from_config(&cfg_lock);
                    // Apply TTS config
                    tts::configure(
                        &cfg_lock.tts.voice,
                        cfg_lock.tts.speed,
                        &cfg_lock.tts.engine,
                        cfg_lock.tts.volume,
                    );
                    // Re-register hotkeys — replace the manager so the old one
                    // drops (unregistering old hotkeys) and the new one stays alive.
                    match hotkeys::register_all(&cfg_lock) {
                        Ok(mgr) => {
                            hotkey_manager = Some(mgr);
                        }
                        Err(e) => {
                            error!("Failed to re-register hotkeys: {}", e);
                        }
                    }
                }

                AppEvent::IpcMessage { panel, body } => {
                    handle_ipc(&mut panel_mgr, &cfg, &panel, &body);
                }

                AppEvent::Quit => {
                    info!("Quit requested");
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },

            _ => {}
        }

        // Suppress unused variable warning — the manager must stay alive
        // to keep hotkeys registered.
        let _ = &hotkey_manager;
    });
}

/// Handle an IPC message from a webview panel.
fn handle_ipc(
    panel_mgr: &mut panels::PanelManager,
    cfg: &Arc<Mutex<config::Config>>,
    panel: &str,
    body: &str,
) {
    let msg: serde_json::Value = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(e) => {
            error!("IPC parse error: {}", e);
            return;
        }
    };

    let msg_type = msg.get("type").and_then(|t| t.as_str()).unwrap_or("");

    match msg_type {
        "get_config" => {
            let cfg_lock = cfg.lock().unwrap();
            if let Ok(json) = serde_json::to_string(&*cfg_lock) {
                let script = format!("config = {}; populateUI();", json);
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "save_config" => {
            if let Some(new_cfg) = msg.get("config") {
                let mut cfg_lock = cfg.lock().unwrap();

                // General settings
                if let Some(v) = new_cfg
                    .get("clipboard_interval_ms")
                    .and_then(|v| v.as_u64())
                {
                    cfg_lock.clipboard_interval_ms = v;
                }
                if let Some(v) = new_cfg.get("api_url").and_then(|v| v.as_str()) {
                    cfg_lock.api_url = v.into();
                }
                if let Some(v) = new_cfg.get("api_token").and_then(|v| v.as_str()) {
                    cfg_lock.api_token = v.into();
                }
                if let Some(v) = new_cfg.get("sync_interval_secs").and_then(|v| v.as_u64()) {
                    cfg_lock.sync_interval_secs = v;
                }

                // TTS config
                if let Some(tts) = new_cfg.get("tts") {
                    if let Some(v) = tts.get("engine").and_then(|v| v.as_str()) {
                        cfg_lock.tts.engine = v.into();
                    }
                    if let Some(v) = tts.get("voice").and_then(|v| v.as_str()) {
                        cfg_lock.tts.voice = v.into();
                    }
                    if let Some(v) = tts.get("speed").and_then(|v| v.as_i64()) {
                        cfg_lock.tts.speed = v as i32;
                    }
                    if let Some(v) = tts.get("volume").and_then(|v| v.as_u64()) {
                        cfg_lock.tts.volume = v as u32;
                    }
                }

                // Hotkeys
                if let Some(hks) = new_cfg.get("hotkeys").and_then(|v| v.as_array()) {
                    cfg_lock.hotkeys = hks
                        .iter()
                        .filter_map(|hk| {
                            Some(config::HotkeyBinding {
                                keys: hk.get("keys")?.as_str()?.into(),
                                action: hk.get("action")?.as_str()?.into(),
                                runtime_id: None,
                            })
                        })
                        .collect();
                }

                // Hotstrings
                if let Some(hss) = new_cfg.get("hotstrings").and_then(|v| v.as_array()) {
                    cfg_lock.hotstrings = hss
                        .iter()
                        .filter_map(|hs| {
                            Some(config::Hotstring {
                                trigger: hs.get("trigger")?.as_str()?.into(),
                                expansion: hs.get("expansion")?.as_str()?.into(),
                                replace_trigger: hs
                                    .get("replace_trigger")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(true),
                            })
                        })
                        .collect();
                }

                // Apply TTS settings immediately
                tts::configure(
                    &cfg_lock.tts.voice,
                    cfg_lock.tts.speed,
                    &cfg_lock.tts.engine,
                    cfg_lock.tts.volume,
                );

                // Save to disk
                if let Err(e) = cfg_lock.save() {
                    error!("Failed to save config: {}", e);
                }

                info!("Config saved via IPC");
            }
        }

        "list_voices" => {
            let voices = tts::list_voices();
            if let Ok(json) = serde_json::to_string(&voices) {
                let script = format!("populateVoices({});", json);
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "get_clips" => {
            let cfg_lock = cfg.lock().unwrap();
            let clips: Vec<serde_json::Value> = cfg_lock
                .clip_slots
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "content": s.content,
                        "timestamp": s.timestamp,
                        "tags": [],
                        "pinned": false,
                    })
                })
                .collect();
            if let Ok(json) = serde_json::to_string(&clips) {
                let script = format!("clips = {}; renderSlots(); renderClips();", json);
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "test_tts" => {
            std::thread::spawn(|| {
                tts::speak("ClipSync text to speech is working.");
            });
        }

        "tts_stop" => {
            std::thread::spawn(|| {
                tts::stop();
            });
        }

        _ => {
            info!("Unknown IPC message type: '{}'", msg_type);
        }
    }
}
