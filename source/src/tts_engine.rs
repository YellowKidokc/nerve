//! Persistent SAPI speech engine.
//!
//! One long-lived `SAPI.SpVoice` COM object, owned by one dedicated thread,
//! driven by a command channel. This is deliberately the same shape as the
//! AutoHotkey BetterTTS module it replaces, for the same reason: SAPI's
//! `SVSFPurgeBeforeSpeak` flag only cancels speech queued on *the same voice
//! object*. Spawning a fresh PowerShell per utterance — the previous design —
//! cannot interrupt anything, because each utterance belongs to a different
//! voice in a different process.
//!
//! Two consequences fall out of holding the object:
//!
//! * **Interrupting works.** A new `Speak` with the purge flag stops whatever
//!   is talking and starts immediately, so a second Ctrl+C → speak never has
//!   to wait for the first to finish.
//! * **Text is passed as a `BSTR`, not as shell text.** Nothing is escaped,
//!   quoted, or squeezed through a command line, so `$`, quotes, newlines and
//!   the 32 KB command-line ceiling stop mattering.
//!
//! We talk to SAPI through `IDispatch` rather than the typed `ISpVoice`
//! interface. That is not incidental: voices supplied by NaturalVoiceSAPIAdapter
//! (every "Online (Natural)" voice) are virtual tokens under
//! `Speech\Voices\TokenEnums\NaturalVoiceEnumerator`, and the low-level
//! interface does not enumerate or accept them. The automation layer does.
//!
//! Fuller background — the three bugs this replaced, why each was unfixable in
//! the previous design, and the traps to avoid when changing it — is in
//! `docs/TTS_ENGINE_NOTES.md`. Read it before restructuring this module.

use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender};
use std::sync::{mpsc, Mutex, OnceLock};
use std::time::Duration;

use tracing::{error, info, warn};

use windows::core::{IUnknown, Interface, BSTR, GUID, PCWSTR, VARIANT};
use windows::Win32::System::Com::{
    CLSIDFromProgID, CoCreateInstance, CoInitializeEx, IDispatch, CLSCTX_ALL,
    COINIT_APARTMENTTHREADED, DISPATCH_METHOD, DISPATCH_PROPERTYGET, DISPATCH_PROPERTYPUT,
    DISPATCH_PROPERTYPUTREF, DISPPARAMS, EXCEPINFO,
};
use windows::Win32::System::Variant::{VariantChangeType, VAR_CHANGE_FLAGS, VT_UNKNOWN};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
};

// ── SAPI speak flags ────────────────────────────────────────────────────────
//
// SPF_ASYNC returns immediately instead of blocking until the audio ends.
// SPF_PURGE discards everything already queued on this voice — the interrupt.
// SPF_NOT_XML tells SAPI the string is literal text, so a stray `<` in a
// pasted selection can never be parsed as markup and swallow the rest.

const SPF_ASYNC: i32 = 1;
const SPF_PURGE: i32 = 2;
const SPF_NOT_XML: i32 = 16;

/// DISPID reserved for the value being assigned in a property put.
const DISPID_PROPERTYPUT: i32 = -3;

/// Chunk size for long text, in characters.
///
/// SAPI degrades on very long single utterances — the failure the AutoHotkey
/// version showed as "reads a while, then stops partway with no error". Queued
/// chunks avoid it: SAPI plays them back to back with no audible seam, and a
/// purge still cancels the whole remaining queue at once.
const CHUNK_CHARS: usize = 1200;

// ── Commands ────────────────────────────────────────────────────────────────

pub enum Cmd {
    /// Build the voice object and enumerate tokens without speaking, so the
    /// first real request does not pay that cost.
    Warm,
    /// Speak text, interrupting whatever is currently playing.
    Speak(String),
    /// Cancel all queued and playing speech.
    Stop,
    /// Pause if speaking, resume if paused.
    PauseToggle,
    /// Select a voice by display name or token id (substring, case-insensitive).
    SetVoice(String),
    /// SAPI rate, -10..=10.
    SetRate(i32),
    /// SAPI volume, 0..=100.
    SetVolume(i32),
    /// Render to a .wav file on a throwaway voice, so live speech is undisturbed.
    SaveWav {
        text: String,
        path: String,
        reply: SyncSender<Result<(), String>>,
    },
}

