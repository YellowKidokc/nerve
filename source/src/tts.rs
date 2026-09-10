use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use std::path::PathBuf;

use crate::tts_engine;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Hide console window when spawning processes
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Registry path for legacy SAPI voice tokens
const SAPI_VOICES_PATH: &str =
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices";

/// Registry path for OneCore/Neural voice tokens
const ONECORE_VOICES_PATH: &str =
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech_OneCore\\Voices";

/// Subprocess timeout for edge_tts --list-voices (Change 5)
const EDGE_TTS_TIMEOUT_SECS: u64 = 8;

/// How long `save_audio` waits for the engine to finish writing a file.
/// Online (Natural) voices render over the network, so this is generous.
const SAVE_TIMEOUT_SECS: u64 = 120;

/// Volume scale factor for OneCore voices (Change 6).
/// OneCore voices render ~10-15% louder than SAPI Desktop voices at the same
/// volume value.
const ONECORE_VOLUME_FACTOR: f32 = 0.87;

// ── Voice info ──────────────────────────────────────────────────────────────

/// Information about a single TTS voice, from any engine/hive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    /// Full token ID (registry path for SAPI/OneCore, ShortName for Edge)
    pub id: String,
    /// Human-readable display name
    pub name: String,
    /// Language/locale code
    pub lang: String,
    /// Source hive: "sapi", "onecore", or "edge"
    pub hive: String,
}

// ── TTS settings ────────────────────────────────────────────────────────────

/// TTS settings (loaded from config, changeable at runtime)
pub struct TtsSettings {
    pub voice: String,      // voice name for matching against enumerated list
    pub speed: i32,         // SAPI rate: -10 to 10 (0=normal, 2=slightly fast, 5=fast)
    pub engine: String,     // "edge" or "sapi"
    pub volume: u32,        // 0-100
}

static TTS_SETTINGS: OnceLock<Mutex<TtsSettings>> = OnceLock::new();

fn settings() -> &'static Mutex<TtsSettings> {
    TTS_SETTINGS.get_or_init(|| {
        Mutex::new(TtsSettings {
            // Full name, not "Brian": a bare "Brian" also matches the
            // different voice "Brian Online (Natural)".
            voice: "BrianMultilingual".into(),
            speed: 2,              // slightly fast
            engine: "sapi".into(), // works out of the box
            volume: 100,
        })
    })
}

/// PID of the currently speaking subprocess (for stop functionality)
static SPEAKING_PID: OnceLock<Mutex<Option<u32>>> = OnceLock::new();

fn speaking_pid() -> &'static Mutex<Option<u32>> {
    SPEAKING_PID.get_or_init(|| Mutex::new(None))
}

/// In-memory cache of the last successful voice enumeration, used for fast
/// lookups during speak() without re-enumerating.
static VOICE_LIST: OnceLock<Mutex<Vec<VoiceInfo>>> = OnceLock::new();

fn voice_list_store() -> &'static Mutex<Vec<VoiceInfo>> {
    VOICE_LIST.get_or_init(|| Mutex::new(Vec::new()))
}

/// Update TTS settings at runtime
pub fn configure(voice: &str, speed: i32, engine: &str, volume: u32) {
    let mut s = settings().lock().unwrap();
    s.voice = voice.into();
    s.speed = speed;
    s.engine = engine.into();
    s.volume = volume;
    drop(s);

    // Push settings onto the live voice object so they apply to the next
    // utterance without rebuilding anything.
    tts_engine::send(tts_engine::Cmd::SetVoice(voice.to_string()));
    tts_engine::send(tts_engine::Cmd::SetRate(speed));
    tts_engine::send(tts_engine::Cmd::SetVolume(volume as i32));

    info!("TTS configured: voice={}, speed={}, engine={}, vol={}", voice, speed, engine, volume);
}

// ── Dual-hive SAPI voice enumeration (Change 1) ────────────────────────────

