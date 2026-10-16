# HANDOFF: make the API PIPELINE prototype a real Nerve panel (CKG + Stories first)

You are continuing David Lowe's pipeline work. Read this whole file before touching anything.

## The goal

Turn the black-and-gold API PIPELINE prototype into a **working panel inside Nerve**, wired for real runs, starting with **two groups only**:

1. **01_CKG**: stations `20_CKG_RUN` and `25_CKG_V4` (V4 is the new readable CKG review).
2. **041_STORIES**: stations `41_STORY` and `24_CRAFT_PASS`.

David strongly prefers Nerve over anything that needs a server: "it always loads, it's always worked." **Do not add a localhost server.**

## Where things are

- **Prototype (design only, no backend):** `API PIPELINE/index.html` in `github.com/YellowKidokc/pipeline-workflows` (run `git pull` in `D:\GitHub\pipeline-workflows`). Read its `START HERE.md`: it records the agreed layout (API groups; Run / Settings / Results / Backside views; Backside = the whole template broken into calls, each a prompt plus an editable output template). Keep its look.
- **Nerve:** `D:\GitHub\nerve`, a Rust desktop app (tao/wry). HTML panels live in `source/html/`, and are built and installed by `setup.bat` to `%LOCALAPPDATA%\ClipSync`.
- **Nerve already has a Station Console** (Ctrl+Alt+W), in `source/src/pipeline.rs`. **Read it first.** It runs `D:\GitHub\pipeline-workflows\API\_system\engine\console.py list`, which prints JSON for every station (label, family, buttons, inbox/outbox counts, paths). It then starts a station's own `.bat` button exactly as a double-click would, and answers the button's questions through stdin. **Reuse this plumbing; do not reimplement station logic.**
- **Stations with editable calls:** `API\01_CKG\025_CKG_V4\BACKSIDE\` (and `API\02_C_P_E_L\023_CPEL\BACKSIDE\`) contain `ckg.json` (call order, on/off, page order) and `calls\NNN_NAME\prompt.md` + `output.md`. That is exactly the prototype's "Backside."

## Map the prototype onto real data

- **API groups:** the `family` field from `console.py list`, filtered to 01_CKG and 041_STORIES for now.
- **Run:** pick a button; stream the process output live into the activity log; show inbox and outbox counts. No invented progress percentages or costs.
- **Results:** list the station's OUTBOX files, newest first, and open on click.
- **Backside:** for stations that have `BACKSIDE\ckg.json` and `calls\`, list the calls in order and show `prompt.md` and `output.md` side by side, editable. Save writes back to those files. Reordering writes `run_order` / `page_order` in `ckg.json`. Make a timestamped backup before every write.
- **Settings:** per-station `FOCUS.md` (the console can already write this) and the workers count passed to the button.

## Rules

- Pipeline logic stays in the pipeline folder. Nerve only lists, starts, streams and edits files.
- `pipeline.rs` currently allows writing only `FOCUS.md`. Extend the allowlist **only** to `BACKSIDE\ckg.json` and `BACKSIDE\calls\*\prompt.md` / `output.md`, with backups.
- Never put API keys in files. The stations read `DEEPSEEK_API_KEY` from the user environment.
- No simulated activity. If something isn't wired yet, say so in the panel.
- Don't modify station scripts (`25_ckg_v4.py`, `24_craft_pass.py`, etc.) or `_system\engine`.

## Done looks like

1. The panel opens from Nerve (hotkey or tray) with no server running.
2. From the panel, run **25_CKG_V4** on one paper, for example `C:\Users\David\Documents\faiththruphysics.com\Canon\AXIOM\Good\AXIOM\01_GOD_AS_ROOT\AX_GI_01_SHORT_GOD_AS_ROOT.md`. The live log shows the eight calls (SYNOPSIS > LADDER > LEGS > ARGUMENTS > CLAIMS > EVIDENCE > WALK > AUDIT), and `<paper> · CKG.md` appears under Results.
3. Edit one V4 prompt in Backside, save, and confirm the file changed on disk with a backup beside it.
4. 24_CRAFT_PASS runs from the Stories group the same way, including its **3 REPAIR FINISHED PAPERS** button.
5. A short README in the panel folder: what is wired, what isn't, how to rebuild Nerve.

Ask David before widening scope beyond these two groups.
