//! Selection capture layer.
//!
//! Nerve acts as the capture and display front end for the Canonical Content
//! OS. This module owns three jobs and nothing else:
//!
//! 1. Capture the current selection without destroying the user's clipboard.
//! 2. Recognize what the selection looks like, so the toolbar can offer only
//!    the actions that make sense.
//! 3. Hand the captured text to an action executor.
//!
//! Canon logic — identity, staging, validation, receipts, promotion — lives
//! behind the local canon API. This process never promotes anything, and it
//! never holds a promotion credential.

use crate::config::{Config, SelectionRule};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tracing::{info, warn};

/// A captured selection plus everything the UI needs to reason about it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Capture {
    /// The selected text, trimmed.
    pub text: String,
    /// Recognizer names that matched, most specific first.
    pub kinds: Vec<String>,
    /// Character count of `text`.
    pub len: usize,
    /// Cursor position at capture time, for placing the floating surfaces.
    pub cursor_x: i32,
    pub cursor_y: i32,
    /// Unix seconds.
    pub captured_at: u64,
    /// Title of the window the text came from — provenance for the candidate.
    pub source_window: String,
    /// Content hash of `text`, so a candidate can be addressed immutably.
    pub content_hash: String,

    // ── Derived recognition metadata ────────────────────────────────────
    //
    // Everything below is *derived* from `text` by the current recognizer,
    // not captured from the world. It is deliberately kept out of canonical
    // identity: `content_hash` covers `text` alone, and
    // `CandidateRequest::from_capture` names the atom's fields explicitly
    // rather than serializing this struct. Improving the classifier must
    // never make a previously captured statement look like a different atom.
    //
    // All three default, so captures serialized before these fields existed
    // still deserialize.
    /// Broad subject areas, e.g. `physics`, `theology`, `formal_methods`.
    /// Often empty — most selections belong to no particular domain.
    #[serde(default)]
    pub domains: Vec<String>,
    /// How confident the *recognizer* is, in `[0,1]`.
    ///
    /// This describes recognition, never truth: a false statement can be
    /// recognized as `claimish` with confidence 0.99.
    #[serde(default)]
    pub confidence: Option<f32>,
    /// Observable characteristics that drove the classification, e.g.
    /// `contains_equation`. Deterministic for identical input.
    #[serde(default)]
    pub features: Vec<String>,
}

/// The most recent capture. The toolbar and capsule both read from here rather
/// than re-copying, so a single Ctrl+C round trip serves the whole interaction.
fn current() -> &'static Mutex<Capture> {
    static CURRENT: OnceLock<Mutex<Capture>> = OnceLock::new();
    CURRENT.get_or_init(|| Mutex::new(Capture::default()))
}

pub fn last_capture() -> Capture {
    current().lock().unwrap().clone()
}

/// The node type the user asked for when a capture was started directly from a
/// hotkey (e.g. Ctrl+Alt+Q for a claim). Empty when the toolbar will choose.
fn pending_node() -> &'static Mutex<String> {
    static PENDING: OnceLock<Mutex<String>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(String::new()))
}

pub fn set_pending_node(node_type: &str) {
    *pending_node().lock().unwrap() = node_type.to_string();
}

pub fn take_pending_node() -> String {
    let mut guard = pending_node().lock().unwrap();
    std::mem::take(&mut *guard)
}

/// Current clipboard text, for actions that fall back to it when nothing is
/// selected. Stratum's action contract reads `selection` first, then
/// `clipboard`, so this has to be real content rather than an empty string.
pub fn clipboard_text() -> String {
    get_clipboard_text().unwrap_or_default()
}

fn store(capture: Capture) {
    *current().lock().unwrap() = capture;
}

// ── Capture ─────────────────────────────────────────────────────────────────

