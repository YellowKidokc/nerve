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
  "title": "Law X: Trinity Closure",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0054_D19.10_D19_10_Law_X_Trinity_Closure.md",
  "source_sha256": "d67974dd4261f4939a61173abb7c0077ba0d27a769e6b6659d6f4d8e6bf7b0c5",
  "captured_at": "2026-09-06T23:51:52.970407+00:00",
  "word_count": 355,
  "tags": [
    "information-theory",
    "theology"
  ],
  "source_outline": [
    "Canonical Sequential Record 0054: D19.10",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.10 \u2014 Law X: Trinity Closure",
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

# Canonical Sequential Record 0054: D19.10

| Field | Value |
|---|---|
| Registry position | 54 of 191 |
| Canonical ID | `D19.10` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_10_Law_X_Trinity_Closure.md` |
| Source SHA-256 | `f00c1de0cef3cfea187acda3db69016b51c00fc86138088d1c8562684e4c6d70` |

## Sequence Navigation

Previous: [0053 - D19.9 - Law IX: Grace Non-Unitarity](0053_D19.9_D19_9_Law_IX_Grace_Non-Unitarity.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0055 - E19.1 - Full Master Equation](0055_E19.1_E19_1_Full_Master_Equation.md)

## Registered Dependency Links

[BC4 - Three Observers Required](0006_BC4_BC4_Three_Observers_Required.md)

## Canonical Source Text

---
id: "D19.10"
legacy_id: "AX-145"
title: "Law X: Trinity Closure"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "BC4 (Three Observers Required)"
depends_on: "depends on anchor: BC4 (Three Observers Required)"
date: "2026-08-17"
---

# 📐 D19.10 — Law X: Trinity Closure

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-145`

---

## Statement

Law X: Complete measurement closure requires exactly three observer-operators.

## Formal Expression

```
N_observers = 3; perichoretic: each measures the other two
```

$$
N_observers = 3; perichoretic: each measures the other two
$$

## Maximum Defensible Position

Definition: Law X — Trinity Closure.

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

**Anchor:** BC4 (Three Observers Required)
**Depends On:** depends on anchor: BC4 (Three Observers Required)
**Chain Tier:** 13 — Master Laws

## References

Shannon (error correction N=3); Nicene Creed

## Defeat Conditions

Show N≠3 suffices for closure.

## Limitations (from workbook)

The perichoretic requirement needs formal justification beyond error correction.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::80c861f1-4687-4ab2-99ce-5e1a36f2addc::"Complete measurement closure requires exactly three observer-operators"::null::@Engineering%%
%%tag::Requirement::2f1ac702-077c-4f84-8ab1-a8e49d63bbcd::"Each observer measures the other two observers"::80c861f1-4687-4ab2-99ce-5e1a36f2addc::@Engineering%%
%%tag::Dependency::056afef6-397a-4847-8a2c-c5314261b38e::"Depends on BC4 (Three Observers Required)"::null::@Engineering%%
%%tag::Risk::4051bd46-693a-4c09-b139-8e4d2ba8a7b9::"Perichoretic requirement lacks formal justification beyond error correction"::null::@Engineering%%
%%tag::OpenQuestion::b6093735-2bb4-40fa-a290-ab3709bf22c4::"Whether N≠3 suffices for closure"::null::@Engineering%%
%%tag::Reference::9fa1f187-a42f-4cc9-9092-36f7137cba5a::"Shannon error correction N=3"::null::@Engineering%%
%%tag::Reference::a68a1bcc-2fc7-4a3d-9724-2d2d6c088dec::"Nicene Creed"::null::@Legal%%
%%--- END SEMANTIC TAGS ---%%
