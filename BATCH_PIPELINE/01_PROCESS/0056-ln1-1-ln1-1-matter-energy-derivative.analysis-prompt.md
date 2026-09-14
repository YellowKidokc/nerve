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
  "title": "Matter-Energy Derivative",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0056_LN1.1_LN1_1_Matter-Energy_Derivative.md",
  "source_sha256": "e806509d0f8bb29890263466e0d661674bb9bc14fa41fcf06e5cff8432f28591",
  "captured_at": "2026-09-06T23:53:35.880571+00:00",
  "word_count": 367,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0056: LN1.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN1.1 \u2014 Matter-Energy Derivative",
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

# Canonical Sequential Record 0056: LN1.1

| Field | Value |
|---|---|
| Registry position | 56 of 191 |
| Canonical ID | `LN1.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN1_1_Matter-Energy_Derivative.md` |
| Source SHA-256 | `55fb775a48e55d8721101fb8e39d0bea3b0cea3df512b34852016c0854f19557` |

## Sequence Navigation

Previous: [0055 - E19.1 - Full Master Equation](0055_E19.1_E19_1_Full_Master_Equation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0057 - LN1.2 - It From Bit](0057_LN1.2_LN1_2_It_From_Bit.md)

## Registered Dependency Links

[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "LN1.1"
legacy_id: "AX-006"
title: "Matter-Energy Derivative"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A1.1, A1.2"
depends_on: "A1.1, A1.2"
date: "2026-08-17"
---

# 🔶 LN1.1 — Matter-Energy Derivative

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-006`

---

## Statement

Matter and energy are derivative of information, not fundamental.

## Formal Expression

```
∀m,E: Derivative(m, Information) ∧ Derivative(E, Information)
```

$$
∀m,E: Derivative(m, Information) ∧ Derivative(E, Information)
$$

## Maximum Defensible Position

Framework claim: Matter and energy are derivative of information, not fundamental.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Logical Necessity` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `logical_necessity` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A1.1, A1.2
**Depends On:** A1.1, A1.2

## References

Wheelerism; Verlinde 2010

## Defeat Conditions

Show matter/energy is more fundamental than information.

## Limitations (from workbook)

Bold claim. Standard physics has matter/energy as fundamental. 'It from Bit' is minority view.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::acf00518-b8c5-4b8b-991f-07f5890116fd::"Matter and energy must be derivative of information, not fundamental"::null::@Engineering,Product%%
%%tag::Decision::ce71d87f-3e7a-470e-812c-41204417e26e::"Adopt the framework claim that matter and energy are derivative of information"::null::@Product%%
%%tag::Dependency::407195e2-4751-411e-8740-390a792945d3::"Depends on A1.1 Existence and A1.2 Distinction"::null::@Engineering%%
%%tag::Risk::03a43892-6c87-4c2a-a56c-90fcc81251e6::"Standard physics treats matter/energy as fundamental, conflicting with this claim"::null::@Engineering,Product%%
%%tag::Assumption::77d92703-fe04-46a4-bc7e-ae32d83d90cd::"Assumes 'It from Bit' is a valid ontological perspective"::null::@Product%%
%%tag::OpenQuestion::244437e7-0299-42ed-8415-a2b523e1a038::"How to reconcile this claim with standard physics where matter/energy are fundamental"::null::@Engineering,Product%%
%%tag::Stakeholder::e12c52f6-2925-410d-a425-6f01f17fd897::"Wheeler and Verlinde referenced as intellectual sources"::null::@Product%%
%%tag::Metric::3749ebf5-87a7-4cc5-b3ca-387c544dfe15::"Registry position 56 of 191"::null::@Operations%%
%%--- END SEMANTIC TAGS ---%%
