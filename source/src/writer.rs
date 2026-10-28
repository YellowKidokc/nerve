//! Select-and-fix writing assistant.
//!
//! One hotkey: use the current selection, or select the whole field when
//! nothing is selected, send it to the configured AI provider, and show the
//! corrected text in the `writer` popup. The popup can accept (paste over the
//! original), redo with a note ("don't drop the location"), or remember that
//! note as a standing rule that rides along on every later request.

use crate::ai::{self, ChatMessage};
use crate::config::{AiProvider, AiWorkflow, Config, WriterConfig};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone, Default, Serialize)]
pub struct State {
    /// Bumped on every new fix so a slow reply cannot overwrite a newer one.
    pub seq: u64,
    /// "idle" | "working" | "ready" | "error"
    pub status: String,
    pub original: String,
    pub result: String,
    pub error: String,
    pub provider: String,
    /// Notes given with "Redo" for this text, oldest first.
    pub notes: Vec<String>,
    /// Window the text came from; focus returns here before pasting.
    #[serde(skip)]
    pub source_hwnd: isize,
}

fn state() -> &'static Mutex<State> {
    static S: OnceLock<Mutex<State>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(State::default()))
}

pub fn snapshot() -> State {
    state().lock().unwrap().clone()
}

/// Grab the text to fix. Returns false when there is nothing to work on.
///
/// An existing selection is used as-is; only an empty selection triggers
/// Ctrl+A, so pressing the hotkey on already-selected text never widens it.
pub fn capture(cfg: &Config) -> bool {
    let hwnd = foreground_hwnd();
    let mut captured = crate::selection::capture(cfg);
    if captured.is_none() && cfg.writer.select_all_when_empty {
        send_ctrl_a();
        std::thread::sleep(Duration::from_millis(60));
        captured = crate::selection::capture(cfg);
    }
    let Some(capture) = captured else {
        info!("writer: nothing to fix");
        return false;
    };

    let mut s = state().lock().unwrap();
    s.seq += 1;
    s.status = "working".into();
    s.original = capture.text;
    s.result.clear();
    s.error.clear();
    s.notes.clear();
    s.source_hwnd = hwnd;
    true
}

/// Add a redo note for the current text. Returns false if there is no text.
pub fn add_note(note: &str) -> bool {
    let mut s = state().lock().unwrap();
    if s.original.is_empty() {
        return false;
    }
    s.seq += 1;
    s.status = "working".into();
    s.error.clear();
    let note = note.trim();
    if !note.is_empty() {
        s.notes.push(note.to_string());
    }
    true
}

/// Run the AI request for the current state and store the outcome.
pub async fn run(cfg: &Config) {
    let (seq, original, previous, notes) = {
        let s = state().lock().unwrap();
        (s.seq, s.original.clone(), s.result.clone(), s.notes.clone())
    };

    let outcome = match pick_provider(cfg) {
        Ok(provider) => {
            let name = provider.name.clone();
            let workflow = workflow(&cfg.writer);
            let (history, input) = conversation(&original, &previous, &notes);
            ai::chat(&provider, Some(&workflow), &history, &input)
                .await
                .map(|text| (name, clean(&text)))
        }
        Err(e) => Err(e),
    };

    let mut s = state().lock().unwrap();
    if s.seq != seq {
        return; // superseded by a newer fix or redo
    }
    match outcome {
        Ok((provider, text)) => {
            s.status = "ready".into();
            s.provider = provider;
            s.result = text;
        }
        Err(e) => {
            warn!("writer: {:#}", e);
            s.status = "error".into();
            s.error = format!("{:#}", e);
        }
    }
}

/// Return focus to the source window and paste the result over the selection.
///
/// The selection made at capture time is still active in the source app, so
/// the paste replaces exactly the text that was fixed.
pub fn accept(text: &str) {
    let hwnd = {
        let mut s = state().lock().unwrap();
        s.status = "idle".into();
        s.source_hwnd
    };
    let previous = crate::clipboard::current_text();
    focus_window(hwnd);
    std::thread::sleep(Duration::from_millis(120));
    crate::clipboard::paste_text(text);
    // Let the host app read the clipboard before restoring what was there.
    std::thread::sleep(Duration::from_millis(250));
    if let Some(prev) = previous {
        crate::selection::copy_result(&prev);
    }
}

/// Hand focus back without pasting (Escape / Cancel).
pub fn cancel() {
    let hwnd = {
        let mut s = state().lock().unwrap();
        s.seq += 1; // drop any reply still in flight
        s.status = "idle".into();
        s.source_hwnd
    };
    focus_window(hwnd);
}

