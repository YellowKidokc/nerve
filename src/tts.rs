use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Command, Output};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::{error, info, warn};
use wait_timeout::ChildExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows::{
    core::{w, PCWSTR, PWSTR},
    Win32::{
        Foundation::S_FALSE,
        Media::Speech::{
            IEnumSpObjectTokens, ISpObjectToken, ISpObjectTokenCategory, SpObjectTokenCategory,
        },
        System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED},
    },
};

/// Hide console window when spawning processes
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
const SAPI_VOICES_PATH: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices";
#[cfg(target_os = "windows")]
const ONECORE_VOICES_PATH: &str = "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech_OneCore\\Voices";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub id: String,
    pub name: String,
    pub lang: String,
    pub hive: String, // sapi | onecore | edge
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceListResult {
    pub voices: Vec<VoiceInfo>,
    pub stale: bool,
    pub warnings: Vec<String>,
}

/// TTS settings (loaded from config, changeable at runtime)
pub struct TtsSettings {
    pub voice: String,  // token id, short name, or display name
    pub speed: i32,     // SAPI rate: -10 to 10 (0=normal, 2=slightly fast, 5=fast)
    pub engine: String, // "edge" or "sapi"
    pub volume: u32,    // 0-100
}

static TTS_SETTINGS: OnceLock<Mutex<TtsSettings>> = OnceLock::new();

fn settings() -> &'static Mutex<TtsSettings> {
    TTS_SETTINGS.get_or_init(|| {
        Mutex::new(TtsSettings {
            voice: "Brian".into(), // Microsoft Brian Online — natural male
            speed: 2,              // slightly fast
            engine: "sapi".into(), // works out of the box
            volume: 100,
        })
    })
}

/// Update TTS settings at runtime
pub fn configure(voice: &str, speed: i32, engine: &str, volume: u32) {
    let mut s = settings().lock().unwrap();
    s.voice = voice.into();
    s.speed = speed;
    s.engine = engine.into();
    s.volume = volume;
    info!(
        "TTS configured: voice={}, speed={}, engine={}, vol={}",
        voice, speed, engine, volume
    );
}

/// Read the currently selected text aloud.
pub fn read_selection() {
    send_ctrl_c();
    std::thread::sleep(std::time::Duration::from_millis(150));

    let text = match get_clipboard_text() {
        Ok(t) if !t.is_empty() => t,
        _ => {
            info!("TTS: no text selected");
            return;
        }
    };

    info!("TTS: reading {} chars", text.len());
    speak(&text);
}

/// Speak text with configured voice/speed
pub fn speak(text: &str) {
    let s = settings().lock().unwrap();
    let voice = s.voice.clone();
    let speed = s.speed;
    let volume = s.volume;
    let engine = s.engine.clone();
    drop(s); // Release lock before spawning

    let res = match engine.as_str() {
        "edge" => speak_edge_tts(text, &voice, speed),
        _ => speak_sapi(text, &voice, speed, volume),
    };

    if let Err(e) = res {
        error!("TTS failed: {}", e);
    }
}

/// Stop any currently playing speech
pub fn stop() {
    let _ = new_hidden_command("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Add-Type -AssemblyName System.Speech; \
             (New-Object System.Speech.Synthesis.SpeechSynthesizer).SpeakAsyncCancelAll()",
        ])
        .spawn();
    let _ = new_hidden_command("taskkill")
        .args(["/IM", "edge-playback.exe", "/F"])
        .output();
    info!("TTS: stopped");
}

