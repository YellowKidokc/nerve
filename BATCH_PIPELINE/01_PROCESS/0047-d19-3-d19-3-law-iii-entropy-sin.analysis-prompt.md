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
  "title": "Law III: Entropy-Sin",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0047_D19.3_D19_3_Law_III_Entropy-Sin.md",
  "source_sha256": "fc0e69c41e56027db2a8adba7fcf7b5a46cf42dbc8ecc55f8f0d172e5993501e",
  "captured_at": "2026-09-06T23:45:43.881986+00:00",
  "word_count": 414,
  "tags": [
    "information-theory"
  ],
  "source_outline": [
    "Canonical Sequential Record 0047: D19.3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.3 \u2014 Law III: Entropy-Sin",
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

# Canonical Sequential Record 0047: D19.3

| Field | Value |
|---|---|
| Registry position | 47 of 191 |
| Canonical ID | `D19.3` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_3_Law_III_Entropy-Sin.md` |
| Source SHA-256 | `6d31f66972f9f82b8e0183a133025b33b3c16bf2104f9e2525f90070e23f8f92` |

## Sequence Navigation

Previous: [0046 - D19.2 - Law II: Ten Variables](0046_D19.2_D19_2_Law_II_Ten_Variables.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0048 - D19.4 - Law IV: Grace Dynamics](0048_D19.4_D19_4_Law_IV_Grace_Dynamics.md)

## Registered Dependency Links

[T3.1 - Coherence Cannot Self-Increase](0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md); [A8.2 - Sign Conservation](0092_A8.2_A8_2_Sign_Conservation.md)

## Canonical Source Text

---
id: "D19.3"
legacy_id: "AX-138"
title: "Law III: Entropy-Sin"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "T3.1, A8.2"
depends_on: "depends on anchor: T3.1, A8.2"
date: "2026-08-17"
---

# 📐 D19.3 — Law III: Entropy-Sin

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-138`

---

## Statement

Law III: The Entropy-Sin Correspondence.

## Formal Expression

```
S_physical ↔ S_moral; dS/dt ≥ 0
```

$$
S_physical ↔ S_moral; \frac{dS}{dt} \ge 0
$$

## Maximum Defensible Position

Definition: Law III — Entropy-Sin.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Unified |

## Dependency Chain

**Anchor:** T3.1, A8.2
**Depends On:** depends on anchor: T3.1, A8.2
**Chain Tier:** 13 — Master Laws

## References

2nd Law; Augustinian Hamartiology

## Defeat Conditions

Show entropy and sin are not isomorphic.

## Limitations (from workbook)

The correspondence is structural, not literal.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::0086bfd8-a6b0-479e-a228-7132926638a4::"Law III must define the Entropy-Sin Correspondence"::null::@Engineering,Product%%
%%tag::Requirement::d9c483c2-ccb4-47e6-9657-2b792d2cbcd9::"Law III must express the formal relationship S_physical ↔ S_moral with dS/dt ≥ 0"::null::@Engineering%%
%%tag::Requirement::2f7734cc-e0d1-4095-b425-d1f9ea533fb3::"Law III must maintain the structural (not literal) nature of the entropy-sin correspondence"::null::@Engineering,Product%%
%%tag::Dependency::ad48b598-a7af-4282-b116-120571928c26::"Law III depends on T3.1 - Coherence Cannot Self-Increase"::null::@Engineering%%
%%tag::Dependency::ecf313a5-718a-4f44-874c-12cc5e8e43bb::"Law III depends on A8.2 - Sign Conservation"::null::@Engineering%%
%%tag::Risk::b010f807-b634-4f45-87e8-6d5712e44818::"Risk that entropy and sin are shown not to be isomorphic (defeat condition)"::null::@Engineering,Product%%
%%tag::Assumption::2481c622-cd14-4dfd-8604-deac5ca5291d::"Assumption that the correspondence between entropy and sin is structural, not literal"::null::@Engineering,Product%%
%%tag::Decision::ed62c1a8-6f96-4e37-a2e8-21ace074d775::"Decision to classify Law III as a formal definition/primitive formalization"::null::@Engineering,Product%%
%%tag::Decision::c9404640-0a1f-43cf-bf8d-35d4f2a5d724::"Decision to anchor Law III to T3.1 and A8.2"::null::@Engineering%%
%%tag::Stakeholder::d1efe1ac-aa57-4271-88c7-c92006c9f504::"Stakeholder: Augustinian Hamartiology (referenced as a source for the law)"::null::@Legal,Product%%
%%tag::Stakeholder::4d14c336-27d4-4f8d-b11c-280aa8b9c856::"Stakeholder: 2nd Law of Thermodynamics (referenced as a source for the law)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
