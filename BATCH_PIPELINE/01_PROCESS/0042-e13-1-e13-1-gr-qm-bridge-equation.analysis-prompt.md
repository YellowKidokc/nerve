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
  "title": "GR-QM Bridge Equation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0042_E13.1_E13_1_GR-QM_Bridge_Equation.md",
  "source_sha256": "813d0f8628b9efe8992189d7c4017a4a51d73ecf2848f5eac2014e6b64cac6a5",
  "captured_at": "2026-09-06T23:41:30.362200+00:00",
  "word_count": 381,
  "tags": [
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0042: E13.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E13.1 \u2014 GR-QM Bridge Equation",
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

# Canonical Sequential Record 0042: E13.1

| Field | Value |
|---|---|
| Registry position | 42 of 191 |
| Canonical ID | `E13.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E13_1_GR-QM_Bridge_Equation.md` |
| Source SHA-256 | `1a819699bc4279340c529d5bf6949770a5d4e55f9e54376dcb7543433bd29069` |

## Sequence Navigation

Previous: [0041 - D13.1 - Unified Field Lagrangian](0041_D13.1_D13_1_Unified_Field_Lagrangian.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0043 - D14.1 - Cosmological Grace Function](0043_D14.1_D14_1_Cosmological_Grace_Function.md)

## Registered Dependency Links

[D13.1 - Unified Field Lagrangian](0041_D13.1_D13_1_Unified_Field_Lagrangian.md); [BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "E13.1"
legacy_id: "AX-103"
title: "GR-QM Bridge Equation"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Physics"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6, D13.1"
date: "2026-08-17"
---

# 📐 E13.1 — GR-QM Bridge Equation

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Physics | **Legacy ID:** `AX-103`

---

## Statement

GR-QM bridge equation — modified Einstein field equation with χ-field source.

## Formal Expression

```
G_μν + Λg_μν = (8πG/c⁴)T_μν + κχ_μν
```

$$
G_μν + Λg_μν = (8πG/c⁴)T_μν + κχ_μν
$$

## Maximum Defensible Position

Model: Modified Einstein equation with χ source term.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Physics |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6, D13.1
**Chain Tier:** 12 — Physics Unification

## References

Einstein 1915; Quantum Field Theory

## Defeat Conditions

Show χ_μν term violates energy-momentum conservation. Demonstrate wrong predictions for known gravity.

## Limitations (from workbook)

Novel equation. Needs to reproduce all known GR results plus make NEW predictions.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::5dcd26f6-04b8-4c4b-80dc-cbc8850801a5::"GR-QM bridge equation must reproduce all known GR results"::null::@Engineering%%
%%tag::Requirement::8d6ba63a-d4c0-4fd3-abac-a5826974d521::"GR-QM bridge equation must make new predictions"::null::@Engineering%%
%%tag::Decision::7adb82a4-b8d8-44ec-86bd-3fb3d3de65e9::"Use modified Einstein field equation with χ-field source"::null::@Engineering%%
%%tag::Risk::b0739a16-eb53-4cd5-8483-8993a070ef02::"χ_μν term may violate energy-momentum conservation"::null::@Engineering%%
%%tag::Risk::84ae09e5-bff0-42ab-baa8-13c8fcb27a8a::"Equation may produce wrong predictions for known gravity"::null::@Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Dependency::24bf93c4-ce97-4402-a99f-11e095d11b4d::"Depends on D13.1 (Unified Field Lagrangian)"::null::@Engineering%%
%%tag::OpenQuestion::c8adb3de-e476-4ed8-b40c-f5f9d4afc95e::"Whether the equation can reproduce all known GR results"::null::@Engineering%%
%%tag::OpenQuestion::0d59d1ca-4ae4-4d7c-a4dc-027287ae98b6::"What new predictions the equation will make"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
