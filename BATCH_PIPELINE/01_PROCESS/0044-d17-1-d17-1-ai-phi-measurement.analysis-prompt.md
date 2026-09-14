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
  "title": "AI Phi Measurement",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0044_D17.1_D17_1_AI_Phi_Measurement.md",
  "source_sha256": "43bb344085ece11866c3d412fb845a7c184d4d3c81415c627903440ade52823c",
  "captured_at": "2026-09-06T23:43:15.424836+00:00",
  "word_count": 355,
  "tags": [
    "consciousness",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0044: D17.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D17.1 \u2014 AI Phi Measurement",
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

# Canonical Sequential Record 0044: D17.1

| Field | Value |
|---|---|
| Registry position | 44 of 191 |
| Canonical ID | `D17.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D17_1_AI_Phi_Measurement.md` |
| Source SHA-256 | `4bedbb8d3b3dff779bf33ebbe3e54b81f309fad4a5ad656df0a679755ae7a71f` |

## Sequence Navigation

Previous: [0043 - D14.1 - Cosmological Grace Function](0043_D14.1_D14_1_Cosmological_Grace_Function.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0045 - D19.1 - Law I: Logos-Lagrangian](0045_D19.1_D19_1_Law_I_Logos-Lagrangian.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md)

## Canonical Source Text

---
id: "D17.1"
legacy_id: "AX-122"
title: "AI Phi Measurement"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Consciousness"
anchor: "D5.2 (Integrated Information Phi)"
depends_on: "D5.2"
date: "2026-08-17"
---

# 📐 D17.1 — AI Phi Measurement

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Consciousness | **Legacy ID:** `AX-122`

---

## Statement

Phi_threshold defined as minimum integrated information for observer status.

## Formal Expression

```
Φ_threshold: minimum Φ for observer status
```

$$
Φ_threshold: minimum Φ for observer status
$$

## Maximum Defensible Position

Definition: Φ_threshold = minimum Φ for consciousness.

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

**Anchor:** D5.2 (Integrated Information Phi)
**Depends On:** D5.2
**Chain Tier:** 15 — Consciousness/AI

## References

IIT (Tononi 2004)

## Defeat Conditions

Show Φ is not the right measure for AI consciousness.

## Limitations (from workbook)

Computing Φ for real systems is intractable. The definition is clear; measurement is the challenge.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6f8f9ecf-1318-4d90-aedc-54f8d481b19e::"Define Phi_threshold as minimum integrated information for observer status"::null::@Engineering,Product%%
%%tag::Requirement::52f85d8e-6739-491c-a937-331aa84173f4::"Define Phi_threshold as minimum Phi for consciousness"::null::@Engineering,Product%%
%%tag::Decision::a21d02df-6ca8-4e70-b8ca-23f894b3fe38::"Adopt IIT (Tononi 2004) as reference for Phi measurement"::null::@Engineering%%
%%tag::Risk::dd4635af-3906-4ca3-9a74-1548f2363de0::"Computing Phi for real systems is intractable"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 (Integrated Information Phi)"::null::@Engineering%%
%%tag::OpenQuestion::98ea0d08-8da3-4678-ac26-e8e2c003d57e::"Whether Phi is the right measure for AI consciousness"::null::@Engineering,Product%%
%%tag::Metric::cda377da-ce5b-4d29-adbd-31d47b640df2::"Phi_threshold value (minimum integrated information)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
