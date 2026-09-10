#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ai;
mod canon;
mod clipboard;
mod config;
mod hotkeys;
mod hotstrings;
mod idle;
mod mouse;
mod panels;
mod review;
mod scripts;
mod selection;
mod sync_client;
mod tray;
mod tts;
mod tts_engine;
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
    /// Hide a panel by name (floating surfaces dismiss themselves)
    HidePanel(String),
    /// Mouse trigger fired — payload is the target panel name
    MouseTrigger(String),
    /// Quit the application
    Quit,
    /// Config was reloaded from remote
    ConfigReloaded,
    /// IPC message from a webview panel
    IpcMessage { panel: String, body: String },
}

/// Claim the single-instance lock, or return `None` if another copy holds it.
///
/// With Start Menu, Desktop, and Startup shortcuts all pointing at the same
/// binary, launching twice is easy — and the symptoms are confusing rather
/// than obvious: the second process silently loses every global hotkey to the
/// first and leaves a duplicate tray icon behind. The mutex is session-local,
/// which is the right scope for a per-user tray app.
///
/// The returned handle must stay alive for the life of the process; Windows
/// releases the mutex when the process exits, including on a crash.
fn acquire_single_instance() -> Option<windows::Win32::Foundation::HANDLE> {
    use windows::core::w;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;

    unsafe {
        let handle = CreateMutexW(None, true, w!("NerveAgentSingleInstance")).ok()?;
        if GetLastError() == ERROR_ALREADY_EXISTS {
            return None;
        }
        Some(handle)
    }
}

