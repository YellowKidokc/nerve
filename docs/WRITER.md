# Writer — select-and-fix

Inspired by [everyday-writer](https://github.com/ruwaifatahir/everyday-writer), built into Nerve.

## Use

1. Press **Ctrl+Space** in any text box.
   - Text already selected → that selection is fixed (it is never widened).
   - Nothing selected → Nerve presses Ctrl+A and fixes the whole field.
2. The popup shows the corrected text (editable) and a word diff:
   red = removed, green = added. Dropped details are easy to spot.
3. **Ctrl+Enter** (or Enter on the focused Replace button) pastes it over the original.
   **Esc** cancels. Your clipboard is restored afterwards.

## Feedback box

Type what's wrong ("don't take out the location", "less formal") and press
Enter or **Redo**. The model gets its previous attempt plus your note.
Tick **Always** to save the note as a standing rule; rules are appended to the
system prompt on every request. Remove rules under *Remembered rules*.

Shipped default rule: *Never remove or alter a location, place name or address.*

## Providers

Any provider from AI settings. Pick it from the popup's dropdown
(saved as `writer.provider`). A **DeepSeek** provider
(`https://api.deepseek.com/chat/completions`, `deepseek-chat`) is now included;
add its API key in Settings like the others.

## Config (`writer` block)

| key | default |
|---|---|
| `provider` | `OpenAI` (provider name; falls back to provider type) |
| `model` | empty = provider's model |
| `system_prompt` | copy-editor prompt that keeps names, places, numbers, links |
| `rules` | standing rules list |
| `select_all_when_empty` | `true` |
| `temperature` | `0.2` |

The hotkey is the `writer_fix` action; rebind it under `hotkeys`.
Note that Ctrl+Space is global while Nerve runs, so it overrides Ctrl+Space in
other apps (e.g. Excel's select-column, IDE autocomplete).
