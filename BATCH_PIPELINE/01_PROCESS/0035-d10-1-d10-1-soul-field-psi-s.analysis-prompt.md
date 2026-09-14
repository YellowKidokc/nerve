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
  "title": "Soul Field Psi_S",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0035_D10.1_D10_1_Soul_Field_Psi_S.md",
  "source_sha256": "7ad2b49b9e1e0fa8037781758bd1387804ba2a723c782118f3117368d5c585a3",
  "captured_at": "2026-09-06T23:35:18.525159+00:00",
  "word_count": 403,
  "tags": [
    "consciousness",
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0035: D10.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D10.1 \u2014 Soul Field Psi_S",
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

# Canonical Sequential Record 0035: D10.1

| Field | Value |
|---|---|
| Registry position | 35 of 191 |
| Canonical ID | `D10.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D10_1_Soul_Field_Psi_S.md` |
| Source SHA-256 | `d14676b0fc374066c01bb80fa79faf63e3cef7cf8558c20df800cd85301907a5` |

## Sequence Navigation

Previous: [0034 - E9.1 - Grace Function G(t)](0034_E9.1_E9_1_Grace_Function_Gt.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0036 - E10.1 - Soul Field Equation](0036_E10.1_E10_1_Soul_Field_Equation.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D10.1"
legacy_id: "AX-084"
title: "Soul Field Psi_S"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Consciousness"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D10.1 — Soul Field Psi_S

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Consciousness | **Legacy ID:** `AX-084`

---

## Statement

Soul field Ψ_S defined — complex-valued field representing individual consciousness.

## Formal Expression

```
Ψ_S: M → ℂ — soul field (complex-valued on manifold)
```

$$
Ψ_S: M → ℂ — soul field (complex-valued on manifold)
$$

## Maximum Defensible Position

Definition: Ψ_S = soul field.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Consciousness |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 11 — Soul + Moral + Eschatology

## References

Quantum mechanics (wavefunction); IIT; Penrose-Hameroff

## Defeat Conditions

Show consciousness doesn't need field representation. Demonstrate soul without Ψ_S structure.

## Limitations (from workbook)

The soul-as-field is a model commitment. Whether consciousness has field structure is unknown.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::881f4921-58d6-43eb-8db1-ea4ab62c6bf3::"Soul field Ψ_S must be defined as a complex-valued field representing individual consciousness"::null::@Engineering%%
%%tag::Requirement::f1d45668-dd08-45ab-82e6-5f05d0d3d885::"Soul field Ψ_S must be formally expressed as a map from manifold M to complex numbers"::null::@Engineering%%
%%tag::Decision::291c969c-2dbe-4aa2-9ac6-f06c14c735b1::"Adopt soul-as-field model commitment for consciousness representation"::null::@Product,Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 Infinite Energy Source"::null::@Engineering%%
%%tag::Risk::e5bd398b-90ce-4133-bad2-f3d210d4ad8e::"Risk that consciousness may not need field representation, invalidating the definition"::null::@Engineering,Product%%
%%tag::Risk::608f6478-8aef-4624-aeef-dec2cf562c3b::"Risk that consciousness may not have field structure, making the model unsupported"::null::@Engineering,Product%%
%%tag::Assumption::a3b65c50-758e-4ff5-ad88-46ac65133091::"Assumption that consciousness can be represented as a field"::null::@Engineering,Product%%
%%tag::OpenQuestion::13fc9433-36b7-40ed-a21a-2111b8d3546c::"Whether consciousness has field structure is unknown"::null::@Engineering,Product%%
%%tag::Stakeholder::f6390607-9867-4ace-bb72-7a5a1a7a3542::"Consciousness research community interested in field-based models"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
