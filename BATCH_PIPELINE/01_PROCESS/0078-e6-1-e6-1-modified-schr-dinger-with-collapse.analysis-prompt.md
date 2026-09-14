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
  "title": "Modified Schr\u00f6dinger With Collapse",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0078_E6.1_E6_1_Modified_Schr_dinger_With_Collapse.md",
  "source_sha256": "a14820990c48f56e87cc96933fb27b64507c543277e57b840ca4ba8a851ffa9e",
  "captured_at": "2026-09-07T00:09:29.545862+00:00",
  "word_count": 364,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0078: E6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 E6.1 \u2014 Modified Schr\u00f6dinger With Collapse",
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

# Canonical Sequential Record 0078: E6.1

| Field | Value |
|---|---|
| Registry position | 78 of 191 |
| Canonical ID | `E6.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `E6_1_Modified_Schrödinger_With_Collapse.md` |
| Source SHA-256 | `4649f5f9d666875ca6b58ae40e3ba496ff6249187ca925a70c89ac99d60e8226` |

## Sequence Navigation

Previous: [0077 - A6.3 - Irreversibility](0077_A6.3_A6_3_Irreversibility.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0079 - P6.1 - Collapse Rate Proportional To Phi](0079_P6.1_P6_1_Collapse_Rate_Proportional_To_Phi.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [D6.1 - Collapse Rate Gamma](0029_D6.1_D6_1_Collapse_Rate_Gamma.md)

## Canonical Source Text

---
id: "E6.1"
legacy_id: "AX-050"
title: "Modified Schrödinger With Collapse"
mode: "FW_EXTENDED"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1, D6.1"
date: "2026-08-17"
---

# 🔶 E6.1 — Modified Schrödinger With Collapse

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-050`

---

## Statement

Modified Schrödinger equation with objective collapse term.

## Formal Expression

```
iℏ∂|ψ⟩/∂t = Ĥ|ψ⟩ - (Γ/2)·(|ψ⟩ - P̂|ψ⟩)
```

$$
iℏ∂|ψ⟩/∂t = Ĥ|ψ⟩ - (Γ/2)·(|ψ⟩ - P̂|ψ⟩)
$$

## Maximum Defensible Position

Framework claim: Modified Schrödinger equation with objective collapse term.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `equation` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1
**Depends On:** A5.1, D6.1

## References

GRW theory; Penrose objective collapse

## Defeat Conditions

Show standard Schrödinger suffices.

## Limitations (from workbook)

Similar to GRW but with Φ-dependent Γ. Distinguishable from GRW experimentally.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::ec3d4b8b-8e03-4b4a-aa9c-42824d8203e7::"Modified Schrödinger equation must include objective collapse term"::null::@Engineering%%
%%tag::Requirement::1f90f790-056c-4834-95f2-29c34cbba7ad::"Collapse rate Gamma must be proportional to Phi"::null::@Engineering%%
%%tag::Decision::59b5961f-15ae-4c85-815e-a7822a732402::"Adopt modified Schrödinger equation with objective collapse term"::null::@Engineering%%
%%tag::Decision::70d245d3-b153-412b-8826-060c686e3ef6::"Use GRW theory and Penrose objective collapse as references"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::ae87e58b-39b9-4979-adc9-5ca648784445::"Depends on D6.1 Collapse Rate Gamma"::null::@Engineering%%
%%tag::Risk::394e24f9-cd0d-4181-9a0a-ed15e243b30b::"Risk that standard Schrödinger equation suffices, invalidating this modification"::null::@Engineering%%
%%tag::Assumption::5e2b0858-e815-44a0-94cc-f2ae172e0980::"Assumption that the modified equation is distinguishable from GRW experimentally"::null::@Engineering%%
%%tag::Metric::646059e6-341f-41a8-97d5-c2073a65f0e0::"Collapse rate Gamma value"::null::@Engineering%%
%%tag::Stakeholder::30cc2c9d-06f4-44ff-8d0f-63478ea03a8c::"Framework maintainers interested in ontological equations"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
