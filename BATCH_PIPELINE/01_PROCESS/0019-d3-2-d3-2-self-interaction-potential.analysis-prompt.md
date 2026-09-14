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
  "title": "Self-Interaction Potential",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0019_D3.2_D3_2_Self-Interaction_Potential.md",
  "source_sha256": "cb6a344ed8607b5f59c8f282505af6fdebd31700306b8082c8aa6d1d48ec9984",
  "captured_at": "2026-09-06T23:18:47.196517+00:00",
  "word_count": 401,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0019: D3.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D3.2 \u2014 Self-Interaction Potential",
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

# Canonical Sequential Record 0019: D3.2

| Field | Value |
|---|---|
| Registry position | 19 of 191 |
| Canonical ID | `D3.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D3_2_Self-Interaction_Potential.md` |
| Source SHA-256 | `850f6b4ef67714f2c8cdddbe7765b05acf52fbc9b6bd1b81caec238630e1c797` |

## Sequence Navigation

Previous: [0018 - D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0020 - D3.3 - Interaction Lagrangian](0020_D3.3_D3_3_Interaction_Lagrangian.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D3.2"
legacy_id: "AX-019"
title: "Self-Interaction Potential"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D3.2 — Self-Interaction Potential

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-019`

---

## Statement

Self-interaction potential of the χ-field — spontaneous symmetry breaking form.

## Formal Expression

```
V(χ) = λ(χ² - v²)² — Mexican hat potential
```

$$
V(χ) = λ(χ² - v²)² — Mexican hat potential
$$

## Maximum Defensible Position

Definition: V(χ) = quartic self-interaction potential.

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
**Chain Tier:** 4 — Coherence System

## References

Higgs mechanism; Goldstone theorem; Spontaneous symmetry breaking

## Defeat Conditions

Show V(χ) has different form. Demonstrate quartic potential is wrong for informational field.

## Limitations (from workbook)

Borrowed from Higgs physics. Whether information fields have quartic potentials is assumed, not derived.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::cd2d517c-072f-4e83-84a9-2ebd818ff86f::"Self-interaction potential of the χ-field must be defined as spontaneous symmetry breaking form"::null::@Engineering%%
%%tag::Requirement::a4f2240c-82bd-4685-b6b0-4d75fc51deb9::"V(χ) must equal λ(χ² - v²)² (Mexican hat potential)"::null::@Engineering%%
%%tag::Requirement::abfd6dee-c0ab-431d-bc57-6a2228ac95d1::"V(χ) must be a quartic self-interaction potential"::null::@Engineering%%
%%tag::Decision::8bc1613b-55a6-4b11-8032-5e3f9c6f2104::"Adopted Mexican hat potential for self-interaction"::null::@Engineering%%
%%tag::Decision::e73aed32-0453-4b65-9baf-5c364cca0bb5::"Borrowed quartic potential form from Higgs physics"::null::@Engineering%%
%%tag::Assumption::fcb34ec1-10ef-4300-bdce-7f1629f24bf0::"Information fields have quartic potentials (assumed, not derived)"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Risk::a9d4f111-59f0-4917-9aa4-1bd0b923a39b::"If V(χ) has different form, definition is defeated"::null::@Engineering%%
%%tag::Risk::6222fa0e-66c7-4e4b-bc60-64f6937c0f65::"If quartic potential is wrong for informational field, definition is defeated"::null::@Engineering%%
%%tag::Stakeholder::ebe914ea-16e2-4463-b2ef-2b6121f926a2::"Higgs mechanism reference community"::null::@Engineering%%
%%tag::Stakeholder::492e0d04-ce74-4c6d-a508-dbb2b03d497a::"Goldstone theorem reference community"::null::@Engineering%%
%%tag::Stakeholder::3b878fdb-0f25-4d34-a71e-1e22f396e56a::"Spontaneous symmetry breaking research community"::null::@Engineering%%
%%tag::OpenQuestion::4bc4e445-5907-4110-88fe-a6a1c6392da3::"Whether information fields have quartic potentials is unresolved"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