static TX: OnceLock<Mutex<Sender<Cmd>>> = OnceLock::new();

/// Send a command to the speech thread, starting it on first use.
pub fn send(cmd: Cmd) {
    let tx = TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Cmd>();
        std::thread::Builder::new()
            .name("tts-engine".into())
            .spawn(move || run(rx))
            .expect("spawn tts engine thread");
        Mutex::new(tx)
    });

    if let Err(e) = tx.lock().unwrap().send(cmd) {
        error!("TTS engine channel closed: {e}");
    }
}

// ── IDispatch helpers ───────────────────────────────────────────────────────

/// A thin `IDispatch` wrapper: late-bound method calls and property access,
/// which is all the SAPI automation surface needs.
#[derive(Clone)]
struct Disp(IDispatch);

impl Disp {
    fn dispid(&self, name: &str) -> Result<i32, String> {
        let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let names = [PCWSTR(wide.as_ptr())];
        let mut id: i32 = 0;
        unsafe {
            self.0
                .GetIDsOfNames(&GUID::zeroed(), names.as_ptr(), 1, 0, &mut id)
                .map_err(|e| format!("no member '{name}': {e}"))?;
        }
        Ok(id)
    }

    /// `args` is in call order; DISPPARAMS wants it reversed, which we do here
    /// so callers never have to think about it.
    fn invoke(&self, name: &str, flags: u16, args: &[VARIANT]) -> Result<VARIANT, String> {
        let id = self.dispid(name)?;
        let mut reversed: Vec<VARIANT> = args.iter().rev().cloned().collect();

        // A property put names its single argument with DISPID_PROPERTYPUT.
        let is_put = flags & (DISPATCH_PROPERTYPUT.0 | DISPATCH_PROPERTYPUTREF.0) != 0;
        let mut named = DISPID_PROPERTYPUT;

        let params = DISPPARAMS {
            rgvarg: if reversed.is_empty() {
                std::ptr::null_mut()
            } else {
                reversed.as_mut_ptr()
            },
            rgdispidNamedArgs: if is_put {
                &mut named
            } else {
                std::ptr::null_mut()
            },
            cArgs: reversed.len() as u32,
            cNamedArgs: if is_put { 1 } else { 0 },
        };

        let mut result = VARIANT::default();
        let mut excep = EXCEPINFO::default();
        let mut arg_err: u32 = 0;

        unsafe {
            self.0
                .Invoke(
                    id,
                    &GUID::zeroed(),
                    0,
                    windows::Win32::System::Com::DISPATCH_FLAGS(flags),
                    &params,
                    Some(&mut result),
                    Some(&mut excep),
                    Some(&mut arg_err),
                )
                .map_err(|e| {
                    // ManuallyDrop: read the description without taking
                    // ownership of a BSTR that Invoke still owns.
                    let desc = excep.bstrDescription.to_string();
                    if desc.is_empty() {
                        format!("{name}: {e}")
                    } else {
                        format!("{name}: {e} ({desc})")
                    }
                })?;
        }

        Ok(result)
    }

    fn call(&self, name: &str, args: &[VARIANT]) -> Result<VARIANT, String> {
        self.invoke(name, DISPATCH_METHOD.0, args)
    }

    fn get(&self, name: &str) -> Result<VARIANT, String> {
        self.invoke(name, DISPATCH_PROPERTYGET.0, &[])
    }

    fn put(&self, name: &str, value: VARIANT) -> Result<(), String> {
        self.invoke(name, DISPATCH_PROPERTYPUT.0, &[value]).map(|_| ())
    }

    /// Assign an object-valued property (VB's `Set x.P = obj`).
    fn put_ref(&self, name: &str, value: VARIANT) -> Result<(), String> {
        self.invoke(name, DISPATCH_PROPERTYPUTREF.0, &[value])
            .map(|_| ())
    }
}

fn variant_i32(v: i32) -> VARIANT {
    VARIANT::from(v)
}

fn variant_str(s: &str) -> VARIANT {
    VARIANT::from(BSTR::from(s))
}

