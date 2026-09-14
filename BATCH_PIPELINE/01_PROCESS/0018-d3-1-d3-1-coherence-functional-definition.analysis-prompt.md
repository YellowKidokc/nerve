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
  "title": "Coherence Functional Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0018_D3.1_D3_1_Coherence_Functional_Definition.md",
  "source_sha256": "365b2300b337f9f5c9af3864bd6dcd63878b2aac502f9918fcd5f1cc31ed2ae4",
  "captured_at": "2026-09-06T23:17:56.761201+00:00",
  "word_count": 379,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0018: D3.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D3.1 \u2014 Coherence Functional Definition",
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

# Canonical Sequential Record 0018: D3.1

| Field | Value |
|---|---|
| Registry position | 18 of 191 |
| Canonical ID | `D3.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D3_1_Coherence_Functional_Definition.md` |
| Source SHA-256 | `945a90da267f9206f50b43135ffaf6c2fcf0febcaec33e6886f7c5abc4fced5d` |

## Sequence Navigation

Previous: [0017 - E2.1 - Master Equation First Form](0017_E2.1_E2_1_Master_Equation_First_Form.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0019 - D3.2 - Self-Interaction Potential](0019_D3.2_D3_2_Self-Interaction_Potential.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D3.1"
legacy_id: "AX-018"
title: "Coherence Functional Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D3.1 — Coherence Functional Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-018`

---

## Statement

Coherence functional defined as normalized gradient measure of χ.

## Formal Expression

```
C[χ] = ∫|∇χ|²dV / ∫|χ|²dV — gradient coherence functional
```

$$
C[χ] = ∫|∇χ|²dV / ∫|χ|²dV — gradient coherence functional
$$

## Maximum Defensible Position

Definition: C[χ] = coherence functional.

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

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 4 — Coherence System

## References

Functional analysis; Sobolev spaces; IIT

## Defeat Conditions

Show coherence is better measured differently. Demonstrate this functional misses key coherence features.

## Limitations (from workbook)

The gradient measure captures spatial coherence. Temporal coherence may need separate treatment.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::57d5437f-15e0-42a7-a1df-d67fd83185e4::"Coherence functional must be defined as normalized gradient measure of χ"::null::@Engineering%%
%%tag::Requirement::2f043cad-cd29-49ea-8449-fb31ef937f19::"Coherence functional must be expressed as C[χ] = ∫|∇χ|²dV / ∫|χ|²dV"::null::@Engineering%%
%%tag::Decision::e80e000a-5a4f-49e1-822a-b43a4d6abc99::"Adopt gradient coherence functional as formal definition of coherence"::null::@Engineering%%
%%tag::Risk::33a5a886-a04d-4412-845a-0120ecf9aea1::"Coherence functional may miss key coherence features"::null::@Engineering%%
%%tag::Risk::86cc4763-ebf8-4129-83c9-bde0d01b7e4c::"Temporal coherence may not be captured by gradient measure"::null::@Engineering%%
%%tag::Assumption::f05268ed-6219-4c9e-8e19-928e72d6a7a3::"Gradient measure adequately captures spatial coherence"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::OpenQuestion::dd78fd60-88ad-43dd-b2ca-27968d14fa73::"Is coherence better measured differently?"::null::@Engineering%%
%%tag::OpenQuestion::d254b366-bbf8-4c15-a2b6-8d76631495b1::"Does the functional miss key coherence features?"::null::@Engineering%%
%%tag::OpenQuestion::0f183564-37fc-4a72-8982-2dcaa9f27342::"How to treat temporal coherence separately?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
