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
  "title": "Chi Field Properties",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0016_D2.2_D2_2_Chi_Field_Properties.md",
  "source_sha256": "4c48be867606f7168859ca95fce9375a0d1c3d191954b62b29c9171a84359663",
  "captured_at": "2026-09-06T23:16:11.275508+00:00",
  "word_count": 351,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0016: D2.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D2.2 \u2014 Chi Field Properties",
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

# Canonical Sequential Record 0016: D2.2

| Field | Value |
|---|---|
| Registry position | 16 of 191 |
| Canonical ID | `D2.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D2_2_Chi_Field_Properties.md` |
| Source SHA-256 | `9b30a1943e0450d64de133c608c257e5d838db7f9af1443daa8a0c1111e455d4` |

## Sequence Navigation

Previous: [0015 - D2.1 - Logos Field Definition](0015_D2.1_D2_1_Logos_Field_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0017 - E2.1 - Master Equation First Form](0017_E2.1_E2_1_Master_Equation_First_Form.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D2.2"
legacy_id: "AX-011"
title: "Chi Field Properties"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D2.2 — Chi Field Properties

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-011`

---

## Statement

Properties of the χ-field: scalar, real-valued, non-negative, Lorentz-invariant.

## Formal Expression

```
χ: scalar, real-valued, non-negative, Lorentz-invariant
```

$$
χ: scalar, real-valued, non-negative, Lorentz-invariant
$$

## Maximum Defensible Position

Definition: χ-field properties specified.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 3 — Field Definitions

## References

Scalar field theory; Klein-Gordon equation

## Defeat Conditions

Show χ needs to be vector/tensor. Demonstrate Lorentz-invariance is wrong symmetry.

## Limitations (from workbook)

Why scalar? The simplest choice. But gravity is tensor (spin-2). May need generalization.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::e963bef8-72ca-4a3d-a0ab-d2d493deeb36::"χ-field must be scalar, real-valued, non-negative, and Lorentz-invariant"::null::@Engineering%%
%%tag::Decision::a2ed4de3-d623-4318-a1c7-e337b6a77650::"χ-field defined as scalar (simplest choice)"::e963bef8-72ca-4a3d-a0ab-d2d493deeb36::@Engineering%%
%%tag::Risk::98b48a34-13c6-49ef-ab42-bc207b7afe09::"Gravity is tensor (spin-2), may require generalization of χ-field"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::OpenQuestion::e04c4d48-85a2-426d-a911-c56113cbe4bb::"Whether χ needs to be vector/tensor (defeat condition)"::null::@Engineering%%
%%tag::OpenQuestion::431bb241-1aac-4a2c-be30-b5804cd1ea9e::"Whether Lorentz-invariance is the correct symmetry (defeat condition)"::null::@Engineering%%
%%tag::Assumption::67f20eba-b627-4d58-a883-464d06e1f788::"Scalar field theory and Klein-Gordon equation are applicable references"::null::@Engineering%%
%%tag::Metric::f26e9323-eafe-4cdd-89b3-17145c3c5018::"Chain Tier: 3 (Field Definitions)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
