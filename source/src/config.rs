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
    /// Window chrome. Floating capture surfaces (toolbar, capsule) run frameless.
    #[serde(default = "default_true")]
    pub decorations: bool,
    /// Open centred on the mouse cursor instead of at x/y.
    #[serde(default)]
    pub follow_cursor: bool,
    /// Open this panel automatically when the agent starts.
    #[serde(default)]
    pub open_at_startup: bool,
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
    // Match the full name, not "Brian": a bare "Brian" also matches
    // "Brian Online (Natural)", which is a different voice.
    "BrianMultilingual".into()
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
                    name: "Ollama Desktop".into(),
                    provider_type: "ollama".into(),
                    api_key: String::new(),
                    endpoint: "http://127.0.0.1:11434/v1/chat/completions".into(),
                    model: "qwen3:4b-instruct".into(),
                    enabled: true,
                },
                AiProvider {
                    name: "Ollama NAS".into(),
                    provider_type: "ollama".into(),
                    api_key: String::new(),
                    endpoint: "https://ollama.dlowehomelab.com/v1/chat/completions".into(),
                    model: "llama3.2:latest".into(),
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

// -- Selection capture layer ------------------------------------------------
// Nerve is the capture/display front end for the Canonical Content OS.
// Canon logic lives behind the local canon API - never in this process.

/// What a captured selection looks like, used to decide which actions appear.
/// `kinds` are recognizer names; `contains` is a literal test since we
/// deliberately avoid pulling in a regex engine for the first slice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionMatcher {
    /// Recognizer names: any / url / email / number / scripture / math / code / path / claimish
    #[serde(default)]
    pub kinds: Vec<String>,
    /// Optional literal substring the selection must contain (case-insensitive).
    #[serde(default)]
    pub contains: String,
    /// Bounds on selection length; 0 means unbounded.
    #[serde(default)]
    pub min_len: usize,
    #[serde(default)]
    pub max_len: usize,
}

impl Default for SelectionMatcher {
    fn default() -> Self {
        Self {
            kinds: vec!["any".into()],
            contains: String::new(),
            min_len: 0,
            max_len: 0,
        }
    }
}

/// One button on the floating toolbar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionRule {
    /// Stable id used by the toolbar IPC.
    pub id: String,
    /// Button label.
    pub label: String,
    /// Semantic node glyph from NODE_ICON_ASSIGNMENTS.
    #[serde(default)]
    pub glyph: String,
    /// Node type key (claim, evidence, definition, mathematics, story, bridge).
    /// Empty for non-canon utility actions.
    #[serde(default)]
    pub node_type: String,
    /// Action kind: canon / ai / tts / search / copy / panel
    pub action: String,
    /// Action argument - AI workflow name, search URL template with {{q}},
    /// panel name, or canon capture route.
    #[serde(default)]
    pub arg: String,
    /// Output: capsule / replace / clipboard / none
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default)]
    pub matcher: SelectionMatcher,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_output() -> String {
    "capsule".into()
}

/// Local Canonical Content OS service. Nerve only ever POSTs candidates and
/// GETs agendas - it holds no promotion credential by design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonConfig {
    #[serde(default = "default_canon_url")]
    pub base_url: String,
    /// Route that accepts an immutable candidate.
    #[serde(default = "default_candidate_route")]
    pub candidate_route: String,
    /// Route that returns the canon-session agenda.
    #[serde(default = "default_agenda_route")]
    pub agenda_route: String,
    /// Recorded as the candidate's author. Not a credential and not an
    /// authorization - the engine's promotion token is separate and Nerve
    /// never holds it.
    #[serde(default = "default_canon_author")]
    pub author: String,
    #[serde(default)]
    pub enabled: bool,
}

fn default_canon_author() -> String {
    std::env::var("USERNAME").unwrap_or_default()
}

fn default_canon_url() -> String {
    "http://127.0.0.1:8765".into()
}
fn default_candidate_route() -> String {
    "/candidate".into()
}
fn default_agenda_route() -> String {
    "/agenda".into()
}

