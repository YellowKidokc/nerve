# Batch Atom Classifier — Design Spec (Nerve)

**Date:** 2026-09-22  
**Project:** Nerve (`D:\GitHub\nerve`)  
**Topic:** A batch-runnable Python pipeline that classifies atoms one at a time and produces canonization-ready receipts.

---

## 1. Purpose

Nerve already has the `BATCH_PIPELINE` for turning raw corpus files into CKG master records. This spec adds a second, focused stage: **claim-classification**. It reads the enriched atoms already preserved in `BATCH_PIPELINE/02_OUTBOX`, classifies each one against the `claim-classification/1.0.0` schema, and writes a classification receipt plus an optional `C2_CANDIDATE` packet ready for human ruling.

The pipeline is intentionally conservative: it **proposes**, never **admits**. Every output lands as `candidate`/`proposed` and awaits the Canon Engine / human ruling before promotion.

---

## 2. Scope

**In scope for first deliverable:**
- Python classifier script: `BATCH_PIPELINE/03_SCRIPTS/classify_atoms.py`
- Windows batch launcher: `BATCH_PIPELINE/RUN_CLASSIFY.bat`
- JSON receipt output conforming to `faith-through-physics-atoms/_schema/claim_classification_output_v1.schema.json`
- Resumable run via a local progress file
- Optional `--candidate` flag that prepares a `C2_CANDIDATE` packet under `BATCH_PIPELINE/02_OUTBOX/classifications/`
- Tests for schema validation, prompt assembly, and resumability

**Out of scope for first deliverable:**
- Automatic GitHub PR creation
- Multi-worker parallelism (kept serial so each atom can be inspected and costs are predictable)
- Full integration with the Nerve `worker` or `atom-cloud` services

**Starting corpus:** `BATCH_PIPELINE/02_OUTBOX/*_CKG_MASTER.md` master records, because they already preserve the exact source bytes and the CKG enrichment.

---

## 3. Inputs and Outputs

### Input
- Source tree (default: `BATCH_PIPELINE/02_OUTBOX`)
- Each source is a CKG master record `.md` file (and its sidecar `.json`)
- The classifier extracts:
  - Frontmatter (`source_sha256`, `title`, `inferred_type`, `tags`, `semantic_model`)
  - The CKG analysis body (everything between `## Intake assessment` and `## Complete preserved source`)
  - The preserved source itself (for anchor verification)

### Output
- One classification receipt per atom:
  - Path: `BATCH_PIPELINE/02_OUTBOX/classifications/<stem>.classification.json`
  - Schema: `claim-classification/1.0.0`
  - Required fields mirror the Faith-through-physics-atoms schema
- A run manifest:
  - Path: `BATCH_PIPELINE/02_OUTBOX/classifications/RUN-<timestamp>-manifest.json`
  - Contains: run id, source tree, provider/model, counts, errors, skipped files
- A progress file (only when `--resume` is used):
  - Path: `BATCH_PIPELINE/02_OUTBOX/classifications/progress.json`
  - Contains: set of completed source paths keyed by `source_sha256`
- Optional `C2_CANDIDATE` packets:
  - Path: `BATCH_PIPELINE/02_OUTBOX/classifications/candidates/<stem>.candidate.json`
  - Contains: classification receipt + source hash + proposed object type + proposed register

---

## 4. Components

### 4.1 `classify_atoms.py`
Main entry point. Responsibilities:
- Parse CLI args (`--input-dir`, `--output-dir`, `--provider`, `--model`, `--resume`, `--candidate`, `--limit`).
- Discover CKG master records.
- Load progress and skip already-completed items (by `source_sha256`).
- For each atom:
  1. Extract frontmatter and analysis body.
  2. Build classification prompt.
  3. Call the configured LLM endpoint (default: OpenRouter via existing `semantic_analysis.py` patterns).
  4. Parse and validate JSON against the classification schema.
  5. If `--candidate`, assemble a `C2_CANDIDATE` packet.
  6. Write receipt and candidate packet.
  7. Update progress.
- Write manifest at end.

### 4.2 Prompt builder
A dedicated function `build_classification_prompt(master_record_text: str, metadata: dict) -> str` that:
- Injects the `claim-classification/1.0.0` schema as the required JSON structure.
- Provides one worked example (from `faith-through-physics-atoms/_runtime/claim_capsule_app/runs/20260831T002220Z-3217ae84.json`).
- Includes the CKG analysis body and the original source verbatim.
- Forbids invented evidence or unsupported canonical status.

