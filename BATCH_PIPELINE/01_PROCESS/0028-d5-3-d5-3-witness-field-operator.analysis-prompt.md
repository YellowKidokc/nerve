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
  "title": "Witness Field Operator",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0028_D5.3_D5_3_Witness_Field_Operator.md",
  "source_sha256": "302089eb2e61f0fc59c5e1867bfb840a46ada1363e1c574eaf87bac6d6ddb987",
  "captured_at": "2026-09-06T23:27:12.178435+00:00",
  "word_count": 345,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0028: D5.3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D5.3 \u2014 Witness Field Operator",
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

# Canonical Sequential Record 0028: D5.3

| Field | Value |
|---|---|
| Registry position | 28 of 191 |
| Canonical ID | `D5.3` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D5_3_Witness_Field_Operator.md` |
| Source SHA-256 | `f58f24ef3501fa7eff5fee8b90c90319c32637136b4ad36adf7653c288f21985` |

## Sequence Navigation

Previous: [0027 - D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0029 - D6.1 - Collapse Rate Gamma](0029_D6.1_D6_1_Collapse_Rate_Gamma.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "D5.3"
legacy_id: "AX-039"
title: "Witness Field Operator"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A5.1 (Observation Requirement)"
depends_on: "A5.1"
date: "2026-08-17"
---

# 📐 D5.3 — Witness Field Operator

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-039`

---

## Statement

Witness field operator defined — formal representation of observation.

## Formal Expression

```
Ŵ = witness field operator; W(x) = ⟨Ŵ⟩
```

$$
Ŵ = witness field operator; W(x) = ⟨Ŵ⟩
$$

## Maximum Defensible Position

Definition: Ŵ = witness field operator.

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

**Anchor:** A5.1 (Observation Requirement)
**Depends On:** A5.1
**Chain Tier:** 6 — Observer System

## References

Quantum measurement theory; Von Neumann 1932

## Defeat Conditions

Show observation doesn't need field operator formalism.

## Limitations (from workbook)

Novel formalism. Needs connection to established measurement theory.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::84e6d0b0-6d87-43f8-9978-64a3a6839e3f::"Observation must be formally represented by a witness field operator"::null::@Engineering%%
%%tag::Decision::415d8aa6-b6d4-43ac-948e-dfc4c0e00e56::"Adopt witness field operator formalism for observation"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::b57a5161-a84f-42d6-be9d-22182a188e81::"Novel formalism lacks connection to established measurement theory"::null::@Engineering%%
%%tag::OpenQuestion::fc1b9123-c3f4-47a3-80b4-f49b7f7ab6f3::"How does the witness field operator connect to established measurement theory?"::null::@Engineering%%
%%tag::Stakeholder::8a301196-190f-4c0d-bd44-40c818d6bdd1::"Quantum measurement theory community (Von Neumann legacy)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
