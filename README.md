# Nerve — Debug & Stabilize Pass

**Repo:** https://github.com/YellowKidokc/nerve
**Branch:** `main` (this is the baseline)

## What this is

Nerve is a Windows desktop micro-agent built in Rust. It runs as a system tray app and manages WebView2 panels (clipboard manager, TTS, prompts, links, research, dashboard, settings, chat, task calendar, theophysics hub). It uses global hotkeys (Ctrl+Alt+C, P, L, R, A, G, S, T), clipboard monitoring, text-to-speech via Windows SAPI, hotstring expansion, and config sync via a Cloudflare Workers API.

## Tech stack

- **Rust** with `tao` (window management), `wry` (WebView2), `global-hotkey`, `clipboard-win`, Win32 APIs
- - **HTML/JS** panels loaded as local `file:///` URLs in WebView2 windows
  - - **IPC** between Rust and HTML panels via `window.chrome.webview.postMessage` / `ipc_handler`
    - - Config stored as JSON at `%APPDATA%/clipsync-agent/config.json`
     
      - ## Current repo structure
     
      - ```
        nerve/
        ├── Cargo.toml
        ├── html/                          ← OLD panels (being replaced)
        │   ├── chat.html
        │   ├── clipboard.html
        │   ├── dashboard.html
        │   ├── links.html
        │   ├── prompts.html
        │   ├── research.html
        │   ├── settings.html
        │   └── shortcuts.html
        ├── clipboard3.html                ← NEW replacement for html/clipboard.html
        ├── nexus-dashboard.html           ← NEW replacement for html/dashboard.html
        ├── prompt_picker.html             ← NEW replacement for html/prompts.html
        ├── research_links.html            ← NEW replacement for html/links.html + html/research.html
        ├── task-calendar.html             ← NEW panel (task/calendar management)
        ├── theophysics-hub.html           ← NEW panel (theophysics research hub)
        ├── chrome_JsXvUna3Mt.png         ← Design reference: TTS panel layout
        ├── claude_C9yzLt1F0e.png         ← Design reference: Settings/hotkey/hotstring builder
        └── src/
            ├── main.rs          (event loop, hotkey dispatch, IPC routing)
            ├── config.rs         (Config struct, load/save, defaults)
            ├── panels.rs         (PanelManager, WebView2 creation, IPC handling)
            ├── tts.rs            (text-to-speech via SAPI/Edge)
            ├── clipboard.rs      (clipboard monitoring, slot cache)
            ├── hotkeys.rs        (global hotkey registration)
            ├── hotstrings.rs     (keyboard hook, text expansion)
            ├── sync_client.rs    (Cloudflare API sync)
            ├── tray.rs           (system tray icon/menu)
            └── window_mgmt.rs    (dark titlebar, window utilities)
        ```

        ## TASK 1: Move and rename the new HTML panels

        The new panel HTML files are in the repo root. They need to be moved into `html/` and renamed to match what `src/config.rs` expects:

        1. `clipboard3.html` → `html/clipboard.html` (replace old)
        2. 2. `nexus-dashboard.html` → `html/dashboard.html` (replace old)
           3. 3. `prompt_picker.html` → `html/prompts.html` (replace old)
              4. 4. `research_links.html` → `html/research.html` (replace old, also replaces html/links.html functionality)
                 5. 5. `task-calendar.html` → `html/task-calendar.html` (new panel — add to config defaults)
                    6. 6. `theophysics-hub.html` → `html/theophysics-hub.html` (new panel — add to config defaults)
                       7. 7. Delete the old HTML files that were replaced
                          8. 8. Move the `.png` screenshots into a `docs/` folder for reference
                            
                             9. ## TASK 2: Fix critical Rust compile errors from bad merge
                            
                             10. ### src/config.rs
                             11. - `TtsConfig` struct is defined TWICE (duplicate definition around line 60 and again around line 130). Remove the second copy.
                                 - - The `Config` struct has `pub tts: TtsConfig` declared TWICE. Remove the duplicate field.
                                   - - `default_tts_voice`, `default_tts_speed`, `default_tts_engine`, `default_tts_volume` functions are all duplicated. Remove the second copies.
                                     - - Add `task-calendar` and `theophysics-hub` to `default_panels()`.
                                      
                                       - ### src/panels.rs
                                       - - Has LITERAL leftover Git conflict markers (`<<<<<<< codex/...`, `=======`, `>>>>>>> main`) in the `PanelManager::new()` method. Remove them.
                                         - - The `PanelManager` struct needs BOTH `window_to_name: HashMap<WindowId, String>` AND `proxy: EventLoopProxy<AppEvent>`. Make sure both fields exist.
                                           - - The `new()` constructor must accept a proxy parameter and store it.
                                             - - `create_panel()` has TWO `with_ipc_handler` calls on the same WebViewBuilder. One uses `AppEvent::IpcMessage(String, String)`, the other uses `AppEvent::IpcMessage { panel, body }`. Pick the tuple variant `IpcMessage(String, String)` and remove the duplicate handler.
                                               - - `send_to_panel()` references `panel._webview` — should be `panel.webview`.
                                                 - - `self.proxy.clone()` is called but proxy may not be stored properly in the struct.
                                                  
                                                   - ### src/main.rs
                                                   - - The `AppEvent` enum likely has conflicting `IpcMessage` variants from the merge. Unify to ONE variant: `IpcMessage(String, String)` where the tuple is (panel_name, json_body).
                                                     - - The match arm in the event loop must match the chosen variant.
                                                      
                                                       - ## TASK 3: Verify IPC plumbing
                                                      
                                                       - - All HTML panels use `window.chrome.webview.postMessage(JSON.stringify(msg))` to send messages to Rust.
                                                         - - The Rust side receives these in `panels.rs handle_ipc()` and routes by `msg_type`.
                                                           - - The initialization script in `create_panel()` shims `window.chrome.webview` for WebView2.
                                                             - - `send_to_panel()` dispatches JSON back to the HTML via `window.dispatchEvent(new MessageEvent(...))`.
                                                               - - Make sure the new panels' IPC calls match what the Rust side expects (get_config, save_config, get_clips, list_voices, speak_text, tts_stop, etc).
                                                                
                                                                 - ## TASK 4: Verify TTS
                                                                
                                                                 - - `src/tts.rs` has `configure()`, `apply_config()`, `read_selection()`, `speak()`, and `list_voices()`.
                                                                   - - The TTS panel in clipboard3.html has integrated TTS controls.
                                                                     - - The design reference `chrome_JsXvUna3Mt.png` shows the target TTS layout (Voice Settings with Interface, Voice, Volume, Speed, Pitch, Speak/Pause/Stop).
                                                                       - - Make sure settings panel can change TTS voice/speed/engine and changes propagate.
                                                                        
                                                                         - ## TASK 5: Settings panel + hotkeys/hotstrings
                                                                        
                                                                         - - The design reference `claude_C9yzLt1F0e.png` shows the target settings layout.
                                                                           - - Settings needs tabs for: Shortcuts, Hotkeys, Hotstrings, TTS config, Sync config.
                                                                             - - Hotkey builder: Type (Hotkey/Hotstring), Action, Modifiers (Ctrl/Alt/Shift/Win), Key, Description, Output/Payload.
                                                                               - - Library table showing all registered hotkeys and hotstrings.
                                                                                 - - Add/Save, Test, Delete, Export .ahk buttons.
                                                                                   - - Enable Hotkeys / Enable Hotstrings toggles.
                                                                                    
                                                                                     - ## Rules
                                                                                    
                                                                                     - - Make it compile cleanly with `cargo build`
                                                                                       - - Unify the IPC pattern across all files
                                                                                         - - Don't redesign the HTML panels — only fix Rust-side issues, IPC wiring, and file organization
                                                                                           - - Keep the dark theme aesthetic (IBM Plex Mono, #0a0a0f background, amber #f59e0b accents)
                                                                                             - - Goal: compile, run, working IPC between Rust and all HTML panels
