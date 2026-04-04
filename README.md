# Nerve — Round 2 Debug & Stabilize Script

**Repo:** https://github.com/YellowKidokc/nerve
**Branch:** `main`
**Previous pass:** Codex PR #5 reorganized files, resolved merge regressions, added TTS panel + IPC plumbing.

---

## Context for the debugging agent

Nerve is a Windows desktop micro-agent (Rust + WebView2). It runs as a system-tray app managing clipboard sync, global hotkeys, a hotstring engine, TTS, and multiple WebView2 panels. The companion Cloudflare Workers PWA is a separate repo — this agent just needs `sync_client.rs` wired to push/pull correctly.

Codex did a reorganization pass (PR #5) that cleaned up major merge damage. This script is the **second-round audit**. Walk through every task below in order. For each one, read the relevant source files, identify every remaining bug, fix it, and move on. Do not skip files. Do not assume things compile — verify.

---

## TASK 0 — Verify the build compiles

Run `cargo build` (or read every `.rs` file and trace imports/types manually). The project targets Windows (`windows_subsystem = "windows"` in release). Identify every compile error before changing anything. List them all in a comment at the top of your commit message.

Known crate versions (from Cargo.toml):
- `tao 0.34`, `wry 0.47`, `tray-icon 0.19`, `global-hotkey 0.6`
- - `clipboard-win 5`, `windows 0.58`
  - - `reqwest 0.12` (rustls-tls, json), `tokio 1`
    - - `serde 1`, `serde_json 1`, `tracing 0.1`, `anyhow 1`, `dirs 6`
     
      - **Check specifically:**
      - 1. Does `wry 0.47` `WebViewBuilder::new()` exist, or does it require `WebViewBuilder::new(&window)` ? The `wry` API changed between versions — in 0.47 `WebViewBuilder::new()` takes no arguments and you call `.build(&window)` at the end. Verify panels.rs matches the actual API.
        2. 2. Does `tao 0.34` have `EventLoopBuilder::<AppEvent>::with_user_event().build()` ? Or is it `EventLoop::with_user_event()` ?
           3. 3. `windows 0.58` — does `KEYBDINPUT` use field `dwExtraInfo: 0` or `dwExtraInfo: ULONG_PTR(0)` ? Check the actual type.
              4. 4. `global-hotkey 0.6` — does `GlobalHotKeyEvent::receiver()` exist or is it just `GlobalHotKeyEvent::receiver` (a static)? Is it `try_recv()` or `recv()` ?
                 5. 5. `tray-icon 0.19` — does `Menu::new()` take no args? Does `menu.append()` exist or is it `menu.append_items()` ?
                    6. 6. `clipboard-win 5` — is the API `get_clipboard(formats::Unicode)` or something else?
                       7. 7. `tao::platform::windows::WindowExtWindows` — does `.hwnd()` return `isize` or `*mut c_void` ? The cast in `window_mgmt.rs` may be wrong.
                         
                          8. **Fix every compile error you find.** Do not leave `// TODO` stubs for compile errors.
                         
                          9. ---

                          ## TASK 1 — Audit `src/config.rs`

                          The config file looks clean after Codex's pass but verify:

                          1. `TtsConfig` struct is defined exactly once.
                          2. 2. `Config` struct has `tts: TtsConfig` exactly once.
                             3. 3. All `default_*` functions exist and are referenced correctly.
                                4. 4. `default_panels()` references HTML files that actually exist in `html/`:
                                   5.    - `clipboard.html`, `prompts.html`, `research.html`, `chat.html`, `dashboard.html`, `settings.html`, `tts.html`, `task-calendar.html`, `theophysics-hub.html`
                                         -    - Currently `links` panel points to `research.html` — is that intentional or should there be a separate `links.html`? If `links.html` doesn't exist in `html/`, either create it or have `links` panel point to `research.html` (current behavior is fine, just note it).
                                              - 5. `Config::html_dir()` resolves to `<exe_dir>/html/` — make sure the path joining works on Windows with backslashes and `file:///` URLs.
                                                6. 6. `normalize()` method backfills missing hotkeys and panels — make sure it doesn't create duplicates on repeated calls.
                                                  
                                                   7. ---
                                                  
                                                   8. ## TASK 2 — Audit `src/panels.rs`
                                                  
                                                   9. This is the most complex file. Check:
                                                  
                                                   10. 1. **IPC handler initialization script** — The `with_initialization_script` block defines `window.chrome.webview.postMessage`. Does this actually work with WebView2 IPC? In wry, the IPC handler receives messages from `window.ipc.postMessage()`. The init script bridges `window.chrome.webview.postMessage` → `window.ipc.postMessage`. **Verify the bridge actually calls `window.ipc.postMessage`** — currently it does NOT. The init script sets `postMessage: (msg) => window.ipc.postMessage(msg)` which is correct. Good.
                                                      
                                                       2. 2. **`send_to_panel` method** — It does `serde_json::to_string(json_payload)` which double-encodes the JSON (the payload is already a JSON string, and `to_string` wraps it in quotes with escapes). The `dispatchEvent(new MessageEvent('message', { data: ... }))` then sends a string-of-a-string. **The HTML panels need to call `JSON.parse(event.data)` and then `JSON.parse()` again on the result, or the send_to_panel method needs to NOT double-encode.**
                                                         
                                                          3.    Fix: Change `send_to_panel` to inject the raw JSON directly:
                                                          4.   ```rust
                                                                  fn send_to_panel(&self, panel_name: &str, json_payload: &str) {
                                                                      if let Some(panel) = self.windows.get(panel_name) {
                                                                          let script = format!(
                                                                              "window.dispatchEvent(new MessageEvent('message', {{ data: {} }}));",
                                                                              json_payload
                                                                          );
                                                                          let _ = panel.webview.evaluate_script(&script);
                                                                      }
                                                                  }
                                                                  ```

                                                               3. **`handle_ipc` — `speak_text` handler** — It acquires the config lock four separate times in a row (once for engine, voice, speed, volume). These should be consolidated into a single lock acquisition to avoid unnecessary contention and potential deadlock if another thread holds the lock.
                                                            
                                                               4. 4. **`handle_ipc` — `tts_download` handler** — It calls `tts::download_audio()` which saves to a file. But there's no IPC response back to the panel telling it where the file was saved or offering a download link. The panel will just fire and forget. At minimum, after `download_audio` returns, send a response like `{"type":"download_complete","path":"tts-1234.wav"}`.
                                                                 
                                                                  5. 5. **`handle_ipc` — `Incoming` struct** — It deserializes the whole `Config` for `save_config`. Make sure the HTML panels actually send a full Config object (they probably don't — they likely send partial updates). If the panels send partial config, this will fail to deserialize. Consider making `config` field `Option<serde_json::Value>` and doing a merge instead.
                                                                    
                                                                     6. 6. **Missing IPC handlers** — The HTML panels may send message types that aren't handled:
                                                                        7.    - `get_prompts` — prompts panel needs to load prompts
                                                                              -    - `save_prompts` — prompts panel needs to save prompts
                                                                                   -    - `get_hotstrings` / `save_hotstrings` — settings panel
                                                                                        -    - `get_hotkeys` / `save_hotkeys` — settings panel
                                                                                             -    - Check each HTML panel's JavaScript to see what IPC messages it sends, and make sure `handle_ipc` has a handler for each one. Add `tracing::warn!` for unrecognized message types in the `_ => {}` catch-all.
                                                                                              
                                                                                                  - ---

                                                                                                  ## TASK 3 — Audit `src/main.rs`

                                                                                                  1. **`AppEvent::HotkeyTriggered(u32)`** variant is defined but never used (hotkeys are checked inline via `GlobalHotKeyEvent::receiver()`). Either remove it or use it. Dead code is confusing.

                                                                                                  2. **Hotkey polling** — The code calls `global_hotkey::GlobalHotKeyEvent::receiver().try_recv()` at the top of every event loop iteration. But this only processes ONE hotkey event per loop iteration. If multiple hotkey events queue up, they'll be delayed. Use a `while let Ok(event) = ... { }` loop instead of `if let Ok(event) = ...`.
                                                                                              
                                                                                                  3. 3. **Lock contention** — The hotkey handler holds `cfg.lock()` while sending proxy events. The proxy.send_event is synchronous and shouldn't block, but holding the lock during the match arms is unnecessary. Clone the action string and drop the lock before the match.
                                                                                                    
                                                                                                     4. 4. **`_` wildcard in UserEvent match** — The `_ => {}` arm at the end of the `AppEvent` match silently swallows `HotkeyTriggered`. Remove the wildcard and handle all variants explicitly, or add a comment explaining why it exists.
                                                                                                       
                                                                                                        5. 5. **Window close events** — There's no handler for `WindowEvent::CloseRequested`. When a user clicks the X button on a panel window, does it close? Does the app crash? Tao may close the window automatically, but the `PanelManager.windows` HashMap still holds a reference to the dead window. Add a `CloseRequested` handler that removes the panel from the manager (or at minimum hides it).
                                                                                                          
                                                                                                           6. 6. **`WindowEvent::Moved`** — This saves panel position on every move. On Windows, window dragging generates hundreds of Moved events. Consider debouncing (e.g., only save on `WindowEvent::Resized` or when the window focus changes).
                                                                                                             
                                                                                                              7. ---
                                                                                                             
                                                                                                              8. ## TASK 4 — Audit `src/tts.rs`
                                                                                                             
                                                                                                              9. 1. **`download_audio` mutates global settings** — It modifies TTS_SETTINGS as a side effect. If the user downloads with different settings than their configured defaults, their defaults are now overwritten. Clone the settings instead of mutating them, or save and restore after.
                                                                                                                
                                                                                                                 2. 2. **`save_audio` path** — `download_audio` saves to `tts-{timestamp}.wav` with a relative path. This will save relative to the working directory (probably the exe directory). It should save to a user-accessible location like `dirs::download_dir()` or `dirs::desktop_dir()`, and the path should be returned to the caller.
                                                                                                                   
                                                                                                                    3. 3. **`speak` spawns PowerShell but doesn't track the process** — Multiple speak calls will stack up. The second call doesn't cancel the first. Add a mechanism to kill previous speech before starting new speech (or at least document this behavior).
                                                                                                                      
                                                                                                                       4. 4. **`stop()` creates a NEW SpeechSynthesizer just to call `SpeakAsyncCancelAll()`** — This won't stop speech from the previous `speak()` call because that was a DIFFERENT SpeechSynthesizer instance. SAPI `SpeakAsyncCancelAll` only cancels async speech on the same instance. Fix: track the PowerShell process PID from `speak_sapi()` and kill it in `stop()`, OR use a single long-lived synthesizer instance.
                                                                                                                         
                                                                                                                          5. 5. **Edge TTS fallback** — `speak_edge_tts` falls back to `speak_sapi` on error, but passes the already-escaped text (with `\"` sequences). The SAPI escaping is different (`''` for quotes). The double-escaped text will sound wrong. Pass the original unescaped text to the fallback.
                                                                                                                            
                                                                                                                             6. 6. **`send_ctrl_c` / `get_clipboard_text`** — These are duplicated between `tts.rs` and `clipboard.rs`. Factor them out into a shared utility module (or have `tts.rs` call `clipboard.rs` functions).
                                                                                                                               
                                                                                                                                7. ---
                                                                                                                               
                                                                                                                                8. ## TASK 5 — Audit `src/clipboard.rs`
                                                                                                                               
                                                                                                                                9. 1. **`monitor` never saves config to disk** — When a new clip is added to `clip_slots`, the config in memory is updated but `save()` is never called. If the app crashes, recent clips are lost. Add `c.save()` after truncating.
                                                                                                                                  
                                                                                                                                   2. 2. **Thread safety of `slot_cache`** — `refresh_slots_from_config` is called from both the clipboard monitor thread and the main thread (on `ConfigReloaded`). The `Mutex<Vec<String>>` handles this correctly, but verify there's no scenario where the slot cache and `Config.clip_slots` get out of sync.
                                                                                                                                     
                                                                                                                                      3. 3. **Clipboard race condition** — The monitor reads the clipboard every 500ms. If the user does Ctrl+C and then immediately Ctrl+Shift+1 to paste slot 1, the slot cache might not have updated yet. The `paste_slot` reads from the slot cache, not from config. This is probably fine but worth noting.
                                                                                                                                        
                                                                                                                                         4. ---
                                                                                                                                        
                                                                                                                                         5. ## TASK 6 — Audit `src/hotstrings.rs`
                                                                                                                                        
                                                                                                                                         6. 1. **`vk_to_char` only handles lowercase** — Shift+key combinations are not handled. If a user types a hotstring trigger that includes uppercase letters, it won't match because the buffer only contains lowercase. Either handle shift state or do case-insensitive matching on the buffer.
                                                                                                                                           
                                                                                                                                            2. 2. **`expand_hotstring` uses clipboard for pasting** — This overwrites whatever is currently on the clipboard. The user's clipboard content is lost after every hotstring expansion. Save and restore the clipboard content around the expansion.
                                                                                                                                              
                                                                                                                                               3. 3. **`SENDING` flag race condition** — The flag is set inside the spawned thread (`expand_hotstring`), but the keyboard hook runs on the hook thread. Since `SENDING` is thread-local, the flag set in the expansion thread has NO EFFECT on the hook thread. The hook proc will still process injected keystrokes from the expansion. This could cause infinite loops if the expansion text contains a hotstring trigger. **This is a critical bug.** Fix: use an `AtomicBool` instead of thread-local `RefCell<bool>`.
                                                                                                                                                 
                                                                                                                                                  4. ---
                                                                                                                                                 
                                                                                                                                                  5. ## TASK 7 — Audit `src/hotkeys.rs`
                                                                                                                                                 
                                                                                                                                                  6. 1. Generally clean. Verify that `global_hotkey 0.6`'s `HotKey::new(Some(modifiers), code)` signature is correct. Some versions use `HotKey::new(modifiers, code)` without the Option wrapper.
                                                                                                                                                    
                                                                                                                                                     2. 2. **Re-registration** — When config is reloaded, `register_all` is called with a new `GlobalHotKeyManager`. The old manager is dropped, which should unregister old hotkeys. But verify that global-hotkey actually unregisters on drop. If not, you'll get duplicate registrations.
                                                                                                                                                       
                                                                                                                                                        3. ---
                                                                                                                                                       
                                                                                                                                                        4. ## TASK 8 — Audit `src/sync_client.rs`
                                                                                                                                                       
                                                                                                                                                        5. 1. **`pull_config` only syncs hotstrings and clip history** — It doesn't sync prompts, hotkeys, panels, or TTS config from the remote. The README says the Cloudflare API should support push/pull for config, prompts, and clips. Add stub endpoints for:
                                                                                                                                                           2.    - `GET /config/prompts` → pull prompts
                                                                                                                                                                 -    - `POST /config/prompts` → push prompts
                                                                                                                                                                      -    - `GET /config/full` → pull full config
                                                                                                                                                                           -    - `POST /config/full` → push full config
                                                                                                                                                                            
                                                                                                                                                                                -    These can be no-ops if the API isn't built yet, but the client-side code should be structured to call them.
                                                                                                                                                                            
                                                                                                                                                                                -    2. **Push is fire-and-forget** — `push_clip` is called from a spawned thread and errors are just logged. That's fine for now, but add a counter or last-sync status that can be displayed in the dashboard.
                                                                                                                                                                                 
                                                                                                                                                                                     3. 3. **No push for config changes** — When the user saves config via IPC, it's saved locally but never pushed to the Cloudflare API. Add `push_config()` that sends the full config to the API after local save.
                                                                                                                                                                                       
                                                                                                                                                                                        4. ---
                                                                                                                                                                                       
                                                                                                                                                                                        5. ## TASK 9 — Audit `src/tray.rs`
                                                                                                                                                                                       
                                                                                                                                                                                        6. 1. **Missing tray items** — The tray menu has: Clipboard, Prompts, Links, Research, AI Chat, Dashboard, Settings, Quit. But the app also has TTS and Task Calendar and Theophysics Hub panels. Add tray items for the missing panels.
                                                                                                                                                                                          
                                                                                                                                                                                           2. 2. **Separator used twice** — `item_separator` is appended twice. Each `PredefinedMenuItem::separator()` is a unique item — can the same instance be appended twice? Test this. If not, create a second separator.
                                                                                                                                                                                             
                                                                                                                                                                                              3. ---
                                                                                                                                                                                             
                                                                                                                                                                                              4. ## TASK 10 — Audit `src/window_mgmt.rs`
                                                                                                                                                                                             
                                                                                                                                                                                              5. 1. **`HWND` cast** — `window.hwnd()` in tao returns an `isize` (or `HWND` type depending on version). The cast `HWND(hwnd as *mut _)` may not compile if `hwnd` is already an `HWND` or if the types don't match. Verify against `tao 0.34`.
                                                                                                                                                                                                
                                                                                                                                                                                                 2. 2. **`set_always_on_top` and `get_window_position`** — These functions exist but are never called from anywhere. Either wire them up or mark as `#[allow(dead_code)]`.
                                                                                                                                                                                                   
                                                                                                                                                                                                    3. ---
                                                                                                                                                                                                   
                                                                                                                                                                                                    4. ## TASK 11 — Audit all HTML panels
                                                                                                                                                                                                    
                                                                                                                                                                                                    For each file in `html/`, check:
                                                                                                                                                                                                    1. IPC messages sent via `window.chrome.webview.postMessage(JSON.stringify({...}))` — verify the message types match what `handle_ipc` in `panels.rs` expects.
                                                                                                                                                                                                    2. 2. IPC responses received via `window.addEventListener('message', ...)` — verify they parse correctly given the `send_to_panel` encoding (see TASK 2 point 2).
                                                                                                                                                                                                       3. 3. Design language consistency — all panels should use: IBM Plex Mono font, `#0a0a0f` background, amber `#f59e0b` accents, green `#10b981` accents, "POF 2828" branding. If any panel has a different design, update it to match.
                                                                                                                                                                                                          4. 4. The standalone `tts.html` panel must have: text input area, text normalization toggles (strip URLs, hashtags, @mentions, math notation, code blocks, emojis — just checkboxes with the structure, not all filters need to work yet), voice selector dropdown, speed/volume/pitch sliders, Speak/Pause/Stop buttons, and an audio Download button that sends `tts_download` IPC message.
                                                                                                                                                                                                            
                                                                                                                                                                                                             5. ---
                                                                                                                                                                                                            
                                                                                                                                                                                                             6. ## TASK 12 — Final checks
                                                                                                                                                                                                            
                                                                                                                                                                                                             7. 1. **No git conflict markers** — grep all files for `<<<<<<<`, `=======`, `>>>>>>>`. Remove any found.
                                                                                                                                                                                                                2. 2. **No `Stop Claude`** — grep all files for "Stop Claude" text (some raw files may have this from the GitHub viewer). Remove if found in actual source.
                                                                                                                                                                                                                   3. 3. **No duplicate function/struct definitions** — grep for `fn default_tts_voice` (should appear exactly once), `struct TtsConfig` (exactly once), `struct Config` (exactly once).
                                                                                                                                                                                                                      4. 4. **`#[allow(dead_code)]`** — remove these if the code is actually used, or if it's truly dead code, delete it.
                                                                                                                                                                                                                         5. 5. **Commit message** — list every change and every bug fixed. Group by file.
                                                                                                                                                                                                                           
                                                                                                                                                                                                                            6. ---
                                                                                                                                                                                                                           
                                                                                                                                                                                                                            7. ## Design references
                                                                                                                                                                                                                            
                                                                                                                                                                                                                            Screenshots in `docs/`:
                                                                                                                                                                                                                            - `chrome_JsXvUna3Mt.png` — TTS panel layout (Voice Settings, Volume/Speed/Pitch sliders, Speak/Pause/Stop buttons)
                                                                                                                                                                                                                            - - `claude_C9yzLt1F0e.png` — Settings/hotkey/hotstring builder (AI-HUB v2 layout with Type, Action, Modifiers, Key, Description, Output/Payload fields; Library table; Add/Save/Test/Delete/Export buttons)
                                                                                                                                                                                                                            
                                                                                                                                                                                                                            ## Repo structure (current, post-Codex PR #5)
                                                                                                                                                                                                                            
                                                                                                                                                                                                                            ```
                                                                                                                                                                                                                            nerve/
                                                                                                                                                                                                                            ├── docs/
                                                                                                                                                                                                                            │   ├── chrome_JsXvUna3Mt.png
                                                                                                                                                                                                                            │   └── claude_C9yzLt1F0e.png
                                                                                                                                                                                                                            ├── html/
                                                                                                                                                                                                                            │   ├── chat.html
                                                                                                                                                                                                                            │   ├── clipboard.html
                                                                                                                                                                                                                            │   ├── dashboard.html
                                                                                                                                                                                                                            │   ├── prompts.html
                                                                                                                                                                                                                            │   ├── research.html
                                                                                                                                                                                                                            │   ├── settings.html
                                                                                                                                                                                                                            │   ├── task-calendar.html
                                                                                                                                                                                                                            │   ├── theophysics-hub.html
                                                                                                                                                                                                                            │   └── tts.html
                                                                                                                                                                                                                            ├── src/
                                                                                                                                                                                                                            │   ├── main.rs
                                                                                                                                                                                                                            │   ├── config.rs
                                                                                                                                                                                                                            │   ├── panels.rs
                                                                                                                                                                                                                            │   ├── tts.rs
                                                                                                                                                                                                                            │   ├── clipboard.rs
                                                                                                                                                                                                                            │   ├── hotkeys.rs
                                                                                                                                                                                                                            │   ├── hotstrings.rs
                                                                                                                                                                                                                            │   ├── sync_client.rs
                                                                                                                                                                                                                            │   ├── tray.rs
                                                                                                                                                                                                                            │   └── window_mgmt.rs
                                                                                                                                                                                                                            ├── Cargo.toml
                                                                                                                                                                                                                            ├── Cargo.lock
                                                                                                                                                                                                                            ├── README.md
                                                                                                                                                                                                                            ├── install.ps1
                                                                                                                                                                                                                            └── .gitignore
                                                                                                                                                                                                                            ```
