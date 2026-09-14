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
  "title": "Participatory Universe",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0071_A5.2_A5_2_Participatory_Universe.md",
  "source_sha256": "a627b03bdf8f6acfaa7cb32d02a7dbb4aab15a2e3dddfb0bbce5fab91b3b8427",
  "captured_at": "2026-09-07T00:04:07.737708+00:00",
  "word_count": 344,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0071: A5.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A5.2 \u2014 Participatory Universe",
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

# Canonical Sequential Record 0071: A5.2

| Field | Value |
|---|---|
| Registry position | 71 of 191 |
| Canonical ID | `A5.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A5_2_Participatory_Universe.md` |
| Source SHA-256 | `789bbfab909b6c5edbdd36f3c2103bc4d7a6be41c87aebebd770aebd0308850f` |

## Sequence Navigation

Previous: [0070 - LN4.1 - Universe As Compression Algorithm](0070_LN4.1_LN4_1_Universe_As_Compression_Algorithm.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0072 - P5.1 - Phi Admits Degrees](0072_P5.1_P5_1_Phi_Admits_Degrees.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "A5.2"
legacy_id: "AX-036"
title: "Participatory Universe"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1"
date: "2026-08-17"
---

# 🔶 A5.2 — Participatory Universe

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-036`

---

## Statement

The universe is participatory — observers co-create reality through observation.

## Formal Expression

```
Reality = f(χ, Observer)
```

$$
Reality = f(χ, Observer)
$$

## Maximum Defensible Position

Framework claim: The universe is participatory — observers co-create reality through observation.

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

**Anchor:** A5.1
**Depends On:** A5.1

## References

Wheeler 1983; Copenhagen; QBism

## Defeat Conditions

Show reality independent of all observers.

## Limitations (from workbook)

Framework stance. MWI denies it. Bohmian mechanics denies it.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::3e09d324-e06f-497e-906f-315c8daa59ef::"The universe must be participatory, with observers co-creating reality through observation"::null::@Product%%
%%tag::Decision::d9cf28bd-ef5d-4be2-bcd8-7edf10ed6217::"Adopt the participatory universe claim as a framework commitment"::null::@Product%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::57b010af-a16c-4df6-83db-87c8405c4ed7::"Defeat condition: showing reality independent of all observers would invalidate the claim"::null::@Product%%
%%tag::Assumption::566ab500-4740-4b6b-bc3e-5ea046812fd6::"Assumes observers exist and observation affects reality"::null::@Product%%
%%tag::OpenQuestion::207da4cd-2b94-440f-9cd8-3a64b3e97a0b::"How to reconcile with MWI and Bohmian mechanics which deny this claim"::null::@Engineering,Product%%
%%tag::Stakeholder::4fdcbf0f-4981-4142-b76d-ea0b33003a3f::"Wheeler, Copenhagen and QBism proponents as referenced sources"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
