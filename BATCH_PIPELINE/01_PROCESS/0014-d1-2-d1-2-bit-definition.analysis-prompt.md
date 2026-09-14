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
  "title": "Bit Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0014_D1.2_D1_2_Bit_Definition.md",
  "source_sha256": "c1c3be4ebf13fcda2afec364a6bed5ccd2e41dd4dd4269d5bf6d3e81a0759cd3",
  "captured_at": "2026-09-06T23:12:39.759567+00:00",
  "word_count": 409,
  "tags": [
    "information-theory",
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0014: D1.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D1.2 \u2014 Bit Definition",
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

# Canonical Sequential Record 0014: D1.2

| Field | Value |
|---|---|
| Registry position | 14 of 191 |
| Canonical ID | `D1.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D1_2_Bit_Definition.md` |
| Source SHA-256 | `1c12ab823748b0e9fdfe069d59eb304fc2899fa677eb8f1ff285db1753bec13b` |

## Sequence Navigation

Previous: [0013 - D1.1 - Information Definition](0013_D1.1_D1_1_Information_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0015 - D2.1 - Logos Field Definition](0015_D2.1_D2_1_Logos_Field_Definition.md)

## Registered Dependency Links

[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "D1.2"
legacy_id: "AX-005"
title: "Bit Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Primordial"
anchor: "A1.1, A1.2"
depends_on: "A1.1, A1.2"
date: "2026-08-17"
---

# 📐 D1.2 — Bit Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Primordial | **Legacy ID:** `AX-005`

---

## Statement

A bit is the minimal unit of information — one binary distinction.

## Formal Expression

```
bit = log₂(2) = 1; minimal unit of distinction
```

$$
bit = log₂(2) = 1; minimal unit of distinction
$$

## Maximum Defensible Position

Definition: Bit = minimal information unit.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Primordial |

## Dependency Chain

**Anchor:** A1.1, A1.2
**Depends On:** A1.1, A1.2
**Chain Tier:** 1 — Primordial Foundation

## References

Shannon 1948; Landauer 1961

## Defeat Conditions

Show a smaller unit of information exists. Demonstrate continuous information without discretization.

## Limitations (from workbook)

Quantum information uses qubits (continuous superposition). Framework may need to address qubit vs bit.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::79d758aa-e3aa-45ec-a3ce-ef09dd946bd1::"Bit must be defined as the minimal unit of information"::null::@Engineering%%
%%tag::Requirement::87db4b6c-f6d7-48f8-94ea-33e8cd130684::"Bit must represent one binary distinction"::null::@Engineering%%
%%tag::Decision::6270295a-9c25-4779-a618-6c4dff98d516::"Adopt formal definition of bit as log₂(2) = 1"::null::@Engineering%%
%%tag::Decision::98bdef6a-50a2-435f-b1a8-029f1c2f1c7d::"Classify D1.2 as a definition in Primordial domain"::null::@Product%%
%%tag::Dependency::a7cd912c-e842-496c-a857-6197e0964246::"Depends on A1.1 Existence"::null::@Engineering%%
%%tag::Dependency::308a8972-6f52-4e15-bee3-3c4af5fd72a2::"Depends on A1.2 Distinction"::null::@Engineering%%
%%tag::Risk::9e2cb72e-2800-4499-a1b8-5b67a9be6542::"Quantum information may require qubit vs bit distinction"::null::@Engineering,Product%%
%%tag::Risk::ce1dfa1c-533a-43cb-b4bb-e66759a91aea::"Defeat condition: smaller unit of information could be shown"::null::@Engineering%%
%%tag::Risk::f9e666e0-83bd-4933-ab24-338ebd041b54::"Defeat condition: continuous information without discretization"::null::@Engineering%%
%%tag::Assumption::4febb1c1-959d-4c2c-bc95-fe8a2942e5d7::"Information is fundamentally discrete and binary"::null::@Engineering%%
%%tag::Assumption::64e5b6a3-612c-40b2-80b5-468c8d57bb43::"Shannon and Landauer definitions are authoritative"::null::@Engineering%%
%%tag::Metric::2ff68a01-24ae-4205-aae5-dcd97fd3cdda::"Bit value = 1 (log₂(2))"::null::@Engineering%%
%%tag::OpenQuestion::3799a83f-5589-454b-a1a9-4cabf6cd35a4::"How to address qubit vs bit in the framework"::null::@Engineering,Product%%
%%tag::Stakeholder::e9643841-544d-4a8d-b4b1-87a192ed2177::"Framework maintainers (AXIOM_CHAIN_MASTER_v2)"::null::@Product%%
%%tag::Stakeholder::ee9d098b-0acd-4b25-84b8-76fe4e93d2a5::"Users of the definition (engineers, researchers)"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