/// Keep the local Mission Control surface behind its tray/hotkey entry alive.
/// If another process already owns 7860, it is left untouched.
fn ensure_mission_control_running() {
    use std::net::{SocketAddr, TcpStream};
    use std::process::Command;
    use std::time::Duration;
    use windows::Win32::System::Threading::CREATE_NO_WINDOW;
    use std::os::windows::process::CommandExt;

    let address: SocketAddr = "127.0.0.1:7860".parse().expect("static socket address");
    if TcpStream::connect_timeout(&address, Duration::from_millis(200)).is_ok() {
        info!("Mission Control already listening on 127.0.0.1:7860");
        return;
    }

    let Some(script) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join("mission-control").join("mission_control_gui.py")))
        .filter(|path| path.exists())
    else {
        error!("Mission Control script is not installed; tray entry will remain available for an externally started service.");
        return;
    };

    for interpreter in ["pythonw.exe", "python.exe"] {
        match Command::new(interpreter)
            .arg(&script)
            .arg("--no-browser")
            .creation_flags(CREATE_NO_WINDOW.0)
            .spawn()
        {
            Ok(_) => {
                info!("Mission Control started with {}", interpreter);
                return;
            }
            Err(e) => error!("Could not start Mission Control with {}: {}", interpreter, e),
        }
    }
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("clipsync_agent=info".parse().unwrap()),
        )
        .init();

    // `--tts-say <text>` speaks one utterance and exits, without taking the
    // instance lock. It exercises the same engine the agent uses, so it is the
    // way to check voice, rate and audio output while the agent is running.
    let args: Vec<String> = std::env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "--tts-say") {
        let text = args.get(pos + 1).cloned().unwrap_or_default();
        let cfg = config::Config::load()?;
        tts::configure(&cfg.tts.voice, cfg.tts.speed, "sapi", cfg.tts.volume);
        let _ = tts::speak(&text);

        // `--interrupt` re-speaks after two seconds; the first utterance must
        // cut off mid-word rather than finishing or overlapping.
        if args.iter().any(|a| a == "--interrupt") {
            std::thread::sleep(std::time::Duration::from_secs(2));
            info!("--- interrupting ---");
            let _ = tts::speak("Interrupted. The first utterance should have stopped instantly.");
        }

        // Async speech: hold the process open long enough to hear it.
        let seconds = args
            .iter()
            .position(|a| a == "--seconds")
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(20);
        std::thread::sleep(std::time::Duration::from_secs(seconds));
        return Ok(());
    }

    info!("ClipSync Agent starting...");

    // Exactly one copy may own the global hotkeys and the tray icon.
    let _instance_lock = match acquire_single_instance() {
        Some(handle) => handle,
        None => {
            info!("Another instance is already running — exiting.");
            return Ok(());
        }
    };

    // Load config
    let cfg = config::Config::load()?;
    let cfg = Arc::new(Mutex::new(cfg));
    ensure_mission_control_running();
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

    // Start mouse trigger thread (low-level mouse hook)
    let mouse_proxy = proxy.clone();
    let mouse_cfg = Arc::clone(&cfg);
    std::thread::spawn(move || {
        if let Err(e) = mouse::engine(mouse_proxy, mouse_cfg) {
            error!("Mouse trigger engine error: {}", e);
        }
    });

    // Open the review queue when work is waiting.
    //
    // Discovering what ran overnight must not require typing a command or
    // finding a folder. If nothing is pending the panel stays closed, so an
    // empty queue never becomes noise.
    {
        let root = review::output_root(&cfg.lock().unwrap());
        match review::load(&root) {
            Ok(q) if q.total > 0 => {
                info!("review: {} pending, opening the queue", q.total);
                let _ = proxy.send_event(AppEvent::OpenPanel("review".into()));
            }
            Ok(_) => info!("review: nothing pending"),
            Err(e) => error!("review: could not read the queue: {}", e),
        }
    }

    // Start idle canon worker thread (deterministic C0 capture only)
    let idle_cfg = Arc::clone(&cfg);
    std::thread::spawn(move || {
        if let Err(e) = idle::engine(idle_cfg) {
            error!("Idle canon worker error: {}", e);
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

    // tray-icon's tao integration requires creation after the event loop has
    // actually started. Creating it before `run` can make Shell_NotifyIconW
    // return E_FAIL on Windows.
    let mut tray_icon: Option<tray_icon::TrayIcon> = None;

    // Forward global hotkey events into the tao event loop so shortcuts work
    // even when the app is otherwise idle.
    let hotkey_proxy = proxy.clone();
    std::thread::spawn(move || loop {
        if let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().recv() {
            let _ = hotkey_proxy.send_event(AppEvent::HotkeyTriggered(event.id()));
        }
    });

    // Panel manager — holds open webview windows
    let mut panel_mgr = panels::PanelManager::new(proxy.clone());

    // Warm the speech engine now rather than on the first hotkey press.
    // Starting it costs COM initialization plus a voice enumeration; paying
    // that here keeps the first CapsLock+C as fast as every later one.
    tts::warm_up();

    // Panels the user wants up as soon as the agent is running. Queued as
    // events so the event loop creates them once it is initialized — panels
    // cannot be built before then.
    {
        let cfg_lock = cfg.lock().unwrap();
        for panel in cfg_lock.panels.iter().filter(|p| p.open_at_startup) {
            info!("Opening '{}' at startup", panel.name);
            let _ = proxy.send_event(AppEvent::OpenPanel(panel.name.clone()));
        }
    }

    info!("ClipSync Agent ready.");

    // Main event loop
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Event loop initialized");
                if tray_icon.is_none() {
                    match tray::create_tray(&proxy) {
                        Ok(icon) => {
                            info!("System tray icon registered successfully.");
                            tray_icon = Some(icon);
                        }
                        Err(e) => {
                            error!("Failed to create system tray icon: {}. App will continue running without tray.", e);
                        }
                    }
                }
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

                AppEvent::HidePanel(name) => {
                    panel_mgr.hide(&name);
                }

                AppEvent::MouseTrigger(target) => {
                    // The toolbar and capsule need a selection to be useful;
                    // the Stratum panel falls back to clipboard text.
                    if target == "stratum" {
                        spawn_capture_or_open(&proxy, &cfg, &target);
                    } else {
                        let node = if target == "capsule" { "claim" } else { "" };
                        spawn_capture(&proxy, &cfg, node, &target);
                    }
                }

                AppEvent::HotkeyTriggered(id) => {
                    let cfg_lock = cfg.lock().unwrap();
                    if let Some(action) = cfg_lock.hotkey_action(id) {
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
                            "toggle_shortcuts" => {
                                let _ = proxy.send_event(AppEvent::TogglePanel("shortcuts".into()));
                            }
                            "toggle_mission-control" => {
                                let _ = proxy.send_event(AppEvent::TogglePanel("mission-control".into()));
                            }
                            // Capture the selection, then raise the floating
                            // toolbar over it. Capture blocks on a clipboard
                            // round trip, so it runs off the event loop.
                            "selection_toolbar" => {
                                spawn_capture(&proxy, &cfg, "", "toolbar");
                            }
                            // Stratum's action popup, with the selection
                            // already captured. Unlike the toolbar this opens
                            // even with nothing selected, because the actions
                            // fall back to clipboard text.
                            "stratum_actions" => {
                                spawn_capture_or_open(&proxy, &cfg, "stratum");
                            }
                            "tts_read_selection" => {
                                info!("TTS: reading selection");
                                std::thread::spawn(|| {
                                    tts::read_selection();
                                });
                            }
                            "tts_stop" => tts::stop(),
                            "tts_pause" => tts::pause_toggle(),
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
                                if let Some(text) = other.strip_prefix("send_text:") {
                                    clipboard::paste_text(text);
                                } else if let Some(cmd) = other.strip_prefix("run_command:") {
                                    let cmd_str = cmd.to_string();
                                    std::thread::spawn(move || {
                                        info!("Launching hotkey command: {}", cmd_str);
                                        #[cfg(windows)]
                                        {
                                            let _ = std::process::Command::new("cmd")
                                                .args(["/C", &cmd_str])
                                                .spawn();
                                        }
                                    });
                                } else if let Some(node) = other.strip_prefix("selection_") {
                                    spawn_capture(&proxy, &cfg, node, "capsule");
                                } else if let Some(panel_name) = other.strip_prefix("toggle_") {
                                    let _ = proxy.send_event(AppEvent::TogglePanel(panel_name.into()));
                                } else {
                                    info!("Hotkey action: {}", other);
                                }
                            }
                        }
                    }
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
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                panel_mgr.hide_by_window_id(window_id);
            }

            _ => {}
        }

        // Suppress unused variable warning — the manager must stay alive
        // to keep hotkeys registered.
        let _ = &hotkey_manager;
    });
}

