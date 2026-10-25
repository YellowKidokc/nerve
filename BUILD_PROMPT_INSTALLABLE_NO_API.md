# BUILD PROMPT: nerve, installable, with zero API code
POF 2828 | Oct 25, 2026

TASK: Make nerve an installable Windows utility with ZERO API/AI code.

Repo: YellowKidokc/nerve. Rust app lives in /source (tao, wry, tray-icon,
global-hotkey, clipboard-win, windows crate). Start a new branch from
feat/move-ai-panels-html named feat/installable-no-api. Open a PR at the end.

READ RECEIPT FIRST: before writing code, list every module and HTML panel,
say which you will keep, which you will archive, and why. Then proceed.

ARCHITECTURE RULE (non-negotiable): nerve is a tier-1 utility. It is always
on, starts with Windows, and makes NO network calls. All AI and API work
belongs to a separate service (Forge, repo Rust_AI_API). Clipboard sync
belongs to a separate repo (Rust_Clipsync). Background AI jobs belong to
Rust_Idle_Runner. nerve does not absorb any of them.

1. REMOVE ALL API CODE, BUT DELETE NOTHING.
   - Move ai.rs, sync_client.rs, and any AI/provider/Cloudflare/HTTP code
     (including AI paths in canon.rs, review.rs, idle.rs, config.rs,
     main.rs, selection.rs, tts.rs) into /_archive_api/ with a README
     saying what each file did and that Forge now owns this.
     (idle.rs's tested idle-detection patterns are being reused by
     Rust_Idle_Runner; archive, don't delete.)
   - Remove reqwest, and tokio if nothing else needs it, from Cargo.toml.
   - Remove API keys and provider settings from config. Old configs that
     still contain them must load without crashing (ignore unknown fields).
   - TTS stays, but local only: Windows SAPI (Win32_Media_Speech). No
     cloud voices.
   - Move AI/research/theophysics HTML panels into /_archive_panels/.
     Keep only the panels the utility itself uses (clipboard, shortcuts,
     settings, toolbar, prompts, links, and any others the remaining Rust
     code opens). List them in the PR.
   - The build must succeed with zero warnings about unused code from the
     removal.

2. KEEP AND VERIFY THESE FEATURES WORKING:
   tray icon and menu; global hotkeys; clipboard history; selection popup;
   window management; mouse features; local TTS; single-instance lock;
   settings panel.

3. MAKE IT INSTALLABLE.
   - Release build with a Windows icon and version info embedded.
   - Build an MSI installer (cargo-wix preferred; Inno Setup acceptable)
     that: installs to Program Files; adds a Start Menu shortcut; offers
     "Start with Windows" (on by default, via the HKCU Run key); puts
     user config in %APPDATA%\nerve and never overwrites an existing one;
     installs or bootstraps the WebView2 runtime if it is missing; and
     uninstalls cleanly (leaving the user's config unless they choose to
     remove it).
   - Add a GitHub Actions workflow (windows-latest) that builds the MSI on
     every tag v* and attaches it to a GitHub Release. Tag v0.2.0 when done.
   - The same build must also work locally: `cargo build --release` plus a
     documented one-command MSI build in README.

4. ACCEPTANCE (state each result in the PR):
   - Fresh Windows machine: run the MSI, nerve starts, tray icon appears.
   - Hotkeys, clipboard history, selection popup, and TTS work.
   - Reboot: nerve starts automatically.
   - No network: grep the code and Cargo.lock for reqwest/hyper/ureq/http
     clients and confirm none remain. Nerve works fully offline.
   - Uninstall removes the program and the startup entry.
   - Nothing was deleted; everything removed is in /_archive_api/ or
     /_archive_panels/.

5. HOTSTRINGS VIA ESPANSO (separate program; do not copy its code).
   - Espanso (github.com/espanso/espanso, GPL-3.0) owns text expansion.
     nerve must NOT also expand hotstrings: disable hotstrings.rs's
     keyboard hook (move it to /_archive_api/ or behind a config flag
     that defaults to off) so nothing expands twice.
   - On first run, nerve checks whether Espanso is installed. If not,
     it shows a tray notice with a one-click link to the official
     Espanso Windows installer. Do not bundle Espanso inside the MSI
     unless its license terms for redistribution are followed exactly;
     prefer the link.
   - Migrate existing hotstrings: convert nerve's current hotstrings
     config into an Espanso match file at
     %APPDATA%\espanso\match\nerve.yml. Also add an importer for
     AutoHotkey hotstring files (lines like ::btw::by the way) into the
     same format. Never overwrite the user's other Espanso files.
   - Tray menu: "Hotstrings: on/off" (calls `espanso start` / `espanso
     stop`), "Edit hotstrings" (opens nerve.yml), "Reload".
   - Acceptance: typing a migrated trigger in Notepad, a browser, and
     Word expands exactly once.

6. APP-LAUNCH SHORTCUTS (in nerve, using its existing global hotkeys).
   - Add a "launch" action to the hotkey config: hotkey, program path,
     optional arguments, optional working folder. Example:
     { "hotkey": "Ctrl+Alt+O", "action": "launch",
       "path": "C:\\Program Files\\Obsidian\\Obsidian.exe" }
   - Also support opening a folder or a URL the same way.
   - Settings panel: add, edit, test, and delete launch shortcuts, and
     warn when a hotkey is already taken.
   - Acceptance: three configured shortcuts launch their targets; a
     duplicate hotkey is refused with a clear message.

Do not add new features beyond these. Do not re-add any API calls under
another name. If something cannot be done, say exactly what and keep
building the rest.
