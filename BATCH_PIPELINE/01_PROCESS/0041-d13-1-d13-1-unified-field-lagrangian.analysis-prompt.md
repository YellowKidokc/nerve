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
  "title": "Unified Field Lagrangian",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0041_D13.1_D13_1_Unified_Field_Lagrangian.md",
  "source_sha256": "a0f6ebc71815c510cae3c53694e259a59dcf4729fd854c6810d3091de2293586",
  "captured_at": "2026-09-06T23:40:37.774464+00:00",
  "word_count": 381,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0041: D13.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D13.1 \u2014 Unified Field Lagrangian",
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

# Canonical Sequential Record 0041: D13.1

| Field | Value |
|---|---|
| Registry position | 41 of 191 |
| Canonical ID | `D13.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D13_1_Unified_Field_Lagrangian.md` |
| Source SHA-256 | `8441493a68047d721eae8b98b2ecaf7126dc9b780b03f242f61c1fda50258eef` |

## Sequence Navigation

Previous: [0040 - E12.1 - Destiny Equation](0040_E12.1_E12_1_Destiny_Equation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0042 - E13.1 - GR-QM Bridge Equation](0042_E13.1_E13_1_GR-QM_Bridge_Equation.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D13.1"
legacy_id: "AX-102"
title: "Unified Field Lagrangian"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Physics"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D13.1 — Unified Field Lagrangian

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Physics | **Legacy ID:** `AX-102`

---

## Statement

Coupling constant κ between geometry and Logos Field defined.

## Formal Expression

```
κ: χ ↔ g_μν coupling constant
```

$$
κ: χ ↔ g_μν coupling constant
$$

## Maximum Defensible Position

Definition: κ = geometry-χ coupling.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Physics |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 12 — Physics Unification

## References

General Relativity; Yang-Mills theory

## Defeat Conditions

Measure κ and show it's zero. Show κ leads to contradictions.

## Limitations (from workbook)

κ is a free parameter. Without measured value, predictions remain parametric.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::92741b3b-c5c2-45d3-a2a7-7a7d55a4bab4::"Define coupling constant κ between geometry and Logos Field"::null::@Engineering%%
%%tag::Requirement::70aa6f4f-1591-4ce7-953d-c9c21525e735::"Measure κ and show it's zero to defeat the definition"::null::@Engineering%%
%%tag::Requirement::0aeee887-44e2-42fb-ad26-a55661ee90f6::"Show κ leads to contradictions to defeat the definition"::null::@Engineering%%
%%tag::Decision::165cd194-250c-4b2e-ac6a-3240966fb3a6::"Adopt κ as the geometry-χ coupling constant"::null::@Engineering%%
%%tag::Decision::b441b5bc-dec3-4a39-b985-f6eddb1c3983::"Classify D13.1 as a formal definition in AX_SCAFFOLD mode"::null::@Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Risk::93972b85-15b0-4c05-b162-e4789cc9c82f::"κ is a free parameter without measured value, predictions remain parametric"::null::@Engineering%%
%%tag::Assumption::042f9af7-839f-4bb9-b7f8-73adb7d36c67::"κ is non-zero and can be measured"::null::@Engineering%%
%%tag::OpenQuestion::c6096b69-bf86-492d-9e9b-f7be21c6ff86::"What is the measured value of κ?"::null::@Engineering%%
%%tag::Stakeholder::e9c843a9-b753-46ef-a621-89f87334e6e0::"Physics community interested in unified field theories"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
