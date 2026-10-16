# API PIPELINE inside Nerve

Open **Station Console** from Nerve’s tray, or press **Ctrl+Alt+W**. This is a local HTML/WebView panel using Nerve IPC; no localhost server is needed. The black-and-gold styles and Run / Settings / Results / Backside navigation come from the supplied prototype.

## Wired

- Only `01_CKG` (`20_CKG_RUN`, `25_CKG_V4`) and `041_STORIES` (`41_STORY`, `24_CRAFT_PASS`) are shown. The direct-root Stories station is grouped correctly even when `console.py` returns an empty family.
- Inventory, buttons, counts and paths come from the existing Station Console’s `console.py list`. Runtime paths use `PipelineConfig.roots` (default `D:\GitHub\pipeline-workflows\API`), never a bundled snapshot.
- Run starts the station’s actual `.bat`, streams stdout/stderr, accepts answers over stdin, reports its exit code, and stops the child process tree. All discovered buttons are available, including **3 REPAIR FINISHED PAPERS**. Logs remain available when switching stations or roots while this panel stays open.
- Settings reads/writes the station’s existing `FOCUS.md`. Worker preferences are retained per root/station in WebView local storage, captured at run start, and sent over stdin when the standard `How many at once? (1-30, Enter = N)` question arrives. **Send workers** is a fallback for changed prompts. Other questions, selections, confirmations, and a final batch `PAUSE` require an answer in the log’s input box.
- Results lists OUTBOX files recursively, newest first, excluding symlinks and nested station internals. Click a supported document/image to open it with its Windows association. Nested station results belong to that station. Refresh reads disk again; counts refresh at process exit. File timestamps are not proof that a run succeeded.
- Backside reads `BACKSIDE/ckg.json` and its `calls/<folder>/prompt.md` and `output.md`. Prompt and output templates are editable side by side. Up/down/position controls reorder execution independently from assembled document order; **Save order** writes only that order field, preserving IDs and other settings. Entries without a `run_order` (such as ORIGINAL ARTICLE) never acquire an execution step through page reordering.
- Every changed editable file gets a unique timestamped sibling backup (`<name>.console-<timestamp>.bak`) before writing. Only existing FOCUS files and the four stations’ exact Backside allowlist can be changed. Raw JSON edits and station-script edits are refused. A two-file save is sequential: if one fails, the status reports the error and edits remain dirty for retry; each completed write has its own backup.

## Boundaries and validation

No station engine, script, prompt, API key, or sample activity is bundled into the panel. Credentials remain in the Windows user environment (`DEEPSEEK_API_KEY`). There are no invented progress percentages or costs. Unsupported Backside editors display an explicit message. Run output is bounded to the last 500,000 characters per run. Keep this panel open until runs finish; reopening/restarting Nerve does not reattach the in-memory output history.

The temporary `integrations/pipeline-snapshot` reference copy was removed as its README requested. The real pipeline repository remains authoritative.

Linux cloud validation covers the production Rust filesystem module and browser interactions with a test-only IPC peer; it cannot execute Windows station buttons, open Windows Explorer, or validate paid API results. The full desktop binary must also be built and exercised on Windows. A browser IPC test is not evidence that a paper was processed.

## Rebuild and check

On Windows, with Rust/MSVC build tools and WebView2 installed, close the running Nerve instance and run `setup.bat` from the repository root. It builds `source`, copies `nerve.exe` and `source/html` into `%LOCALAPPDATA%\ClipSync`, and offers to launch. The optional Cloudflare Worker deployment is unrelated to this panel and can be skipped. Existing hotkey/tray wiring and the `station-console` panel entry are reused.

Portable backend checks (Rust installed):

```sh
cargo test --manifest-path source/tests/pipeline-check/Cargo.toml
```

Browser checks, with Python Playwright and Chromium installed (opens a `file:` URL; no server):

```sh
python source/tests/panel_ui.py
```

Set `NERVE_TEST_CHROMIUM` to an existing Chromium executable if needed. For Linux compile checking with a MinGW cross compiler and Rust Windows target installed:

```sh
cd source
cargo check --target x86_64-pc-windows-gnu -j 4
```

Windows acceptance checks still required:

1. Open Station Console. Select `25_CKG_V4`, choose **2 RUN ON FOLDER**, and supply a single real paper, for example `C:\Users\David\Documents\faiththruphysics.com\Canon\AXIOM\Good\AXIOM\01_GOD_AS_ROOT\AX_GI_01_SHORT_GOD_AS_ROOT.md`. Answer the station’s remaining questions and confirmation. Verify actual SYNOPSIS → LADDER → LEGS → ARGUMENTS → CLAIMS → EVIDENCE → WALK → AUDIT output and the resulting `<paper> · CKG.md` under Results.
2. Edit and save a V4 prompt; verify the new disk content and the previous content in its sibling backup. Test run order and page order independently.
3. Exercise Stories and Craft Pass, including **3 REPAIR FINISHED PAPERS**, and verify actual output, exit code and files. Repair’s behavior and whether a button calls an API remain defined by the pipeline.
