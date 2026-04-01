use tracing::{info, error};
use std::process::Command;

/// Read the currently selected text aloud using Edge TTS.
/// 1. Copies current selection (Ctrl+C)
/// 2. Reads clipboard
/// 3. Passes to edge-tts or Windows SAPI
pub fn read_selection() {
    // First, copy the selection
    send_ctrl_c();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Read clipboard
    let text = match get_clipboard_text() {
        Ok(t) if !t.is_empty() => t,
        _ => {
            info!("TTS: no text selected");
            return;
        }
    };

    info!("TTS: reading {} chars", text.len());

    // Try Edge TTS first (best quality), fall back to Windows SAPI
    if !speak_edge_tts(&text) {
        speak_sapi(&text);
    }
}

/// Speak using Edge TTS via PowerShell (uses Microsoft Edge voices - free, high quality)
fn speak_edge_tts(text: &str) -> bool {
    // Use PowerShell with the built-in speech synthesizer + SSML for Edge voices
    // Edge TTS can be called via the `edge-tts` Python package or directly via PowerShell
    // For simplicity, we use Windows built-in speech with the best available voice

    // First try: use edge-playback if installed (pip install edge-tts)
    let escaped = text.replace('"', r#"\""#).replace('\n', " ");

    let result = Command::new("cmd")
        .args(["/C", &format!(
            "echo {} | edge-playback --voice en-US-GuyNeural --rate=+0%%",
            &escaped
        )])
        .spawn();

    match result {
        Ok(_) => {
            info!("TTS: using edge-tts");
            true
        }
        Err(_) => {
            // edge-tts not installed, fall back
            false
        }
    }
}

/// Speak using Windows SAPI (built-in, always available)
fn speak_sapi(text: &str) {
    let escaped = text
        .replace('\'', "''")
        .replace('\n', " ")
        .replace('\r', "");

    // Use PowerShell to invoke SAPI
    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         $synth.Rate = 2; \
         $synth.Speak('{}')",
        escaped
    );

    match Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .spawn()
    {
        Ok(_) => info!("TTS: using Windows SAPI"),
        Err(e) => error!("TTS failed: {}", e),
    }
}

/// Save speech to audio file using Edge TTS
pub fn save_audio(text: &str, output_path: &str, voice: &str, rate: &str) {
    let escaped = text.replace('"', r#"\""#).replace('\n', " ");

    let result = Command::new("cmd")
        .args(["/C", &format!(
            "edge-tts --voice {} --rate={} --text \"{}\" --write-media \"{}\"",
            voice, rate, escaped, output_path
        )])
        .spawn();

    match result {
        Ok(_) => info!("TTS: saving audio to {}", output_path),
        Err(e) => error!("TTS save failed: {}", e),
    }
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
    let text: String = get_clipboard(formats::Unicode)
        .map_err(|e| anyhow::anyhow!("clipboard: {:?}", e))?;
    Ok(text)
}