/// Handle an IPC message from a webview panel.
/// Capture the current selection on a worker thread, then open `target_panel`.
///
/// Nothing opens when there is no selection — a floating window over an empty
/// capture is just noise.
fn spawn_capture(
    proxy: &tao::event_loop::EventLoopProxy<AppEvent>,
    cfg: &Arc<Mutex<config::Config>>,
    node_type: &str,
    target_panel: &str,
) {
    let proxy = proxy.clone();
    let cfg = Arc::clone(cfg);
    let node_type = node_type.to_string();
    let target_panel = target_panel.to_string();

    std::thread::spawn(move || {
        let snapshot = { cfg.lock().unwrap().clone() };
        if !snapshot.selection.enabled {
            info!("selection: capture layer disabled in config");
            return;
        }

        selection::set_pending_node(&node_type);

        match selection::capture(&snapshot) {
            Some(_) => {
                let _ = proxy.send_event(AppEvent::OpenPanel(target_panel));
            }
            None => {
                // Clear the pending node so a later toolbar open does not
                // inherit an intent the user never completed.
                selection::take_pending_node();
            }
        }
    });
}

/// Capture if something is selected, then open the panel either way.
///
/// Stratum actions read `selection` but fall back to `clipboard`, so an empty
/// selection is a normal case here rather than a reason to show nothing.
fn spawn_capture_or_open(
    proxy: &tao::event_loop::EventLoopProxy<AppEvent>,
    cfg: &Arc<Mutex<config::Config>>,
    target_panel: &str,
) {
    let proxy = proxy.clone();
    let cfg = Arc::clone(cfg);
    let target_panel = target_panel.to_string();

    std::thread::spawn(move || {
        let snapshot = { cfg.lock().unwrap().clone() };
        if snapshot.selection.enabled {
            selection::capture(&snapshot);
        }
        let _ = proxy.send_event(AppEvent::OpenPanel(target_panel));
    });
}

