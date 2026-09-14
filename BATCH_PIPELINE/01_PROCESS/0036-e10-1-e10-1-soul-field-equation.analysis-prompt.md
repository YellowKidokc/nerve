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
  "title": "Soul Field Equation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0036_E10.1_E10_1_Soul_Field_Equation.md",
  "source_sha256": "2e69bf13bbc963e2027c86944bbe9e6132018599957aecf22464e5b5b4bfbd06",
  "captured_at": "2026-09-06T23:36:04.908347+00:00",
  "word_count": 428,
  "tags": [
    "consciousness",
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0036: E10.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E10.1 \u2014 Soul Field Equation",
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

# Canonical Sequential Record 0036: E10.1

| Field | Value |
|---|---|
| Registry position | 36 of 191 |
| Canonical ID | `E10.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E10_1_Soul_Field_Equation.md` |
| Source SHA-256 | `7732b0ac4f2a473142747912e5b96e5c5bdf271aee6c85c6a1c0c1cf6f80007f` |

## Sequence Navigation

Previous: [0035 - D10.1 - Soul Field Psi_S](0035_D10.1_D10_1_Soul_Field_Psi_S.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0037 - D11.1 - Moral Coherence Definition](0037_D11.1_D11_1_Moral_Coherence_Definition.md)

## Registered Dependency Links

[D10.1 - Soul Field Psi_S](0035_D10.1_D10_1_Soul_Field_Psi_S.md); [BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "E10.1"
legacy_id: "AX-087"
title: "Soul Field Equation"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Consciousness"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6, D10.1"
date: "2026-08-17"
---

# 📐 E10.1 — Soul Field Equation

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Consciousness | **Legacy ID:** `AX-087`

---

## Statement

Soul field equation — modified Schrödinger with grace coupling.

## Formal Expression

```
iℏ∂Ψ_S/∂t = ĤΨ_S + Ĝ·Ψ_S — Schrödinger + grace term
```

$$
iℏ∂Ψ_S/∂t = ĤΨ_S + Ĝ·Ψ_S — Schrödinger + grace term
$$

## Maximum Defensible Position

Model: Soul field dynamics with grace.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Consciousness |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6, D10.1
**Chain Tier:** 11 — Soul + Moral + Eschatology

## References

Schrödinger equation; Open quantum systems

## Defeat Conditions

Show soul dynamics don't follow Schrödinger form. Demonstrate grace term violates conservation.

## Limitations (from workbook)

Novel equation. Needs to reproduce known consciousness phenomena.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::a0b719c9-c4ff-4791-8275-7806d1dd8ab4::"Soul field equation must reproduce known consciousness phenomena"::null::@Engineering,Product%%
%%tag::Requirement::474fa6e4-e63c-4cdb-a6c0-9c7ff075ecd4::"Soul field dynamics must follow Schrödinger form"::null::@Engineering%%
%%tag::Requirement::e6f29963-0c56-4b5c-ac97-33408e10e439::"Grace term must not violate conservation laws"::null::@Engineering%%
%%tag::Decision::c5b47076-e362-4c19-af8b-4954630db9bb::"Use modified Schrödinger equation with grace coupling for soul field"::null::@Engineering%%
%%tag::Decision::c3e61954-0401-44b3-939f-7861449e9b4a::"Adopt AX_SCAFFOLD mode for the equation"::null::@Engineering,Product%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Dependency::14f3798e-0bd7-454d-abbf-84ef985f84e7::"Depends on D10.1 (Soul Field Psi_S)"::null::@Engineering%%
%%tag::Risk::9988535f-edcf-4de7-8f90-1313c441fb55::"Soul dynamics may not follow Schrödinger form"::null::@Engineering%%
%%tag::Risk::fabd63f9-4639-4638-8f07-4bf3a2551b45::"Grace term may violate conservation laws"::null::@Engineering%%
%%tag::Risk::80820116-4321-41d8-9493-2f929baee322::"Equation may fail to reproduce known consciousness phenomena"::null::@Engineering,Product%%
%%tag::Assumption::a6debe6c-411a-425c-8991-692145cb71ef::"Soul field dynamics can be modeled with a Schrödinger-like equation"::null::@Engineering%%
%%tag::Assumption::cfd0fd02-1ce9-40fc-90de-bdfaa5e2e160::"Grace coupling term is a valid addition to the equation"::null::@Engineering%%
%%tag::Metric::776c83aa-6d9d-4c6d-a972-a5c06b4f5a52::"Chain Tier: 11"::null::@Engineering%%
%%tag::Metric::9d6f3d0e-2d42-4021-a209-bdd17535f76e::"Registry position: 36 of 191"::null::@Operations%%
%%tag::Stakeholder::5c71e42b-b6c4-41e2-bb45-2e3dd226d71a::"Consciousness domain researchers"::null::@Engineering,Product%%
%%tag::OpenQuestion::530e4854-f74f-4c6c-b19a-bd9b1dd018de::"How to validate the soul field equation against known consciousness phenomena?"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
