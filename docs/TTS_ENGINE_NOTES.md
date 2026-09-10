# TTS engine — why it is built this way

Notes for whoever works on `source/src/tts_engine.rs` next.

This subsystem was rewritten because three bugs resisted fixing for roughly two
months. All three came from a single architectural decision, and none of them
can be fixed while that decision stands. If you are about to change how speech
is produced, read this first — the obvious simplifications are the exact thing
that was tried and did not work.

## The rule

**One `SAPI.SpVoice` object, held for the life of the process, on one thread,
reached through `IDispatch`.**

Every part of that sentence is load-bearing. The rest of this document is why.

---

## Bug 1: speech could not be interrupted

**Symptom.** Select text, press the read hotkey, then select something else and
press it again. The second request would not start until the first finished —
or worse, both would talk at once.

**Cause.** The old implementation spawned a new `powershell.exe` per utterance,
each constructing its own `SpeechSynthesizer` and calling a *blocking* `Speak`.

SAPI's interrupt mechanism is the `SVSFPurgeBeforeSpeak` flag, and it only
purges speech queued **on the same voice object**. Two utterances in two
processes have two voice objects, so there is nothing to purge. No flag, delay,
or retry fixes this. The old code tried to compensate by tracking a PID and
calling `taskkill`, which is both slow and racy: the second `Speak` overwrote
the stored PID before the first was killed, orphaning a process that was still
talking and could no longer be stopped.

**Fix.** Hold the voice object. `Cmd::Speak` sends `SPF_ASYNC | SPF_PURGE`,
which cancels whatever is playing and starts immediately.

> **Do not** go back to spawning a process per utterance, however convenient it
> looks. Interruption is the single most-used behaviour in this feature and it
> is unimplementable that way.

## Bug 2: long text stopped partway through

**Symptom.** A long selection would read for a while and then stop mid-sentence
with no error. The workaround was to find where it stopped, delete everything
before that point, and re-trigger.

**Two independent causes**, which is why partial fixes never held:

1. **The command line.** Text was interpolated into
   `powershell -Command "… $synth.Speak('<the entire selection>')"`. The old
   escaping handled `'` and newlines but not `$`, `"`, `;`, `&`, `(`, `)`, or
   curly quotes. A single `$` in the selection ended the string early and
   PowerShell spoke only what came before it. Windows also caps a command line
   near 32 KB, which silently truncates anything longer.

2. **Utterance length.** Even with the shell out of the picture, SAPI degrades
   on very long single utterances. The AutoHotkey version had no shell problem
   and still exhibited this.

**Fix for (1).** Text is passed as a `BSTR` through `IDispatch`. There is no
shell, so there is nothing to escape and no length ceiling. `SPF_NOT_XML` is
also set, so a literal `<` in a selection can never be parsed as markup and
swallow the remainder.

**Fix for (2).** `chunk()` splits at sentence boundaries (~1200 chars), and the
chunks are queued on the voice. SAPI plays them back to back with no audible
seam. Only the *first* chunk carries the purge flag — the rest must not, or each
chunk would cancel the one before it and you would hear only the last one.

> `chunk()` is covered by unit tests that assert the pieces reassemble to
> exactly the input. Splitting text that a user is listening to must never lose
> or duplicate a word; keep those tests passing.

## Bug 3: high speech rates "got hung up"

Mostly a symptom of Bug 2 — a faster rate reaches the truncation point sooner,
so it read as rate-related. Separately, the rate was passed through unclamped;
SAPI's valid range is −10..10 and out-of-range values are accepted by some
voices and garbled by others. Now clamped in `Engine::set_rate`.

---

## Why `IDispatch` and not the typed `ISpVoice`

This is the part most likely to be "cleaned up" by someone who does not know.

windows-rs exposes a proper typed `ISpVoice`. It is nicer to write. **It cannot
see most of the voices on this machine.**

Voices supplied by NaturalVoiceSAPIAdapter — every "Online (Natural)" voice,
which here is 17 of 19 — are virtual tokens registered under:

```
HKLM\SOFTWARE\Microsoft\Speech\Voices\TokenEnums\NaturalVoiceEnumerator\
```

The low-level SAPI interfaces enumerate real token categories and do not resolve
that enumerator. The automation (`IDispatch`) layer does. This is also why voice
*enumeration* in `tts.rs` shells out to PowerShell rather than using the typed
COM path: PowerShell's `New-Object -ComObject SAPI.SpVoice` is the automation
layer too.