/// Copy the current selection, preserving the user's existing clipboard.
///
/// Returns `None` when nothing is selected. We detect that by comparing
/// against the pre-existing clipboard content: if Ctrl+C produced no change
/// and the host app had nothing selected, we would otherwise capture whatever
/// the user copied earlier and treat stale text as a fresh selection.
pub fn capture(cfg: &Config) -> Option<Capture> {
    let delay = Duration::from_millis(cfg.selection.capture_delay_ms.max(40));
    let preserve = cfg.selection.preserve_clipboard;

    let previous = get_clipboard_text().unwrap_or_default();

    // A sentinel makes "nothing was selected" distinguishable from "the
    // selection happens to equal the old clipboard".
    let sentinel = "\u{0}nerve-capture-probe\u{0}";
    let _ = set_clipboard_text(sentinel);

    send_ctrl_c();
    std::thread::sleep(delay);

    let copied = get_clipboard_text().unwrap_or_default();

    if preserve {
        // Give the host app a moment to finish before we put the old value back.
        std::thread::sleep(Duration::from_millis(20));
        let _ = set_clipboard_text(&previous);
    }

    if copied == sentinel || copied.trim().is_empty() {
        info!("selection: nothing selected");
        return None;
    }

    let text = copied.trim().to_string();
    // One recognition pass; `kinds` and the derived metadata come from the
    // same result so they can never disagree with each other.
    let recognition = recognize_full(&text);
    let capture = Capture {
        kinds: recognition.kinds,
        domains: recognition.domains,
        confidence: Some(recognition.confidence),
        features: recognition.features,
        len: text.chars().count(),
        content_hash: content_hash(&text),
        cursor_x: cursor_pos().0,
        cursor_y: cursor_pos().1,
        captured_at: now_secs(),
        source_window: foreground_window_title(),
        text,
    };

    info!(
        "selection: captured {} chars, kinds={:?}, confidence={:?}, from '{}'",
        capture.len, capture.kinds, capture.confidence, capture.source_window
    );

    store(capture.clone());
    Some(capture)
}

// ── Recognition ─────────────────────────────────────────────────────────────

/// A compact, deliberately shallow description of a selection.
///
/// This recognizes *characteristics*, not subject matter. It is not a taxonomy
/// and should not grow into one — domain modules enrich it downstream. The
/// whole point is that it stays cheap enough to run on every selection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Recognition {
    /// Matching recognizer names, most specific first, always ending `any`.
    pub kinds: Vec<String>,
    /// Broad subject areas. Empty is the common case and is not a failure.
    pub domains: Vec<String>,
    /// Recognition confidence in `[0,1]` — never a claim about truth.
    pub confidence: f32,
    /// Observable signals that drove the classification. Sorted, so identical
    /// input always yields an identical vector.
    pub features: Vec<String>,
}

/// Classify a selection. Returns every matching recognizer, most specific
/// first, always ending with `any` so catch-all rules still match.
///
/// Retained with its original signature: `matching_rules` routes on `kinds`
/// alone, and `Capture.kinds` is part of the candidate payload the canon
/// engine hashes. Callers wanting the richer description use
/// [`recognize_full`].
pub fn recognize(text: &str) -> Vec<String> {
    recognize_full(text).kinds
}