fn variant_disp(d: &IDispatch) -> VARIANT {
    // VARIANT takes ownership of the interface, so hand it a fresh reference.
    VARIANT::from(d.cast::<IUnknown>().expect("IDispatch is an IUnknown"))
}

fn as_i32(v: &VARIANT) -> i32 {
    i32::try_from(v).unwrap_or(0)
}

/// Pull an interface out of a VARIANT.
///
/// SAPI returns objects as VT_DISPATCH, but `TryFrom<&VARIANT>` in windows-core
/// only accepts VT_UNKNOWN — which is why a direct conversion silently yields
/// nothing and leaves the voice list empty. Coerce the type first.
fn as_disp(v: &VARIANT) -> Option<IDispatch> {
    if let Ok(unk) = IUnknown::try_from(v) {
        return unk.cast::<IDispatch>().ok();
    }

    let mut coerced = VARIANT::default();
    unsafe {
        VariantChangeType(&mut coerced, v, VAR_CHANGE_FLAGS(0), VT_UNKNOWN).ok()?;
    }
    IUnknown::try_from(&coerced).ok()?.cast::<IDispatch>().ok()
}

/// Create a COM object from its ProgID and hand back an `IDispatch` for it.
fn create(prog_id: &str) -> Result<Disp, String> {
    let wide: Vec<u16> = prog_id.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let clsid = CLSIDFromProgID(PCWSTR(wide.as_ptr()))
            .map_err(|e| format!("CLSIDFromProgID({prog_id}): {e}"))?;
        let disp: IDispatch = CoCreateInstance(&clsid, None, CLSCTX_ALL)
            .map_err(|e| format!("CoCreateInstance({prog_id}): {e}"))?;
        Ok(Disp(disp))
    }
}

// ── Chunking ────────────────────────────────────────────────────────────────

