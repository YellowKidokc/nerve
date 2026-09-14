# Consilience Atlas — Autonomous Large-Batch CKG Pipeline

This autonomous batch runner processes large corpus sets (such as the 191 foundational axioms) cleanly through the strict CKG research gate **without using the HTML browser interface**. The web UI remains available for human inspection, interactive review, and manual refinement.

---

## Architecture

```text
00_INBOX (waiting to be processed / original untouched bytes)
      ↓
01_PROCESS (source metadata, SHA-256, prompt packets, revisions)
      ↓
AI ANALYSIS (OpenRouter DeepSeek R1 Free -> DeepSeek V3.2 Paid Fallback)
      ↓
STRICT QUALITY GATE (validate_ckg: 14 required headings, no filler, no 'sorry' stubs, truth predicates)
      ↓
02_OUTBOX (protected processed master records: .md + .json)
      ↓
SYNOLOGY MIRROR (C:\Users\David\Documents\faiththruphysics.com\999_API_CALLS_OUTPUT)
```

---

## Directory Structure

```text
BATCH_PIPELINE/
│
├── 00_INBOX/                   # Drop your raw source files here (.md, .txt, etc.)
│   └── ARCHIVED_INPUTS/        # Source files are safely moved here once processed
│
├── 01_PROCESS/                 # Prepared working packets (.intake.json, .knowledge.md, .prompt.md)
│   └── REVISIONS/              # Prior versions preserved if re-run with improved templates
│
├── 02_OUTBOX/                  # Protected master records (*_CKG_MASTER.md and *_CKG_MASTER.json)
│
├── 03_SCRIPTS/                 # Engine code & prompt templates
│   ├── PROMPTS/
│   │   └── knowledge_record.md # Strict 14-heading contract with Truth Predicates
│   ├── run_packet.py           # Intake packet generator & SHA-256 hasher
│   ├── semantic_analysis.py    # OpenRouter caller with DeepSeek R1 + V3.2 fallback
│   ├── validate_ckg.py         # 100% CKG compliance validator
│   └── run_batch.py            # End-to-end orchestrator
│
├── LOGS/                       # Timestamped run logs and error receipts
│
├── RUN_PILOT_5.bat             # 1-Click: Stages 5 sample axioms from Vault & runs test
├── STAGE_ALL_191.bat           # 1-Click: Copies all 191 axioms from Vault into 00_INBOX
└── RUN_BATCH.bat               # 1-Click: Processes all files currently in 00_INBOX
```

---

## Quality Gate Enforcement

1. **Deterministic Header Identification**: Files with canonical axiom indicators (`Canonical Sequential Record`, `atlas_type: "Axiom"`, `Mode | AX_CORE`, `Sealed chain authority`) are automatically classified as `axiom_companion` without NLP guesswork.
2. **Substantive Non-Empty Sections**: Rejects generic filler like `"It explains meaning, math, dependencies..."`.
3. **Formal Verification Hardening**: Disallows any unproven Lean code ending in `sorry` or `admit` in *Best evidence and sources*. An unproven stub is never treated as a formal result.
4. **Truth Predicates**: Requires a structured table under *System or model* containing extracted atomic truth predicates with formal mathematical notation.
5. **Three Epistemic Rows**: Enforces substantive entries (>= 80 non-whitespace chars) for *What survives*, *What this does not establish*, and *Formal or testable path*.
