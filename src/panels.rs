use crate::config::Config;
use crate::tts;
use crate::window_mgmt;
use crate::AppEvent;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tao::dpi::{LogicalPosition, LogicalSize};
use tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};
use tao::window::WindowBuilder;
use tao::window::WindowId;
use tracing::{error, info};
use wry::WebViewBuilder;

/// Manages open webview panel windows
pub struct PanelManager {
    /// Panel name → window + webview
    windows: HashMap<String, PanelWindow>,
    window_to_name: HashMap<WindowId, String>,
}

struct PanelWindow {
    window: tao::window::Window,
    webview: wry::WebView,
    visible: bool,
}

impl PanelManager {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            windows: HashMap::new(),
<<<<<<< codex/update-tts-voice-selection-and-speed-settings-bwm8va
            window_to_name: HashMap::new(),
=======
            proxy,
>>>>>>> main
        }
    }

    /// Open a panel (create if not exists, show if hidden)
    pub fn open(
        &mut self,
        name: &str,
        event_loop: &EventLoopWindowTarget<AppEvent>,
        cfg: &Arc<Mutex<Config>>,
    ) {
        if let Some(panel) = self.windows.get_mut(name) {
            panel.window.set_visible(true);
            panel.window.set_focus();
            panel.visible = true;
            return;
        }

        // Create new panel window
        self.create_panel(name, event_loop, cfg);
    }

    /// Toggle a panel's visibility
    pub fn toggle(
        &mut self,
        name: &str,
        event_loop: &EventLoopWindowTarget<AppEvent>,
        cfg: &Arc<Mutex<Config>>,
    ) {
        if let Some(panel) = self.windows.get_mut(name) {
            panel.visible = !panel.visible;
            panel.window.set_visible(panel.visible);
            if panel.visible {
                panel.window.set_focus();
            }
            return;
        }

        // Doesn't exist yet — create and show
        self.create_panel(name, event_loop, cfg);
    }

    /// Evaluate JavaScript in a panel's webview
    pub fn evaluate_script(&self, panel_name: &str, script: &str) {
        if let Some(panel) = self.windows.get(panel_name) {
            if let Err(e) = panel.webview.evaluate_script(script) {
                error!("Script eval error in '{}': {}", panel_name, e);
            }
        }
    }

    fn create_panel(
        &mut self,
        name: &str,
        event_loop: &EventLoopWindowTarget<AppEvent>,
        cfg: &Arc<Mutex<Config>>,
    ) {
        let cfg_lock = cfg.lock().unwrap();
        let panel_def = match cfg_lock.panel(name) {
            Some(p) => p.clone(),
            None => {
                error!("No panel definition for '{}'", name);
                return;
            }
        };
        drop(cfg_lock);

        let mut builder = WindowBuilder::new()
            .with_title(&panel_def.title)
            .with_inner_size(LogicalSize::new(panel_def.width, panel_def.height))
            .with_decorations(true)
            .with_always_on_top(panel_def.always_on_top);

        // Restore saved position
        if let (Some(x), Some(y)) = (panel_def.x, panel_def.y) {
            builder = builder.with_position(LogicalPosition::new(x, y));
        }

        let window = match builder.build(event_loop) {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to create window for '{}': {}", name, e);
                return;
            }
        };

        // Apply dark title bar on Windows 11
        window_mgmt::set_dark_titlebar(&window);

        let url = &panel_def.url;
        let name_owned = name.to_string();
        let proxy = event_loop.create_proxy();

        // Set up IPC handler — routes messages to the main event loop
        let ipc_proxy = self.proxy.clone();
        let ipc_panel_name = name.to_string();

        let webview = match WebViewBuilder::new()
            .with_url(url)
            .with_initialization_script(
                r#"
                window.chrome = window.chrome || {};
                window.chrome.webview = {
                  postMessage: (msg) => window.ipc.postMessage(msg),
                  addEventListener: (name, cb) => {
                    if (name !== 'message') return;
                    window.addEventListener('message', (e) => cb({ data: e.data }));
                  }
                };
                "#,
            )
            .with_ipc_handler(move |req| {
                let payload = req.body().to_string();
                let _ = proxy.send_event(AppEvent::IpcMessage(name_owned.clone(), payload));
            })
            .with_devtools(cfg!(debug_assertions))
            .with_transparent(false)
            .with_ipc_handler(move |req| {
                let body = req.body().clone();
                let _ = ipc_proxy.send_event(AppEvent::IpcMessage {
                    panel: ipc_panel_name.clone(),
                    body,
                });
            })
            .build(&window)
        {
            Ok(wv) => wv,
            Err(e) => {
                error!("Failed to create webview for '{}': {}", name, e);
                return;
            }
        };

        info!("Panel '{}' opened → {}", name, url);

        let id = window.id();
        self.window_to_name.insert(id, name.to_string());
        self.windows.insert(
            name.to_string(),
            PanelWindow {
                window,
                webview,
                visible: true,
            },
        );
    }

    pub fn handle_ipc(&mut self, panel_name: &str, message: &str, cfg: &Arc<Mutex<Config>>) {
        #[derive(serde::Deserialize)]
        struct Incoming {
            #[serde(rename = "type")]
            msg_type: String,
            config: Option<Config>,
        }

        let parsed: Incoming = match serde_json::from_str(message) {
            Ok(v) => v,
            Err(e) => {
                error!("IPC parse error from '{}': {}", panel_name, e);
                return;
            }
        };

        match parsed.msg_type.as_str() {
            "get_config" => {
                let current = { cfg.lock().unwrap().clone() };
                if let Ok(json) = serde_json::to_string(&serde_json::json!({
                    "type": "config",
                    "config": current
                })) {
                    self.send_to_panel(panel_name, &json);
                }
            }
            "save_config" => {
                if let Some(mut incoming_cfg) = parsed.config {
                    incoming_cfg.normalize();
                    {
                        let mut state = cfg.lock().unwrap();
                        incoming_cfg.hotkey_map = state.hotkey_map.clone();
                        *state = incoming_cfg.clone();
                        if let Err(e) = state.save() {
                            error!("Saving config failed: {}", e);
                        }
                        tts::apply_config(&state.tts);
                    }
                    self.send_to_panel(panel_name, r#"{"type":"saved"}"#);
                }
            }
            "get_clips" => {
                let clips = { cfg.lock().unwrap().clip_slots.clone() };
                if let Ok(json) = serde_json::to_string(&serde_json::json!({
                    "type": "clips",
                    "clips": clips
                })) {
                    self.send_to_panel(panel_name, &json);
                }
            }
            "list_voices" => {
                let voices = tts::list_voices();
                if let Ok(json) = serde_json::to_string(&serde_json::json!({
                    "type": "voices",
                    "voices": voices
                })) {
                    self.send_to_panel(panel_name, &json);
                }
            }
            _ => {}
        }
    }

    pub fn update_position(&self, window_id: WindowId, x: i32, y: i32, cfg: &Arc<Mutex<Config>>) {
        if let Some(name) = self.window_to_name.get(&window_id) {
            let mut c = cfg.lock().unwrap();
            c.update_panel_position(name, x, y);
            let _ = c.save();
        }
    }

    fn send_to_panel(&self, panel_name: &str, json_payload: &str) {
        if let Some(panel) = self.windows.get(panel_name) {
            let script = format!(
                "window.dispatchEvent(new MessageEvent('message', {{ data: {} }}));",
                serde_json::to_string(json_payload).unwrap_or_else(|_| "\"{}\"".into())
            );
            let _ = panel._webview.evaluate_script(&script);
        }
    }
}
