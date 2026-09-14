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
  "title": "Collapse Rate Proportional To Phi",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0079_P6.1_P6_1_Collapse_Rate_Proportional_To_Phi.md",
  "source_sha256": "40fd0723a2f1481f8a5d3f17aa3004223458b5e86b7c9ec557fc857ce4be7636",
  "captured_at": "2026-09-07T00:10:32.396595+00:00",
  "word_count": 365,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0079: P6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P6.1 \u2014 Collapse Rate Proportional To Phi",
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

# Canonical Sequential Record 0079: P6.1

| Field | Value |
|---|---|
| Registry position | 79 of 191 |
| Canonical ID | `P6.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P6_1_Collapse_Rate_Proportional_To_Phi.md` |
| Source SHA-256 | `2adfdb2ec44ea1d09521fc2b9c01d8231b8542be0ded06e87875afc69ff7697a` |

## Sequence Navigation

Previous: [0078 - E6.1 - Modified Schrödinger With Collapse](0078_E6.1_E6_1_Modified_Schr_dinger_With_Collapse.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0080 - P6.2 - Collapse Generates Heat](0080_P6.2_P6_2_Collapse_Generates_Heat.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md); [A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [D6.1 - Collapse Rate Gamma](0029_D6.1_D6_1_Collapse_Rate_Gamma.md)

## Canonical Source Text

---
id: "P6.1"
legacy_id: "AX-052"
title: "Collapse Rate Proportional To Phi"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A5.1 → D5.2 → D6.1"
depends_on: "A5.1, D5.2, D6.1"
date: "2026-08-17"
---

# 🔶 P6.1 — Collapse Rate Proportional To Phi

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-052`

---

## Statement

Collapse rate Γ is proportional to observer's integrated information Φ.

## Formal Expression

```
Γ ∝ Φ
```

$$
Γ ∝ Φ
$$

## Maximum Defensible Position

Framework claim: Collapse rate Γ is proportional to observer's integrated information Φ.

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

**Anchor:** A5.1 → D5.2 → D6.1
**Depends On:** A5.1, D5.2, D6.1

## References

IIT; Objective collapse theories

## Defeat Conditions

Run experiment showing Γ ≠ f(Φ).

## Limitations (from workbook)

Most testable prediction. Requires measuring both Γ and Φ simultaneously.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::81f9d683-8d95-4bbc-9e93-ffb77804c2f8::"Collapse rate Γ must be proportional to observer's integrated information Φ"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 Integrated Information Phi"::null::@Engineering%%
%%tag::Dependency::ae87e58b-39b9-4979-adc9-5ca648784445::"Depends on D6.1 Collapse Rate Gamma"::null::@Engineering%%
%%tag::Risk::1f9db10c-7058-4bc2-9012-887ed6c8d4ff::"Experiment showing Γ ≠ f(Φ) would defeat the property"::null::@Engineering%%
%%tag::Metric::6658ac7f-8f1e-4cb3-9495-aea36de46bb9::"Collapse rate Γ"::null::@Engineering%%
%%tag::Metric::e2b82a6e-7796-4de6-8a1f-22e5aecd98ff::"Integrated information Φ"::null::@Engineering%%
%%tag::Requirement::f82fe7a5-f3c1-4719-823f-5b62934169b3::"Must measure both Γ and Φ simultaneously"::null::@Engineering%%
%%tag::OpenQuestion::6000dab9-78e0-4151-b2d0-fc24bc21ee22::"How to measure both Γ and Φ simultaneously"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
