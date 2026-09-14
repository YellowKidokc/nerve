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
  "title": "Information Anchor Necessity",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0060_LN2.1_LN2_1_Information_Anchor_Necessity.md",
  "source_sha256": "745f9f8e522579dc6e538e384298438cdba3b78e01d15a4b3bd98693aae03c5a",
  "captured_at": "2026-09-06T23:56:54.761334+00:00",
  "word_count": 320,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0060: LN2.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN2.1 \u2014 Information Anchor Necessity",
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

# Canonical Sequential Record 0060: LN2.1

| Field | Value |
|---|---|
| Registry position | 60 of 191 |
| Canonical ID | `LN2.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN2_1_Information_Anchor_Necessity.md` |
| Source SHA-256 | `6a9abe075bfcaed55174a6b5c61d429eab9be4378ba492720103add741b1b1ae` |

## Sequence Navigation

Previous: [0059 - P2.2 - Chi Semantic Content](0059_P2.2_P2_2_Chi_Semantic_Content.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0061 - A3.1 - Order Requirement](0061_A3.1_A3_1_Order_Requirement.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "LN2.1"
legacy_id: "AX-015"
title: "Information Anchor Necessity"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 LN2.1 — Information Anchor Necessity

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-015`

---

## Statement

Information requires an anchor point — it cannot be free-floating.

## Formal Expression

```
∀I: ∃anchor(I) — information requires ground
```

$$
∀I: ∃anchor(I) — information requires ground
$$

## Maximum Defensible Position

Framework claim: Information requires an anchor point — it cannot be free-floating.

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

**Anchor:** A2.2
**Depends On:** A2.2

## References

Grounding theories; Aristotle (hylomorphism)

## Defeat Conditions

Show free-floating information.

## Limitations (from workbook)

Strengthens A2.1. Constrains information to be always grounded.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::c62f27ac-24e2-4456-9d56-509cb60a6060::"Information must have an anchor point"::null::@Engineering,Product%%
%%tag::Requirement::40c770e5-097d-43ce-9dfc-0a0a39217d3c::"Information cannot be free-floating"::null::@Engineering,Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Assumption::567503a2-af80-45db-9dd8-8e17d34b50a8::"Grounding theories and Aristotle's hylomorphism are valid references"::null::@Engineering%%
%%tag::Risk::4b3bc663-35fe-499a-b40c-d2a108a95feb::"If free-floating information is demonstrated, the necessity is defeated"::null::@Engineering%%
%%tag::OpenQuestion::444f8786-4624-46e6-9c52-f9e4bbd9a9c4::"Can free-floating information be shown to exist?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
