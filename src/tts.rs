use tracing::{info, error};
use std::process::Command;
use std::sync::Mutex;
use std::sync::OnceLock;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Hide console window when spawning processes
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// TTS settings (loaded from config, changeable at runtime)
pub struct TtsSettings {
    pub voice: String,      // e.g. "Guy" or "David" or full name
    pub speed: i32,         // SAPI rate: -10 to 10 (0=normal, 2=slightly fast, 5=fast)
    pub engine: String,     // "edge" or "sapi"
    pub volume: u32,        // 0-100
}

static TTS_SETTINGS: OnceLock<Mutex<TtsSettings>> = OnceLock::new();

fn settings() -> &'static Mutex<TtsSettings> {
    TTS_SETTINGS.get_or_init(|| {
        Mutex::new(TtsSettings {
            voice: String::new(),  // empty = auto-pick best
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
    info!("TTS configured: voice={}, speed={}, engine={}, vol={}", voice, speed, engine, volume);
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

    match engine.as_str() {
        "edge" => speak_edge_tts(text, &voice, speed),
        _ => speak_sapi(text, &voice, speed, volume),
    }
}

/// Stop any currently playing speech
pub fn stop() {
    let _ = new_hidden_command("powershell")
        .args(["-NoProfile", "-Command",
            "Add-Type -AssemblyName System.Speech; \
             (New-Object System.Speech.Synthesis.SpeechSynthesizer).SpeakAsyncCancelAll()"])
        .spawn();
    let _ = new_hidden_command("taskkill")
        .args(["/IM", "edge-playback.exe", "/F"])
        .output();
    info!("TTS: stopped");
}

/// Speak using Windows SAPI with voice selection and speed control
fn speak_sapi(text: &str, voice_name: &str, speed: i32, volume: u32) {
    let escaped = text
        .replace('\'', "''")
        .replace('\n', " ")
        .replace('\r', "")
        .replace('`', "'");

    let voice_selection = if voice_name.is_empty() || voice_name == "default" {
        // Auto-pick: prefer natural/online voices, then any available
        "$voices = $synth.GetInstalledVoices() | Where-Object { $_.Enabled }; \
         $natural = $voices | Where-Object { $_.VoiceInfo.Name -match 'Online|Natural' } | Select-Object -First 1; \
         if ($natural) { $synth.SelectVoice($natural.VoiceInfo.Name) }".into()
    } else {
        format!(
            "$voices = $synth.GetInstalledVoices() | Where-Object {{ $_.Enabled }}; \
             $match = $voices | Where-Object {{ $_.VoiceInfo.Name -match '{}' }} | Select-Object -First 1; \
             if ($match) {{ $synth.SelectVoice($match.VoiceInfo.Name) }}",
            voice_name.replace('\'', "''")
        )
    };

    let rate = speed.clamp(-10, 10);

    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         {}; \
         $synth.Rate = {}; \
         $synth.Volume = {}; \
         $synth.Speak('{}')",
        voice_selection, rate, volume, escaped
    );

    match new_hidden_command("powershell")
        .args(["-NoProfile", "-Command", &script])
        .spawn()
    {
        Ok(_) => info!("TTS SAPI: voice={}, rate={}, vol={}", voice_name, rate, volume),
        Err(e) => error!("TTS SAPI failed: {}", e),
    }
}

/// Speak using Edge TTS (pip install edge-tts for natural voices)
fn speak_edge_tts(text: &str, voice: &str, speed: i32) {
    let escaped = text.replace('"', r#"\""#).replace('\n', " ");
    let rate_pct = speed * 10;
    let rate_str = if rate_pct >= 0 { format!("+{}%", rate_pct) } else { format!("{}%", rate_pct) };
    let voice = if voice.is_empty() { "en-US-GuyNeural" } else { voice };

    match new_hidden_command("edge-playback")
        .args(["--voice", voice, "--rate", &rate_str, "--text", &escaped])
        .spawn()
    {
        Ok(_) => info!("TTS Edge: voice={}, rate={}", voice, rate_str),
        Err(_) => {
            info!("edge-tts not installed, falling back to SAPI");
            speak_sapi(&escaped, "default", speed, 100);
        }
    }
}

/// Save speech to audio file
#[allow(dead_code)]
pub fn save_audio(text: &str, output_path: &str) {
    let s = settings().lock().unwrap();
    let voice = s.voice.clone();
    let speed = s.speed;
    let engine = s.engine.clone();
    drop(s);

    let escaped = text.replace('\'', "''").replace('\n', " ").replace('\r', "");

    match engine.as_str() {
        "edge" => {
            let rate_pct = speed * 10;
            let rate_str = if rate_pct >= 0 { format!("+{}%", rate_pct) } else { format!("{}%", rate_pct) };
            let v = if voice.is_empty() { "en-US-GuyNeural".into() } else { voice };
            let _ = new_hidden_command("edge-tts")
                .args(["--voice", &v, "--rate", &rate_str,
                       "--text", &escaped, "--write-media", output_path])
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
                speed, output_path.replace('\'', "''"), escaped
            );
            let _ = new_hidden_command("powershell")
                .args(["-NoProfile", "-Command", &script])
                .spawn();
        }
    }
    info!("TTS: saving audio to {}", output_path);
}

/// List available SAPI voices
#[allow(dead_code)]
pub fn list_voices() -> Vec<String> {
    let output = new_hidden_command("powershell")
        .args(["-NoProfile", "-Command",
            "Add-Type -AssemblyName System.Speech; \
             $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
             $synth.GetInstalledVoices() | ForEach-Object { $_.VoiceInfo.Name }"])
        .output();

    match output {
        Ok(out) => {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        }
        Err(_) => vec!["Microsoft David Desktop".into()],
    }
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