### 4.3 Schema validator
Uses `jsonschema` if available; otherwise performs a manual required-fields check so the script has no hard dependency beyond the stdlib. The receipt is still written even if validation warns, so the human can inspect malformed outputs.

### 4.4 `RUN_CLASSIFY.bat`
A Windows batch wrapper that:
- Checks for `OPENROUTER_API_KEY` (reuses existing registry/env lookup from `semantic_analysis.py`).
- Activates the project environment if one exists.
- Runs `python 03_SCRIPTS\classify_atoms.py --input-dir 02_OUTBOX --output-dir 02_OUTBOX\classifications --resume`.
- Pauses at the end so the user sees the summary.

### 4.5 Candidate packet assembler
When `--candidate` is passed, the script wraps the validated classification receipt with:
- `packet_type`: `C2_CANDIDATE`
- `canonization_class`: `C2_CANDIDATE`
- `status`: `PROPOSED`
- `authority`: `{ "candidate": true, "admitted": false, "canonical": false }`
- `source_sha256` and `ckg_master_path`
- `classification_origin`: `AI_PROPOSED`

This packet is ready for the Canon Engine to consume but is **not** admitted.

---

## 5. Data Flow

```text
BATCH_PIPELINE/02_OUTBOX/*_CKG_MASTER.md
   |
   v
classify_atoms.py  ----prompt---->  LLM (OpenRouter / DeepSeek / Kimi)
   |                                   |
   |<-------------JSON-----------------+
   |
   v
schema validate  ----(if --candidate)-->  Candidate packet assembler
   |                                         |
   v                                         v
classification receipt               C2_CANDIDATE packet
   |
   v
02_OUTBOX/classifications/ + progress.json + manifest.json
```

---

## 6. Path to Canonization

1. **Intake** — raw corpus file dropped into `BATCH_PIPELINE/00_INBOX`.
2. **CKG enrichment** — existing `run_batch.py` produces a preserved master record in `02_OUTBOX`.
3. **Claim classification** (this pipeline) — master record is typed as `CLAIM`, `EVIDENCE`, `PROOF`, or `PROCESS`; species, register, anatomy, burdens, and WHY-outcome are produced.
4. **Candidate packet preparation** — optional `C2_CANDIDATE` packet is assembled with `AI_PROPOSED` origin.
5. **Canon Engine pre-review** — deterministic rules check required fields, source hash, and anchor presence.
6. **Human ruling** — a signed human event promotes `C2_CANDIDATE` → `C3_CANONICAL`. The model never performs this flip.

---

## 7. Combining the Proofs and Claims Layers

The classification schema's `object_type` field tells the system whether an atom is:
- `CLAIM` — a load-bearing assertion.
- `EVIDENCE` — a citation, observation, or source.
- `PROOF` — a deductive or formal support object.
- `PROCESS` — a methodology, rule, or workflow atom.

A complete argument chain is built as **edges** between these objects, not by merging them into one record:
- A `CLAIM` carries edges pointing to its supporting `EVIDENCE` and `PROOF` objects.
- Classification assigns the correct `object_type` to each atom so the edge drawer knows which end of a link it is holding.
- The classifier never invents edges; it only labels atoms. Edge proposals are left to a later curator pass or to the Canon Engine.

---

## 8. Error Handling and Resumability

- **Transient LLM errors** (timeout, rate limit): log error, stop the loop, and write progress so the run can resume.
- **Schema validation failures**: write the receipt with `status: CLASSIFICATION_INCOMPLETE` and include the raw LLM output plus validation errors.
- **Missing API key**: exit immediately with a clear message pointing to `RUN_CLASSIFY.bat` setup.
- **Duplicate source**: if progress file shows the source SHA-256 was already processed, skip by default when `--resume` is set; overwrite only if `--force` is passed.

---

## 9. Testing

- Unit test `build_classification_prompt` ensures required schema fields appear in the prompt.
- Unit test schema validation against a known-good classification receipt.
- Unit test resumability: progress file is read, completed items skipped, failed/incomplete items retried.
- Integration test with the LLM call mocked to return the known-good receipt; verify output file is written and matches schema.

---

## 10. Files Created

- `BATCH_PIPELINE/03_SCRIPTS/classify_atoms.py`
- `BATCH_PIPELINE/RUN_CLASSIFY.bat`
- `BATCH_PIPELINE/03_SCRIPTS/test_classify_atoms.py`
- `BATCH_PIPELINE/03_SCRIPTS/classification_schema.json` (copied from Faith-through-physics-atoms schema)
- This design doc: `docs/specs/2026-09-22-batch-atom-classifier-design.md`
