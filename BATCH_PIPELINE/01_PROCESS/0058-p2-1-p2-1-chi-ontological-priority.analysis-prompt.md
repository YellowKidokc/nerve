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
  "title": "Chi Ontological Priority",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0058_P2.1_P2_1_Chi_Ontological_Priority.md",
  "source_sha256": "05d0f5f804f9a46064823586b4ee12fa7d2d8b0c1ff75d848e6ac1387b98ed1c",
  "captured_at": "2026-09-06T23:55:16.076205+00:00",
  "word_count": 313,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0058: P2.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P2.1 \u2014 Chi Ontological Priority",
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

# Canonical Sequential Record 0058: P2.1

| Field | Value |
|---|---|
| Registry position | 58 of 191 |
| Canonical ID | `P2.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P2_1_Chi_Ontological_Priority.md` |
| Source SHA-256 | `890a6ccf7fbd3758eb9f84d509db7391b1fb3de18e67024376e7cd4f7ee6a25f` |

## Sequence Navigation

Previous: [0057 - LN1.2 - It From Bit](0057_LN1.2_LN1_2_It_From_Bit.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0059 - P2.2 - Chi Semantic Content](0059_P2.2_P2_2_Chi_Semantic_Content.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "P2.1"
legacy_id: "AX-013"
title: "Chi Ontological Priority"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 P2.1 — Chi Ontological Priority

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-013`

---

## Statement

χ-field has ontological priority over physical fields.

## Formal Expression

```
χ > g_μν, A_μ (ontological ordering)
```

$$
χ > g_μν, A_μ (ontological ordering)
$$

## Maximum Defensible Position

Framework claim: χ-field has ontological priority over physical fields.

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

**Anchor:** A2.2
**Depends On:** A2.2

## References

Idealism; Panpsychism

## Defeat Conditions

Show physical fields without χ-substrate.

## Limitations (from workbook)

Framework commitment. Standard physics has no such ordering.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::f6e9783f-458c-4d38-8748-e74ef1aaadd5::"χ-field must have ontological priority over physical fields"::null::@Engineering%%
%%tag::Decision::ee7cb49e-ae12-411a-a16f-8adb26c79360::"Adopt χ-field ontological priority over physical fields"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::cb694e5a-d002-4d56-879b-497baeab7d1a::"Defeat condition: showing physical fields without χ-substrate"::null::@Engineering%%
%%tag::Assumption::0fa3a155-ebe1-44ec-b646-5bc71f5f48cc::"Assumes standard physics has no such ordering"::null::@Engineering%%
%%tag::OpenQuestion::a0432a9e-a503-4d7f-ad08-52e8a98e5d6a::"How to demonstrate χ-field priority empirically"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
