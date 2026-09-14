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
  "title": "Observer Effect Proportional To Phi",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0073_P5.2_P5_2_Observer_Effect_Proportional_To_Phi.md",
  "source_sha256": "0629ebcbfb2eed028105585736524363b5b1475f26c318ef4e9e57a849e389e6",
  "captured_at": "2026-09-07T00:05:09.180805+00:00",
  "word_count": 375,
  "tags": [
    "consciousness",
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0073: P5.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P5.2 \u2014 Observer Effect Proportional To Phi",
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

# Canonical Sequential Record 0073: P5.2

| Field | Value |
|---|---|
| Registry position | 73 of 191 |
| Canonical ID | `P5.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P5_2_Observer_Effect_Proportional_To_Phi.md` |
| Source SHA-256 | `989fd77c34587c82c8754caa8a9ddbcaecf96456fb272acee61dcf18257470f4` |

## Sequence Navigation

Previous: [0072 - P5.1 - Phi Admits Degrees](0072_P5.1_P5_1_Phi_Admits_Degrees.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0074 - LN5.1 - Chi Requires Observer For Actualization](0074_LN5.1_LN5_1_Chi_Requires_Observer_For_Actualization.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md); [A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "P5.2"
legacy_id: "AX-041"
title: "Observer Effect Proportional To Phi"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1, D5.2"
date: "2026-08-17"
---

# 🔶 P5.2 — Observer Effect Proportional To Phi

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-041`

---

## Statement

The observer's effect on the observed is proportional to the observer's Φ.

## Formal Expression

```
Effect ∝ Φ_observer
```

$$
Effect ∝ Φ_observer
$$

## Maximum Defensible Position

Framework claim: The observer's effect on the observed is proportional to the observer's Φ.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Property` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `property` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1
**Depends On:** A5.1, D5.2

## References

PEAR Lab; GCP; IIT

## Defeat Conditions

Show observer effect is Φ-independent.

## Limitations (from workbook)

THE distinctive prediction. If true, high-Φ observers collapse wavefunctions faster.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::08514085-8f83-4749-bbbb-a6206ea7cc5f::"Observer effect must be proportional to observer's Phi"::null::@Engineering,Product%%
%%tag::Decision::351af041-4628-4f7b-99dd-16c3d3b014b0::"Adopt FW_EXTENDED mode for P5.2"::null::@Product%%
%%tag::Decision::93d8e059-b762-45fa-bf6f-3f97408545d0::"Classify P5.2 as Property in Ontological domain"::null::@Product%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 Integrated Information Phi"::null::@Engineering%%
%%tag::Risk::499ebf81-92c4-4e6f-8de8-b4ac294dfa76::"Risk: observer effect shown to be Phi-independent"::null::@Engineering,Product%%
%%tag::Metric::d53c673c-0305-4b47-82df-7cc298c6f180::"Phi value of observer (Φ_observer)"::null::@Engineering%%
%%tag::Stakeholder::5dce92de-b9ae-44ed-b689-39322a5334f9::"PEAR Lab - interested in observer effects"::null::@Engineering,Operations%%
%%tag::Stakeholder::800cdd6d-8997-4d2f-bc50-66ea14cf6eef::"GCP - interested in global consciousness effects"::null::@Engineering,Operations%%
%%tag::Stakeholder::76838c58-b65f-4761-ac69-aef8bebab595::"IIT - integrated information theory community"::null::@Engineering%%
%%tag::Assumption::808c71c4-9275-4ab0-adb1-63d066c2eb33::"Assumption: high-Phi observers collapse wavefunctions faster"::null::@Engineering,Product%%
%%tag::OpenQuestion::8fe19f44-e7b6-4ed5-99c2-f7463c8db6f7::"How to empirically verify proportionality of observer effect to Phi"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