impl Default for CanonConfig {
    fn default() -> Self {
        Self {
            base_url: default_canon_url(),
            candidate_route: default_candidate_route(),
            agenda_route: default_agenda_route(),
            author: default_canon_author(),
            enabled: false,
        }
    }
}

/// Mouse trigger for the selection surfaces.
///
/// Stratum's AHK glue used middle-click for its popup, so that is the default
/// here. Side buttons (`x1`/`x2`) are usually a better choice: middle-click
/// already means "close tab" in a browser and "autoscroll" elsewhere, so
/// `swallow` has to take it away from those apps to avoid doing both.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseTrigger {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// middle / x1 / x2. An unrecognised name disables the trigger.
    #[serde(default = "default_mouse_button")]
    pub button: String,
    /// Optional held modifier: "" / ctrl / alt / shift.
    #[serde(default)]
    pub modifier: String,
    /// How many clicks fire the trigger. 2 means double-click, which keeps a
    /// single click from opening the popup by accident.
    #[serde(default = "default_clicks")]
    pub clicks: u32,
    /// Window for counting a double-click. 0 uses the system setting.
    #[serde(default)]
    pub double_window_ms: u32,
    /// Stop the click reaching the app underneath. With middle-click this is
    /// what prevents a browser closing a tab at the same time.
    #[serde(default = "default_true")]
    pub swallow: bool,
    /// Which surface opens: toolbar / stratum / capsule.
    #[serde(default = "default_mouse_target")]
    pub target: String,
}

fn default_mouse_button() -> String {
    "middle".into()
}

fn default_clicks() -> u32 {
    2
}

fn default_mouse_target() -> String {
    "toolbar".into()
}