/// The full recognition, including derived metadata.
pub fn recognize_full(text: &str) -> Recognition {
    let t = text.trim();
    let lower = t.to_lowercase();
    let mut kinds = Vec::new();
    let mut features = Vec::new();
    let mut domains = Vec::new();

    if is_url(&lower) {
        kinds.push("url".into());
    }
    if is_email(t) {
        kinds.push("email".into());
    }
    if is_path(t) {
        kinds.push("path".into());
    }
    if is_number(t) {
        kinds.push("number".into());
    }
    if is_scripture(t) {
        kinds.push("scripture".into());
    }
    if is_math(t) {
        kinds.push("math".into());
    }
    // Lean is checked before `code` so it appears first — kinds are ordered
    // most-specific-first, and a Lean selection is also legitimately code.
    let lean = is_lean(t);
    if lean {
        kinds.push("lean".into());
    }
    if is_code(t) || lean {
        kinds.push("code".into());
    }
    if is_citation(t) {
        kinds.push("citation".into());
    }
    if is_claimish(t) {
        kinds.push("claimish".into());
    }

    kinds.push("any".into());

    // ── Derived metadata ────────────────────────────────────────────────
    // Features restate *why* a kind matched, in terms a downstream module can
    // act on without re-running the detectors.
    if kinds.iter().any(|k| k == "math") {
        features.push("contains_equation".into());
    }
    if kinds.iter().any(|k| k == "claimish") {
        features.push("contains_assertion".into());
    }
    if lean {
        features.push("lean_syntax".into());
        if has_lean_declaration(t) {
            features.push("theorem_declaration".into());
        }
    }
    if kinds.iter().any(|k| k == "citation") {
        features.push("bibliographic_reference".into());
    }
    if kinds.iter().any(|k| k == "scripture") {
        features.push("scripture_reference".into());
    }

    if lean {
        domains.push("formal_methods".into());
    }
    if kinds.iter().any(|k| k == "scripture") {
        domains.push("theology".into());
    }
    if kinds.iter().any(|k| k == "math") && !lean {
        domains.push("mathematics".into());
    }

    // Sorted so identical input yields an identical vector regardless of the
    // order the checks happen to run in.
    features.sort();
    features.dedup();
    domains.sort();
    domains.dedup();

    let confidence = confidence_for(&kinds, &features);

    Recognition {
        kinds,
        domains,
        confidence,
        features,
    }
}

/// Recognition confidence, clamped to `[0,1]`.
///
/// Syntactically decisive kinds (a URL, an email, Lean syntax) score high
/// because the detector is checking structure. Heuristic kinds — `claimish`
/// above all, which is really "reads like an assertion" — score lower, and a
/// bare `any` scores lowest of all because it means nothing matched.
fn confidence_for(kinds: &[String], features: &[String]) -> f32 {
    let base: f32 = if kinds.iter().any(|k| k == "lean") {
        if features.iter().any(|f| f == "theorem_declaration") {
            0.96
        } else {
            0.88
        }
    } else if kinds
        .iter()
        .any(|k| matches!(k.as_str(), "url" | "email" | "path" | "number"))
    {
        0.97
    } else if kinds.iter().any(|k| k == "citation") {
        0.85
    } else if kinds.iter().any(|k| k == "scripture") {
        0.92
    } else if kinds.iter().any(|k| k == "math") {
        0.88
    } else if kinds.iter().any(|k| k == "code") {
        0.80
    } else if kinds.iter().any(|k| k == "claimish") {
        0.70
    } else {
        // Only `any` matched: nothing was recognized.
        0.10
    };
    base.clamp(0.0, 1.0)
}

fn is_url(lower: &str) -> bool {
    let one_token = !lower.trim().contains(char::is_whitespace);
    one_token
        && (lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("www.")
            || (lower.contains('.') && lower.contains('/') && !lower.contains('\\')))
}

fn is_email(t: &str) -> bool {
    let t = t.trim();
    if t.contains(char::is_whitespace) {
        return false;
    }
    match t.split_once('@') {
        Some((user, domain)) => {
            !user.is_empty() && domain.contains('.') && !domain.starts_with('.')
                && !domain.ends_with('.')
        }
        None => false,
    }
}

fn is_path(t: &str) -> bool {
    let t = t.trim();
    if t.contains('\n') {
        return false;
    }
    // Windows drive path or UNC share.
    let drive = t.len() > 3
        && t.as_bytes()[0].is_ascii_alphabetic()
        && &t[1..3] == ":\\";
    drive || t.starts_with("\\\\")
}

fn is_number(t: &str) -> bool {
    let cleaned: String = t.chars().filter(|c| !matches!(c, ',' | ' ' | '_')).collect();
    let cleaned = cleaned
        .trim_start_matches(['$', '£', '€', '+', '-'])
        .trim_end_matches('%');
    !cleaned.is_empty() && cleaned.parse::<f64>().is_ok()
}