/// Enumerate voices from a specific SAPI registry hive using COM.
///
/// Uses `ISpObjectTokenCategory::SetId()` with the full registry path string
/// instead of the `SPCAT_VOICES` constant, which is the key to accessing the
/// OneCore hive (there is no SPCAT_ constant for it).
fn enumerate_from_path(path: &str, hive: &str) -> Result<Vec<VoiceInfo>, String> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::S_FALSE;
    use windows::Win32::Media::Speech::{
        SpObjectTokenCategory, ISpObjectTokenCategory, ISpObjectToken,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
    };

    unsafe {
        // Ensure COM is initialized on this thread (ignored if already init'd)
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let category: ISpObjectTokenCategory =
            CoCreateInstance(&SpObjectTokenCategory, None, CLSCTX_ALL)
                .map_err(|e| format!("CoCreateInstance(SpObjectTokenCategory): {e}"))?;

        let wide_path: Vec<u16> = path.encode_utf16()
            .chain(std::iter::once(0u16))
            .collect();

        // SetId accepts an arbitrary registry path string, not just SPCAT_
        // constants. This is the key to enumerating the OneCore hive.
        category
            .SetId(PCWSTR(wide_path.as_ptr()), false)
            .map_err(|e| format!("SetId({path}): {e}"))?;

        let enum_tokens = category
            .EnumTokens(PCWSTR::null(), PCWSTR::null())
            .map_err(|e| format!("EnumTokens({path}): {e}"))?;

        let mut result = Vec::new();
        loop {
            let mut token: Option<ISpObjectToken> = None;
            let mut fetched: u32 = 0;
            let hr = enum_tokens.Next(
                1,
                &mut token as *mut _ as _,
                Some(&mut fetched as *mut u32),
            );

            match hr {
                Ok(_) => {}
                Err(e) if e.code() == S_FALSE => break,
                Err(e) => {
                    warn!("EnumTokens.Next error in {hive} hive: {e}");
                    break;
                }
            }

            if fetched == 0 {
                break;
            }

            if let Some(tok) = token {
                let id_ptr = tok.GetId()
                    .map_err(|e| format!("GetId: {e}"))?;
                let id = id_ptr.to_string()
                    .unwrap_or_default();

                // Read Name and Language from the Attributes subkey
                let (name, lang) = match tok.OpenKey(windows::core::w!("Attributes")) {
                    Ok(attrs) => {
                        let name = attrs
                            .GetStringValue(windows::core::w!("Name"))
                            .ok()
                            .and_then(|v: windows::core::PWSTR| v.to_string().ok())
                            .unwrap_or_else(|| id.clone());
                        let lang = attrs
                            .GetStringValue(windows::core::w!("Language"))
                            .ok()
                            .and_then(|v: windows::core::PWSTR| v.to_string().ok())
                            .unwrap_or_default();
                        (name, lang)
                    }
                    Err(_) => (id.clone(), String::new()),
                };

                result.push(VoiceInfo {
                    id,
                    name,
                    lang,
                    hive: hive.to_string(),
                });
            }
        }

        Ok(result)
    }
}

