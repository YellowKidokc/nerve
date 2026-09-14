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
  "title": "Substrate Requirement",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0003_A2_1_Substrate_Requirement.md",
  "source_sha256": "eb69624140f46b26b13bb1295fb178063dbc9002ea1acfe4a3844f543bc41725",
  "captured_at": "2026-09-06T22:59:56.770856+00:00",
  "word_count": 324,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0003: A2.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 A2.1 \u2014 Substrate Requirement",
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

# Canonical Sequential Record 0003: A2.1

| Field | Value |
|---|---|
| Registry position | 3 of 191 |
| Canonical ID | `A2.1` |
| Mode | `AX_CORE` |
| Canonical source filename | `A2_1_Substrate_Requirement.md` |
| Source SHA-256 | `938432aa07743781b8307774099128b637f9abdaf2968cc089a02643c4aa9dc3` |

## Sequence Navigation

Previous: [0002 - A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0004 - A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Registered Dependency Links

[A1.3 - Information Primacy](0008_A1.3_A1_3_Information_Primacy.md)

## Canonical Source Text

---
id: "A2.1"
legacy_id: "AX-008"
title: "Substrate Requirement"
mode: "AX_CORE"
atlas_type: "Axiom"
status: "primitive"
domain: "Ontological"
anchor: "A2.1 (Substrate Requirement)"
depends_on: "A1.3"
date: "2026-08-17"
---

# 🟢 A2.1 — Substrate Requirement

**Mode:** `AX_CORE` | **Atlas Type:** `Axiom` | **Status:** `primitive`
**Domain:** Ontological | **Legacy ID:** `AX-008`

---

## Statement

Information requires a substrate — it cannot exist as free-floating abstraction.

## Formal Expression

```
∃S: Grounds(S, Information)
```

$$
∃S: Grounds(S, Information)
$$

## Maximum Defensible Position

The framework requires: Information requires a substrate.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_CORE` |
| **Atlas Object Type** | `Axiom` |
| **Workbook Role** | `AXIOM` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `primitive` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.1 (Substrate Requirement)
**Depends On:** A1.3
**Chain Tier:** 2 — Ontological Substrate

## References

Aristotle (hylomorphism); Floridi 2011

## Defeat Conditions

Show information existing without any substrate. Demonstrate pure Platonic information.

## Limitations (from workbook)

Mathematical Platonists (Tegmark) disagree. If math exists without substrate, A2.1 falls.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::cf4dcb55-1bab-40d0-8d48-f65e267f4c7e::"Information requires a substrate"::null::@Engineering,Product%%
%%tag::Dependency::6548ebb2-4ace-40a4-b569-0801ae27ff70::"Depends on A1.3 Information Primacy"::null::@Engineering%%
%%tag::Risk::c42eae87-6de2-4486-aba1-25b1ae86f9a3::"Mathematical Platonists (Tegmark) disagree with substrate requirement"::null::@Engineering,Product%%
%%tag::Assumption::2bdb2b27-eb25-4d89-9678-33797d9a7921::"Information cannot exist as free-floating abstraction"::null::@Engineering,Product%%
%%tag::OpenQuestion::e12dbf66-0460-4b33-a0b7-990cf1c4ef37::"Can information exist without any substrate?"::null::@Engineering,Product%%
%%tag::Stakeholder::adb473e9-f909-4a48-9f7a-28f562382a78::"Mathematical Platonists (Tegmark) who disagree with the axiom"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
