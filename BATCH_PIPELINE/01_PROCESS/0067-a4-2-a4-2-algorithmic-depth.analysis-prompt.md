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
  "title": "Algorithmic Depth",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0067_A4.2_A4_2_Algorithmic_Depth.md",
  "source_sha256": "ed2fa5147137cf3547479a9360754b1c3788496a53cf211d170d03ad3e90fefa",
  "captured_at": "2026-09-07T00:01:12.370966+00:00",
  "word_count": 333,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0067: A4.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A4.2 \u2014 Algorithmic Depth",
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

# Canonical Sequential Record 0067: A4.2

| Field | Value |
|---|---|
| Registry position | 67 of 191 |
| Canonical ID | `A4.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A4_2_Algorithmic_Depth.md` |
| Source SHA-256 | `80e2f07944b9953f923666d2b3373b0c7a0da5434f42b3d395b459a1fb7e2b96` |

## Sequence Navigation

Previous: [0066 - A4.1 - Parsimony](0066_A4.1_A4_1_Parsimony.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0068 - T4.1 - Laws Are Low-K Descriptions](0068_T4.1_T4_1_Laws_Are_Low-K_Descriptions.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "A4.2"
legacy_id: "AX-028"
title: "Algorithmic Depth"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 A4.2 — Algorithmic Depth

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-028`

---

## Statement

Reality has algorithmic depth — it compresses non-trivially.

## Formal Expression

```
Depth(Universe) >> 0
```

$$
Depth(Universe) >> 0
$$

## Maximum Defensible Position

Framework claim: Reality has algorithmic depth — it compresses non-trivially.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Claim` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `framework_commitment` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2
**Depends On:** A2.2

## References

Bennett 1988 (logical depth); Kolmogorov 1963

## Defeat Conditions

Show universe has no algorithmic depth.

## Limitations (from workbook)

Bennett's logical depth is well-defined. Whether the universe has it is an empirical question.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6bde003c-9987-478f-a6c3-650ee08c4cc1::"Reality must have algorithmic depth, compressing non-trivially"::null::@Engineering%%
%%tag::Decision::28242802-06e5-4987-9932-dc33d3b2904b::"Adopt algorithmic depth as a framework commitment"::null::@Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::f8a9cca3-9002-495a-9835-d8f6f0ea70c7::"Risk that universe lacks algorithmic depth, defeating the claim"::null::@Engineering%%
%%tag::Assumption::851ca6fb-27d6-46db-a46d-996c5f7bd187::"Assumes Bennett's logical depth is well-defined"::null::@Engineering%%
%%tag::OpenQuestion::6728e37a-d222-429a-b4d0-63a5963aa392::"Whether the universe has algorithmic depth is an empirical question"::null::@Engineering%%
%%tag::Metric::07d3528f-3f7e-44cc-8598-1944b52ba603::"Algorithmic depth value (Depth(Universe) >> 0)"::null::@Engineering%%
%%tag::Stakeholder::a233357b-9beb-4aa0-b6ce-45bc7a6a2a93::"Framework maintainers interested in ontological claims"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