impl Default for MouseTrigger {
    fn default() -> Self {
        Self {
            enabled: true,
            button: default_mouse_button(),
            modifier: String::new(),
            clicks: default_clicks(),
            double_window_ms: 0,
            swallow: true,
            target: default_mouse_target(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Milliseconds to wait after Ctrl+C before reading the clipboard.
    #[serde(default = "default_capture_delay")]
    pub capture_delay_ms: u64,
    /// Restore the user's previous clipboard after capturing.
    #[serde(default = "default_true")]
    pub preserve_clipboard: bool,
    #[serde(default = "default_selection_rules")]
    pub rules: Vec<SelectionRule>,
    #[serde(default)]
    pub mouse: MouseTrigger,
    /// Auto-dismiss the floating toolbar after this long untouched.
    /// 0 disables the timeout. Modelled on ExtraClipboard's fading popup:
    /// a quick-pick surface that lingers is just clutter.
    #[serde(default = "default_fade_ms")]
    pub toolbar_fade_ms: u64,
}

fn default_fade_ms() -> u64 {
    4000
}

fn default_capture_delay() -> u64 {
    140
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            capture_delay_ms: default_capture_delay(),
            preserve_clipboard: true,
            rules: default_selection_rules(),
            mouse: MouseTrigger::default(),
            toolbar_fade_ms: default_fade_ms(),
        }
    }
}

fn default_selection_rules() -> Vec<SelectionRule> {
    let canon = |id: &str, label: &str, glyph: &str, node: &str| SelectionRule {
        id: id.into(),
        label: label.into(),
        glyph: glyph.into(),
        node_type: node.into(),
        action: "canon".into(),
        arg: "candidate".into(),
        output: "capsule".into(),
        matcher: SelectionMatcher {
            kinds: vec!["any".into()],
            contains: String::new(),
            min_len: 3,
            max_len: 0,
        },
        enabled: true,
    };

    vec![
        canon("claim", "Claim", "\u{25c6}", "claim"),
        canon("evidence", "Evidence", "E", "evidence"),
        canon("definition", "Definition", "\u{225d}", "definition"),
        canon("mathematics", "Mathematics", "\u{222b}", "mathematics"),
        canon("story", "Story", "\u{270d}", "story"),
        canon("bridge", "Bridge", "\u{2245}", "bridge"),
        SelectionRule {
            id: "stratum_panel".into(),
            label: "Actions".into(),
            glyph: "\u{2318}".into(),
            node_type: String::new(),
            action: "panel".into(),
            arg: "stratum".into(),
            output: "none".into(),
            matcher: SelectionMatcher::default(),
            enabled: true,
        },
        SelectionRule {
            id: "speak".into(),
            label: "Speak".into(),
            glyph: "\u{1f50a}".into(),
            node_type: String::new(),
            action: "tts".into(),
            arg: String::new(),
            output: "none".into(),
            matcher: SelectionMatcher::default(),
            enabled: true,
        },
        SelectionRule {
            id: "copy".into(),
            label: "Copy".into(),
            glyph: "\u{29c9}".into(),
            node_type: String::new(),
            action: "copy".into(),
            arg: String::new(),
            output: "clipboard".into(),
            matcher: SelectionMatcher::default(),
            enabled: true,
        },
        SelectionRule {
            id: "open_url".into(),
            label: "Open".into(),
            glyph: "\u{2197}".into(),
            node_type: String::new(),
            action: "search".into(),
            arg: "{{q}}".into(),
            output: "none".into(),
            matcher: SelectionMatcher {
                kinds: vec!["url".into()],
                contains: String::new(),
                min_len: 0,
                max_len: 0,
            },
            enabled: true,
        },
        SelectionRule {
            id: "web_search".into(),
            label: "Search".into(),
            glyph: "\u{2315}".into(),
            node_type: String::new(),
            action: "search".into(),
            arg: "https://duckduckgo.com/?q={{q}}".into(),
            output: "none".into(),
            matcher: SelectionMatcher {
                kinds: vec!["any".into()],
                contains: String::new(),
                min_len: 0,
                max_len: 300,
            },
            enabled: true,
        },
    ]
}


/// Idle Canon Worker settings.
///
/// Every field here is a *bound*, not a capability. The worker's authority
/// ceiling is `C0_SIDECAR`; nothing in this struct can raise it, and there is
/// deliberately no field that could. Defaults are chosen so that a user who
/// enables the feature without configuring it does nothing at all: with no
/// approved roots and no output root, preflight halts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdleWorkerConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Continuous idle time before a run may start.
    #[serde(default = "default_idle_minutes")]
    pub idle_minutes: u64,
    /// User activity always stops a run. Exposed so the acceptance test can
    /// assert it is true, not so it can be turned off.
    #[serde(default = "default_true")]
    pub stop_on_user_activity: bool,
    #[serde(default = "default_max_run_minutes")]
    pub max_run_minutes: u64,
    #[serde(default = "default_max_files_per_run")]
    pub max_files_per_run: usize,
    #[serde(default = "default_max_bytes_per_file")]
    pub max_bytes_per_file: u64,
    /// Absolute paths the worker may read. Empty means the worker never runs.
    #[serde(default)]
    pub approved_roots: Vec<String>,
    /// Absolute path the worker may write. Must not lie inside a source root.
    #[serde(default)]
    pub output_root: String,
    /// Minimum free bytes on the output volume before a run may start.
    #[serde(default = "default_min_free_bytes")]
    pub min_free_bytes: u64,
    /// File extensions eligible for deterministic capture.
    #[serde(default = "default_idle_extensions")]
    pub extensions: Vec<String>,
    /// Reserved for implementation order step 7. Reading this flag is the only
    /// thing the current build does with it.
    #[serde(default)]
    pub ollama_enabled: bool,
    #[serde(default)]
    pub ollama_model: String,
}

fn default_idle_minutes() -> u64 {
    60
}
fn default_max_run_minutes() -> u64 {
    45
}
fn default_max_files_per_run() -> usize {
    50
}
fn default_max_bytes_per_file() -> u64 {
    5_000_000
}
fn default_min_free_bytes() -> u64 {
    1_000_000_000
}
fn default_idle_extensions() -> Vec<String> {
    vec!["md".into(), "markdown".into(), "txt".into()]
}

impl Default for IdleWorkerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            idle_minutes: default_idle_minutes(),
            stop_on_user_activity: true,
            max_run_minutes: default_max_run_minutes(),
            max_files_per_run: default_max_files_per_run(),
            max_bytes_per_file: default_max_bytes_per_file(),
            approved_roots: Vec::new(),
            output_root: String::new(),
            min_free_bytes: default_min_free_bytes(),
            extensions: default_idle_extensions(),
            ollama_enabled: false,
            ollama_model: String::new(),
        }
    }
}