/// Run one selection action. Returns the JS callback to fire in the panel.
fn run_selection_action(
    cfg: &Arc<Mutex<config::Config>>,
    proxy: &tao::event_loop::EventLoopProxy<AppEvent>,
    panel: &str,
    rule: config::SelectionRule,
    capture: selection::Capture,
    capsule_fields: serde_json::Value,
    request_id: String,
) {
    let cfg = Arc::clone(cfg);
    let proxy = proxy.clone();
    let panel = panel.to_string();

    // The reply closure owns its own proxy handle so the outer one stays
    // usable for actions that also need to raise a panel.
    let reply_proxy = proxy.clone();
    let reply = move |payload: serde_json::Value| {
        let script = format!(
            "onActionResult({}, {});",
            serde_json::to_string(&request_id).unwrap(),
            payload
        );
        let _ = reply_proxy.send_event(AppEvent::IpcMessage {
            panel: panel.clone(),
            body: serde_json::json!({ "type": "_eval", "script": script }).to_string(),
        });
    };

    match rule.action.as_str() {
        // Stage an immutable candidate with the local canon service.
        "canon" => {
            let node_type = if rule.node_type.is_empty() {
                "claim".to_string()
            } else {
                rule.node_type.clone()
            };
            std::thread::spawn(move || {
                let canon_cfg = { cfg.lock().unwrap().canon.clone() };
                let request = canon::CandidateRequest::from_capture(
                    &capture,
                    &node_type,
                    capsule_fields,
                    &canon_cfg.author,
                );

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let outcome =
                    rt.block_on(async { canon::submit_candidate(&canon_cfg, &request).await });

                reply(serde_json::to_value(&outcome).unwrap_or(serde_json::Value::Null));
            });
        }

        // Send the selection through a configured AI workflow.
        "ai" => {
            let workflow_name = rule.arg.clone();
            std::thread::spawn(move || {
                let (provider, workflow) = {
                    let c = cfg.lock().unwrap();
                    let workflow = c
                        .ai
                        .workflows
                        .iter()
                        .find(|w| w.name == workflow_name)
                        .cloned();
                    let provider_name = workflow
                        .as_ref()
                        .map(|w| w.provider.clone())
                        .unwrap_or_else(|| c.ai.default_provider.clone());
                    let provider = c
                        .ai
                        .providers
                        .iter()
                        .find(|p| p.name == provider_name || p.provider_type == provider_name)
                        .cloned();
                    (provider, workflow)
                };

                let provider = match provider {
                    Some(p) => p,
                    None => {
                        reply(serde_json::json!({
                            "state": "error",
                            "detail": format!("No AI provider configured for workflow '{}'", workflow_name),
                        }));
                        return;
                    }
                };

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let result = rt.block_on(async {
                    ai::chat(&provider, workflow.as_ref(), &[], &capture.text).await
                });

                match result {
                    Ok(text) => {
                        if rule.output == "replace" {
                            selection::replace_selection(&text);
                        } else if rule.output == "clipboard" {
                            selection::copy_result(&text);
                        }
                        reply(serde_json::json!({ "state": "ok", "text": text }));
                    }
                    Err(e) => reply(serde_json::json!({
                        "state": "error",
                        "detail": e.to_string(),
                    })),
                }
            });
        }

        // Hand off to a Stratum action module.
        "script" => {
            let action_id = rule.arg.clone();
            let output = rule.output.clone();
            std::thread::spawn(move || {
                let scripts_cfg = { cfg.lock().unwrap().scripts.clone() };
                let clipboard = selection::clipboard_text();
                match scripts::run_action(&scripts_cfg, &action_id, &capture.text, &clipboard) {
                    Ok(text) => {
                        if output == "replace" {
                            selection::replace_selection(&text);
                        } else if output == "clipboard" {
                            selection::copy_result(&text);
                        }
                        reply(serde_json::json!({ "state": "ok", "text": text }));
                    }
                    Err(e) => reply(serde_json::json!({
                        "state": "error",
                        "detail": e.to_string(),
                    })),
                }
            });
        }

        "tts" => {
            let text = capture.text.clone();
            std::thread::spawn(move || {
                if let Err(e) = tts::speak(&text) {
                    error!("selection: tts failed: {}", e);
                }
            });
            reply(serde_json::json!({ "state": "ok", "detail": "Speaking" }));
        }

        "copy" => {
            selection::copy_result(&capture.text);
            reply(serde_json::json!({ "state": "ok", "detail": "Copied" }));
        }

        "search" => {
            let url = build_search_url(&rule.arg, &capture.text);
            match open_url(&url) {
                Ok(()) => reply(serde_json::json!({ "state": "ok", "detail": url })),
                Err(e) => reply(serde_json::json!({
                    "state": "error",
                    "detail": format!("Could not open '{}': {}", url, e),
                })),
            }
        }

        "panel" => {
            let _ = proxy.send_event(AppEvent::OpenPanel(rule.arg.clone()));
            reply(serde_json::json!({ "state": "ok", "detail": rule.arg }));
        }

        other => {
            reply(serde_json::json!({
                "state": "error",
                "detail": format!("Unknown selection action '{}'", other),
            }));
        }
    }
}

/// Substitute the selection into a search template, percent-encoding it.
fn build_search_url(template: &str, query: &str) -> String {
    if template.is_empty() || template == "{{q}}" {
        // "Open" on a bare URL — normalize a scheme-less host.
        let q = query.trim();
        if q.starts_with("http://") || q.starts_with("https://") {
            return q.to_string();
        }
        return format!("https://{}", q);
    }
    template.replace("{{q}}", &percent_encode(query))
}

fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            b' ' => out.push('+'),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}

/// Open a URL in the default browser without going through a shell.
fn open_url(url: &str) -> std::io::Result<()> {
    // `cmd /c start` would interpret the URL; ShellExecute via `rundll32` keeps
    // the argument opaque.
    std::process::Command::new("rundll32.exe")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn()
        .map(|_| ())
}

