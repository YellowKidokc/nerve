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
  "title": "Compression Ratio",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0024_D4.2_D4_2_Compression_Ratio.md",
  "source_sha256": "9c21c980005d85704b6755a2b996d259e318be178c2d89eb9e59cbb468d34c67",
  "captured_at": "2026-09-06T23:23:55.349136+00:00",
  "word_count": 359,
  "tags": [
    "information-theory"
  ],
  "source_outline": [
    "Canonical Sequential Record 0024: D4.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D4.2 \u2014 Compression Ratio",
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

# Canonical Sequential Record 0024: D4.2

| Field | Value |
|---|---|
| Registry position | 24 of 191 |
| Canonical ID | `D4.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D4_2_Compression_Ratio.md` |
| Source SHA-256 | `a928a63f6556bedf07f25f44dc9f8d83ee0d056d728fc32b300b1a46d70d620e` |

## Sequence Navigation

Previous: [0023 - D4.1 - Kolmogorov Complexity](0023_D4.1_D4_1_Kolmogorov_Complexity.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0025 - E4.1 - Complexity Decrease Under Chi](0025_E4.1_E4_1_Complexity_Decrease_Under_Chi.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D4.2"
legacy_id: "AX-030"
title: "Compression Ratio"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D4.2 — Compression Ratio

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-030`

---

## Statement

Compression ratio defined as Kolmogorov complexity over raw length.

## Formal Expression

```
CR(x) = K(x) / |x| — compression ratio
```

$$
CR(x) = K(x) / |x| — compression ratio
$$

## Maximum Defensible Position

Definition: CR(x) = K(x)/|x|.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 5 — Compression Theory

## References

Data compression theory; Algorithmic information theory

## Defeat Conditions

Show compression ratio is wrong measure. Demonstrate better complexity measure.

## Limitations (from workbook)

Standard definition. Low vulnerability.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::25f49e95-3ddb-44f9-a712-2df84c0eb198::"Compression ratio must be defined as Kolmogorov complexity over raw length"::null::@Engineering%%
%%tag::Requirement::d1e71a0f-2f01-421d-ab77-abdd10c0720e::"Compression ratio must be expressed as CR(x) = K(x) / |x|"::null::@Engineering%%
%%tag::Decision::53676bea-91b6-4765-a276-53e7224557a5::"Adopt compression ratio as formal definition for D4.2"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Risk::5f03794f-517f-442b-8a97-2dfdd56f9225::"Compression ratio may be the wrong measure"::null::@Engineering%%
%%tag::Risk::854dbf85-cdd7-4667-9c89-8cb4da1ec1e9::"A better complexity measure may exist"::null::@Engineering%%
%%tag::Assumption::fae740ae-7d85-4087-8e36-d4f904f42db6::"Compression ratio is the standard definition"::null::@Engineering%%
%%tag::Assumption::0b972f7d-2e8c-4f11-9a20-bf8ecde3c683::"The definition has low vulnerability"::null::@Engineering%%
%%tag::OpenQuestion::949fb81d-1ac6-4923-a461-1a5ee18ba21f::"Is compression ratio the wrong measure?"::null::@Engineering%%
%%tag::OpenQuestion::8232e05f-c3dc-47a3-84b6-5a5828f9ca35::"Is there a better complexity measure?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