/// Enumerate all SAPI voices using SpVoice.GetVoices() via PowerShell/COM.
///
/// This goes through the SAPI automation (IDispatch) layer, which means it
/// respects NaturalVoiceSAPIAdapter and sees all 19+ voices.  The low-level
/// windows-rs ISpVoice interface does NOT see the adapter's virtual tokens.
pub fn enumerate_all_voices() -> Vec<VoiceInfo> {
    let script = r#"
$v = New-Object -ComObject SAPI.SpVoice
$voices = $v.GetVoices()
$result = @()
for ($i = 0; $i -lt $voices.Count; $i++) {
    $tok = $voices.Item($i)
    $desc = $tok.GetDescription()
    $id = $tok.Id
    $result += "$id|||$desc"
}
$result -join "`n"
"#;

    let output = match new_hidden_command("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            warn!("SAPI voice enumeration via PowerShell failed: {e}");
            return Vec::new();
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut voices: Vec<VoiceInfo> = stdout
        .lines()
        .filter(|line| line.contains("|||"))
        .map(|line| {
            let mut parts = line.splitn(2, "|||");
            let id = parts.next().unwrap_or("").trim().to_string();
            let desc = parts.next().unwrap_or("").trim().to_string();

            // Extract short name from description (e.g. "Microsoft Brian Online (Natural) - English" → "Brian")
            let name = desc
                .strip_prefix("Microsoft ")
                .unwrap_or(&desc)
                .split(" - ")
                .next()
                .unwrap_or(&desc)
                .to_string();

            let hive = if id.contains("OneCore") {
                "onecore"
            } else {
                "sapi"
            }.to_string();

            VoiceInfo { id, name, lang: "en-US".into(), hive }
        })
        .collect();

    info!("SAPI (via PowerShell): {} voices found", voices.len());

    // Dedup by Name, not ID.
    voices.sort_by(|a, b| a.name.cmp(&b.name));
    voices.dedup_by(|a, b| a.name == b.name);

    voices
}

// ── Edge TTS preflight + voice fetch (Changes 2, 5) ────────────────────────

/// Check that the Python launcher and edge-tts package are available.
/// Returns Ok(()) if both are working, or a specific, actionable error message.
pub fn check_edge_tts_available() -> Result<(), String> {
    // Step 1: confirm py launcher is on PATH and functional
    let py = new_hidden_command("py")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|_| {
            "Python launcher 'py' not found on PATH. \
             Install Python from python.org and ensure the 'py' launcher is included."
                .to_string()
        })?;

    if !py.status.success() {
        return Err(
            "Python launcher 'py' is present but not functional. Reinstall Python."
                .to_string(),
        );
    }

    // Step 2: confirm edge-tts package is importable in the resolved Python
    let pkg = new_hidden_command("py")
        .args(["-c", "import edge_tts; print('ok')"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to probe edge_tts package: {e}"))?;

    if !pkg.status.success() {
        let stderr = String::from_utf8_lossy(&pkg.stderr);
        return Err(format!(
            "edge-tts package not installed or broken. \
             Run: py -m pip install edge-tts\nDetails: {}",
            stderr.trim()
        ));
    }

    Ok(())
}

/// Fetch Edge TTS voices with preflight checks and subprocess timeout.
pub fn get_edge_voices() -> Result<Vec<VoiceInfo>, String> {
    check_edge_tts_available()?;

    let output = run_with_timeout(
        new_hidden_command("py")
            .args([
                "-c",
                "import asyncio,json,edge_tts; print(json.dumps(asyncio.run(edge_tts.list_voices())))",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
        Duration::from_secs(EDGE_TTS_TIMEOUT_SECS),
    )?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("edge_tts list_voices failed: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_edge_voices(&stdout)
}

/// Run a command with a timeout (Change 5).
///
/// Uses a poll loop with `try_wait` instead of an external dependency.
/// Returns the output if the process finishes within the deadline, or kills
/// the process and returns an error on timeout.
fn run_with_timeout(
    cmd: &mut Command,
    timeout: Duration,
) -> Result<std::process::Output, String> {
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Subprocess spawn failed: {e}"))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return child
                    .wait_with_output()
                    .map_err(|e| format!("Failed to read subprocess output: {e}"));
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait(); // reap zombie
                    return Err(format!(
                        "Subprocess timed out after {} seconds. \
                         This usually means a network issue reaching \
                         speech.platform.bing.com.",
                        timeout.as_secs()
                    ));
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return Err(format!("Failed to check subprocess status: {e}")),
        }
    }
}

/// Parse the JSON output of `edge_tts --list-voices` into VoiceInfo entries.
fn parse_edge_voices(output: &str) -> Result<Vec<VoiceInfo>, String> {
    // edge_tts --list-voices outputs a JSON array of objects with fields:
    // Name, ShortName, Gender, Locale, FriendlyName, etc.
    let entries: Vec<serde_json::Value> = serde_json::from_str(output)
        .map_err(|e| format!("Failed to parse edge_tts voice list as JSON: {e}"))?;

    let mut voices = Vec::new();
    for entry in &entries {
        let short_name = entry["ShortName"].as_str().unwrap_or_default();
        let locale = entry["Locale"].as_str().unwrap_or_default();
        let friendly = entry["FriendlyName"]
            .as_str()
            .or_else(|| entry["Name"].as_str())
            .unwrap_or(short_name);

        if !short_name.is_empty() {
            voices.push(VoiceInfo {
                id: short_name.to_string(),
                name: friendly.to_string(),
                lang: locale.to_string(),
                hive: "edge".to_string(),
            });
        }
    }

    Ok(voices)
}

// ── Voice list cache fallback (Change 3) ────────────────────────────────────

fn cache_path() -> PathBuf {
    let mut p = dirs::data_local_dir().expect("LOCALAPPDATA not resolvable");
    p.push("ClipSync");
    p.push("voice_cache.json");
    p
}

fn save_voice_cache(voices: &[VoiceInfo]) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match serde_json::to_string(voices) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                warn!("Failed to write voice cache: {e}");
            } else {
                info!("Voice cache saved: {} voices to {}", voices.len(), path.display());
            }
        }
        Err(e) => warn!("Failed to serialize voice cache: {e}"),
    }
}