/// Review-queue IPC. Returns true when the message was consumed.
///
/// The only mutating action reachable from here is `accept_to_candidate`,
/// which stops at `C2`. There is deliberately no admission action: promoting
/// to `C3_CANONICAL` is a signed individual act performed by the canon engine.
fn handle_review_ipc(
    panel_mgr: &mut panels::PanelManager,
    cfg: &Arc<Mutex<config::Config>>,
    msg: &serde_json::Value,
) -> bool {
    let root = { review::output_root(&cfg.lock().unwrap()) };

    match msg.get("action").and_then(|a| a.as_str()).unwrap_or("") {
        "reload" => {
            push_queue(panel_mgr, &root);
            true
        }
        "accept_to_candidate" => {
            let ids: Vec<String> = msg
                .get("ids")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            match review::accept_to_candidate(&root, &ids) {
                Ok(result) => {
                    info!(
                        "review: {} accepted to candidate, {} refused",
                        result.accepted.len(),
                        result.refused.len()
                    );
                    let json = serde_json::to_string(&result).unwrap_or_else(|_| "{}".into());
                    panel_mgr
                        .evaluate_script("review", &format!("window.acceptResult({})", json));
                }
                // A failure here is shown in the panel rather than only logged:
                // the user is looking at the queue, not the log.
                Err(e) => {
                    error!("review: accept failed: {}", e);
                    let payload = serde_json::json!({
                        "accepted": [],
                        "refused": [["batch", e.to_string()]],
                    });
                    panel_mgr.evaluate_script(
                        "review",
                        &format!("window.acceptResult({})", payload),
                    );
                }
            }
            true
        }
        _ => false,
    }
}