/// Rough scripture-reference test: "John 3:16", "1 Cor 13:4-7", "Gen 1:1".
fn is_scripture(t: &str) -> bool {
    let t = t.trim();
    if t.len() > 60 || !t.contains(':') {
        return false;
    }
    let has_book_word = t
        .split_whitespace()
        .any(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 3);
    let (_, after) = match t.split_once(':') {
        Some(v) => v,
        None => return false,
    };
    let verse_ok = after
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false);
    has_book_word && verse_ok
}

fn is_math(t: &str) -> bool {
    let math_chars = ['=', '∫', '∑', '≅', '≝', '≈', '≤', '≥', '∂', '√', '∇', '^'];
    let hits = t.chars().filter(|c| math_chars.contains(c)).count();
    let latex = t.contains("\\frac") || t.contains("\\sum") || t.contains("$$");
    latex || (hits > 0 && t.chars().any(|c| c.is_ascii_digit() || c.is_alphabetic()) && hits >= 1)
}

fn is_code(t: &str) -> bool {
    let markers = ["{", "}", "();", "=>", "->", "def ", "fn ", "class ", "import ", "const "];
    let hits = markers.iter().filter(|m| t.contains(**m)).count();
    hits >= 2
}

/// Lean 4 source, by syntax rather than vocabulary.
///
/// The word "theorem" appears constantly in ordinary mathematical prose, so it
/// is never sufficient on its own. A declaration keyword only counts when it is
/// accompanied by syntax prose does not contain — `:=`, `by`, a `#command`, or
/// a Mathlib import — or when two distinct Lean signals co-occur.
fn is_lean(t: &str) -> bool {
    let t = t.trim();
    if t.is_empty() {
        return false;
    }

    // Unambiguous: these forms do not occur in prose.
    if t.contains("#print axioms")
        || t.contains("#check")
        || t.contains("#eval")
        || t.contains("import Mathlib")
    {
        return true;
    }

    let has_decl = has_lean_declaration(t);
    // `:=` is Lean's definitional equals; `by` opens tactic mode. Both are
    // checked as tokens so "standby" or a stray colon cannot match.
    let has_assign = t.contains(":=");
    let has_tactic = t
        .split(|c: char| c.is_whitespace() || c == '(' || c == ')')
        .any(|w| w == "by");
    let has_sorry = t.split_whitespace().any(|w| w == "sorry");

    if has_decl && (has_assign || has_tactic || has_sorry) {
        return true;
    }

    // No declaration keyword, but several structural signals together.
    let structural = [has_assign, has_tactic, t.contains("⊢"), t.contains("∀"), t.contains("→")]
        .iter()
        .filter(|b| **b)
        .count();
    structural >= 2 && has_assign
}

/// A Lean declaration keyword at the start of a line, as a whole word.
///
/// Anchoring to line starts is what keeps "the theorem states that..." out:
/// prose mentions these words mid-sentence, Lean declares them at column zero.
fn has_lean_declaration(t: &str) -> bool {
    const DECLS: [&str; 8] = [
        "theorem", "lemma", "def", "structure", "inductive", "instance", "example", "abbrev",
    ];
    t.lines().any(|line| {
        let line = line.trim_start();
        DECLS.iter().any(|d| {
            line.strip_prefix(*d)
                .map(|rest| rest.starts_with(char::is_whitespace))
                .unwrap_or(false)
        })
    })
}