#[cfg(target_os = "windows")]
fn enumerate_from_hive(path: &str, hive: &str) -> Result<Vec<VoiceInfo>, String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .map_err(|e| format!("COM init failed for {hive}: {e}"))?;

        let result = (|| {
            let category: ISpObjectTokenCategory =
                SpObjectTokenCategory::new().map_err(|e| format!("Category create failed: {e}"))?;

            let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0u16)).collect();
            category
                .SetId(PCWSTR(wide_path.as_ptr()), false)
                .map_err(|e| format!("SetId failed for {path}: {e}"))?;

            let enum_tokens: IEnumSpObjectTokens = category
                .EnumTokens(None, None)
                .map_err(|e| format!("EnumTokens failed for {path}: {e}"))?;

            let mut voices = Vec::new();
            loop {
                let mut token: Option<ISpObjectToken> = None;
                let next_result = enum_tokens.Next(1, &mut token as *mut _ as _, None);
                match next_result {
                    Ok(_) => {
                        if let Some(tok) = token {
                            let id = tok
                                .GetId()
                                .and_then(|p| p.to_string())
                                .map_err(|e| format!("GetId failed: {e}"))?;

                            let attrs = tok
                                .OpenKey(w!("Attributes"))
                                .map_err(|e| format!("OpenKey(Attributes) failed: {e}"))?;

                            let name = attrs
                                .GetStringValue(w!("Name"))
                                .unwrap_or_else(|_| PWSTR::null())
                                .to_string()
                                .unwrap_or_else(|_| id.clone());

                            let lang = attrs
                                .GetStringValue(w!("Language"))
                                .unwrap_or_else(|_| PWSTR::null())
                                .to_string()
                                .unwrap_or_default();

                            voices.push(VoiceInfo {
                                id,
                                name,
                                lang,
                                hive: hive.to_string(),
                            });
                        }
                    }
                    Err(e) if e.code() == S_FALSE => break,
                    Err(e) => return Err(format!("Token enumeration failed for {path}: {e}")),
                }
            }

            Ok(voices)
        })();

        CoUninitialize();
        result
    }
}

#[cfg(not(target_os = "windows"))]
fn enumerate_from_hive(_path: &str, _hive: &str) -> Result<Vec<VoiceInfo>, String> {
    Ok(Vec::new())
}

fn enumerate_sapi_and_onecore() -> (Vec<VoiceInfo>, Vec<String>) {
    let mut voices = Vec::new();
    let mut warnings = Vec::new();

    #[cfg(target_os = "windows")]
    {
        match enumerate_from_hive(SAPI_VOICES_PATH, "sapi") {
            Ok(v) => voices.extend(v),
            Err(e) => warnings.push(format!("SAPI hive enumeration failed: {e}")),
        }

        match enumerate_from_hive(ONECORE_VOICES_PATH, "onecore") {
            Ok(v) => voices.extend(v),
            Err(e) => warnings.push(format!("OneCore hive enumeration failed: {e}")),
        }

        voices.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        voices.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
    }

    (voices, warnings)
}

fn preflight_edge_tts() -> Result<(), String> {
    let py = new_hidden_command("py")
        .arg("--version")
        .output()
        .map_err(|_| "Python launcher 'py' not found on PATH".to_string())?;

    if !py.status.success() {
        let stderr = String::from_utf8_lossy(&py.stderr);
        return Err(format!(
            "Python launcher present but not functional: {stderr}"
        ));
    }

    let pkg = new_hidden_command("py")
        .args(["-c", "import edge_tts; print('ok')"])
        .output()
        .map_err(|e| format!("Failed to probe edge_tts package: {e}"))?;

    if !pkg.status.success() {
        let stderr = String::from_utf8_lossy(&pkg.stderr);
        return Err(format!(
            "edge-tts package missing or broken in resolved Python install: {stderr}"
        ));
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct EdgeVoiceRaw {
    #[serde(rename = "ShortName")]
    short_name: String,
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
    #[serde(rename = "Locale")]
    locale: Option<String>,
}

fn parse_edge_voices(output: &str) -> Result<Vec<VoiceInfo>, String> {
    if let Ok(rows) = serde_json::from_str::<Vec<EdgeVoiceRaw>>(output) {
        return Ok(rows
            .into_iter()
            .map(|v| VoiceInfo {
                id: v.short_name.clone(),
                name: v.friendly_name.unwrap_or(v.short_name),
                lang: v.locale.unwrap_or_default(),
                hive: "edge".to_string(),
            })
            .collect());
    }

    let mut voices = Vec::new();
    for line in output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        if let Ok(item) = serde_json::from_str::<EdgeVoiceRaw>(line) {
            voices.push(VoiceInfo {
                id: item.short_name.clone(),
                name: item.friendly_name.unwrap_or(item.short_name),
                lang: item.locale.unwrap_or_default(),
                hive: "edge".to_string(),
            });
        }
    }

    if voices.is_empty() {
        return Err("Unable to parse edge_tts --list-voices output".to_string());
    }

    Ok(voices)
}

fn run_with_timeout(mut command: Command, timeout: Duration) -> Result<Output, String> {
    let mut child = command
        .spawn()
        .map_err(|e| format!("Subprocess spawn failed: {e}"))?;

    match child
        .wait_timeout(timeout)
        .map_err(|e| format!("Failed while waiting for subprocess: {e}"))?
    {
        Some(_) => child
            .wait_with_output()
            .map_err(|e| format!("Failed to capture subprocess output: {e}")),
        None => {
            child
                .kill()
                .map_err(|e| format!("edge_tts timed out after 8s and kill failed: {e}"))?;
            let _ = child.wait();
            Err("edge_tts --list-voices timed out after 8 seconds".to_string())
        }
    }
}

fn get_edge_voices() -> Result<Vec<VoiceInfo>, String> {
    preflight_edge_tts()?;

    let mut cmd = new_hidden_command("py");
    cmd.args(["-m", "edge_tts", "--list-voices"]);

    let output = run_with_timeout(cmd, Duration::from_secs(8))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("edge_tts --list-voices failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_edge_voices(&stdout)
}

fn cache_path() -> std::path::PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir);
    path.push("ClipSync");
    path.push("voice_cache.json");
    path
}

