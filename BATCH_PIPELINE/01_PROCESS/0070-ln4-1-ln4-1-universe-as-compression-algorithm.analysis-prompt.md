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
  "title": "Universe As Compression Algorithm",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0070_LN4.1_LN4_1_Universe_As_Compression_Algorithm.md",
  "source_sha256": "e16e8c0b8605b1cfc38a529422b2712e6d350e927434639d51e6e7c82cc0f664",
  "captured_at": "2026-09-07T00:03:12.047483+00:00",
  "word_count": 362,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0070: LN4.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN4.1 \u2014 Universe As Compression Algorithm",
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

# Canonical Sequential Record 0070: LN4.1

| Field | Value |
|---|---|
| Registry position | 70 of 191 |
| Canonical ID | `LN4.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN4_1_Universe_As_Compression_Algorithm.md` |
| Source SHA-256 | `77c407f7ed5d5620d61fac561a39fd8e9a1a8a6e077aa6938c2d4cf4b3b0533d` |

## Sequence Navigation

Previous: [0069 - T4.2 - Action Principle As Minimal-K](0069_T4.2_T4_2_Action_Principle_As_Minimal-K.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0071 - A5.2 - Participatory Universe](0071_A5.2_A5_2_Participatory_Universe.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [T4.1 - Laws Are Low-K Descriptions](0068_T4.1_T4_1_Laws_Are_Low-K_Descriptions.md); [T4.2 - Action Principle As Minimal-K](0069_T4.2_T4_2_Action_Principle_As_Minimal-K.md)

## Canonical Source Text

---
id: "LN4.1"
legacy_id: "AX-034"
title: "Universe As Compression Algorithm"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2, T4.1, T4.2"
date: "2026-08-17"
---

# 🔶 LN4.1 — Universe As Compression Algorithm

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-034`

---

## Statement

The universe is a compression algorithm — it computes the simplest description of itself.

## Formal Expression

```
Universe = min_K(self-description)
```

$$
Universe = min_K(self-description)
$$

## Maximum Defensible Position

Framework claim: The universe is a compression algorithm — it computes the simplest description of itself.

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
**Depends On:** A2.2, T4.1, T4.2

## References

Digital physics (Zuse, Wolfram); It from Bit

## Defeat Conditions

Show universe isn't compressing.

## Limitations (from workbook)

Very strong claim. Testable in principle via cosmic complexity measures.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::f5cf1678-828d-45ff-9e77-a1f6055e0b15::"Universe must compute the simplest description of itself"::null::@Engineering%%
%%tag::Decision::76fbf301-3920-426b-ac3d-229fc2781249::"Adopt the universe as a compression algorithm framework"::null::@Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::1fd3cacf-909f-43b9-a953-9a9de0f8de37::"Depends on T4.1 Laws Are Low-K Descriptions"::null::@Engineering%%
%%tag::Dependency::ad321d41-296d-47a5-9ac3-444933114ddb::"Depends on T4.2 Action Principle As Minimal-K"::null::@Engineering%%
%%tag::Risk::8d017603-8340-4d9f-9282-8647997c04c9::"Very strong claim may be untestable"::null::@Engineering%%
%%tag::Metric::bcd62d7c-31a8-423d-8042-a4ad54d747a2::"Cosmic complexity measures for testability"::null::@Engineering%%
%%tag::OpenQuestion::6bef2803-7870-4a39-b6a2-ba1aadd72cc5::"How to demonstrate the universe is not compressing?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
