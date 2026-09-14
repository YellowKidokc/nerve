from pathlib import Path

config_path = Path("src/config.rs")
text = config_path.read_text(encoding="utf-8")

target = """        PanelDef {
            name: "atom-builder".into(),
            title: "Claim Atom Builder".into(),
            url: to_url("atoms/atom-builder.html"),
            width: 1100,
            height: 860,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
        },"""

replacement = """        PanelDef {
            name: "atom-builder".into(),
            title: "Axiom and Claim Builder — Consilience Atlas".into(),
            url: to_url("atoms/axiom-builder.html"),
            width: 1400,
            height: 920,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
        },
        PanelDef {
            name: "lean-registry".into(),
            title: "Lean 4 Atom Registry and Verification Explorer".into(),
            url: "http://127.0.0.1:8989/".into(),
            width: 1400,
            height: 920,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
        },"""

assert target in text, "Target block not found in config.rs"
text = text.replace(target, replacement, 1)
config_path.write_text(text, encoding="utf-8")
print("Updated config.rs successfully")

# Also update tray.rs
tray_path = Path("src/tray.rs")
tray_text = tray_path.read_text(encoding="utf-8")

old_tray_decl = """    let item_workbench = MenuItem::new("Canon Workbench   (Ctrl+Alt+U)", true, None);
    let item_atom_builder = MenuItem::new("Atom Builder      (Ctrl+Alt+B)", true, None);"""

new_tray_decl = """    let item_workbench = MenuItem::new("Canon Workbench   (Ctrl+Alt+U)", true, None);
    let item_atom_builder = MenuItem::new("Axiom Builder     (Ctrl+Alt+B)", true, None);
    let item_lean_registry = MenuItem::new("Lean 4 Registry   (Port 8989)", true, None);"""

assert old_tray_decl in tray_text, "Old tray decl not found in tray.rs"
tray_text = tray_text.replace(old_tray_decl, new_tray_decl, 1)

old_tray_append = """    menu.append(&item_workbench)?;
    menu.append(&item_atom_builder)?;"""

new_tray_append = """    menu.append(&item_workbench)?;
    menu.append(&item_atom_builder)?;
    menu.append(&item_lean_registry)?;"""

assert old_tray_append in tray_text, "Old tray append not found in tray.rs"
tray_text = tray_text.replace(old_tray_append, new_tray_append, 1)

old_tray_id = """    let workbench_id = item_workbench.id().clone();
    let atom_builder_id = item_atom_builder.id().clone();"""

new_tray_id = """    let workbench_id = item_workbench.id().clone();
    let atom_builder_id = item_atom_builder.id().clone();
    let lean_registry_id = item_lean_registry.id().clone();"""

assert old_tray_id in tray_text, "Old tray id not found in tray.rs"
tray_text = tray_text.replace(old_tray_id, new_tray_id, 1)

old_tray_event = """            } else if event.id == atom_builder_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("atom-builder".into()));"""

new_tray_event = """            } else if event.id == atom_builder_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("atom-builder".into()));
            } else if event.id == lean_registry_id {
                let _ = proxy_clone.send_event(AppEvent::TogglePanel("lean-registry".into()));"""

assert old_tray_event in tray_text, "Old tray event not found in tray.rs"
tray_text = tray_text.replace(old_tray_event, new_tray_event, 1)

tray_path.write_text(tray_text, encoding="utf-8")
print("Updated tray.rs successfully")