fn load_voice_cache() -> Option<Vec<VoiceInfo>> {
    let path = cache_path();
    let json = std::fs::read_to_string(&path).ok()?;
    let voices: Vec<VoiceInfo> = serde_json::from_str(&json).ok()?;
    if voices.is_empty() {
        None
    } else {
        info!("Loaded {} voices from cache ({})", voices.len(), path.display());
        Some(voices)
    }
}

// ── Public voice list (combines SAPI + Edge + cache) ────────────────────────

/// Result of a voice enumeration, indicating freshness and any errors.
#[derive(Debug, Clone, Serialize)]
pub struct VoiceListResult {
    pub voices: Vec<VoiceInfo>,
    pub stale: bool,
    pub errors: Vec<String>,
}

/// Get all available voices across all engines, with cache fallback.
///
/// This is the primary entry point for the settings UI to populate the voice
/// dropdown. It enumerates SAPI (both hives) and Edge TTS, saves to cache on
/// success, and falls back to cache on total failure.
pub fn get_voices() -> VoiceListResult {
    let mut voices = Vec::new();
    let mut errors = Vec::new();

    // SAPI + OneCore (COM dual-hive)
    let sapi_voices = enumerate_all_voices();
    if sapi_voices.is_empty() {
        errors.push("No SAPI voices found from either registry hive".to_string());
    }
    voices.extend(sapi_voices);

    // Edge TTS (with preflight checks + timeout)
    match get_edge_voices() {
        Ok(edge) => {
            info!("Edge TTS: {} voices", edge.len());
            voices.extend(edge);
        }
        Err(e) => {
            warn!("Edge TTS unavailable: {e}");
            errors.push(e);
        }
    }

    if !voices.is_empty() {
        // Fresh enumeration produced results — update both caches
        save_voice_cache(&voices);
        *voice_list_store().lock().unwrap() = voices.clone();
        VoiceListResult {
            voices,
            stale: false,
            errors,
        }
    } else {
        // Complete failure — fall back to file cache
        match load_voice_cache() {
            Some(cached) => {
                warn!("Using cached voice list ({} voices)", cached.len());
                errors.push("Using cached voice list — live enumeration failed".to_string());
                *voice_list_store().lock().unwrap() = cached.clone();
                VoiceListResult {
                    voices: cached,
                    stale: true,
                    errors,
                }
            }
            None => {
                errors.push("No voice cache available".to_string());
                VoiceListResult {
                    voices: Vec::new(),
                    stale: true,
                    errors,
                }
            }
        }
    }
}

// ── Speech functions ────────────────────────────────────────────────────────

/// Copy the current selection without destroying what the user had on the
/// clipboard.
///
/// A sentinel distinguishes "nothing was selected" from "the selection happens
/// to equal the previous clipboard"; without it, pressing the read hotkey with
/// no selection would re-read whatever was copied earlier.
fn capture_selection() -> Option<String> {
    const SENTINEL: &str = "\u{0}nerve-tts-probe\u{0}";

    let previous = get_clipboard_text().unwrap_or_default();
    let _ = set_clipboard_text(SENTINEL);

    send_ctrl_c();
    std::thread::sleep(Duration::from_millis(90));

    let copied = get_clipboard_text().unwrap_or_default();

    // Let the source app finish its copy before we put the old value back.
    std::thread::sleep(Duration::from_millis(20));
    let _ = set_clipboard_text(&previous);

    if copied == SENTINEL || copied.trim().is_empty() {
        None
    } else {
        Some(copied.trim().to_string())
    }
}

/// Read the currently selected text aloud, interrupting any current speech.
pub fn read_selection() {
    match capture_selection() {
        Some(text) => {
            info!("TTS: reading {} chars", text.chars().count());
            if let Err(e) = speak(&text) {
                error!("TTS speak failed: {}", e);
            }
        }
        None => info!("TTS: no text selected"),
    }
}

/// Speak text with the configured voice, cancelling anything already playing.
///
/// Returns immediately: the work happens on the engine thread, so the hotkey
/// that triggered this is never held up by audio.
pub fn speak(text: &str) -> Result<(), String> {
    let s = settings().lock().unwrap();
    let engine = s.engine.clone();
    let voice_name = s.voice.clone();
    let speed = s.speed;
    drop(s);

    // Edge TTS stays a subprocess: it is a separate synthesizer with its own
    // player, not something SAPI can drive.
    if engine == "edge" {
        speak_edge_tts(text, &voice_name, speed);
        return Ok(());
    }

    tts_engine::send(tts_engine::Cmd::Speak(text.to_string()));
    Ok(())
}

