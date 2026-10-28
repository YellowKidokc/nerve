# Night Shift

A queue of jobs that Nerve runs while the computer is idle. Open it with **Ctrl+Alt+Z**, or open the `night-shift` panel.

## Lifecycle of a job

```text
DEFINE  ──►  WALK THROUGH (you watch it run once)  ──►  APPROVE  ──►  runs every idle night
   ▲                                                                    │
   └──────────── editing what the job does voids the approval ◄─────────┘
```

Approval stores a SHA-256 fingerprint of the job definition and its `max_minutes`.
If either changes, the job stops running until you walk it through again.
Renaming a job or editing its notes does not void the approval.

## Timing

A **session** opens after `idle_minutes` with no keyboard or mouse input.
The session clock starts from your last input, not from when Nerve noticed.

| Schedule | Meaning |
|---|---|
| `after_minutes: 30` | 30 minutes after your last keystroke |
| `after_task: "scan", gap_minutes: 180` | 3 hours after `scan` finishes in the same session |

Jobs run one at a time, so Ollama never competes with a scan for the disk or the GPU.
When you come back, the session closes.
Each job with `stop_on_return` is killed, and its run is recorded as `USER_RETURNED`.
Sleep and shutdown stop everything. For overnight runs, leave the PC on and let the screen turn off.

## Job kinds

| Kind | Does | Writes |
|---|---|---|
| `change_scan` | Hashes the listed folders and compares them with the last scan. Reports files that were added, modified, deleted, **moved** (same bytes at a new path, old path gone), or **copied** (same bytes, original still there). | `changes/latest.json`, a timestamped copy, and the manifest |
| `command` | Runs any program, such as David OS, a vectorizer, or an API batch. | Whatever that program writes |
| `ollama_review` | Sends each file from the latest change scan to local Ollama, together with the canon file and your prompt. | One Markdown report. All findings are `AI_PROPOSED`. |

A `command` job gets these environment variables:

- `NIGHTSHIFT_CHANGES`: the path to `changes/latest.json`. David OS reads this to update FIS pages.
- `NIGHTSHIFT_OUTPUT`: `<output_root>/night_shift`
- `NIGHTSHIFT_RUN_ID`

### `changes/latest.json`

```json
{
  "scanned_at": "2026-10-28T04:36:00Z",
  "roots": ["O:\\_Theophysics_v5"],
  "baseline": false,
  "added":    ["O:\\...\\new.md"],
  "modified": ["O:\\...\\edited.md"],
  "deleted":  ["O:\\...\\gone.md"],
  "moved":    [{"from": "O:\\...\\old.md", "to": "O:\\...\\renamed.md"}],
  "copied":   [{"from": "O:\\...\\a.md",   "to": "O:\\...\\a - Copy.md"}]
}
```

The first scan only records a baseline (`"baseline": true`, with every list empty).

## Files

- Queue: `%APPDATA%\clipsync-agent\night_shift.json`
- Output: `<output_root>\night_shift\{runs,changes,manifests,reports}\`
- Every run, attended or not, writes `runs/<run_id>.json` and `runs/<run_id>.log`.

## Relation to the Idle Canon Worker

`idle.rs` still does its single fixed job, deterministic C0 capture, under its own config.
Night Shift is the general-purpose scheduler that runs beside it.
Night Shift never writes to source files itself. A `command` job can, and that is why every job has to be walked through first.