fn pick_provider(cfg: &Config) -> Result<AiProvider> {
    let want = &cfg.writer.provider;
    let mut provider = cfg
        .ai
        .providers
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(want))
        .or_else(|| {
            cfg.ai
                .providers
                .iter()
                .find(|p| p.provider_type.eq_ignore_ascii_case(want))
        })
        .cloned()
        .ok_or_else(|| anyhow!("Writer provider '{}' not found in AI settings", want))?;
    if !cfg.writer.model.is_empty() {
        provider.model = cfg.writer.model.clone();
    }
    Ok(provider)
}

fn workflow(w: &WriterConfig) -> AiWorkflow {
    AiWorkflow {
        name: "Writer".into(),
        description: String::new(),
        provider: w.provider.clone(),
        system_prompt: system_prompt(w),
        user_template: String::new(),
        max_tokens: 4096,
        temperature: w.temperature,
    }
}

/// Base prompt plus the remembered rules, which take precedence.
pub fn system_prompt(w: &WriterConfig) -> String {
    let rules: Vec<&str> = w
        .rules
        .iter()
        .map(|r| r.trim())
        .filter(|r| !r.is_empty())
        .collect();
    if rules.is_empty() {
        return w.system_prompt.clone();
    }
    let mut out = w.system_prompt.clone();
    out.push_str("\n\nThe author's standing rules. They override anything above:");
    for r in rules {
        out.push_str("\n- ");
        out.push_str(r);
    }
    out
}

/// First pass sends the text alone. A redo replays the previous answer and
/// asks for a fresh correction of the original with every note applied.
fn conversation(original: &str, previous: &str, notes: &[String]) -> (Vec<ChatMessage>, String) {
    if notes.is_empty() || previous.is_empty() {
        return (Vec::new(), original.to_string());
    }
    let history = vec![
        ChatMessage { role: "user".into(), content: original.to_string() },
        ChatMessage { role: "assistant".into(), content: previous.to_string() },
    ];
    let mut input = String::from(
        "Correct my original text again, applying this feedback on your last attempt:",
    );
    for n in notes {
        input.push_str("\n- ");
        input.push_str(n);
    }
    input.push_str("\n\nReply with the corrected text only.");
    (history, input)
}

/// Strip wrappers models add despite instructions: a code fence or quotes
/// around the whole reply.
fn clean(text: &str) -> String {
    let mut t = text.trim();
    if t.starts_with("```") && t.ends_with("```") && t.len() >= 6 {
        t = &t[3..t.len() - 3];
        // Drop a language tag on the fence line.
        if let Some(nl) = t.find('\n') {
            if !t[..nl].contains(' ') {
                t = &t[nl + 1..];
            }
        }
        t = t.trim();
    }
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') && t[1..t.len() - 1].find('"').is_none() {
        t = &t[1..t.len() - 1];
    }
    t.to_string()
}

// ── Win32 ───────────────────────────────────────────────────────────────────

fn foreground_hwnd() -> isize {
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    unsafe { GetForegroundWindow().0 as isize }
}

fn focus_window(hwnd: isize) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
    if hwnd == 0 {
        return;
    }
    unsafe {
        let _ = SetForegroundWindow(HWND(hwnd as *mut _));
    }
}

fn send_ctrl_a() {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let key = |vk: VIRTUAL_KEY, up: bool| INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let inputs = [
        key(VK_CONTROL, false),
        key(VK_A, false),
        key(VK_A, true),
        key(VK_CONTROL, true),
    ];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_appended_to_the_prompt() {
        let w = WriterConfig {
            rules: vec!["Keep locations.".into(), "  ".into()],
            ..WriterConfig::default()
        };
        let p = system_prompt(&w);
        assert!(p.starts_with(&w.system_prompt));
        assert!(p.ends_with("- Keep locations."));
        assert_eq!(p.matches("\n- ").count(), 1);
    }

    #[test]
    fn no_rules_leaves_prompt_alone() {
        let w = WriterConfig { rules: vec![], ..WriterConfig::default() };
        assert_eq!(system_prompt(&w), w.system_prompt);
    }

    #[test]
    fn redo_replays_previous_attempt_with_notes() {
        let (h, input) = conversation("teh cat", "the cat", &["keep Tulsa".into()]);
        assert_eq!(h.len(), 2);
        assert_eq!(h[1].content, "the cat");
        assert!(input.contains("- keep Tulsa"));
        let (h, input) = conversation("teh cat", "", &[]);
        assert!(h.is_empty());
        assert_eq!(input, "teh cat");
    }

    #[test]
    fn clean_strips_fences_and_wrapping_quotes() {
        assert_eq!(clean("```text\nhello there\n```"), "hello there");
        assert_eq!(clean("\"hello\""), "hello");
        assert_eq!(clean("say \"hi\" now"), "say \"hi\" now");
        assert_eq!(clean("\"a\" and \"b\""), "\"a\" and \"b\"");
    }
}
