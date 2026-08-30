# Nerve Idle Canon Worker — Implementation Specification

Status: `SPECIFICATION_ONLY`  
Target repository: `D:\GitHub\nerve`  
Target branch: `main`  
Authority ceiling: `C0_SIDECAR` and unruled Candidate proposals only

## Purpose

When the Windows computer has been continuously idle for a configured period,
Nerve may run bounded local Canon-assistance jobs. The worker stops when user
activity resumes. It preserves selected material, searches for existing
classifications, prepares sidecars and Candidate proposals, and writes a daily
review report.

The worker never canonizes, admits, deletes, moves, renames, or silently rewrites
source material.

## Default lifecycle

```text
ACTIVE USER
  -> IDLE_PENDING
  -> PREFLIGHT
  -> RUNNING_BOUNDED_JOB
  -> USER_RETURNED | JOB_COMPLETE | SAFE_HALT
  -> RECEIPT + REVIEW QUEUE
```

Default idle threshold: `60 minutes`.

## Safe automated work

The worker may:

1. Inspect only configured inboxes, recent-document lists, and explicit
   Obsidian highlight captures.
2. Create `C0_SIDECAR` capture records for intentionally selected material.
3. Preserve exact selected text, surrounding context, source path, source span,
   source hash, capture time, and capture method.
4. Search for explicit legacy and canonical classifications.
5. Propose classifications with their origin visibly labeled.
6. Detect missing provenance, boundaries, receipts, references, and Lean status.
7. Identify duplicate or conflicting versions without resolving them.
8. Prepare Candidate packets and a daily review report.
9. Queue possible Lean 4 formalization targets without claiming verification.
10. Detect stale HTML projections without rewriting them.

## Forbidden work

The worker must not:

- Create an Admitted or Canonical graph membership.
- Generate or apply a human ruling.
- Modify the live Canon SQLite database.
- Change `proof_label`.
- Replace or infer `one_axiom_classification` as an authoritative value.
- Convert an AI suggestion into an inherited or canonical classification.
- Rewrite, move, rename, or delete corpus files.
- Modify published HTML projections.
- Run paid or external AI queries.
- Start a persistent external service.
- Continue processing after user activity resumes.

## Classification origins

Every discovered or proposed classification carries exactly one origin:

```text
EXPLICIT_CANONICAL
EXPLICIT_LEGACY
DETERMINISTIC_DERIVED
AI_PROPOSED
UNKNOWN
```

Only `EXPLICIT_CANONICAL` is governing. All other origins remain visible review
inputs until a separate human ruling is applied by the Canon Engine.

## Canonization ladder

```text
C0_SIDECAR   automatically captured, no authority over the source
C1_WORKING   human-opened structured draft
C2_CANDIDATE schema/invariant-checked proposal awaiting ruling
C3_CANONICAL exact version admitted through a signed human event
```

This idle worker has an absolute ceiling of `C0_SIDECAR` for automatic captures
and may prepare—but not submit or admit—`C2_CANDIDATE` packets.

## Obsidian highlight capture

An intentional highlight sent to Nerve creates this minimum envelope:

```yaml
sidecar_id: SC-<uuid>
canonization_class: C0_SIDECAR
status: CAPTURED_UNREVIEWED
capture:
  method: OBSIDIAN_HIGHLIGHT
  selected_text: <exact text>
  surrounding_context: <bounded context>
  source_note: <vault-relative path>
  heading: <heading or null>
  block_id: <block id or null>
  source_span: <span or null>
  captured_at: <ISO-8601>
  captured_by: david
  source_sha256: <sha256>
classification:
  inherited: []
  discovered: []
  ai_proposals: []
  human_ruling: null
authority:
  candidate: false
  admitted: false
  canonical: false
```

## Local Ollama boundary

Ollama runs outside the sealed Canon Engine, through Nerve's local action layer.
Its results are decorations on a sidecar or Candidate proposal and must record:

- Provider: `ollama`
- Model and model digest
- Prompt/action identifier and version
- Input object IDs and hashes
- Start/end time
- Exit state
- Raw response hash
- What was proposed
- What was not established

Ollama output is always `AI_PROPOSED`; it never becomes a ruling.

## Configuration contract

Suggested configuration fields:

```json
{
  "enabled": false,
  "idle_minutes": 60,
  "stop_on_user_activity": true,
  "max_run_minutes": 45,
  "max_files_per_run": 50,
  "max_bytes_per_file": 5000000,
  "approved_roots": [],
  "output_root": "",
  "ollama_enabled": false,
  "ollama_model": "",
  "paid_or_external_ai": false,
  "canon_sqlite_write": false,
  "source_mutation": false
}
```

The feature defaults to disabled until the user selects approved roots and an
output root.

## Preflight gates

Before each run, verify:

1. Idle time meets the threshold.
2. No user input occurred during preflight.
3. Another idle run is not active.
4. Approved inputs and output root resolve to explicit absolute paths.
5. Output root is not a source root.
6. Available disk space exceeds the configured minimum.
7. AC/battery and thermal policy permit work.
8. Ollama is reachable locally when enabled.
9. No paid or external endpoint is configured.
10. Canon database access mode is absent or read-only.

Any failed preflight produces a `SAFE_HALT` receipt rather than silently
skipping or broadening scope.

## Stop behavior

User activity is a normal stop condition, not an error. The worker must:

1. Stop accepting new files immediately.
2. Cancel or bound the current Ollama request.
3. Finish only the current atomic write to the staging output.
4. Preserve partial artifacts with `INCOMPLETE_USER_RETURNED` status.
5. Write a receipt and release the single-run lock.

No source mutation may ever need rollback because no source mutation is allowed.

## Output structure

```text
<output_root>/
  sidecars/
  candidate_proposals/
  unresolved/
  receipts/
  reports/
  run_state/
```

Each run receives a UUID and append-only receipt. Reports link to artifacts;
they do not become a second truth store.

## Daily report

The review report should include:

- Files inspected and skipped
- Sidecars created
- Existing classifications found
- AI proposals created
- Conflicts and duplicates
- Missing provenance/boundaries/receipts
- Lean candidates and their current formalization state
- Stale HTML projections
- Incomplete work caused by user return
- Exact errors and safe halts
- Paths and hashes for every produced artifact

## Required acceptance tests

1. No work begins before the configured idle threshold.
2. User input stops the worker within the declared response time.
3. A second worker cannot run concurrently.
4. Inputs outside approved roots are rejected.
5. Output cannot resolve inside a source directory.
6. The worker cannot open Canon SQLite in write mode.
7. The worker cannot emit `ADMITTED` or `C3_CANONICAL` authority.
8. `proof_label` remains byte-identical.
9. `one_axiom_classification` remains independent and unruled.
10. Ollama-off mode performs deterministic intake only.
11. Ollama failure preserves inputs and produces a receipt.
12. User return mid-file preserves a marked partial artifact.
13. Rerunning the same unchanged input is idempotent by source hash.
14. A changed source creates a new sidecar version rather than overwriting one.
15. The daily report can be deleted and regenerated without losing canonical
    or sidecar records.

## Implementation order

```text
1. Idle detector + cancellation signal
2. Explicit configuration and approved-root validation
3. Single-run lock + receipt writer
4. Deterministic C0 sidecar capture
5. Daily report
6. Existing-classification discovery
7. Optional local Ollama proposal action
8. Candidate-packet preparation
9. UI controls and runtime smoke tests
```

Do not begin with AI classification. Earn the idle lifecycle, cancellation,
path boundaries, deterministic capture, and receipts first.