/// A bibliographic reference: DOI, BibTeX, numbered reference, author-year, or
/// a journal-style citation.
///
/// The controls matter more than the patterns here — a year in parentheses and
/// a page range both occur in ordinary writing, so each form requires a second
/// corroborating signal before it counts.
fn is_citation(t: &str) -> bool {
    let t = t.trim();
    let lower = t.to_lowercase();

    // DOIs are unambiguous.
    if lower.contains("doi:") || lower.contains("doi.org/") || lower.contains("10.") && lower.contains("/") && lower.starts_with("10.") {
        return true;
    }
    // BibTeX entries.
    if t.starts_with('@') && t.contains('{') {
        return true;
    }
    // arXiv identifiers.
    if lower.contains("arxiv:") {
        return true;
    }

    // Long prose is not a citation even if it happens to contain a year.
    if t.split_whitespace().count() > 60 {
        return false;
    }

    // "[12] Author, Title, Journal" — a bracketed number *leading* the string,
    // followed by real content.
    let numbered = t.starts_with('[')
        && t.find(']').map(|i| t[1..i].chars().all(|c| c.is_ascii_digit()) && i > 1).unwrap_or(false)
        && t.len() > 12;
    if numbered {
        return true;
    }

    // Author-year: "(Smith 2019)" or "Smith et al., 2019". A bare year is not
    // enough — there must be a capitalised name and citation punctuation.
    let has_year = (1500..=2200).any(|y| t.contains(&y.to_string()));
    if !has_year {
        return false;
    }
    let has_name = t
        .split(|c: char| !c.is_alphabetic())
        .any(|w| w.len() > 2 && w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false));
    let citation_punct = lower.contains("et al") || lower.contains("pp.") || lower.contains("vol.")
        || (t.contains('(') && t.contains(')'))
        || t.matches(',').count() >= 2;

    has_name && citation_punct
}

/// A sentence that reads like an assertion — the shape worth offering "Claim" for.
fn is_claimish(t: &str) -> bool {
    let words = t.split_whitespace().count();
    if words < 4 || words > 200 {
        return false;
    }
    let lower = t.to_lowercase();
    let copulas = [
        " is ", " are ", " was ", " were ", " must ", " cannot ", " implies ",
        " therefore ", " because ", " entails ", " requires ", " means that ",
    ];
    copulas.iter().any(|c| lower.contains(c))
}

// ── Rule matching ───────────────────────────────────────────────────────────

/// Which toolbar buttons apply to this capture, in config order.
pub fn matching_rules(cfg: &Config, capture: &Capture) -> Vec<SelectionRule> {
    cfg.selection
        .rules
        .iter()
        .filter(|r| r.enabled && rule_matches(r, capture))
        .cloned()
        .collect()
}

fn rule_matches(rule: &SelectionRule, capture: &Capture) -> bool {
    let m = &rule.matcher;

    if m.min_len > 0 && capture.len < m.min_len {
        return false;
    }
    if m.max_len > 0 && capture.len > m.max_len {
        return false;
    }
    if !m.contains.is_empty()
        && !capture
            .text
            .to_lowercase()
            .contains(&m.contains.to_lowercase())
    {
        return false;
    }

    // No declared kinds behaves like "any".
    if m.kinds.is_empty() {
        return true;
    }
    m.kinds
        .iter()
        .any(|k| k == "any" || capture.kinds.iter().any(|ck| ck == k))
}

// ── Output ──────────────────────────────────────────────────────────────────

/// Replace the selection in the source app by pasting over it.
///
/// This is destructive in the host application, so it only runs for rules that
/// explicitly declare `output: "replace"`.
pub fn replace_selection(text: &str) {
    crate::clipboard::paste_text(text);
}

/// Put a result on the clipboard without touching the host app.
pub fn copy_result(text: &str) {
    if let Err(e) = set_clipboard_text(text) {
        warn!("selection: clipboard write failed: {:?}", e);
    }
}

// ── Content hash ────────────────────────────────────────────────────────────

/// FNV-1a 64-bit, rendered hex. Candidates need a stable, cheap content
/// address at capture time; the canon service is free to re-hash with its own
/// algorithm when it stages the record.
pub fn content_hash(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64:{:016x}", hash)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Win32 helpers ───────────────────────────────────────────────────────────

fn get_clipboard_text() -> anyhow::Result<String> {
    use clipboard_win::{formats, get_clipboard};
    let text: String =
        get_clipboard(formats::Unicode).map_err(|e| anyhow::anyhow!("clipboard read: {:?}", e))?;
    Ok(text)
}

fn set_clipboard_text(text: &str) -> anyhow::Result<()> {
    use clipboard_win::{formats, set_clipboard};
    set_clipboard(formats::Unicode, text)
        .map_err(|e| anyhow::anyhow!("clipboard write: {:?}", e))?;
    Ok(())
}

/// Current mouse position in screen coordinates.
pub fn cursor_pos() -> (i32, i32) {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    let mut point = POINT::default();
    unsafe {
        if GetCursorPos(&mut point).is_ok() {
            return (point.x, point.y);
        }
    }
    (0, 0)
}

/// Title of the foreground window, used as capture provenance.
fn foreground_window_title() -> String {
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return String::new();
        }
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len <= 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buf[..len as usize])
    }
}