/// Split text into utterance-sized pieces, preferring sentence boundaries.
///
/// Falls back to a word boundary, and finally to a hard cut, so that no input —
/// a wall of minified text, say — can produce a chunk larger than the budget.
fn chunk(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= CHUNK_CHARS {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < chars.len() {
        let remaining = chars.len() - start;
        if remaining <= CHUNK_CHARS {
            chunks.push(chars[start..].iter().collect());
            break;
        }

        let window_end = start + CHUNK_CHARS;

        // Prefer the last sentence end in the window; a sentence end is
        // terminal punctuation followed by whitespace.
        let split = (start + CHUNK_CHARS / 3..window_end)
            .rev()
            .find(|&i| {
                matches!(chars[i], '.' | '!' | '?' | '\n')
                    && chars.get(i + 1).is_none_or(|c| c.is_whitespace())
            })
            .map(|i| i + 1)
            // Otherwise the last word break.
            .or_else(|| {
                (start + CHUNK_CHARS / 3..window_end)
                    .rev()
                    .find(|&i| chars[i].is_whitespace())
                    .map(|i| i + 1)
            })
            // Otherwise cut where the budget runs out.
            .unwrap_or(window_end);

        let piece: String = chars[start..split].iter().collect();
        if !piece.trim().is_empty() {
            chunks.push(piece);
        }
        start = split;
    }

    chunks
}

// ── Engine thread ───────────────────────────────────────────────────────────

/// One enumerated voice: its display description, its registry token id, and
/// the automation object needed to select it.
struct VoiceToken {
    desc: String,
    id: String,
    token: IDispatch,
}

struct Engine {
    voice: Disp,
    /// Cached token list, so voice switching does not re-enumerate.
    tokens: Vec<VoiceToken>,
    paused: bool,
}

impl Engine {
    fn new() -> Result<Self, String> {
        let voice = create("SAPI.SpVoice")?;
        let tokens = enumerate_tokens(&voice);
        info!("TTS engine ready: {} voice tokens", tokens.len());
        Ok(Engine {
            voice,
            tokens,
            paused: false,
        })
    }

    /// Speak text, cancelling whatever is playing first.
    ///
    /// Only the first chunk carries the purge flag; the rest queue behind it,
    /// so a long selection plays as one continuous read but a fresh request
    /// still cuts in immediately.
    fn speak(&mut self, text: &str) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }

        // A purge while paused would leave the voice paused with an empty
        // queue, so the new text would never be heard.
        self.resume_if_paused();

        let pieces = chunk(text);
        info!(
            "TTS speak: {} chars in {} chunk(s)",
            text.chars().count(),
            pieces.len()
        );

        for (i, piece) in pieces.iter().enumerate() {
            let flags = if i == 0 {
                SPF_ASYNC | SPF_PURGE | SPF_NOT_XML
            } else {
                SPF_ASYNC | SPF_NOT_XML
            };
            if let Err(e) = self
                .voice
                .call("Speak", &[variant_str(piece), variant_i32(flags)])
            {
                error!("TTS Speak failed on chunk {}: {e}", i + 1);
                break;
            }
        }
    }

    fn stop(&mut self) {
        self.resume_if_paused();
        // An empty purged utterance is SAPI's cancel: it clears the queue and
        // silences the current one without leaving anything to say.
        if let Err(e) = self.voice.call(
            "Speak",
            &[variant_str(""), variant_i32(SPF_ASYNC | SPF_PURGE)],
        ) {
            warn!("TTS stop failed: {e}");
        }
    }

    fn pause_toggle(&mut self) {
        let method = if self.paused { "Resume" } else { "Pause" };
        match self.voice.call(method, &[]) {
            Ok(_) => {
                self.paused = !self.paused;
                info!("TTS {}", if self.paused { "paused" } else { "resumed" });
            }
            Err(e) => warn!("TTS {method} failed: {e}"),
        }
    }

    fn resume_if_paused(&mut self) {
        if self.paused {
            let _ = self.voice.call("Resume", &[]);
            self.paused = false;
        }
    }

    fn set_voice(&mut self, wanted: &str) {
        if wanted.is_empty() || wanted.eq_ignore_ascii_case("default") {
            return;
        }
        if self.tokens.is_empty() {
            self.tokens = enumerate_tokens(&self.voice);
        }

        // Match on the description or the full token path, so config may name
        // either. Among substring matches prefer the shortest description:
        // "Brian" should select "Brian Online (Natural)" rather than the longer
        // "BrianMultilingual Online (Natural)" that also contains it.
        let needle = wanted.to_lowercase();
        let found = self
            .tokens
            .iter()
            .filter(|v| {
                v.desc.to_lowercase().contains(&needle) || v.id.to_lowercase().contains(&needle)
            })
            .min_by_key(|v| v.desc.len());

        match found {
            Some(v) => match self.voice.put_ref("Voice", variant_disp(&v.token)) {
                Ok(()) => info!("TTS voice: {}", v.desc),
                Err(e) => warn!("TTS could not select voice '{wanted}': {e}"),
            },
            None => warn!(
                "TTS voice '{wanted}' not found among {} tokens",
                self.tokens.len()
            ),
        }
    }

    fn set_rate(&self, rate: i32) {
        // SAPI's documented range. Out-of-range values are accepted by some
        // voices and garbled by others, so clamp rather than pass through.
        let rate = rate.clamp(-10, 10);
        if let Err(e) = self.voice.put("Rate", variant_i32(rate)) {
            warn!("TTS set rate failed: {e}");
        }
    }

    fn set_volume(&self, volume: i32) {
        let volume = volume.clamp(0, 100);
        if let Err(e) = self.voice.put("Volume", variant_i32(volume)) {
            warn!("TTS set volume failed: {e}");
        }
    }

    /// Render to a file using a second voice object, so an in-progress read is
    /// not interrupted by the save.
    fn save_wav(&self, text: &str, path: &str) -> Result<(), String> {
        let stream = create("SAPI.SpFileStream")?;
        // SSFMCreateForWrite = 3.
        stream.call("Open", &[variant_str(path), variant_i32(3), variant_i32(0)])?;

        let writer = create("SAPI.SpVoice")?;
        // Mirror the live voice's settings so the file sounds like playback.
        if let Ok(v) = self.voice.get("Voice") {
            if let Some(token) = as_disp(&v) {
                let _ = writer.put_ref("Voice", variant_disp(&token));
            }
        }
        if let Ok(r) = self.voice.get("Rate") {
            let _ = writer.put("Rate", variant_i32(as_i32(&r)));
        }
        if let Ok(vol) = self.voice.get("Volume") {
            let _ = writer.put("Volume", variant_i32(as_i32(&vol)));
        }

        writer.put_ref("AudioOutputStream", variant_disp(&stream.0))?;
        // Synchronous, so the stream is complete before we close it.
        let result = writer.call("Speak", &[variant_str(text), variant_i32(SPF_NOT_XML)]);
        let _ = stream.call("Close", &[]);
        result.map(|_| ())
    }
}