If you switch to the typed interface, you will get a working build, two
voices — David and Zira Desktop — and a robotic default where the user's voice
used to be.

### The VARIANT trap

`windows-core`'s `TryFrom<&VARIANT> for IUnknown` accepts **only `VT_UNKNOWN`**.
SAPI returns objects as `VT_DISPATCH`. The conversion therefore fails, and
because the failure path is an empty `Option`, the symptom is not an error but
an empty voice list and a silent fallback to the SAPI default voice.

`as_disp()` calls `VariantChangeType` to coerce first. Do not remove it. The
failure mode it prevents is "the wrong voice with no error message", which costs
far more to debug than it looks.

---

## Threading

The engine runs on its own thread, initialized `COINIT_APARTMENTTHREADED`, with
a message pump in the receive loop (`recv_timeout` at 5 ms, then
`PeekMessage`/`DispatchMessage`). This mirrors how SAPI is driven from scripting
hosts. The pump lets SAPI deliver its internal async completions; without one,
async speech can stall behind an unserviced message queue.

The 5 ms poll is a deliberate trade: it costs nothing measurable and keeps
hotkey-to-audio latency imperceptible.

## Clipboard handling

`capture_selection()` writes a sentinel, sends Ctrl+C, reads the result, then
restores the user's previous clipboard. The sentinel is what distinguishes
"nothing was selected" from "the selection happens to equal what was already on
the clipboard" — without it, pressing the hotkey with no selection re-reads
whatever was copied earlier, which is confusing and looks like a hang.

Restoring the clipboard matters for a second reason: the clipboard monitor feeds
the 1–10 paste slots, so a read that clobbers the clipboard also evicts a slot
the user was holding. The old implementation did this on every read.

## Voice name matching

Names are matched as case-insensitive substrings against both the description
and the token id, preferring the **shortest** matching description.

The shortest-match rule exists because `"Brian"` is a substring of both
`"Brian Online (Natural)"` and `"BrianMultilingual Online (Natural)"`. Without
it, the user's configured voice silently resolved to a different voice than the
one they picked. Prefer configuring a full, unambiguous name.

---

## The CapsLock hotkey

CapsLock+C is the primary trigger, and **nerve does not implement it.**

Windows' `RegisterHotKey` — which every other nerve binding uses — accepts only
Ctrl/Alt/Shift/Win as modifiers. CapsLock is not one. Claiming it requires a
low-level keyboard hook (`WH_KEYBOARD_LL`).

Rather than add a second keyboard hook to nerve, `integrations/autohotkey/nerve_tts_shim.ahk`
forwards the CapsLock chords to ordinary nerve hotkeys:

| Chord | Forwards to | Action |
|---|---|---|
| CapsLock+C | Ctrl+Alt+T | read selection |
| CapsLock+S | Ctrl+Alt+F9 | stop |
| CapsLock+P | Ctrl+Alt+F7 | pause / resume |

AutoHotkey already runs a hook for the rest of this setup, so this costs nothing
and keeps nerve a plain `RegisterHotKey` application. If those nerve bindings are
ever changed, update the constants at the top of the shim to match.

---

## Testing

`nerve.exe --tts-say "<text>"` speaks one utterance and exits **without taking
the single-instance lock**, so it can be used while the agent is running.

- `--seconds N` — how long to stay alive for async playback (default 20)
- `--interrupt` — speak, wait 2 s, then speak again; the first utterance must cut
  off mid-word. This is the regression test for Bug 1.

Run it with `RUST_LOG=nerve=info` to see the resolved voice and chunk count:

```
TTS engine ready: 19 voice tokens
TTS voice: Microsoft BrianMultilingual Online (Natural) - English (United States)
TTS speak: 2531 chars in 3 chunk(s)
```

A voice count of 0 or 2 means the `IDispatch` path has broken — see above.

## Known characteristics (not bugs)

- **"Online (Natural)" voices render over the network.** A stall on the *first*
  chunk specifically is connectivity, not the engine. David and Zira Desktop are
  the only fully offline voices here.
- **Edge TTS remains a subprocess.** It is a separate synthesizer with its own
  player and cannot be driven by SAPI. It is only reachable by setting
  `tts.engine` to `"edge"` in config; the SAPI path above is the default and the
  one that gets used.

## Future work

A math/notation normalization pass belongs in `read_selection()`, before
`speak()` — a single call site. Existing implementations to draw from live in
`I:\tts-pipeline\scripts` (`theophysics_normalizer.py`, `normalize_only.py`).