fn send_ctrl_c() {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let key = |vk: VIRTUAL_KEY, up: bool| INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: if up {
                    KEYEVENTF_KEYUP
                } else {
                    KEYBD_EVENT_FLAGS(0)
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let inputs = [
        key(VK_CONTROL, false),
        key(VK_C, false),
        key(VK_C, true),
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
    fn recognizes_urls_but_not_sentences_with_dots() {
        assert!(recognize("https://example.com/x").contains(&"url".to_string()));
        assert!(!recognize("This is a sentence. With dots.").contains(&"url".to_string()));
    }

    #[test]
    fn recognizes_scripture_references() {
        assert!(recognize("John 3:16").contains(&"scripture".to_string()));
        assert!(recognize("1 Cor 13:4-7").contains(&"scripture".to_string()));
        assert!(!recognize("12:30").contains(&"scripture".to_string()));
    }

    #[test]
    fn claimish_needs_an_assertion_shape() {
        assert!(recognize("Entropy is the arrow of time").contains(&"claimish".to_string()));
        assert!(!recognize("hello there").contains(&"claimish".to_string()));
    }

    #[test]
    fn any_always_matches() {
        assert!(recognize("").contains(&"any".to_string()));
    }

    #[test]
    fn content_hash_is_stable_and_distinct() {
        assert_eq!(content_hash("abc"), content_hash("abc"));
        assert_ne!(content_hash("abc"), content_hash("abd"));
    }

    // -- Enrichment ------------------------------------------------------

    const LEAN: &str = "theorem add_comm (a b : Nat) : a + b = b + a := by\n  simp [Nat.add_comm]";

    /// The pre-existing kinds must keep behaving exactly as before, since
    /// `matching_rules` and the canon candidate payload both read them.
    #[test]
    fn existing_kinds_are_unchanged() {
        assert!(recognize("https://example.com/x").contains(&"url".into()));
        assert!(recognize("dave@example.com").contains(&"email".into()));
        assert!(recognize("C:\\Users\\dave\\notes.md").contains(&"path".into()));
        assert!(recognize("1,240.50").contains(&"number".into()));
        assert!(recognize("John 3:16").contains(&"scripture".into()));
        assert!(recognize("Entropy is the arrow of time").contains(&"claimish".into()));
    }

    #[test]
    fn any_is_still_the_final_fallback() {
        for text in ["hello there", "https://example.com/x", LEAN] {
            let kinds = recognize(text);
            assert_eq!(kinds.last().unwrap(), "any", "failed for {:?}", text);
        }
    }

    #[test]
    fn lean_source_is_recognized() {
        let r = recognize_full(LEAN);
        assert!(r.kinds.contains(&"lean".into()));
        // Lean is also code, and the more specific kind comes first.
        assert!(r.kinds.contains(&"code".into()));
        assert!(
            r.kinds.iter().position(|k| k == "lean") < r.kinds.iter().position(|k| k == "code")
        );
        assert!(r.features.contains(&"theorem_declaration".into()));
        assert_eq!(r.domains, vec!["formal_methods".to_string()]);
    }

    #[test]
    fn lean_commands_alone_are_enough() {
        assert!(recognize("#print axioms myThm").contains(&"lean".into()));
        assert!(recognize("import Mathlib.Data.Nat.Basic").contains(&"lean".into()));
    }

    /// Prose about theorems is not Lean. This is the control that matters:
    /// mathematical writing is full of these words.
    #[test]
    fn prose_mentioning_theorems_is_not_lean() {
        for text in [
            "The theorem states that entropy always increases.",
            "By definition, a lemma is a stepping stone to a theorem.",
            "This structure is inductive in the informal sense.",
        ] {
            assert!(
                !recognize(text).contains(&"lean".into()),
                "false positive on {:?}",
                text
            );
        }
    }

    #[test]
    fn citations_are_recognized() {
        for text in [
            "doi:10.1103/PhysRevD.7.2333",
            "@article{bekenstein1973, title={Black holes and entropy}}",
            "arXiv:1706.03762",
            "[12] Bekenstein, J. D., Black holes and entropy, Phys. Rev. D, 1973",
            "(Smith et al., 2019)",
        ] {
            assert!(
                recognize(text).contains(&"citation".into()),
                "missed citation in {:?}",
                text
            );
        }
    }

    #[test]
    fn ordinary_text_with_a_year_is_not_a_citation() {
        for text in [
            "In 1973 the paper was published.",
            "I was born in 1985.",
            "2019",
        ] {
            assert!(
                !recognize(text).contains(&"citation".into()),
                "false positive on {:?}",
                text
            );
        }
    }

    #[test]
    fn several_kinds_can_match_at_once() {
        let r = recognize_full("Energy is conserved because E = mc^2");
        assert!(r.kinds.contains(&"math".into()));
        assert!(r.kinds.contains(&"claimish".into()));
        assert!(r.features.contains(&"contains_equation".into()));
        assert!(r.features.contains(&"contains_assertion".into()));
    }

    #[test]
    fn domains_may_be_empty() {
        let r = recognize_full("hello there");
        assert!(r.domains.is_empty());
        assert_eq!(r.kinds, vec!["any".to_string()]);
    }

    #[test]
    fn confidence_is_bounded_and_ranks_syntax_above_heuristics() {
        for text in ["", "hello there", LEAN, "https://example.com/x", "John 3:16"] {
            let c = recognize_full(text).confidence;
            assert!((0.0..=1.0).contains(&c), "out of range for {:?}: {}", text, c);
        }
        // "reads like an assertion" is a weaker signal than URL syntax, and an
        // unrecognized selection is weaker still.
        let claimish = recognize_full("Entropy is the arrow of time").confidence;
        let url = recognize_full("https://example.com/x").confidence;
        let nothing = recognize_full("hello there").confidence;
        assert!(url > claimish);
        assert!(claimish > nothing);
    }

    #[test]
    fn features_are_deterministic_for_identical_input() {
        let a = recognize_full("Energy is conserved because E = mc^2");
        let b = recognize_full("Energy is conserved because E = mc^2");
        assert_eq!(a, b);
        let mut sorted = a.features.clone();
        sorted.sort();
        assert_eq!(a.features, sorted, "features must be sorted");
    }

    /// Captures written before the enrichment fields existed must still load.
    #[test]
    fn captures_serialized_before_enrichment_still_deserialize() {
        let legacy = r#"{
            "text": "Entropy is the arrow of time",
            "kinds": ["claimish", "any"],
            "len": 28,
            "cursor_x": 0,
            "cursor_y": 0,
            "captured_at": 1000,
            "source_window": "Obsidian",
            "content_hash": "fnv1a64:dead"
        }"#;
        let c: Capture = serde_json::from_str(legacy).unwrap();
        assert_eq!(c.kinds, vec!["claimish".to_string(), "any".to_string()]);
        assert!(c.domains.is_empty());
        assert!(c.confidence.is_none());
        assert!(c.features.is_empty());
    }

    /// Enrichment must never reach canonical identity. `content_hash` covers
    /// the selected text alone, so improving the classifier cannot make a
    /// previously captured statement look like a different atom.
    #[test]
    fn enrichment_does_not_affect_content_hash() {
        let text = "Entropy is the arrow of time";
        let before = content_hash(text);
        let _ = recognize_full(text);
        assert_eq!(content_hash(text), before);
    }

}