fn save_voice_cache(voices: &[VoiceInfo]) -> Result<(), String> {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed creating cache directory: {e}"))?;
    }
    let json = serde_json::to_string_pretty(voices)
        .map_err(|e| format!("Failed serializing voice cache: {e}"))?;
    fs::write(&path, json)
        .map_err(|e| format!("Failed writing voice cache {}: {e}", path.display()))
}

fn load_voice_cache() -> Result<Vec<VoiceInfo>, String> {
    let path = cache_path();
    let raw = fs::read_to_string(&path)
        .map_err(|e| format!("Failed reading voice cache {}: {e}", path.display()))?;
    serde_json::from_str::<Vec<VoiceInfo>>(&raw)
        .map_err(|e| format!("Failed parsing voice cache {}: {e}", path.display()))
}

pub fn list_voices_detailed() -> VoiceListResult {
    let (mut voices, mut warnings) = enumerate_sapi_and_onecore();

    match get_edge_voices() {
        Ok(mut edge) => voices.append(&mut edge),
        Err(e) => {
            warn!("Edge TTS enumeration failed: {e}");
            warnings.push(e);
        }
    }

    if !voices.is_empty() {
        if let Err(e) = save_voice_cache(&voices) {
            warnings.push(format!("Voice cache write failed: {e}"));
        }
        return VoiceListResult {
            voices,
            stale: false,
            warnings,
        };
    }

    match load_voice_cache() {
        Ok(cached) if !cached.is_empty() => {
            warnings.push("Returning stale cached voices from voice_cache.json".to_string());
            VoiceListResult {
                voices: cached,
                stale: true,
                warnings,
            }
        }
        Ok(_) => {
            warnings.push("Voice cache exists but is empty".to_string());
            VoiceListResult {
                voices: Vec::new(),
                stale: true,
                warnings,
            }
        }
        Err(e) => {
            warnings.push(format!("No cache fallback available: {e}"));
            VoiceListResult {
                voices: Vec::new(),
                stale: true,
                warnings,
            }
        }
    }
}

/// List available voices (name-only compatibility for current callers).
#[allow(dead_code)]
pub fn list_voices() -> Vec<String> {
    let result = list_voices_detailed();
    if !result.warnings.is_empty() {
        for w in &result.warnings {
            warn!("Voice enumeration warning: {}", w);
        }
    }
    result.voices.into_iter().map(|v| v.name).collect()
}

fn resolve_requested_voice(requested: &str, voices: &[VoiceInfo]) -> Result<VoiceInfo, String> {
    if requested.is_empty() || requested.eq_ignore_ascii_case("default") {
        return voices
            .iter()
            .find(|v| v.hive == "sapi" || v.hive == "onecore")
            .cloned()
            .ok_or_else(|| "No SAPI/OneCore voice is currently available".to_string());
    }

    if let Some(found) = voices.iter().find(|v| v.id == requested) {
        return Ok(found.clone());
    }

    if let Some(found) = voices
        .iter()
        .find(|v| v.name.eq_ignore_ascii_case(requested))
    {
        return Ok(found.clone());
    }

    Err(format!(
        "voice '{requested}' is no longer available, please reselect"
    ))
}

