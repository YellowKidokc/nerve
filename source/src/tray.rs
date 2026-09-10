use crate::AppEvent;
use anyhow::Result;
use tao::event_loop::EventLoopProxy;
use tracing::info;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

/// Create the system tray icon with menu
pub fn create_tray(proxy: &EventLoopProxy<AppEvent>) -> Result<TrayIcon> {
    let menu = Menu::new();

    let item_clipboard = MenuItem::new("Clipboard  (Ctrl+Alt+C)", true, None);
    let item_prompts = MenuItem::new("Prompts    (Ctrl+Alt+P)", true, None);
    let item_links = MenuItem::new("Links      (Ctrl+Alt+L)", true, None);
    let item_research = MenuItem::new("Research   (Ctrl+Alt+R)", true, None);
    let item_chat = MenuItem::new("AI Chat    (Ctrl+Alt+A)", true, None);
    let item_tts_read = MenuItem::new("TTS Read     (Ctrl+Alt+T)", true, None);
    let item_tts_stop = MenuItem::new("TTS Stop", true, None);
    let item_tts_panel = MenuItem::new("TTS Engine", true, None);
    let item_shortcuts = MenuItem::new("Shortcuts & Hotstrings (Ctrl+Alt+S)", true, None);
    let item_mission_control = MenuItem::new("Mission Control   (Ctrl+Alt+M)", true, None);
    let item_workbench = MenuItem::new("Canon Workbench   (Ctrl+Alt+U)", true, None);
    let item_atom_builder = MenuItem::new("Axiom Builder     (Ctrl+Alt+B)", true, None);
    let item_lean_registry = MenuItem::new("Lean 4 Registry   (Port 8989)", true, None);
    let item_reconciliation = MenuItem::new("Reconciliation    (Ctrl+Alt+N)", true, None);
    let item_stratum = MenuItem::new("Stratum Actions   (Ctrl+Shift+V)", true, None);
    let item_quit = MenuItem::new("Quit", true, None);

    menu.append(&item_shortcuts)?;
    menu.append(&item_mission_control)?;
    menu.append(&item_clipboard)?;
    menu.append(&item_prompts)?;
    menu.append(&item_links)?;
    menu.append(&item_research)?;
    menu.append(&item_chat)?;
    menu.append(&tray_icon::menu::PredefinedMenuItem::separator())?;
    menu.append(&item_tts_read)?;
    menu.append(&item_tts_stop)?;
    menu.append(&item_tts_panel)?;
    menu.append(&tray_icon::menu::PredefinedMenuItem::separator())?;
    menu.append(&item_workbench)?;
    menu.append(&item_atom_builder)?;
    menu.append(&item_lean_registry)?;
    menu.append(&item_reconciliation)?;
    menu.append(&item_stratum)?;
    menu.append(&tray_icon::menu::PredefinedMenuItem::separator())?;
    menu.append(&item_quit)?;

    let shortcuts_id = item_shortcuts.id().clone();
    let mission_control_id = item_mission_control.id().clone();
    let clipboard_id = item_clipboard.id().clone();
    let prompts_id = item_prompts.id().clone();
    let links_id = item_links.id().clone();
    let research_id = item_research.id().clone();
    let chat_id = item_chat.id().clone();
    let tts_read_id = item_tts_read.id().clone();
    let tts_stop_id = item_tts_stop.id().clone();
    let tts_panel_id = item_tts_panel.id().clone();
    let workbench_id = item_workbench.id().clone();
    let atom_builder_id = item_atom_builder.id().clone();
    let lean_registry_id = item_lean_registry.id().clone();
    let reconciliation_id = item_reconciliation.id().clone();
    let stratum_id = item_stratum.id().clone();
    let quit_id = item_quit.id().clone();

    let proxy_clone = proxy.clone();
    std::thread::spawn(move || loop {
        if let Ok(event) = MenuEvent::receiver().recv() {
            if event.id == shortcuts_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("shortcuts".into()));
            } else if event.id == mission_control_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("mission-control".into()));
            } else if event.id == clipboard_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("clipboard".into()));
            } else if event.id == prompts_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("prompts".into()));
            } else if event.id == links_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("links".into()));
            } else if event.id == research_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("research".into()));
            } else if event.id == chat_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("chat".into()));
            } else if event.id == tts_read_id {
                let read_proxy = proxy_clone.clone();
                std::thread::spawn(move || {
                    if let Some(text) = crate::tts::read_selection() {
                        let _ = read_proxy.send_event(crate::AppEvent::TtsTextRead(text));
                    }
                });
            } else if event.id == tts_stop_id {
                std::thread::spawn(|| { crate::tts::stop(); });
            } else if event.id == tts_panel_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("tts".into()));
            } else if event.id == workbench_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("workbench".into()));
            } else if event.id == atom_builder_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("atom-builder".into()));
            } else if event.id == lean_registry_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("lean-registry".into()));
            } else if event.id == reconciliation_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("reconciliation".into()));
            } else if event.id == stratum_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("stratum".into()));
            } else if event.id == quit_id {
                let _ = proxy_clone.send_event(AppEvent::Quit);
            }
        }
    });

    let icon = create_icon();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("ClipSync Agent")
        .with_icon(icon)
        .build()?;

    info!("System tray created");
    Ok(tray)
}

/// Create a simple 16x16 RGBA icon (cyan "C" on dark background)
fn create_icon() -> tray_icon::Icon {
    let size = 16u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            rgba[idx] = 0x1a;
            rgba[idx + 1] = 0x1a;
            rgba[idx + 2] = 0x2e;
            rgba[idx + 3] = 0xff;

            let in_c = (y >= 3 && y <= 12)
                && ((x >= 3 && x <= 5)
                    || (y >= 3 && y <= 5 && x >= 3 && x <= 11)
                    || (y >= 10 && y <= 12 && x >= 3 && x <= 11));

            if in_c {
                rgba[idx] = 0x60;
                rgba[idx + 1] = 0xd0;
                rgba[idx + 2] = 0xff;
            }
        }
    }

    tray_icon::Icon::from_rgba(rgba, size, size).expect("Failed to create tray icon")
}
