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
use tao::event::{Event, StartCause, WindowEvent};
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
    /// IPC message from webview panel
    IpcMessage(String, String),
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
        tts::apply_config(&cfg_lock.tts);
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
    let mut hotkey_manager = {
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
    let mut panel_mgr = panels::PanelManager::new();

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
                    info!("Config reloaded from remote");
                    let mut cfg_lock = cfg.lock().unwrap();
                    cfg_lock.normalize();
                    hotkeys::build_hotkey_map(&mut cfg_lock);
                    clipboard::refresh_slots_from_config(&cfg_lock);
                    tts::apply_config(&cfg_lock.tts);
                    hotkey_manager = match hotkeys::register_all(&cfg_lock) {
                        Ok(mgr) => Some(mgr),
                        Err(e) => {
                            error!("Failed to re-register hotkeys: {}", e);
                            hotkey_manager.take()
                        }
                    };
                }

                AppEvent::IpcMessage(panel, message) => {
                    panel_mgr.handle_ipc(&panel, &message, &cfg);
                }

                AppEvent::Quit => {
                    info!("Quit requested");
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },

            Event::WindowEvent {
                event: WindowEvent::Moved(pos),
                window_id,
                ..
            } => {
                panel_mgr.update_position(window_id, pos.x, pos.y, &cfg);
            }

            _ => {}
        }
    });
}
