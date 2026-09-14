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
  "title": "Coherence Measure",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0062_A3.2_A3_2_Coherence_Measure.md",
  "source_sha256": "347ef0a82f060ad8f3bb7b50d692b6d98923a634accac97a85333029dd5a6061",
  "captured_at": "2026-09-06T23:58:29.536241+00:00",
  "word_count": 318,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0062: A3.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A3.2 \u2014 Coherence Measure",
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

# Canonical Sequential Record 0062: A3.2

| Field | Value |
|---|---|
| Registry position | 62 of 191 |
| Canonical ID | `A3.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A3_2_Coherence_Measure.md` |
| Source SHA-256 | `b339aa3cc8dcfaac815e633b99f2c39ec01224abceb5fdc627e42ebe4037e48f` |

## Sequence Navigation

Previous: [0061 - A3.1 - Order Requirement](0061_A3.1_A3_1_Order_Requirement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0063 - P3.1 - Coherence Non-Negativity](0063_P3.1_P3_1_Coherence_Non-Negativity.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "A3.2"
legacy_id: "AX-017"
title: "Coherence Measure"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 A3.2 — Coherence Measure

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-017`

---

## Statement

Order can be quantified — there exists a coherence measure.

## Formal Expression

```
∃C: C = measure(Order)
```

$$
∃C: C = measure(Order)
$$

## Maximum Defensible Position

Framework claim: Order can be quantified — there exists a coherence measure.

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

IIT; Entropy measures

## Defeat Conditions

Show order without quantification.

## Limitations (from workbook)

Standard assumption in physics. Entropy quantifies disorder; coherence quantifies order.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::4f7acb69-d88a-4475-a185-b18cd24e7e5c::"Order must be quantifiable via a coherence measure"::null::@Engineering%%
%%tag::Decision::27cec20c-fbbf-4bce-969b-1ed6dfef52d1::"Adopt coherence measure as framework commitment"::null::@Engineering%%
%%tag::Assumption::aebc9de9-bf71-4fec-97d7-19960e426bcd::"Standard assumption in physics that entropy quantifies disorder and coherence quantifies order"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::31a55fd1-89b3-4c86-91e4-ab430b86fe2b::"Risk of defeat if order without quantification is shown"::null::@Engineering%%
%%tag::Metric::e358c1eb-f12b-41d3-a6ac-a1ea04a20b21::"Coherence measure C = measure(Order)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
