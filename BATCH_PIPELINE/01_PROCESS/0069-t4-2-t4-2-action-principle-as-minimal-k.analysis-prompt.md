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
  "title": "Action Principle As Minimal-K",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0069_T4.2_T4_2_Action_Principle_As_Minimal-K.md",
  "source_sha256": "9e594a7de2b28ab8215bf35afe4893fb63e6bd0950e86f22f13d172eae1229ad",
  "captured_at": "2026-09-07T00:02:57.371743+00:00",
  "word_count": 368,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0069: T4.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 T4.2 \u2014 Action Principle As Minimal-K",
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

# Canonical Sequential Record 0069: T4.2

| Field | Value |
|---|---|
| Registry position | 69 of 191 |
| Canonical ID | `T4.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `T4_2_Action_Principle_As_Minimal-K.md` |
| Source SHA-256 | `d38994d2cb7d9117ae5404f9b1d541c31e7ef6f982c32d5014ebda07f758420f` |

## Sequence Navigation

Previous: [0068 - T4.1 - Laws Are Low-K Descriptions](0068_T4.1_T4_1_Laws_Are_Low-K_Descriptions.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0070 - LN4.1 - Universe As Compression Algorithm](0070_LN4.1_LN4_1_Universe_As_Compression_Algorithm.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D4.1 - Kolmogorov Complexity](0023_D4.1_D4_1_Kolmogorov_Complexity.md)

## Canonical Source Text

---
id: "T4.2"
legacy_id: "AX-033"
title: "Action Principle As Minimal-K"
mode: "FW_EXTENDED"
atlas_type: "Theorem"
status: "theorem"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2, D4.1"
date: "2026-08-17"
---

# 🔶 T4.2 — Action Principle As Minimal-K

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Theorem` | **Status:** `theorem`
**Domain:** Ontological | **Legacy ID:** `AX-033`

---

## Statement

The principle of least action is a special case of Kolmogorov compression.

## Formal Expression

```
δS = 0 ↔ min K(trajectory)
```

$$
δS = 0 ↔ min K(trajectory)
$$

## Maximum Defensible Position

Framework claim: The principle of least action is a special case of Kolmogorov compression.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Theorem` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `theorem` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2
**Depends On:** A2.2, D4.1

## References

Hamilton's principle; Feynman path integrals

## Defeat Conditions

Show action principle without compression interpretation.

## Limitations (from workbook)

Elegant if true. Connects variational physics to information theory.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::143f9636-1914-4749-b65d-47884f211007::"The principle of least action must be shown as a special case of Kolmogorov compression"::null::@Engineering%%
%%tag::Decision::57397a08-a0ca-4149-830f-b749acce2005::"Adopt the principle of least action as a special case of Kolmogorov compression"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::1db09e44-e14e-4a01-898a-0bee85d0d437::"Depends on D4.1 Kolmogorov Complexity"::null::@Engineering%%
%%tag::Risk::714e32ef-00f1-45c3-84e6-e72d9666f161::"Risk of failing to show action principle without compression interpretation"::null::@Engineering%%
%%tag::Assumption::e502b7fa-904a-49f9-bba2-31dd5453f4ec::"Assumes the principle of least action is elegantly connected to information theory"::null::@Engineering%%
%%tag::OpenQuestion::94c816b3-10d7-4c97-8a51-814f64e06e08::"How to demonstrate the action principle without a compression interpretation"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
