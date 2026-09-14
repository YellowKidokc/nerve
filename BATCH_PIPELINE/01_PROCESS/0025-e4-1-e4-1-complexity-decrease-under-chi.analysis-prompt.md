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
  "title": "Complexity Decrease Under Chi",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0025_E4.1_E4_1_Complexity_Decrease_Under_Chi.md",
  "source_sha256": "ecc38aa56eefb387669ce98cd19aaae87c5a5060608cc164d92459a503ea48b4",
  "captured_at": "2026-09-06T23:24:42.657467+00:00",
  "word_count": 362,
  "tags": [
    "information-theory"
  ],
  "source_outline": [
    "Canonical Sequential Record 0025: E4.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E4.1 \u2014 Complexity Decrease Under Chi",
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

# Canonical Sequential Record 0025: E4.1

| Field | Value |
|---|---|
| Registry position | 25 of 191 |
| Canonical ID | `E4.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E4_1_Complexity_Decrease_Under_Chi.md` |
| Source SHA-256 | `320577418b099e2b44bba574f8f6c8215781d30768e7bcb983d7962d1af001cc` |

## Sequence Navigation

Previous: [0024 - D4.2 - Compression Ratio](0024_D4.2_D4_2_Compression_Ratio.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0026 - D5.1 - Observer Definition](0026_D5.1_D5_1_Observer_Definition.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D4.1 - Kolmogorov Complexity](0023_D4.1_D4_1_Kolmogorov_Complexity.md)

## Canonical Source Text

---
id: "E4.1"
legacy_id: "AX-031"
title: "Complexity Decrease Under Chi"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2, D4.1"
date: "2026-08-17"
---

# 📐 E4.1 — Complexity Decrease Under Chi

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-031`

---

## Statement

Complexity decreases as χ increases — the field organizes information.

## Formal Expression

```
dK/dχ < 0 — complexity decreases with increasing χ
```

$$
dK/dχ < 0 — complexity decreases with increasing χ
$$

## Maximum Defensible Position

Model: χ reduces algorithmic complexity.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2, D4.1
**Chain Tier:** 5 — Compression Theory

## References

Algorithmic information theory; Self-organization

## Defeat Conditions

Show χ increase without complexity decrease. Demonstrate complexity increase with coherence.

## Limitations (from workbook)

This is the formal version of 'order from information.' Needs experimental support.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::85cd9b15-17ca-49f6-9430-7dde3a59986d::"Complexity must decrease as χ increases"::null::@Engineering%%
%%tag::Requirement::ee1d154f-c76b-4ce1-b9cc-a6901c0c7c67::"The field must organize information as χ increases"::null::@Engineering%%
%%tag::Decision::47e6b347-ff79-493c-a4c4-99ae0f097c0c::"Adopt the model that χ reduces algorithmic complexity"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::1db09e44-e14e-4a01-898a-0bee85d0d437::"Depends on D4.1 Kolmogorov Complexity"::null::@Engineering%%
%%tag::Risk::42ca9c17-d059-4051-b7bf-87e4375cf4bf::"Risk of showing χ increase without complexity decrease"::null::@Engineering%%
%%tag::Risk::61cd0a59-ff88-4971-9897-aff3466c5a9e::"Risk of demonstrating complexity increase with coherence"::null::@Engineering%%
%%tag::OpenQuestion::155caf11-9204-4990-ba0c-07b4197fced3::"Need experimental support for the formal version of 'order from information'"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
