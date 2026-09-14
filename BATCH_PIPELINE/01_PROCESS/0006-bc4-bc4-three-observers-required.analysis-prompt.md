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
  "title": "Three Observers Required",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0006_BC4_BC4_Three_Observers_Required.md",
  "source_sha256": "a396387e266d7e92fd55264f64a4ad59b8bd5198c36b7904a90ee318615436a8",
  "captured_at": "2026-09-06T23:03:20.179023+00:00",
  "word_count": 418,
  "tags": [
    "information-theory",
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0006: BC4",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 BC4 \u2014 Three Observers Required",
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

# Canonical Sequential Record 0006: BC4

| Field | Value |
|---|---|
| Registry position | 6 of 191 |
| Canonical ID | `BC4` |
| Mode | `AX_CORE` |
| Canonical source filename | `BC4_Three_Observers_Required.md` |
| Source SHA-256 | `5cd527c923896414f5567f53591de89b9a76b1b78ddfe0ef7211a9c648a6e269` |

## Sequence Navigation

Previous: [0005 - A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0007 - BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Registered Dependency Links

_No registered canonical dependency link extracted._

## Canonical Source Text

---
id: "BC4"
legacy_id: "AX-061"
title: "Three Observers Required"
mode: "AX_CORE"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "BC4 (Three Observers Required) — ROOT"
depends_on: "∅ (core root)"
date: "2026-08-17"
---

# 🟢 BC4 — Three Observers Required

**Mode:** `AX_CORE` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-061`

---

## Statement

Complete measurement closure requires exactly three observer-operators.

## Formal Expression

```
N_observers = 3 for complete measurement closure
```

$$
N_observers = 3 for complete measurement closure
$$

## Maximum Defensible Position

Boundary condition: N=3 observers required.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_CORE` |
| **Atlas Object Type** | `Boundary Condition` |
| **Workbook Role** | `BOUNDARY_CONDITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `boundary` |
| **Domain** | Divine |

## Dependency Chain

**Anchor:** BC4 (Three Observers Required) — ROOT
**Depends On:** ∅ (core root)
**Chain Tier:** 8 — Boundary Conditions

## References

Shannon (error correction N=3); Hamming codes; Quantum error correction; Nicene Creed

## Defeat Conditions

Show N≠3 suffices for closure. Demonstrate measurement completion with fewer observers.

## Limitations (from workbook)

The perichoretic requirement goes beyond N=3 — it specifies the RELATIONSHIP between the three.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::80c861f1-4687-4ab2-99ce-5e1a36f2addc::"Complete measurement closure requires exactly three observer-operators"::null::@Engineering%%
%%tag::Requirement::2d12575f-9e50-4fda-898d-8e775c41bc6d::"N_observers must equal 3 for complete measurement closure"::null::@Engineering%%
%%tag::Decision::e27ab5c8-a286-414c-b948-87565cad26ee::"Adopt N=3 as the boundary condition for measurement closure"::null::@Engineering,Product%%
%%tag::Risk::2e3cddd8-2b9e-4fe3-a7a7-85c31675bf2c::"If N≠3 suffices for closure, the boundary condition is invalidated"::null::@Engineering%%
%%tag::Risk::48c0a5e7-34a6-4bfc-971d-5124bf2cf7eb::"Demonstrating measurement completion with fewer observers would defeat the condition"::null::@Engineering%%
%%tag::Assumption::28cc95e7-02af-4fb6-9213-b99a25d11c14::"The perichoretic relationship between the three observers is required beyond just the count"::null::@Engineering,Design%%
%%tag::Dependency::839296e5-6441-4253-a9c6-1d0035d46fbc::"Depends on core root (∅) as foundational dependency"::null::@Engineering%%
%%tag::Metric::6080f1f0-9f67-44b7-b4bc-38848302d112::"N_observers = 3 (target value for measurement closure)"::null::@Engineering%%
%%tag::OpenQuestion::d1088d96-f791-4808-aa32-80fba9eb1443::"Can measurement completion be demonstrated with fewer than three observers?"::null::@Engineering%%
%%tag::OpenQuestion::2a6dc83c-9697-401f-bd4a-721dfc999515::"What is the exact nature of the perichoretic relationship required between the three observers?"::null::@Engineering,Design%%
%%--- END SEMANTIC TAGS ---%%
