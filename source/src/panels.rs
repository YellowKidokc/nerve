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
    /// Reposition to the cursor each time this panel is shown.
    follow_cursor: bool,
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
            if panel.follow_cursor {
                let size = panel.window.inner_size();
                let (x, y) = cursor_anchor(size.width, size.height);
                panel.window.set_outer_position(LogicalPosition::new(x, y));
            }
            panel.window.set_visible(true);
            panel.window.set_focus();
            let _ = panel.webview.focus();
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
                let _ = panel.webview.focus();
            }
            return;
        }

        // Doesn't exist yet — create and show
        self.create_panel(name, event_loop, cfg);
    }

    /// Get a reference to the event loop proxy
    pub fn proxy(&self) -> &EventLoopProxy<AppEvent> {
        &self.proxy
    }

    /// Hide a panel by its tao window ID (called on close-requested)
    pub fn hide_by_window_id(&mut self, window_id: tao::window::WindowId) {
        for panel in self.windows.values_mut() {
            if panel.window.id() == window_id {
                panel.window.set_visible(false);
                panel.visible = false;
                return;
            }
        }
    }

    /// Hide a panel by name (the toolbar dismisses itself this way)
    pub fn hide(&mut self, name: &str) {
        if let Some(panel) = self.windows.get_mut(name) {
            panel.window.set_visible(false);
            panel.visible = false;
        }
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
            .with_decorations(panel_def.decorations)
            .with_always_on_top(panel_def.always_on_top);

        if panel_def.follow_cursor {
            // Floating capture surfaces appear where the user is looking.
            let (x, y) = cursor_anchor(panel_def.width, panel_def.height);
            builder = builder.with_position(LogicalPosition::new(x, y));
        } else if let (Some(x), Some(y)) = (panel_def.x, panel_def.y) {
            // Restore saved position
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

        // Set up IPC handler — routes messages to the main event loop
        let ipc_proxy = self.proxy.clone();
        let ipc_panel_name = name.to_string();

        // Resolve the HTML directory for serving files via custom protocol
        let html_dir = Config::html_dir();

        // Convert file:// URL to custom protocol URL
        let url = convert_to_custom_protocol(&panel_def.url, &html_dir);

        let webview = match WebViewBuilder::new()
            .with_url(&url)
            .with_devtools(true)
            .with_transparent(false)
            .with_custom_protocol("nerve".into(), move |_webview_id, request| {
                // Serve local files from html directory via nerve:// protocol
                let path = request.uri().path();
                // Strip leading slash
                let file_name = path.trim_start_matches('/');
                let file_path = html_dir.join(file_name);

                let content = std::fs::read(&file_path).unwrap_or_else(|_| {
                    format!("<html><body>File not found: {}</body></html>", file_name)
                        .into_bytes()
                });

                let mime = guess_mime(file_name);

                wry::http::Response::builder()
                    .status(200)
                    .header("Content-Type", mime)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(std::borrow::Cow::Owned(content))
                    .unwrap()
            })
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

        // Focus the webview so it receives input
        let _ = webview.focus();

        info!("Panel '{}' opened → {}", name, url);

        self.windows.insert(
            name.to_string(),
            PanelWindow {
                window,
                webview,
                visible: true,
                follow_cursor: panel_def.follow_cursor,
            },
        );
    }
}

/// Position a floating surface near the mouse without letting it fall off
/// screen. Placed slightly below-right of the cursor so it does not cover the
/// text the user just selected.
fn cursor_anchor(width: u32, height: u32) -> (i32, i32) {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
    };

    let mut point = POINT::default();
    unsafe {
        if GetCursorPos(&mut point).is_err() {
            return (0, 0);
        }

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        let margin = 12;
        let mut x = point.x + margin;
        let mut y = point.y + margin * 2;

        // Flip to the other side of the cursor rather than clipping.
        if x + width as i32 > screen_w {
            x = (point.x - width as i32 - margin).max(0);
        }
        if y + height as i32 > screen_h {
            y = (point.y - height as i32 - margin).max(0);
        }

        (x, y)
    }
}

/// Convert a file:// URL to nerve:// custom protocol URL
fn convert_to_custom_protocol(url: &str, html_dir: &std::path::Path) -> String {
    // If it's a file:// URL pointing to our html directory, convert to nerve://
    if url.starts_with("file:///") {
        let html_dir_str = html_dir
            .to_string_lossy()
            .replace('\\', "/");

        // Extract just the filename from the file URL
        let url_path = url
            .trim_start_matches("file:///")
            .replace('\\', "/");

        if url_path.starts_with(&html_dir_str) {
            let relative = url_path.trim_start_matches(&html_dir_str as &str);
            let relative = relative.trim_start_matches('/');
            return format!("nerve://localhost/{}", relative);
        }

        // Try matching just by filename
        if let Some(filename) = url_path.rsplit('/').next() {
            let local_file = html_dir.join(filename);
            if local_file.exists() {
                return format!("nerve://localhost/{}", filename);
            }
        }
    }

    // For http/https URLs or anything else, leave as-is
    url.to_string()
}

/// Guess MIME type from file extension
fn guess_mime(filename: &str) -> &'static str {
    if filename.ends_with(".html") || filename.ends_with(".htm") {
        "text/html"
    } else if filename.ends_with(".css") {
        "text/css"
    } else if filename.ends_with(".js") {
        "application/javascript"
    } else if filename.ends_with(".json") {
        "application/json"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        "image/jpeg"
    } else if filename.ends_with(".svg") {
        "image/svg+xml"
    } else if filename.ends_with(".ico") {
        "image/x-icon"
    } else if filename.ends_with(".woff2") {
        "font/woff2"
    } else if filename.ends_with(".woff") {
        "font/woff"
    } else {
        "application/octet-stream"
    }
}
