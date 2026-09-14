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
  "title": "Sign Operator",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0032_D8.1_D8_1_Sign_Operator.md",
  "source_sha256": "580fbba99d623eff99d25552bb1792b9a9ad89a9fc83c22f4bffd0a9f01c2db4",
  "captured_at": "2026-09-06T23:30:36.204841+00:00",
  "word_count": 383,
  "tags": [
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0032: D8.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D8.1 \u2014 Sign Operator",
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

# Canonical Sequential Record 0032: D8.1

| Field | Value |
|---|---|
| Registry position | 32 of 191 |
| Canonical ID | `D8.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D8_1_Sign_Operator.md` |
| Source SHA-256 | `3455e4ca08da1291b87cf8436af824d56a66209d751f8d853d7f0984a84dc298` |

## Sequence Navigation

Previous: [0031 - E6.2 - Phi-Dependent Collapse](0031_E6.2_E6_2_Phi-Dependent_Collapse.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0033 - D9.1 - Grace Operator Definition](0033_D9.1_D9_1_Grace_Operator_Definition.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D8.1"
legacy_id: "AX-069"
title: "Sign Operator"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Moral"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D8.1 — Sign Operator

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Moral | **Legacy ID:** `AX-069`

---

## Statement

Sign operator defined — binary moral orientation eigenvalue.

## Formal Expression

```
σ̂|ψ⟩ = σ|ψ⟩; σ ∈ {+1, -1}
```

$$
σ̂|ψ⟩ = σ|ψ⟩; σ ∈ {+1, -1}
$$

## Maximum Defensible Position

Definition: σ̂ = sign (moral orientation) operator.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Moral |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 9 — Sign/Syzygy

## References

Operator algebra; Pauli matrices

## Defeat Conditions

Show moral orientation is not binary. Demonstrate continuous moral spectrum without sign.

## Limitations (from workbook)

The binary claim is strong. Most moral philosophy assumes continuous spectrum.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::eb16bdd5-8e89-46c0-a55c-948b2b0867e6::"Sign operator must be defined as binary moral orientation eigenvalue"::null::@Engineering%%
%%tag::Requirement::10b4c8bd-6d18-435a-80f5-59382b9d61d7::"Sign operator must satisfy the formal expression σ̂|ψ⟩ = σ|ψ⟩ with σ ∈ {+1, -1}"::null::@Engineering%%
%%tag::Decision::71d22d5b-3af6-4b89-8f86-690ff261e3f3::"Adopt binary moral orientation model for sign operator"::null::@Product%%
%%tag::Decision::f96ea7b9-f0bb-4934-ac43-af2cc64e9279::"Classify sign operator as a formal definition/primitive formalization"::null::@Engineering%%
%%tag::Risk::e240678d-f3b4-4ae8-a799-22362d22aef5::"Binary moral orientation claim may be too strong if moral spectrum is continuous"::null::@Product%%
%%tag::Assumption::028e5877-5b3b-4806-9516-d80c3de33725::"Moral orientation is binary (either +1 or -1)"::null::@Product%%
%%tag::Dependency::17909f41-6018-4349-817c-c49c813cf6b6::"D8.1 depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::OpenQuestion::d44f81da-1f49-48ce-b9d9-39681c6528db::"Is moral orientation truly binary or continuous?"::null::@Product%%
%%tag::Metric::a01a954c-e218-44c1-8641-62e789a659ee::"Eigenvalue σ values are +1 and -1"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