/// Stratum integration. Stratum's `04_config/actions.json` remains the single
/// source of truth for which actions exist; Nerve reads it at runtime rather
/// than duplicating the catalogue here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Root of the Stratum install (the folder containing 04_config/).
    #[serde(default = "default_stratum_root")]
    pub stratum_root: String,
    /// Full path to a Python that can run the action modules. The actions are
    /// pure stdlib, so any 3.x interpreter works - PySide6 is only needed by
    /// Stratum's own popup, which Nerve replaces.
    #[serde(default = "default_python_exe")]
    pub python_exe: String,
}

fn default_stratum_root() -> String {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    format!(
        r"{}\Microsoft\Windows\Start Menu\Programs\Startup\AHK\stratum",
        appdata
    )
}

fn default_python_exe() -> String {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    format!(r"{}\Programs\Python\Python312\python.exe", local)
}

impl Default for ScriptsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stratum_root: default_stratum_root(),
            python_exe: default_python_exe(),
        }
    }
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

    #[serde(default)]
    pub selection: SelectionConfig,

    #[serde(default)]
    pub canon: CanonConfig,

    #[serde(default)]
    pub scripts: ScriptsConfig,

    #[serde(default)]
    pub idle_worker: IdleWorkerConfig,

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
            keys: "Ctrl+Alt+S".into(),
            action: "toggle_shortcuts".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+M".into(),
            action: "toggle_mission-control".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+J".into(),
            action: "selection_toolbar".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+Q".into(),
            action: "selection_claim".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+U".into(),
            action: "toggle_workbench".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+B".into(),
            action: "toggle_atom-builder".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+N".into(),
            action: "toggle_reconciliation".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            // F8 is taken by something else on this machine; F9 was free.
            keys: "Ctrl+Alt+F9".into(),
            action: "tts_stop".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Alt+F7".into(),
            action: "tts_pause".into(),
            runtime_id: None,
        },
        HotkeyBinding {
            keys: "Ctrl+Shift+V".into(),
            action: "stratum_actions".into(),
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
        // First in the list so it is the panel that greets the user. Work done
        // while they were away should never need to be navigated to.
        PanelDef {
            name: "review".into(),
            title: "Review Queue".into(),
            url: to_url("atoms/review.html"),
            width: 1100,
            height: 720,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
            name: "clipboard".into(),
            title: "ClipSync".into(),
            url: to_url("clipboard/clipboard.html"),
            width: 380,
            height: 1000,
            x: None,
            y: Some(0),
            always_on_top: true,
            decorations: true,
            follow_cursor: false,
            open_at_startup: true,
        },
        PanelDef {
            name: "prompts".into(),
            title: "Prompts".into(),
            url: to_url("prompts/prompt_picker.html"),
            width: 520,
            height: 680,
            x: None,
            y: None,
            always_on_top: true,
            decorations: true,
            follow_cursor: false,
            open_at_startup: true,
        },
        PanelDef {
            name: "links".into(),
            title: "Links".into(),
            url: to_url("research/research_links.html"),
            width: 500,
            height: 680,
            x: None,
            y: None,
            always_on_top: true,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
            name: "research".into(),
            title: "Research".into(),
            url: to_url("research/research.html"),
            width: 600,
            height: 720,
            x: None,
            y: None,
            always_on_top: true,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
            name: "chat".into(),
            title: "AI Chat".into(),
            url: to_url("prompts/chat.html"),
            width: 500,
            height: 700,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
            name: "tts".into(),
            title: "TTS Engine".into(),
            url: to_url("system/tts-engine.html"),
            width: 480,
            height: 600,
            x: None,
            y: None,
            always_on_top: true,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        // Canon workbench surfaces, grouped under html/atoms/.
        PanelDef {
            name: "workbench".into(),
            title: "Canon Workbench".into(),
            url: to_url("atoms/workbench.html"),
            width: 1100,
            height: 860,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
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
            open_at_startup: false,
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
            open_at_startup: false,
        },
        PanelDef {
            name: "reconciliation".into(),
            title: "Classification Reconciliation".into(),
            url: to_url("atoms/reconciliation.html"),
            width: 1000,
            height: 820,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        // Stratum is a normal work window: users can place and resize it.
        PanelDef {
            name: "stratum".into(),
            title: "Stratum Actions".into(),
            url: to_url("stratum/stratum.html"),
            width: 620,
            height: 720,
            x: None,
            y: None,
            always_on_top: true,
            decorations: true,
            follow_cursor: true,
            open_at_startup: false,
        },
        // Floating selection toolbar - frameless, opens at the cursor.
        PanelDef {
            name: "toolbar".into(),
            title: "Nerve Toolbar".into(),
            url: to_url("stratum/toolbar.html"),
            width: 420,
            height: 64,
            x: None,
            y: None,
            always_on_top: true,
            decorations: false,
            follow_cursor: true,
            open_at_startup: false,
        },
        // Truth Capsule / result popup for a captured selection.
        PanelDef {
            name: "capsule".into(),
            title: "Truth Capsule".into(),
            url: to_url("atoms/capsule.html"),
            width: 560,
            height: 640,
            x: None,
            y: None,
            always_on_top: true,
            decorations: false,
            follow_cursor: true,
            open_at_startup: false,
        },
        PanelDef {
            name: "shortcuts".into(),
            title: "Hotkeys, Hotstrings & AI".into(),
            url: to_url("shortcuts.html"),
            width: 1050,
            height: 720,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
        },
        PanelDef {
            name: "mission-control".into(),
            title: "Theophysics Mission Control".into(),
            url: "http://localhost:7860/".into(),
            width: 1200,
            height: 850,
            x: None,
            y: None,
            always_on_top: false,
            decorations: true,
            follow_cursor: false,
            open_at_startup: false,
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
            selection: SelectionConfig::default(),
            canon: CanonConfig::default(),
            scripts: ScriptsConfig::default(),
            idle_worker: IdleWorkerConfig::default(),
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
            cfg.save()?;
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

        // Preserve user configuration while backfilling newly shipped
        // providers into older installations.
        for required in default_cfg.ai.providers {
            let exists = self.ai.providers.iter().any(|provider| {
                provider.name.eq_ignore_ascii_case(&required.name)
                    || (required.name == "Ollama Desktop"
                        && provider.name.eq_ignore_ascii_case("Ollama")
                        && provider.provider_type.eq_ignore_ascii_case("ollama"))
            });
            if !exists {
                self.ai.providers.push(required);
            }
        }

        // Ensure all core panels exist so tray/hotkeys always open something.
        // Nerve originally stored every HTML surface in one flat directory.
        // Migrate only those known legacy local URLs to the organized paths;
        // explicit custom/http panel URLs remain user-owned.
        for required in default_cfg.panels {
            if let Some(existing) = self.panels.iter_mut().find(|p| p.name == required.name) {
                let legacy_file = match existing.name.as_str() {
                    "review" => Some("review.html"),
                    "clipboard" => Some("clipboard.html"),
                    "prompts" => Some("prompt_picker.html"),
                    "links" => Some("links.html"),
                    "research" => Some("research.html"),
                    "chat" => Some("chat.html"),
                    "dashboard" => Some("dashboard.html"),
                    "settings" => Some("settings.html"),
                    "tts" => Some("tts-engine.html"),
                    "workbench" => Some("workbench.html"),
                    "atom-builder" => Some("atom-builder.html"),
                    "reconciliation" => Some("reconciliation.html"),
                    "stratum" => Some("stratum.html"),
                    "toolbar" => Some("toolbar.html"),
                    "capsule" => Some("capsule.html"),
                    _ => None,
                };
                let normalized_url = existing.url.replace('\\', "/");
                let is_legacy_local = legacy_file
                    .map(|file| normalized_url.ends_with(&format!("/{file}")))
                    .unwrap_or(false);
                if existing.url == "about:blank" || is_legacy_local {
                    existing.url = required.url;
                }
            } else {
                self.panels.push(required);
            }
        }
    }
}
