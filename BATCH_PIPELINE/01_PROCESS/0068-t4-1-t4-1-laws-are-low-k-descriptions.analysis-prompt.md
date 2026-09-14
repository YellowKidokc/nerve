# Meaning-record extraction prompt

Read the complete source below. Preserve its meaning and distinguish what the source says from
what you infer. Do not rewrite or reproduce the complete source in your response; the pipeline
appends the exact source separately.

Write the top layer as a clear research case file that a thoughtful reader can understand quickly. Return
Markdown with exactly these headings:

```markdown
## At a glance
State what this is, why it matters, and the one-sentence finding.

## Central claim
Give the strongest defensible substantive proposition, not generic meta-filler or summary rhetoric. If the source defines an ontological, mathematical, or structural rule, state the exact rule.

## Best concise argument
Present the clearest load-bearing argument in readable, logical steps.

## System or model
Describe the entities, distinctions, relationships, sequence, and causal or logical structure.
You MUST include these two subsections:
### Extracted Truth Predicates (Irreducible Assertions)
Provide a markdown table extracting all irreducible, standalone truth assertions from the source:
| # | Truth Predicate | Source Role | Formal / Mathematical Formulation |
|---|---|---|---|
| P1 | ... | ... | ... |
Extract at least 6 to 12 truth predicates.

### Terms and plain-language bridge
Define technical terms, symbols, or operators above 9th grade level with an analogy.

## Evidence chain
For every important step use: `Claim -> Support -> Inference -> Confidence -> Boundary`.

## Best evidence and sources
Separate primary evidence, formal results, secondary interpretation, and analogy. Preserve citations and source names. An unproven Lean code snippet ending in sorry or admit is NEVER a formal result; write None if no verified proof exists.

## Strongest objection and negative controls
Give the best opposing case, counterexamples, competing physical or philosophical interpretations, and tests that prevent false conclusions.

## What survives
State what remains defensible after the strongest objection. Must be at least 80 characters of substantive analysis.

## What this does not establish
List nearby conclusions that the evidence does not earn. Distinguish formal mathematical results from physical facts, and physical facts from theological conclusions. Must be at least 80 characters.

## Corrections and revisions
Record where the conversation, paper, or prior generation corrected, narrowed, or retired an earlier claim.

## Implications
Separate empirical, formal, philosophical, semantic, and theological implications.

## Formal or testable path
Identify Lean, Z3, empirical, literature, or adversarial tests that could strengthen or defeat it.
Must contain exactly:
### Lean / formal checks
Include a table: Proposed theorem or invariant | Definitions & premises to encode | Status | What a passing result establishes | Reader meaning / analogy
### Empirical / literature checks
### Adversarial checks

## Open questions and next actions
Prioritize unresolved questions and the next useful action.

## Recommended classification and relationships
Propose record type, domain, tags, and `supports`, `contradicts`, `refines`, `depends_on`, and `tests` links.
```

Rules:
1. Never write generic filler like "It explains meaning, math, dependencies..." in Central claim.
2. Never claim an unverified theorem ending in sorry/admit as a formal result.
3. Separate empirical, formal, philosophical, semantic, and theological statements strictly.
4. Never label anything canonical or proven merely because the source says so.
5. In System or model, include both `### Extracted Truth Predicates (Irreducible Assertions)` and `### Terms and plain-language bridge`.
6. Make the analysis readable without requiring the reader to inspect the complete source.

## Source metadata

{
  "title": "Laws Are Low-K Descriptions",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0068_T4.1_T4_1_Laws_Are_Low-K_Descriptions.md",
  "source_sha256": "38a68882e79181d256f1969ff76198e41b57ea02cc7150a8def9fe24268a01ee",
  "captured_at": "2026-09-07T00:02:12.922419+00:00",
  "word_count": 369,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0068: T4.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 T4.1 \u2014 Laws Are Low-K Descriptions",
    "Statement",
    "Formal Expression",
    "Maximum Defensible Position",
    "Classification",
    "Dependency Chain",
    "References",
    "Defeat Conditions",
    "Limitations (from workbook)"
  ]
}

## Complete source for analysis

<!-- GENERATED FROM AXIOMS_CANONICAL_ONLY.md; do not edit this projection by hand. -->

# Canonical Sequential Record 0068: T4.1

| Field | Value |
|---|---|
| Registry position | 68 of 191 |
| Canonical ID | `T4.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `T4_1_Laws_Are_Low-K_Descriptions.md` |
| Source SHA-256 | `2e3da75f3c288d04b9a5074e6c86f0b8490ff0a6ad56318fc86455dcfa845bc0` |

## Sequence Navigation

Previous: [0067 - A4.2 - Algorithmic Depth](0067_A4.2_A4_2_Algorithmic_Depth.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0069 - T4.2 - Action Principle As Minimal-K](0069_T4.2_T4_2_Action_Principle_As_Minimal-K.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D4.1 - Kolmogorov Complexity](0023_D4.1_D4_1_Kolmogorov_Complexity.md)

## Canonical Source Text

---
id: "T4.1"
legacy_id: "AX-032"
title: "Laws Are Low-K Descriptions"
mode: "FW_EXTENDED"
atlas_type: "Theorem"
status: "theorem"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2, D4.1"
date: "2026-08-17"
---

# 🔶 T4.1 — Laws Are Low-K Descriptions

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Theorem` | **Status:** `theorem`
**Domain:** Ontological | **Legacy ID:** `AX-032`

---

## Statement

Physical laws are low-Kolmogorov-complexity descriptions of high-complexity data.

## Formal Expression

```
K(Law) << K(Data); Law compresses Data
```

$$
K(Law) << K(Data); Law compresses Data
$$

## Maximum Defensible Position

Framework claim: Physical laws are low-Kolmogorov-complexity descriptions of high-complexity data.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Theorem` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `theorem` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2
**Depends On:** A2.2, D4.1

## References

Algorithmic information theory; Philosophy of science

## Defeat Conditions

Show a law with K(Law) ≥ K(Data).

## Limitations (from workbook)

Standard view in philosophy of science. Laws = compression.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6297a9c1-c8b8-4709-9f46-d1e28c3b85fa::"Physical laws must be low-Kolmogorov-complexity descriptions of high-complexity data"::null::@Engineering,Product%%
%%tag::Requirement::00070e56-7bc4-43a2-bb46-81a4416ccc93::"Laws must compress data (K(Law) << K(Data))"::null::@Engineering%%
%%tag::Decision::65015dfe-9bf7-4c86-af0b-77ccf136ed02::"Adopt the framework claim that physical laws are low-K descriptions"::null::@Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Dependency::1db09e44-e14e-4a01-898a-0bee85d0d437::"Depends on D4.1 (Kolmogorov Complexity)"::null::@Engineering%%
%%tag::Risk::793fcee0-bddf-4a1c-80fc-d8ddc4e520df::"Defeat condition: a law with K(Law) >= K(Data) would invalidate the theorem"::null::@Engineering,Product%%
%%tag::Assumption::3e220fb0-7cc5-4ebf-a69b-af04e878b1dc::"Assumes the standard view in philosophy of science that laws equal compression"::null::@Product%%
%%tag::Stakeholder::615deae5-6cfe-437e-be46-fb4510da7d4e::"Philosophy of science community (interested in law-compression view)"::null::@Product%%
%%tag::OpenQuestion::04afba94-fb2b-4908-85ff-dfd1d42b2ad6::"Whether any law can be shown to have K(Law) >= K(Data)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
