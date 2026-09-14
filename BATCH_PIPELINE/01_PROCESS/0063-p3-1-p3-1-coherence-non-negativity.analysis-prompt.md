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
  "title": "Coherence Non-Negativity",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0063_P3.1_P3_1_Coherence_Non-Negativity.md",
  "source_sha256": "7cfdbb9dbd2e2f4eea114ec3c8676314d05f04bd9404a7fac0b26cd0c9b65293",
  "captured_at": "2026-09-06T23:59:12.990480+00:00",
  "word_count": 336,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0063: P3.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P3.1 \u2014 Coherence Non-Negativity",
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

# Canonical Sequential Record 0063: P3.1

| Field | Value |
|---|---|
| Registry position | 63 of 191 |
| Canonical ID | `P3.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P3_1_Coherence_Non-Negativity.md` |
| Source SHA-256 | `39819fb3eaff415ba87f78ebc0e1df2ee3e91ca27756665b535dfd7785a74f14` |

## Sequence Navigation

Previous: [0062 - A3.2 - Coherence Measure](0062_A3.2_A3_2_Coherence_Measure.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0064 - P3.2 - Coherence Conservation](0064_P3.2_P3_2_Coherence_Conservation.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Canonical Source Text

---
id: "P3.1"
legacy_id: "AX-023"
title: "Coherence Non-Negativity"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2, D3.1"
date: "2026-08-17"
---

# 🔶 P3.1 — Coherence Non-Negativity

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-023`

---

## Statement

Coherence is non-negative — you can't have negative order.

## Formal Expression

```
C ≥ 0
```

$$
C \ge 0
$$

## Maximum Defensible Position

Framework claim: Coherence is non-negative — you can't have negative order.

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
**Depends On:** A2.2, D3.1

## References

Entropy (S ≥ 0); IIT (Φ ≥ 0)

## Defeat Conditions

Show negative coherence is meaningful.

## Limitations (from workbook)

Standard constraint. Like entropy, coherence has a natural zero.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::928211eb-6ea5-4de6-b2f9-bc290129b492::"Coherence must be non-negative"::null::@Engineering%%
%%tag::Decision::bfbcb376-4452-41e5-884b-3ec494087687::"Adopt coherence non-negativity as a framework property"::null::@Engineering%%
%%tag::Assumption::7d031f3e-11c8-4c94-9fed-b5fe61acf4ea::"Coherence has a natural zero, similar to entropy"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::57615bda-6e3b-482c-a1b6-64989354bcc5::"Depends on D3.1 Coherence Functional Definition"::null::@Engineering%%
%%tag::Risk::d95bf229-33f2-498e-a9d3-a430510a7ad5::"Negative coherence could be shown to be meaningful"::null::@Engineering%%
%%tag::Metric::df7f1261-6d66-425b-a2b0-06bfac458001::"Coherence value C must be greater than or equal to 0"::null::@Engineering%%
%%tag::OpenQuestion::01ce089a-0735-4ac3-a642-1d1799f22d2b::"Can negative coherence be meaningfully defined?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
