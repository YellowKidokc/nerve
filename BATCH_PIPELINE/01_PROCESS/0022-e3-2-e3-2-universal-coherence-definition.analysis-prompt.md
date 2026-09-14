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
  "title": "Universal Coherence Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0022_E3.2_E3_2_Universal_Coherence_Definition.md",
  "source_sha256": "d4efd52e9ffb8100a057ad28fab1f335c6a8ac7f263fd96894b06a93eb69a985",
  "captured_at": "2026-09-06T23:22:08.674711+00:00",
  "word_count": 403,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0022: E3.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E3.2 \u2014 Universal Coherence Definition",
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

# Canonical Sequential Record 0022: E3.2

| Field | Value |
|---|---|
| Registry position | 22 of 191 |
| Canonical ID | `E3.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E3_2_Universal_Coherence_Definition.md` |
| Source SHA-256 | `3ef04b011d317a8fa98f16ef8197acee343d309374a751bd4c511a2d313dcea7` |

## Sequence Navigation

Previous: [0021 - E3.1 - Master Coherence Equation](0021_E3.1_E3_1_Master_Coherence_Equation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0023 - D4.1 - Kolmogorov Complexity](0023_D4.1_D4_1_Kolmogorov_Complexity.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Canonical Source Text

---
id: "E3.2"
legacy_id: "AX-022"
title: "Universal Coherence Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2, D3.1"
date: "2026-08-17"
---

# 📐 E3.2 — Universal Coherence Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-022`

---

## Statement

Universal coherence as weighted sum across all scales.

## Formal Expression

```
C_universal = Σ_i w_i · C_i — weighted sum across scales
```

$$
C_universal = Σ_i w_i · C_i — weighted sum across scales
$$

## Maximum Defensible Position

Model: Universal coherence = weighted multi-scale sum.

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
**Depends On:** A2.2, D3.1
**Chain Tier:** 4 — Coherence System

## References

Multi-scale analysis; Renormalization group

## Defeat Conditions

Show scales are independent (no cross-scale coherence). Demonstrate wrong weighting scheme.

## Limitations (from workbook)

The weights w_i are unspecified. Without them, predictions are qualitative only.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::941f656c-bb94-4446-8e52-c7508f0e12c2::"Universal coherence must be defined as a weighted sum across all scales"::null::@Engineering%%
%%tag::Requirement::179ab20f-1f97-490f-b9ec-f6bb2420d5f7::"The formal expression must be C_universal = Σ_i w_i · C_i"::null::@Engineering%%
%%tag::Decision::d1f37db0-d7dc-40a3-b20d-43a7657a4c3b::"Adopt weighted multi-scale sum as the model for universal coherence"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Dependency::57615bda-6e3b-482c-a1b6-64989354bcc5::"Depends on D3.1 (Coherence Functional Definition)"::null::@Engineering%%
%%tag::Risk::2ccb39a9-29c1-4af1-9a2f-b79dc0ac1229::"Weights w_i are unspecified, limiting predictions to qualitative only"::null::@Engineering%%
%%tag::Risk::bf6d9a8a-5a4a-470a-bc68-fc455d185e4d::"If scales are independent, the universal coherence definition fails"::null::@Engineering%%
%%tag::Risk::07452e83-48e6-4b1b-b2e3-d6376e56e900::"Wrong weighting scheme could invalidate the model"::null::@Engineering%%
%%tag::Assumption::f182444b-27b5-4d2b-abe3-eee74b3bea3b::"Scales exhibit cross-scale coherence"::null::@Engineering%%
%%tag::Assumption::7beeea3f-d3a6-4a5b-aacf-a0d4a63b46a5::"A weighting scheme exists and is meaningful"::null::@Engineering%%
%%tag::OpenQuestion::d35d0d15-f906-47fc-80ac-088af0018e73::"What are the specific values of the weights w_i?"::null::@Engineering%%
%%tag::OpenQuestion::7d16a337-e7d6-4f6b-8f93-8637f3b9d0af::"How to determine the correct weighting scheme?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
