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
  "title": "Law V: Conservation Symmetry",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0049_D19.5_D19_5_Law_V_Conservation_Symmetry.md",
  "source_sha256": "34010c5495935894d5a94b45640ff269e7f8ca6beb7fe63f42c180896c53f37e",
  "captured_at": "2026-09-06T23:47:27.268294+00:00",
  "word_count": 395,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0049: D19.5",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.5 \u2014 Law V: Conservation Symmetry",
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

# Canonical Sequential Record 0049: D19.5

| Field | Value |
|---|---|
| Registry position | 49 of 191 |
| Canonical ID | `D19.5` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_5_Law_V_Conservation_Symmetry.md` |
| Source SHA-256 | `4b7624e759a3d9c5951c0fcb3c1ca256be2e0cb26c76ffc2185c34f03fb6c497` |

## Sequence Navigation

Previous: [0048 - D19.4 - Law IV: Grace Dynamics](0048_D19.4_D19_4_Law_IV_Grace_Dynamics.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0050 - D19.6 - Law VI: Coherence Non-Increase](0050_D19.6_D19_6_Law_VI_Coherence_Non-Increase.md)

## Registered Dependency Links

[P3.2 - Coherence Conservation](0064_P3.2_P3_2_Coherence_Conservation.md)

## Canonical Source Text

---
id: "D19.5"
legacy_id: "AX-140"
title: "Law V: Conservation Symmetry"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "P3.2 (Coherence Conservation)"
depends_on: "depends on anchor: P3.2 (Coherence Conservation)"
date: "2026-08-17"
---

# 📐 D19.5 — Law V: Conservation Symmetry

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-140`

---

## Statement

Law V: Every continuous symmetry of χ generates a conserved quantity.

## Formal Expression

```
Symmetry(χ) → Conservation(Q); Noether applied to χ
```

$$
Symmetry(χ) → Conservation(Q); Noether applied to χ
$$

## Maximum Defensible Position

Definition: Law V — Conservation Symmetry.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Unified |

## Dependency Chain

**Anchor:** P3.2 (Coherence Conservation)
**Depends On:** depends on anchor: P3.2 (Coherence Conservation)
**Chain Tier:** 13 — Master Laws

## References

Noether 1918

## Defeat Conditions

Show Noether's theorem fails for χ-field.

## Limitations (from workbook)

Standard Noether — low vulnerability.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::ad1cc524-2d49-4f9a-96b1-9f9a1a896ca1::"Every continuous symmetry of χ must generate a conserved quantity"::null::@Engineering%%
%%tag::Requirement::0ae8a4b9-4677-4d03-ab6c-2c10e59f00fa::"Noether's theorem must be applicable to the χ-field"::null::@Engineering%%
%%tag::Decision::315d0dde-4597-4dee-a47f-33cce2d06875::"Adopt Noether's theorem as the basis for Law V"::null::@Engineering%%
%%tag::Decision::2b783a39-2dc1-4ef0-a602-f62a565ee7b7::"Define Law V as a formal definition in the Unified domain"::null::@Engineering%%
%%tag::Dependency::22926513-22f0-40db-9789-e64a79d0ed90::"Depends on P3.2 Coherence Conservation"::null::@Engineering%%
%%tag::Dependency::da2723dd-4c52-4f6c-aeef-0bfd7482cbc4::"Depends on Noether's 1918 theorem"::null::@Engineering%%
%%tag::Risk::28468441-c039-4a7b-862d-130099853607::"Risk of Noether's theorem failing for χ-field"::null::@Engineering%%
%%tag::Assumption::86dce00f-59c6-4a76-8bd8-33a722de176f::"Assumes the χ-field has continuous symmetries"::null::@Engineering%%
%%tag::Assumption::0b812088-0a45-4106-bb42-b51adc5a2fcc::"Assumes standard Noether theorem applies with low vulnerability"::null::@Engineering%%
%%tag::Metric::681082f1-6e67-493b-924c-f3580191316f::"Chain tier: 13 (Master Laws)"::null::@Engineering%%
%%tag::Stakeholder::863f6b99-4bd9-45b7-bc00-1784beff825f::"AX Truth Kernel classifications (source of definition)"::null::@Engineering%%
%%tag::OpenQuestion::36cb4c46-017c-4479-bede-757c3a0ec07f::"Unresolved: whether Noether's theorem can be shown to fail for χ-field"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
