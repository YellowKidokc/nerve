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
    #[serde(default)]
    pub description: String,
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
pub struct AiProvider {
    pub name: String,
    pub provider_type: String, // "claude", "openai", "local"
    pub api_key: String,
    #[serde(default)]
    pub endpoint: String, // custom endpoint (required for "local")
    #[serde(default = "default_ai_model")]
    pub model: String,
    #[serde(default)]
    pub enabled: bool,
}

fn default_ai_model() -> String {
    String::new()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiWorkflow {
    pub name: String,
    pub description: String,
    pub provider: String, // which provider to use
    pub system_prompt: String,
    #[serde(default)]
    pub user_template: String, // template with {{input}} placeholder
    #[serde(default = "default_ai_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_ai_temperature")]
    pub temperature: f64,
}

fn default_ai_max_tokens() -> u32 {
    4096
}
fn default_ai_temperature() -> f64 {
    0.7
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    #[serde(default)]
    pub providers: Vec<AiProvider>,
    #[serde(default)]
    pub workflows: Vec<AiWorkflow>,
    #[serde(default = "default_ai_provider")]
    pub default_provider: String,
}

fn default_ai_provider() -> String {
    "claude".into()
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                AiProvider {
                    name: "Claude".into(),
                    provider_type: "claude".into(),
                    api_key: String::new(),
                    endpoint: "https://api.anthropic.com/v1/messages".into(),
                    model: "claude-sonnet-4-20250514".into(),
                    enabled: true,
                },
                AiProvider {
                    name: "OpenAI".into(),
                    provider_type: "openai".into(),
                    api_key: String::new(),
                    endpoint: "https://api.openai.com/v1/chat/completions".into(),
                    model: "gpt-4o".into(),
                    enabled: true,
                },
                AiProvider {
                    name: "Local".into(),
                    provider_type: "local".into(),
                    api_key: String::new(),
                    endpoint: "http://localhost:1234/v1/chat/completions".into(),
                    model: String::new(),
                    enabled: false,
                },
            ],
            workflows: default_ai_workflows(),
            default_provider: "claude".into(),
        }
    }
}

fn default_ai_workflows() -> Vec<AiWorkflow> {
    vec![
        AiWorkflow {
            name: "General Chat".into(),
            description: "Open-ended conversation".into(),
            provider: "claude".into(),
            system_prompt: "You are a helpful assistant.".into(),
            user_template: String::new(),
            max_tokens: 4096,
            temperature: 0.7,
        },
        AiWorkflow {
            name: "Summarize".into(),
            description: "Lossless summary of input text".into(),
            provider: "claude".into(),
            system_prompt: "You produce precise, lossless summaries. Preserve all key facts, numbers, names, and logical structure. Do not editorialize.".into(),
            user_template: "Summarize the following text:\n\n{{input}}".into(),
            max_tokens: 4096,
            temperature: 0.3,
        },
        AiWorkflow {
            name: "Extract Links".into(),
            description: "Pull all URLs and references from text".into(),
            provider: "claude".into(),
            system_prompt: "Extract every URL, file path, and reference from the input. Return them as a clean list.".into(),
            user_template: "{{input}}".into(),
            max_tokens: 2048,
            temperature: 0.0,
        },
    ]
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

    #[serde(default)]
    pub ai: AiConfig,

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
            width: 380,
            height: 1000,
            x: None,
            y: Some(0),
            always_on_top: true,
        },
        PanelDef {
            name: "prompts".into(),
            title: "Prompts".into(),
            url: to_url("prompt_picker.html"),
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
        PanelDef {
            name: "tts".into(),
            title: "TTS Engine".into(),
            url: to_url("tts-engine.html"),
            width: 480,
            height: 600,
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
            ai: AiConfig::default(),
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
        let aliases = match name {
            // Support both historical panel names so tray items and saved configs
            // can interoperate across installs.
            "tts" => &["tts", "tts_engine"][..],
            "tts_engine" => &["tts_engine", "tts"][..],
            _ => std::slice::from_ref(&name),
        };

        self.panels
            .iter()
            .find(|p| aliases.iter().any(|alias| p.name == *alias))
    }

    pub fn update_panel_position(&mut self, name: &str, x: i32, y: i32) {
        let aliases = match name {
            "tts" => &["tts", "tts_engine"][..],
            "tts_engine" => &["tts_engine", "tts"][..],
            _ => std::slice::from_ref(&name),
        };

        if let Some(panel) = self
            .panels
            .iter_mut()
            .find(|p| aliases.iter().any(|alias| p.name == *alias))
        {
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
