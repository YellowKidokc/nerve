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
  "title": "Master Equation First Form",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0017_E2.1_E2_1_Master_Equation_First_Form.md",
  "source_sha256": "6b849490fb968cce4accfb26ad001b5eaaf058a9218505a317657a11b5eba56a",
  "captured_at": "2026-09-06T23:17:06.386071+00:00",
  "word_count": 432,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0017: E2.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E2.1 \u2014 Master Equation First Form",
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

# Canonical Sequential Record 0017: E2.1

| Field | Value |
|---|---|
| Registry position | 17 of 191 |
| Canonical ID | `E2.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E2_1_Master_Equation_First_Form.md` |
| Source SHA-256 | `59007e0743883dd1c8fc73f64f572203542dc59cd6aa7f290378e5ae2e619e02` |

## Sequence Navigation

Previous: [0016 - D2.2 - Chi Field Properties](0016_D2.2_D2_2_Chi_Field_Properties.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0018 - D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "E2.1"
legacy_id: "AX-012"
title: "Master Equation First Form"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 E2.1 — Master Equation First Form

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-012`

---

## Statement

The Master Equation in first form — χ-field dynamics.

## Formal Expression

```
□χ + V'(χ) = J(χ) — Klein-Gordon with self-interaction and source
```

$$
□χ + V'(χ) = J(χ) — Klein-Gordon with self-interaction and source
$$

## Maximum Defensible Position

Model: χ-field equation of motion.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 3 — Field Definitions

## References

Klein-Gordon equation; Higgs field equation

## Defeat Conditions

Show the equation is internally inconsistent. Demonstrate wrong predictions for known phenomena.

## Limitations (from workbook)

Schematic. Needs V(χ) and J(χ) specified to make predictions.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::3bc51785-fa65-4ab4-bdbb-adbf9d999eb9::"Master Equation first form must be internally consistent"::null::@Engineering%%
%%tag::Requirement::0842f22b-0d5a-4d26-a8be-311d1136a54c::"Master Equation must make correct predictions for known phenomena"::null::@Engineering%%
%%tag::Requirement::92a92fe7-6309-4ec4-901b-f8c71b63ac43::"V(χ) and J(χ) must be specified to enable predictions"::null::@Engineering%%
%%tag::Decision::10f73d0b-05f0-4936-bad2-30955c071c1b::"Adopt Klein-Gordon form with self-interaction and source for Master Equation"::null::@Engineering%%
%%tag::Decision::6d1a61da-a50c-4e2f-a0eb-49287de8bed6::"Classify E2.1 as equation/prediction in workbook"::null::@Product%%
%%tag::Risk::5458bc4b-5ebb-4448-8196-c5b5af70ae88::"Internal inconsistency in the equation could invalidate the model"::null::@Engineering%%
%%tag::Risk::ed448438-e8d0-4ada-bed2-9136b26cf745::"Wrong predictions for known phenomena would defeat the equation"::null::@Engineering%%
%%tag::Risk::a6676edf-4710-4668-b7e3-f6f12ea92fa7::"Schematic nature limits predictive power until V(χ) and J(χ) are specified"::null::@Engineering%%
%%tag::Assumption::ae19f0cb-deea-4245-b018-2e80b12d7196::"The χ-field dynamics follow a Klein-Gordon equation with self-interaction"::null::@Engineering%%
%%tag::Assumption::b6ce8988-ad29-40a9-be10-25f9a33cafc6::"The equation can be anchored to A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::fd66c3a0-dc36-45ea-9ba3-708d340340ee::"Depends on A2.2 Self-Grounding definition"::null::@Engineering%%
%%tag::Dependency::f6a0e1e0-4f88-4e18-9d7a-9151602f8800::"Requires specification of V(χ) and J(χ) for predictions"::null::@Engineering%%
%%tag::OpenQuestion::4a4875c7-d88b-48db-a960-a12e0a209be0::"What are the exact forms of V(χ) and J(χ)?"::null::@Engineering%%
%%tag::Stakeholder::a4bab908-120b-457d-aee6-ff521855a736::"AXIOM_CHAIN_MASTER_v2 maintainers (care about equation consistency)"::null::@Engineering%%
%%tag::Stakeholder::68066067-bcd1-42c0-b17e-8ed314ee2db4::"AXIOMS_PART1_MODE_CLASSIFICATION authors (care about classification)"::null::@Product%%
%%tag::Stakeholder::ce77909d-0985-4b9b-a021-ee2b6f48d422::"AX Truth Kernel classifiers (care about epistemic status)"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
