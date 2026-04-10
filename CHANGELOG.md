# Nerve / ClipSync Agent — Development Log

## Session: 2026-04-10

### What We Fixed

#### TTS Engine (Critical — Was Completely Broken)

**Root cause #1: Truncated HTML file**
- `html/tts-engine.html` was only 144 lines — the entire `<script>` block was missing
- No button handlers, no IPC calls, no voice population — the panel was an empty shell
- **Fix**: Copied the complete 435-line file from the downloaded repo

**Root cause #2: `SAPI.SpVoice` COM fails in subprocess context**
- `speak_sapi()` used `SAPI.SpVoice` COM object via PowerShell
- This fails with `0x80045006` (SPERR_NOT_FOUND) when spawned with `CREATE_NO_WINDOW`
- The COM audio session doesn't get a valid output device in hidden subprocess contexts
- **Fix**: Switched primary engine to `System.Speech.SpeechSynthesizer` which works reliably
- Added SAPI.SpVoice as fallback if System.Speech fails

**Root cause #3: Missing Stdio handles on spawned processes**
- `new_hidden_command()` with `CREATE_NO_WINDOW` but no explicit stdio handles
- Windows may hand invalid handles, causing SAPI to fail silently
- **Fix**: Added `.stdout(Stdio::null()).stderr(Stdio::null()).stdin(Stdio::null())` to all spawns

**Root cause #4: Edge-TTS voice name mismatch**
- Config stored SAPI-style names like `"BrianMultilingual Online (Natural)"`
- `edge-playback` needs edge-tts format: `"en-US-BrianMultilingualNeural"`
- **Fix**: Added `map_to_edge_voice()` function that converts friendly names to edge-tts format

**Root cause #5: Dead `winapi` crate conflicting with `windows` crate**
- `Cargo.toml` had both `winapi = "0.3"` and `windows = "0.58"`
- `winapi` was unused but could cause subtle conflicts
- **Fix**: Removed `winapi` dependency entirely

#### Settings Panel Redesign

**Before**: Separate HOTKEYS and HOTSTRINGS tabs with editable rows for both

**After**: Single SHORTCUTS tab with two sections:
- **Registered Hotkeys** — read-only table showing `Ctrl+Alt+C → Clipboard` etc.
- **Hotstrings** — editable table with 3 columns: TRIGGER (caps), DESCRIPTION (new field), EXPANSION

**Changes**:
- `config.rs`: Added `description: String` field to `Hotstring` struct
- `main.rs`: Updated `save_config` IPC handler to parse description from JSON
- `settings.html`: Complete tab bar and layout rewrite

#### Clipboard Panel Redesign

**Before**: Single-column slot cards with 2 lines, 20 per page

**After**:
- **2-column grid** — compact cards side by side
- **22 slots per page** (40 total, 2 pages)
- **3-line slot cards**: title (bold white), description (grey), meta (dim)
- **Big COPY button** at bottom of each card
- **PIN toggle** — pushpin icon pins slots to top of the page (persisted in localStorage)
- **Full dock**: 3 draggable pin icons, lock button, category filters (Docs/Video/Music/Pics), pagination

**History tab**:
- **3 distinct text lines**: title (bold bright white), description (2 lines, grey), detail line (smaller — char count, line count, type hint)
- Big COPY button per item
- PIN button to float items to top

#### Hotstring Engine — Prompt Expansion

**New feature**: Typing `/promptname ` (slash + name + space) anywhere on the system expands the matching prompt template, same as hotstrings.
- Hotstrings take priority over prompts
- Added second matching loop in `hotstrings.rs` keyboard hook
- Matches against `cfg.prompts` by name (case-insensitive)

#### Cloudflare Sync API

**New**: Full sync pipeline for PWA integration

**Desktop side** (`sync_client.rs`):
- `push_clip()` — pushes each clipboard change to `/clipboard/push`
- `push_config()` — NEW — pushes full config (hotkeys, hotstrings, prompts, TTS, clips) on every Settings save
- `sync_loop()` — pulls hotkeys, hotstrings, prompts (NEW), and clipboard history periodically
- Prompts now sync bidirectionally

**Cloudflare Worker** (`worker/src/index.js`):
- KV-backed REST API with Bearer token auth
- Endpoints: `/clipboard/push`, `/clipboard/history`, `/config/hotkeys`, `/config/hotstrings`, `/config/prompts`, `/config/tts`, `/config/sync` (full push), `/config/all` (full dump for PWA)
- CORS enabled for PWA access

**Scripts**:
- `setup.bat` — builds release, installs to AppData, creates startup shortcut, optional worker deploy
- `worker/deploy.bat` — step-by-step Cloudflare Worker deployment

### Files Modified

| File | Change |
|---|---|
| `src/config.rs` | Added `description` field to `Hotstring` struct |
| `src/main.rs` | Updated `save_config` handler for description; added `push_config` call after save |
| `src/hotstrings.rs` | Added prompt expansion loop after hotstring matching |
| `src/tts.rs` | Replaced SAPI.SpVoice with System.Speech; added Stdio handles; added edge voice mapping |
| `src/sync_client.rs` | Added `push_config()`, added prompts pull to `sync_loop()` |
| `Cargo.toml` | Removed dead `winapi` dependency |
| `html/settings.html` | Complete redesign — merged SHORTCUTS tab |
| `html/clipboard.html` | Complete redesign — 2-col grid, 3-line cards, pin, dock |
| `html/tts-engine.html` | Restored from truncated 144 lines to full 435 lines |
| `setup.bat` | NEW — full build/install/deploy script |
| `worker/*` | NEW — Cloudflare Worker sync API |

### Lessons Learned

1. **Always check file length** — the TTS panel was broken simply because the HTML was truncated. The Rust code was fine.
2. **SAPI.SpVoice COM fails silently in hidden subprocesses** — `System.Speech.SpeechSynthesizer` is the reliable alternative for .NET-capable systems.
3. **Edge-tts voice names are a different format** — SAPI uses "Microsoft Brian Online (Natural)", edge-tts uses "en-US-BrianNeural". Need a mapper.
4. **Always set Stdio handles on hidden processes** — `CREATE_NO_WINDOW` without explicit handles = undefined behavior.
5. **Don't mix `winapi` and `windows` crates** — they cover the same APIs and can conflict.
6. **Config files accumulate garbage** — empty hotkey entries (`keys: ""`) cause startup errors. Clean on save.