/// Speak using Windows SAPI with voice selection and speed control
fn speak_sapi(text: &str, voice_name_or_id: &str, speed: i32, volume: u32) -> Result<(), String> {
    let voice_result = list_voices_detailed();
    if !voice_result.warnings.is_empty() {
        for w in &voice_result.warnings {
            warn!("Voice enumeration warning before speak: {}", w);
        }
    }

    let available: Vec<VoiceInfo> = voice_result
        .voices
        .into_iter()
        .filter(|v| v.hive == "sapi" || v.hive == "onecore")
        .collect();

    let selected = resolve_requested_voice(voice_name_or_id, &available)?;

    let effective_volume = if selected.hive == "onecore" {
        ((volume.min(100) as f32) * 0.87).round() as u32
    } else {
        volume.min(100)
    };

    let escaped = text
        .replace('\'', "''")
        .replace('\n', " ")
        .replace('\r', "")
        .replace('`', "'");

    let rate = speed.clamp(-10, 10);
    let selected_name = selected.name.replace('\'', "''");
    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         $voices = $synth.GetInstalledVoices() | Where-Object {{ $_.Enabled }}; \
         $match = $voices | Where-Object {{ $_.VoiceInfo.Name -eq '{selected_name}' }} | Select-Object -First 1; \
         if (-not $match) {{ throw \"voice {selected_name} no longer available, please reselect\" }}; \
         $synth.SelectVoice($match.VoiceInfo.Name); \
         $synth.Rate = {rate}; \
         $synth.Volume = {effective_volume}; \
         $synth.Speak('{escaped}')"
    );

    new_hidden_command("powershell")
        .args(["-NoProfile", "-Command", &script])
        .spawn()
        .map_err(|e| format!("TTS SAPI failed for '{}': {}", selected.name, e))?;

    info!(
        "TTS SAPI: voice={} (hive={}), rate={}, vol={} (requested={})",
        selected.name, selected.hive, rate, effective_volume, volume
    );

    Ok(())
}

/// Speak using Edge TTS (pip install edge-tts for natural voices)
fn speak_edge_tts(text: &str, voice: &str, speed: i32) -> Result<(), String> {
    let escaped = text.replace('"', r#"\""#).replace('\n', " ");
    let rate_pct = speed * 10;
    let rate_str = if rate_pct >= 0 {
        format!("+{}%", rate_pct)
    } else {
        format!("{}%", rate_pct)
    };
    let voice = if voice.is_empty() {
        "en-US-GuyNeural"
    } else {
        voice
    };

    new_hidden_command("edge-playback")
        .args(["--voice", voice, "--rate", &rate_str, "--text", &escaped])
        .spawn()
        .map_err(|e| format!("Edge playback failed: {e}"))?;

    info!("TTS Edge: voice={}, rate={}", voice, rate_str);
    Ok(())
}

/// Save speech to audio file
#[allow(dead_code)]
pub fn save_audio(text: &str, output_path: &str) {
    let s = settings().lock().unwrap();
    let voice = s.voice.clone();
    let speed = s.speed;
    let engine = s.engine.clone();
    drop(s);

    let escaped = text
        .replace('\'', "''")
        .replace('\n', " ")
        .replace('\r', "");

    match engine.as_str() {
        "edge" => {
            let rate_pct = speed * 10;
            let rate_str = if rate_pct >= 0 {
                format!("+{}%", rate_pct)
            } else {
                format!("{}%", rate_pct)
            };
            let v = if voice.is_empty() {
                "en-US-GuyNeural".into()
            } else {
                voice
            };
            let _ = new_hidden_command("edge-tts")
                .args([
                    "--voice",
                    &v,
                    "--rate",
                    &rate_str,
                    "--text",
                    &escaped,
                    "--write-media",
                    output_path,
                ])
                .spawn();
        }
        _ => {
            let script = format!(
                "Add-Type -AssemblyName System.Speech; \
                 $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                 $synth.Rate = {}; \
                 $synth.SetOutputToWaveFile('{}'); \
                 $synth.Speak('{}'); \
                 $synth.SetOutputToDefaultAudioDevice()",
                speed,
                output_path.replace('\'', "''"),
                escaped
            );
            let _ = new_hidden_command("powershell")
                .args(["-NoProfile", "-Command", &script])
                .spawn();
        }
    }
    info!("TTS: saving audio to {}", output_path);
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
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_C,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_C,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn get_clipboard_text() -> anyhow::Result<String> {
    use clipboard_win::{formats, get_clipboard};
    let text: String =
        get_clipboard(formats::Unicode).map_err(|e| anyhow::anyhow!("clipboard: {:?}", e))?;
    Ok(text)
}
