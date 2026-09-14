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
  "title": "Uniqueness From Boundary Conditions",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0084_A7.2_A7_2_Uniqueness_From_Boundary_Conditions.md",
  "source_sha256": "103b6235f0d312e6a8e11ccf4db15b12b3d63930000ca7a7226c68dad186d513",
  "captured_at": "2026-09-07T00:15:08.706536+00:00",
  "word_count": 305,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0084: A7.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A7.2 \u2014 Uniqueness From Boundary Conditions",
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

# Canonical Sequential Record 0084: A7.2

| Field | Value |
|---|---|
| Registry position | 84 of 191 |
| Canonical ID | `A7.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A7_2_Uniqueness_From_Boundary_Conditions.md` |
| Source SHA-256 | `35b9024c41ef8bf6b6caf2dadb7e73c05cc762052208552f9df0780a2eeb057e` |

## Sequence Navigation

Previous: [0083 - A7.1 - Closure Requirement](0083_A7.1_A7_1_Closure_Requirement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0085 - BC1 - Terminal Observer Exists](0085_BC1_BC1_Terminal_Observer_Exists.md)

## Registered Dependency Links

[A7.1 - Closure Requirement](0083_A7.1_A7_1_Closure_Requirement.md)

## Canonical Source Text

---
id: "A7.2"
legacy_id: "AX-057"
title: "Uniqueness From Boundary Conditions"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A7.1"
depends_on: "A7.1"
date: "2026-08-17"
---

# 🔶 A7.2 — Uniqueness From Boundary Conditions

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-057`

---

## Statement

Sufficient boundary conditions yield a unique solution.

## Formal Expression

```
Sufficient BCs → ∃! solution
```

$$
Sufficient BCs → ∃! solution
$$

## Maximum Defensible Position

Framework claim: Sufficient boundary conditions yield a unique solution.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Claim` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `framework_commitment` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A7.1
**Depends On:** A7.1

## References

PDE uniqueness theorems; Cauchy-Kowalevski

## Defeat Conditions

Show BCs admit multiple solutions.

## Limitations (from workbook)

Standard math. The question is whether the 8 BCs are sufficient.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::732123cd-017e-4be8-a069-07278dc3d0da::"Sufficient boundary conditions must yield a unique solution"::null::@Engineering%%
%%tag::Dependency::843c5490-f502-4bf6-9749-f9994d29239b::"Depends on A7.1 Closure Requirement"::null::@Engineering%%
%%tag::OpenQuestion::e33d214e-3dba-4c6c-8198-56bd334bd3bc::"Whether the 8 boundary conditions are sufficient"::null::@Engineering%%
%%tag::Risk::50110cec-e6c3-4a33-ab2f-8a7bd2fdfb41::"Boundary conditions may admit multiple solutions"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