/// Stop any currently playing speech.
pub fn stop() {
    tts_engine::send(tts_engine::Cmd::Stop);

    // Edge playback is a separate process, so it still needs killing.
    let _ = new_hidden_command("taskkill")
        .args(["/IM", "edge-playback.exe", "/F"])
        .output();
    info!("TTS: stopped");
}

/// Start the speech engine ahead of first use.
///
/// The engine thread initializes COM and enumerates voice tokens on creation.
/// Doing that lazily makes the first spoken request noticeably slower than the
/// rest, which reads as a hang on the hotkey. Called once at startup.
pub fn warm_up() {
    tts_engine::send(tts_engine::Cmd::Warm);
}

/// Pause if speaking, resume if paused.
pub fn pause_toggle() {
    tts_engine::send(tts_engine::Cmd::PauseToggle);
}

/// Map a friendly/SAPI voice name to an edge-tts neural voice name.
/// e.g. "BrianMultilingual" → "en-US-BrianMultilingualNeural"
fn map_to_edge_voice(name: &str) -> String {
    let lower = name.to_lowercase();

    // If already in edge-tts format (contains "Neural"), pass through
    if lower.contains("neural") {
        return name.to_string();
    }

    // Common mappings: strip "Online (Natural)" suffixes, add Neural
    let clean = name
        .replace(" Online (Natural)", "")
        .replace("Microsoft ", "")
        .replace("- English (United States)", "")
        .trim()
        .replace(' ', "");

    // Map to en-US by default
    let edge_name = format!("en-US-{}Neural", clean);
    info!("Edge voice mapped: '{}' -> '{}'", name, edge_name);
    edge_name
}

/// Speak using Edge TTS (pip install edge-tts for natural voices)
fn speak_edge_tts(text: &str, voice: &str, speed: i32) {
    let escaped = text.replace('"', r#"\""#).replace('\n', " ");
    let rate_pct = speed * 10;
    let rate_str = if rate_pct >= 0 { format!("+{}%", rate_pct) } else { format!("{}%", rate_pct) };
    let voice = if voice.is_empty() || voice == "default" {
        "en-US-BrianMultilingualNeural".to_string()
    } else {
        map_to_edge_voice(voice)
    };
    let voice = voice.as_str();

    match new_hidden_command("edge-playback")
        .args(["--voice", voice, "--rate", &rate_str, "--text", &escaped])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(child) => {
            *speaking_pid().lock().unwrap() = Some(child.id());
            info!("TTS Edge: voice={}, rate={}", voice, rate_str);
        }
        Err(_) => {
            info!("edge-tts not installed, falling back to SAPI");
            tts_engine::send(tts_engine::Cmd::Speak(text.to_string()));
        }
    }
}

/// Render speech to a .wav file, leaving any in-progress playback alone.
///
/// Blocks until the file is written so callers can report a real result.
pub fn save_audio(text: &str, output_path: &str) -> Result<(), String> {
    let (reply, wait) = std::sync::mpsc::sync_channel(1);
    tts_engine::send(tts_engine::Cmd::SaveWav {
        text: text.to_string(),
        path: output_path.to_string(),
        reply,
    });

    match wait.recv_timeout(Duration::from_secs(SAVE_TIMEOUT_SECS)) {
        Ok(result) => {
            if result.is_ok() {
                info!("TTS: saved audio to {}", output_path);
            }
            result
        }
        Err(_) => Err(format!(
            "Timed out after {SAVE_TIMEOUT_SECS}s waiting for the audio file. \
             Online (Natural) voices need network access to render."
        )),
    }
}

/// List available voice names (backward-compatible wrapper around get_voices).
#[allow(dead_code)]
pub fn list_voices() -> Vec<String> {
    get_voices()
        .voices
        .into_iter()
        .map(|v| v.name)
        .collect()
}

fn set_clipboard_text(text: &str) -> anyhow::Result<()> {
    use clipboard_win::{formats, set_clipboard};
    set_clipboard(formats::Unicode, text)
        .map_err(|e| anyhow::anyhow!("clipboard write: {:?}", e))?;
    Ok(())
}

/// Create a Command that hides the console window on Windows
fn new_hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

fn send_ctrl_c() {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_CONTROL, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_C, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_C, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_CONTROL, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn get_clipboard_text() -> anyhow::Result<String> {
    use clipboard_win::{formats, get_clipboard};
    let text: String = get_clipboard(formats::Unicode)
        .map_err(|e| anyhow::anyhow!("clipboard: {:?}", e))?;
    Ok(text)
}