/// Load the queue and hand it to the panel.
fn push_queue(panel_mgr: &mut panels::PanelManager, root: &std::path::Path) {
    let queue = review::load(root).unwrap_or_else(|e| {
        error!("review: queue load failed: {}", e);
        review::Queue::default()
    });
    let json = serde_json::to_string(&queue).unwrap_or_else(|_| "{}".into());
    panel_mgr.evaluate_script("review", &format!("window.loadQueue({})", json));
}

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

    // The review panel uses "action" rather than "type", and is handled first
    // so its vocabulary cannot collide with the older panel messages.
    if panel == "review" {
        if handle_review_ipc(panel_mgr, cfg, &msg) {
            return;
        }
    }

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
                                description: hs
                                    .get("description")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .into(),
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
                drop(cfg_lock);

                // Push full config to Cloudflare (fire and forget)
                let push_cfg = Arc::clone(&cfg);
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async {
                        let _ = sync_client::push_config(&push_cfg).await;
                    });
                });

                info!("Config saved via IPC");
            }
        }

        "get_shortcuts" => {
            let cfg_lock = cfg.lock().unwrap();
            let mut items: Vec<serde_json::Value> = Vec::new();
            for hk in &cfg_lock.hotkeys {
                items.push(serde_json::json!({
                    "type": "hotkey",
                    "trigger": hk.keys,
                    "action": hk.action,
                    "output": hk.action,
                    "description": ""
                }));
            }
            for hs in &cfg_lock.hotstrings {
                items.push(serde_json::json!({
                    "type": "hotstring",
                    "trigger": hs.trigger,
                    "output": hs.expansion,
                    "description": hs.description
                }));
            }
            if let Ok(json) = serde_json::to_string(&items) {
                let script = format!("items = {}; renderLibrary();", json);
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "save_shortcuts" => {
            if let Some(items) = msg.get("items").and_then(|v| v.as_array()) {
                let mut cfg_lock = cfg.lock().unwrap();
                let mut new_hotkeys = Vec::new();
                let mut new_hotstrings = Vec::new();

                for it in items {
                    let it_type = it.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    let desc = it.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let output = it.get("output").and_then(|v| v.as_str()).unwrap_or("").to_string();

                    if it_type == "hotkey" {
                        let trigger = it.get("trigger").and_then(|v| v.as_str()).unwrap_or("");
                        let action = it.get("action").and_then(|v| v.as_str()).unwrap_or("");
                        let resolved_action = if action == "send_text" {
                            format!("send_text:{}", output)
                        } else if action == "run_command" {
                            format!("run_command:{}", output)
                        } else if !action.is_empty() {
                            action.to_string()
                        } else {
                            output
                        };

                        if !trigger.is_empty() {
                            new_hotkeys.push(config::HotkeyBinding {
                                keys: trigger.to_string(),
                                action: resolved_action,
                                runtime_id: None,
                            });
                        }
                    } else if it_type == "hotstring" {
                        let trigger = it.get("trigger").and_then(|v| v.as_str()).unwrap_or("");
                        if !trigger.is_empty() {
                            new_hotstrings.push(config::Hotstring {
                                trigger: trigger.to_string(),
                                description: desc,
                                expansion: output,
                                replace_trigger: true,
                            });
                        }
                    }
                }

                if !new_hotkeys.is_empty() {
                    cfg_lock.hotkeys = new_hotkeys;
                }
                cfg_lock.hotstrings = new_hotstrings;

                if let Err(e) = cfg_lock.save() {
                    error!("Failed to save shortcuts: {}", e);
                } else {
                    info!("Shortcuts saved via IPC ({} hotkeys, {} hotstrings)", cfg_lock.hotkeys.len(), cfg_lock.hotstrings.len());
                }
                drop(cfg_lock);

                // Notify event loop to rebuild hotkeys
                let _ = panel_mgr.proxy().send_event(AppEvent::ConfigReloaded);
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
                let _ = tts::speak("ClipSync text to speech is working.");
            });
        }

        "tts_stop" => {
            std::thread::spawn(|| {
                tts::stop();
            });
        }

        "speak_text" => {
            let text = msg.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let voice = msg.get("voice").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let engine = msg.get("engine").and_then(|v| v.as_str()).unwrap_or("sapi").to_string();
            let speed = msg.get("speed").and_then(|v| v.as_i64()).unwrap_or(2) as i32;
            let volume = msg.get("volume").and_then(|v| v.as_u64()).unwrap_or(100) as u32;
            std::thread::spawn(move || {
                tts::configure(&voice, speed, &engine, volume);
                let _ = tts::speak(&text);
            });
        }

        "tts_pause" => {
            tts::pause_toggle();
        }

        "tts_download" => {
            let text = msg.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let voice = msg.get("voice").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let engine = msg.get("engine").and_then(|v| v.as_str()).unwrap_or("sapi").to_string();
            let speed = msg.get("speed").and_then(|v| v.as_i64()).unwrap_or(2) as i32;
            let volume = msg.get("volume").and_then(|v| v.as_u64()).unwrap_or(100) as u32;
            std::thread::spawn(move || {
                tts::configure(&voice, speed, &engine, volume);
                let desktop = dirs::desktop_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
                let path = desktop.join("nerve_tts_output.wav");
                if let Err(e) = tts::save_audio(&text, &path.to_string_lossy()) {
                    error!("TTS download failed: {}", e);
                }
            });
        }

        // ── AI Chat ──────────────────────────────────────────────
        "ai_get_providers" => {
            let cfg_lock = cfg.lock().unwrap();
            let providers: Vec<serde_json::Value> = cfg_lock
                .ai
                .providers
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "name": p.name,
                        "provider_type": p.provider_type,
                        "model": p.model,
                        "endpoint": p.endpoint,
                        "enabled": p.enabled,
                        "has_key": !p.api_key.is_empty(),
                    })
                })
                .collect();
            let default = &cfg_lock.ai.default_provider;
            if let Ok(json) = serde_json::to_string(&providers) {
                let script = format!(
                    "onProvidersLoaded({}, {});",
                    json,
                    serde_json::to_string(default).unwrap_or_else(|_| "\"claude\"".into())
                );
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "ai_get_workflows" => {
            let cfg_lock = cfg.lock().unwrap();
            if let Ok(json) = serde_json::to_string(&cfg_lock.ai.workflows) {
                let script = format!("onWorkflowsLoaded({});", json);
                panel_mgr.evaluate_script(panel, &script);
            }
        }

        "ai_chat" => {
            // { type: "ai_chat", provider: "claude", workflow: "General Chat"|null,
            //   messages: [...], input: "user text", request_id: "xxx" }
            let provider_type = msg
                .get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("claude")
                .to_string();
            let workflow_name = msg
                .get("workflow")
                .and_then(|v| v.as_str())
                .map(String::from);
            let input = msg
                .get("input")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let request_id = msg
                .get("request_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Parse conversation history
            let history: Vec<ai::ChatMessage> = msg
                .get("messages")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|m| {
                            Some(ai::ChatMessage {
                                role: m.get("role")?.as_str()?.into(),
                                content: m.get("content")?.as_str()?.into(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let cfg_clone = Arc::clone(cfg);
            let panel_name = panel.to_string();
            let proxy_clone = panel_mgr.proxy().clone();

            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async {
                    let cfg_lock = cfg_clone.lock().unwrap();
                    let provider = cfg_lock
                        .ai
                        .providers
                        .iter()
                        .find(|p| p.provider_type == provider_type);
                    let workflow = workflow_name.as_ref().and_then(|name| {
                        cfg_lock.ai.workflows.iter().find(|w| w.name == *name)
                    });

                    let provider = match provider {
                        Some(p) => p.clone(),
                        None => {
                            let script = format!(
                                "onAiError({}, \"Provider '{}' not found\");",
                                serde_json::to_string(&request_id).unwrap(),
                                provider_type,
                            );
                            let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                                panel: panel_name,
                                body: serde_json::json!({
                                    "type": "_eval",
                                    "script": script,
                                })
                                .to_string(),
                            });
                            return;
                        }
                    };
                    let workflow = workflow.cloned();
                    drop(cfg_lock);

                    match ai::chat(&provider, workflow.as_ref(), &history, &input).await {
                        Ok(response) => {
                            let escaped = serde_json::to_string(&response).unwrap();
                            let req_escaped = serde_json::to_string(&request_id).unwrap();
                            let script =
                                format!("onAiResponse({}, {});", req_escaped, escaped);
                            let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                                panel: panel_name,
                                body: serde_json::json!({
                                    "type": "_eval",
                                    "script": script,
                                })
                                .to_string(),
                            });
                        }
                        Err(e) => {
                            let err_msg = format!("{}", e);
                            let req_escaped = serde_json::to_string(&request_id).unwrap();
                            let err_escaped = serde_json::to_string(&err_msg).unwrap();
                            let script =
                                format!("onAiError({}, {});", req_escaped, err_escaped);
                            let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                                panel: panel_name,
                                body: serde_json::json!({
                                    "type": "_eval",
                                    "script": script,
                                })
                                .to_string(),
                            });
                        }
                    }
                });
            });
        }

        "ai_save_providers" => {
            if let Some(providers) = msg.get("providers").and_then(|v| v.as_array()) {
                let mut cfg_lock = cfg.lock().unwrap();
                cfg_lock.ai.providers = providers
                    .iter()
                    .filter_map(|p| {
                        Some(config::AiProvider {
                            name: p.get("name")?.as_str()?.into(),
                            provider_type: p.get("provider_type")?.as_str()?.into(),
                            api_key: p.get("api_key").and_then(|v| v.as_str()).unwrap_or("").into(),
                            endpoint: p.get("endpoint").and_then(|v| v.as_str()).unwrap_or("").into(),
                            model: p.get("model").and_then(|v| v.as_str()).unwrap_or("").into(),
                            enabled: p.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
                        })
                    })
                    .collect();
                if let Some(default) = msg.get("default_provider").and_then(|v| v.as_str()) {
                    cfg_lock.ai.default_provider = default.into();
                }
                if let Err(e) = cfg_lock.save() {
                    error!("Failed to save AI config: {}", e);
                }
                info!("AI providers saved");
            }
        }

        "ai_save_workflows" => {
            if let Some(workflows) = msg.get("workflows").and_then(|v| v.as_array()) {
                let mut cfg_lock = cfg.lock().unwrap();
                cfg_lock.ai.workflows = workflows
                    .iter()
                    .filter_map(|w| {
                        Some(config::AiWorkflow {
                            name: w.get("name")?.as_str()?.into(),
                            description: w
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .into(),
                            provider: w
                                .get("provider")
                                .and_then(|v| v.as_str())
                                .unwrap_or("claude")
                                .into(),
                            system_prompt: w
                                .get("system_prompt")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .into(),
                            user_template: w
                                .get("user_template")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .into(),
                            max_tokens: w
                                .get("max_tokens")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(4096) as u32,
                            temperature: w
                                .get("temperature")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.7),
                        })
                    })
                    .collect();
                if let Err(e) = cfg_lock.save() {
                    error!("Failed to save workflows: {}", e);
                }
                info!("AI workflows saved");
            }
        }

        // Pin external window — user clicks pencil, then clicks target window
        "pin_external" => {
            let panel_name = panel.to_string();
            let proxy_clone = panel_mgr.proxy().clone();
            std::thread::spawn(move || {
                // Wait 1.5s for user to click on target window
                if let Some(result) = window_mgmt::pin_external_window(1500) {
                    let escaped = serde_json::to_string(&result).unwrap();
                    let script = format!("showToast({});", escaped);
                    let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                        panel: panel_name,
                        body: serde_json::json!({ "type": "_eval", "script": script }).to_string(),
                    });
                }
            });
        }

        // The toolbar and capsule both ask for the live capture plus the
        // actions that apply to it.
        "selection_get" => {
            let capture = selection::last_capture();
            let cfg_lock = cfg.lock().unwrap();
            let rules = selection::matching_rules(&cfg_lock, &capture);
            let canon_enabled = cfg_lock.canon.enabled;
            let fade_ms = cfg_lock.selection.toolbar_fade_ms;
            drop(cfg_lock);

            let payload = serde_json::json!({
                "capture": capture,
                "rules": rules,
                "pending_node": selection::take_pending_node(),
                "canon_enabled": canon_enabled,
                "fade_ms": fade_ms,
            });
            panel_mgr.evaluate_script(panel, &format!("onSelection({});", payload));
        }

        // Execute one toolbar/capsule action against the live capture.
        "selection_run" => {
            let rule_id = msg.get("rule").and_then(|v| v.as_str()).unwrap_or("");
            let request_id = msg
                .get("request_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let capsule_fields = msg
                .get("capsule")
                .cloned()
                .unwrap_or(serde_json::Value::Null);

            let rule = {
                let cfg_lock = cfg.lock().unwrap();
                cfg_lock
                    .selection
                    .rules
                    .iter()
                    .find(|r| r.id == rule_id)
                    .cloned()
            };

            match rule {
                Some(rule) => {
                    let capture = selection::last_capture();
                    let proxy = panel_mgr.proxy().clone();
                    run_selection_action(
                        cfg,
                        &proxy,
                        panel,
                        rule,
                        capture,
                        capsule_fields,
                        request_id,
                    );
                }
                None => {
                    let script = format!(
                        "onActionResult({}, {{\"state\":\"error\",\"detail\":\"Unknown rule\"}});",
                        serde_json::to_string(&request_id).unwrap()
                    );
                    panel_mgr.evaluate_script(panel, &script);
                }
            }
        }

        // Toolbar hands off to the capsule for a chosen node type.
        "selection_capsule" => {
            if let Some(node) = msg.get("node_type").and_then(|v| v.as_str()) {
                selection::set_pending_node(node);
            }
            panel_mgr.hide("toolbar");
            let _ = panel_mgr
                .proxy()
                .send_event(AppEvent::OpenPanel("capsule".into()));
        }

        // A floating surface dismissing itself (Escape, blur, or after an action).
        "selection_dismiss" => {
            let target = msg
                .get("panel")
                .and_then(|v| v.as_str())
                .unwrap_or(panel)
                .to_string();
            panel_mgr.hide(&target);
        }

        // Stratum's action catalogue, read live from its own actions.json.
        "stratum_list" => {
            let capture = selection::last_capture();
            let scripts_cfg = { cfg.lock().unwrap().scripts.clone() };
            let actions = scripts::list_actions(&scripts_cfg);
            info!(
                "stratum_list: {} action(s) from {}",
                actions.len(),
                scripts_cfg.stratum_root
            );

            let payload = serde_json::json!({
                "capture": capture,
                "clipboard": selection::clipboard_text(),
                "actions": actions,
                "enabled": scripts_cfg.enabled,
                "stratum_root": scripts_cfg.stratum_root,
            });
            panel_mgr.evaluate_script(panel, &format!("onStratum({});", payload));
        }

        // Run one Stratum action against the live capture or supplied text.
        "stratum_run" => {
            let action_id = msg
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let request_id = msg
                .get("request_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            // The panel may hand back edited text, so it wins over the capture.
            let text = msg
                .get("text")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_else(|| selection::last_capture().text);

            let scripts_cfg = { cfg.lock().unwrap().scripts.clone() };
            let panel_name = panel.to_string();
            let proxy_clone = panel_mgr.proxy().clone();

            std::thread::spawn(move || {
                let clipboard = selection::clipboard_text();
                let payload = match scripts::run_action(&scripts_cfg, &action_id, &text, &clipboard)
                {
                    Ok(result) => serde_json::json!({ "state": "ok", "text": result }),
                    Err(e) => serde_json::json!({ "state": "error", "detail": e.to_string() }),
                };
                let script = format!(
                    "onActionResult({}, {});",
                    serde_json::to_string(&request_id).unwrap(),
                    payload
                );
                let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                    panel: panel_name,
                    body: serde_json::json!({ "type": "_eval", "script": script }).to_string(),
                });
            });
        }

        // Copy arbitrary text from a panel to the clipboard.
        "copy_text" => {
            if let Some(text) = msg.get("text").and_then(|v| v.as_str()) {
                selection::copy_result(text);
            }
        }

        // Canon-session agenda, read-only.
        "canon_agenda" => {
            let request_id = msg
                .get("request_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let canon_cfg = { cfg.lock().unwrap().canon.clone() };
            let panel_name = panel.to_string();
            let proxy_clone = panel_mgr.proxy().clone();

            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let payload = match rt.block_on(canon::fetch_agenda(&canon_cfg)) {
                    Ok(v) => serde_json::json!({ "state": "ok", "agenda": v }),
                    Err(e) => serde_json::json!({ "state": "error", "detail": e.to_string() }),
                };
                let script = format!(
                    "onAgenda({}, {});",
                    serde_json::to_string(&request_id).unwrap(),
                    payload
                );
                let _ = proxy_clone.send_event(AppEvent::IpcMessage {
                    panel: panel_name,
                    body: serde_json::json!({ "type": "_eval", "script": script }).to_string(),
                });
            });
        }

        // Internal: evaluate script from async callback
        "_eval" => {
            if let Some(script) = msg.get("script").and_then(|v| v.as_str()) {
                panel_mgr.evaluate_script(panel, script);
            }
        }

        _ => {
            info!("Unknown IPC message type: '{}'", msg_type);
        }
    }
}
