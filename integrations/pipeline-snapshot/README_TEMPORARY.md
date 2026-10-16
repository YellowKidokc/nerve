# pipeline-snapshot: TEMPORARY

A read-only copy of the parts of `D:\GitHub\pipeline-workflows\API` that the Nerve station panel touches, put here so online Codex can see them. **Delete this folder once the panel is built.** It is not the source of truth.

- **Real location on David's machine:** `D:\GitHub\pipeline-workflows\API`. Nerve's Station Console reads it from `PipelineConfig.roots` in `source/src/config.rs` (`pipeline.rs` does the work). The panel must use those real paths, never this snapshot.
- **What is here:**
  - `API/_system/engine` (incl. `console.py`, which `pipeline.rs` calls) and `API/_system/config` (`stations.json` etc.);
  - four stations (buttons + BACKSIDE): `01_CKG/020_CKG`, `01_CKG/025_CKG_V4`, `041_STORIES`, `041_STORIES/OUTBOX/024_CRAFT_PASS`. INBOX/OUTBOX contents are left out on purpose.
- **SAMPLE_console_list.json:** the real output of `console.py list`, i.e. the exact data shape the panel receives.
- **SAMPLE_RESULTS/:** one real CKG V4 review, to design the Results view against.
- **PROTOTYPE/:** the black-and-gold design (`index.html`) and its agreed layout (`START HERE.md`). Keep its look.
- **HANDOFF_CKG_PANEL.md:** the task. Read it first.

Any change the pipeline itself needs (station scripts, prompts) goes to the pipeline-workflows repo, not into this copy.
