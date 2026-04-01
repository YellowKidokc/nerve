use crate::config::Config;
use crate::window_mgmt;
use crate::AppEvent;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tao::dpi::{LogicalPosition, LogicalSize};
use tao::event_loop::{EventLoopProxy, EventLoopWindowTarget};
use tao::window::WindowBuilder;
use tracing::{error, info};
use wry::WebViewBuilder;

/// Manages open webview panel windows
pub struct PanelManager {
    /// Panel name → window + webview
    windows: HashMap<String, PanelWindow>,
    /// Event loop proxy for IPC messages
    proxy: EventLoopProxy<AppEvent>,
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
            proxy,
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

        // Set up IPC handler — routes messages to the main event loop
        let ipc_proxy = self.proxy.clone();
        let ipc_panel_name = name.to_string();

        let webview = match WebViewBuilder::new()
            .with_url(url)
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

        self.windows.insert(
            name.to_string(),
            PanelWindow {
                window,
                webview,
                visible: true,
            },
        );
    }
}
