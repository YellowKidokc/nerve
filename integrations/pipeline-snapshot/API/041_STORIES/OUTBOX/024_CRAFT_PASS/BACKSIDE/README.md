# 24 Craft Pass

Sits inside 041_STORIES. Makes a piece more memorable, more human, and more inviting **without changing what it claims**. Markdown and JSON only; no HTML.

## Buttons

- **1 RUN HERE**: every paper in this station's INBOX (00_PRIORITY first).
- **2 RUN ON FOLDER**: drag a folder or one paper onto it, e.g. `041_STORIES\INBOX\01_SERIES\THE_STORY_AX_GI`.
- It also appears as a layer after 041 runs through the shared buttons.
- Papers that already have results are skipped. Use `--redo` to run them again.

## What it does, per paper

1. **Splits** the paper into sections (each `##`) and units (each line or paragraph), IDs like `S04.U03`.
   **Protected (never touched):** the "has and has not done" section, the story station's own "Analysis · Layer" section, the preserved-source section, and any table, callout, code block or comment, wherever it sits.
2. **Section passes, in order: the loop.** Each section lists up to 12 candidate techniques, uses 2 fundamentals + up to 3 extras, and carries the rest forward. David's own cards (`DAVID-001..005` in `david_cards.json`) outrank the craft library (`craft_library.json`, 71 cards from 9 videos). Each section also gets a memorable line and a "never in the same room" analogy (rated strong/weak, mapping stated, where it breaks, no domain repeated, never picturing what the section denies).
3. **Best version, section by section,** so a LONG paper never overflows one reply. Every line keeps the ID of the original line it came from. The title and headings are always David's.

## What comes out (OUTBOX)

- `<paper> · CRAFT.md`: **your original, unchanged, with marked additions under each line:**
  - `((R1))` `((R2))` `((R3))` rewrite options; `((R1*))` is the pass's pick
  - any marker ending in `!` claims more than the original (the gate)
  - `((ML))` memorable line; `((ML=S03.U04))` means an existing line already is it
  - `((AN strong))` / `((AN weak))` analogy, with `((MAP))` and `((BRK))`
  - `((USED))` / `((CARRY))` techniques used and carried forward
- `<paper> · CRAFT.json`: the same data, for OpenWriter's left-right picker.
- `<paper> · BEST.md`: the full best version; `((C))` changed line, `((NEW))` added line, unmarked = kept as written.
- `<paper> · BEST.json`: the best version unit by unit, with source IDs and techniques.
- `<paper> · CRAFT.receipt.json`: hashes, model, tokens and time per call.

## Originals

Before running, every paper is copied once into ORIGINALS (`engine/originals.py`, folder set by `originals_root` in `_system\config\paths.json`). Same bytes already there = skipped.

## Rules built into the prompts

Declaration register (heartfelt, unhedged, no added caveats). Never claim more than the original. Never invent first-person experience for David. The title and headings never change.

## Updating

- New craft videos: run Quick Call `04_CRAFT_EXTRACT`, then `merge_library.py`, then copy the new `craft_library.json` here.
- New personal rules: add a card to `david_cards.json`.
- Prompts: `section_prompt.txt` and `best_version_prompt.txt`.
