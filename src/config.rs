use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyBinding {
    pub keys: String,
    pub action: String,
    #[serde(skip)]
    pub runtime_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotstring {
    pub trigger: String,
    pub expansion: String,
    #[serde(default = "default_true")]
    pub replace_trigger: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub template: String,
    pub shortcut: Option<String>,
    #[serde(default)]
    pub replace: bool,
    #[serde(default)]
    pub popup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelDef {
    pub name: String,
    pub title: String,
    pub url: String,
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    #[serde(default)]
    pub always_on_top: bool,
}

fn default_width() -> u32 {
    480
}
fn default_height() -> u32 {
    720
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipSlot {
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    #[serde(default = "default_tts_voice")]
    pub voice: String,
    #[serde(default = "default_tts_speed")]
    pub speed: i32,
    #[serde(default = "default_tts_engine")]
    pub engine: String,
    #[serde(default = "default_tts_volume")]
    pub volume: u32,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            voice: default_tts_voice(),
            speed: default_tts_speed(),
            engine: default_tts_engine(),
            volume: default_tts_volume(),
        }
    }
}

fn default_tts_voice() -> String {
    "Brian".into()
}
fn default_tts_speed() -> i32 {
    2
}
fn default_tts_engine() -> String {
    "sapi".into()
}
fn default_tts_volume() -> u32 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default)]
    pub api_token: String,
    #[serde(default = "default_clip_interval")]
    pub clipboard_interval_ms: u64,
    #[serde(default = "default_sync_interval")]
    pub sync_interval_secs: u64,

    #[serde(default = "default_hotkeys")]
    pub hotkeys: Vec<HotkeyBinding>,
    #[serde(default)]
    pub hotstrings: Vec<Hotstring>,
    #[serde(default)]
    pub prompts: Vec<Prompt>,
    #[serde(default = "default_panels")]
    pub panels: Vec<PanelDef>,
    #[serde(default)]
    pub clip_slots: Vec<ClipSlot>,

    #[serde(default)]
    pub tts: TtsConfig,

    /// Runtime: hotkey ID → action name mapping (not persisted)
    #[serde(skip)]
    pub hotkey_map: HashMap<u32, String>,
}

fn default_api_url() -> String {
    "https://prophecy-intel-api.lowes-workers.workers.dev".into()
}
fn default_clip_interval() -> u64 {
    500
}
fn default_sync_interval() -> u64 {
    300
}

fn default_hotkeys() -> Vec<HotkeyBinding> {
    vec![
        HotkeyBinding {
            keys: "Ctrl+Alt+C".into(),
            action: "toggle_clipboard".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+P".into(),
            action: "toggle_prompts".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+L".into(),
            action: "toggle_links".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+R".into(),
            action: "toggle_research".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+A".into(),
            action: "toggle_chat".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+T".into(),
            action: "tts_read_selection".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+G".into(),
            action: "toggle_dashboard".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+S".into(),
            action: "toggle_settings".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+1".into(),
            action: "paste_slot_1".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+2".into(),
            action: "paste_slot_2".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+3".into(),
            action: "paste_slot_3".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+4".into(),
            action: "paste_slot_4".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+5".into(),
            action: "paste_slot_5".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+6".into(),
            action: "paste_slot_6".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+7".into(),
            action: "paste_slot_7".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+8".into(),
            action: "paste_slot_8".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+9".into(),
            action: "paste_slot_9".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+0".into(),
            action: "paste_slot_10".into(),
            runtime_id: None,
        },
    ]
}

fn default_panels() -> Vec<PanelDef> {
    // Resolve HTML directory relative to executable
    let html_dir = Config::html_dir();
    let to_url = |file: &str| -> String {
        let path = html_dir.join(file);
        if path.exists() {
            format!("file:///{}", path.to_string_lossy().replace('\\', "/"))
        } else {
            "about:blank".into()
        }
    };

    vec![
        PanelDef {
            name: "clipboard".into(),
            title: "ClipSync".into(),
            url: to_url("clipboard.html"),
            width: 480,
            height: 720,
            x: None,
            y: None,
            always_on_top: true,
        },
        PanelDef {
            name: "prompts".into(),
            title: "Prompts".into(),
            url: to_url("prompts.html"),
            width: 520,
            height: 680,
            x: None,
            y: None,
            always_on_top: true,
        },
        PanelDef {
            name: "links".into(),
            title: "Links".into(),
            url: to_url("links.html"),
            width: 500,
            height: 680,
            x: None,
            y: None,
            always_on_top: true,
        },
        PanelDef {
            name: "research".into(),
            title: "Research".into(),
            url: to_url("research.html"),
            width: 600,
            height: 720,
            x: None,
            y: None,
            always_on_top: true,
        },
        PanelDef {
            name: "chat".into(),
            title: "AI Chat".into(),
            url: to_url("chat.html"),
            width: 500,
            height: 700,
            x: None,
            y: None,
            always_on_top: false,
        },
        PanelDef {
            name: "dashboard".into(),
            title: "Dashboard".into(),
            url: to_url("dashboard.html"),
            width: 900,
            height: 700,
            x: None,
            y: None,
            always_on_top: false,
        },
        PanelDef {
            name: "settings".into(),
            title: "Settings".into(),
            url: to_url("settings.html"),
            width: 560,
            height: 680,
            x: None,
            y: None,
            always_on_top: true,
        },
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
            api_token: String::new(),
            clipboard_interval_ms: default_clip_interval(),
            sync_interval_secs: default_sync_interval(),
            hotkeys: default_hotkeys(),
            hotstrings: vec![],
            prompts: vec![],
            panels: default_panels(),
            clip_slots: vec![],
            tts: TtsConfig::default(),
            hotkey_map: HashMap::new(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("clipsync-agent");
        std::fs::create_dir_all(&dir).ok();
        dir.join("config.json")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            let mut cfg: Config = serde_json::from_str(&data)?;
            cfg.normalize();
            Ok(cfg)
        } else {
            let mut cfg = Config::default();
            cfg.normalize();
            cfg.save()?;
            Ok(cfg)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, data)?;
        Ok(())
    }

    pub fn hotkey_action(&self, id: u32) -> Option<String> {
        self.hotkey_map.get(&id).cloned()
    }

    pub fn panel(&self, name: &str) -> Option<&PanelDef> {
        self.panels.iter().find(|p| p.name == name)
    }

    pub fn update_panel_position(&mut self, name: &str, x: i32, y: i32) {
        if let Some(panel) = self.panels.iter_mut().find(|p| p.name == name) {
            panel.x = Some(x);
            panel.y = Some(y);
        }
    }

    /// HTML directory next to the executable
    pub fn html_dir() -> PathBuf {
        let exe = std::env::current_exe().unwrap_or_default();
        exe.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("html")
    }

    /// Ensure required defaults exist in older configs.
    pub fn normalize(&mut self) {
        let default_cfg = Config::default();

        // Keep existing user bindings, but backfill missing required actions.
        for required in default_cfg.hotkeys {
            let exists = self.hotkeys.iter().any(|h| h.action == required.action);
            if !exists {
                self.hotkeys.push(required);
            }
        }

        // Ensure all core panels exist so tray/hotkeys always open something.
        for required in default_cfg.panels {
            let exists = self.panels.iter().any(|p| p.name == required.name);
            if !exists {
                self.panels.push(required);
            }
        }
    }
}