/// Enumerate voice tokens as (description, token) pairs via the automation
/// layer, which is the only view that includes NaturalVoiceSAPIAdapter voices.
fn enumerate_tokens(voice: &Disp) -> Vec<VoiceToken> {
    let tokens = match voice.call("GetVoices", &[]) {
        Ok(v) => match as_disp(&v) {
            Some(d) => Disp(d),
            None => {
                warn!("TTS GetVoices returned a non-object");
                return Vec::new();
            }
        },
        Err(e) => {
            warn!("TTS GetVoices failed: {e}");
            return Vec::new();
        }
    };

    let count = tokens.get("Count").map(|v| as_i32(&v)).unwrap_or(0);
    let mut out = Vec::new();

    for i in 0..count {
        let Ok(item) = tokens.call("Item", &[variant_i32(i)]) else {
            continue;
        };
        let Some(token) = as_disp(&item) else { continue };
        let token_disp = Disp(token.clone());
        let desc = token_disp
            .call("GetDescription", &[])
            .ok()
            .map(|v| BSTR::try_from(&v).map(|b| b.to_string()).unwrap_or_default())
            .unwrap_or_default();
        let id = token_disp
            .get("Id")
            .ok()
            .and_then(|v| BSTR::try_from(&v).ok())
            .map(|b| b.to_string())
            .unwrap_or_default();

        out.push(VoiceToken { desc, id, token });
    }

    out
}

fn run(rx: Receiver<Cmd>) {
    unsafe {
        // Apartment-threaded with a pump, matching how SAPI is used from
        // scripting hosts. The pump lets SAPI deliver its internal async
        // completions instead of stalling behind an unserviced queue.
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }

    let mut engine = match Engine::new() {
        Ok(e) => e,
        Err(e) => {
            error!("TTS engine failed to start: {e}");
            return;
        }
    };

    loop {
        pump_messages();

        match rx.recv_timeout(Duration::from_millis(5)) {
            Ok(Cmd::Warm) => {} // Reaching here means the engine is already built.
            Ok(Cmd::Speak(text)) => engine.speak(&text),
            Ok(Cmd::Stop) => engine.stop(),
            Ok(Cmd::PauseToggle) => engine.pause_toggle(),
            Ok(Cmd::SetVoice(name)) => engine.set_voice(&name),
            Ok(Cmd::SetRate(r)) => engine.set_rate(r),
            Ok(Cmd::SetVolume(v)) => engine.set_volume(v),
            Ok(Cmd::SaveWav { text, path, reply }) => {
                let result = engine.save_wav(&text, &path);
                if let Err(ref e) = result {
                    error!("TTS save failed: {e}");
                }
                let _ = reply.send(result);
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn pump_messages() {
    unsafe {
        let mut msg = MSG::default();
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_is_one_chunk() {
        assert_eq!(chunk("Hello there."), vec!["Hello there."]);
    }

    #[test]
    fn long_text_splits_on_sentence_ends() {
        let sentence = "This is a sentence of some length. ";
        let text = sentence.repeat(200);
        let pieces = chunk(&text);
        assert!(pieces.len() > 1);
        for p in &pieces {
            assert!(p.chars().count() <= CHUNK_CHARS);
        }
        // Splitting must not lose or duplicate anything.
        assert_eq!(pieces.concat(), text);
        // Every break lands after terminal punctuation.
        for p in &pieces[..pieces.len() - 1] {
            assert!(p.trim_end().ends_with('.'), "bad break: {:?}", p);
        }
    }

    #[test]
    fn text_with_no_breaks_is_still_bounded() {
        let text = "x".repeat(CHUNK_CHARS * 3 + 17);
        let pieces = chunk(&text);
        for p in &pieces {
            assert!(p.chars().count() <= CHUNK_CHARS);
        }
        assert_eq!(pieces.concat(), text);
    }
}
