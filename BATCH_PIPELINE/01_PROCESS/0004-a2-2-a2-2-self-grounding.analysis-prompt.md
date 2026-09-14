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
  "title": "Self-Grounding",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0004_A2.2_A2_2_Self-Grounding.md",
  "source_sha256": "6ac28a93623326a83468b163bdc909e7bef3b6fa294103e2792360cf7efc5a03",
  "captured_at": "2026-09-06T23:01:47.729401+00:00",
  "word_count": 416,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0004: A2.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 A2.2 \u2014 Self-Grounding",
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

# Canonical Sequential Record 0004: A2.2

| Field | Value |
|---|---|
| Registry position | 4 of 191 |
| Canonical ID | `A2.2` |
| Mode | `AX_CORE` |
| Canonical source filename | `A2_2_Self-Grounding.md` |
| Source SHA-256 | `8e7a5e722f733eb183bd89c9ca593e84653e426e880487bab10a50a0b0103640` |

## Sequence Navigation

Previous: [0003 - A2.1 - Substrate Requirement](0003_A2.1_A2_1_Substrate_Requirement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0005 - A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Registered Dependency Links

[A2.1 - Substrate Requirement](0003_A2.1_A2_1_Substrate_Requirement.md)

## Canonical Source Text

---
id: "A2.2"
legacy_id: "AX-009"
title: "Self-Grounding"
mode: "AX_CORE"
atlas_type: "Axiom"
status: "stance"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.1"
date: "2026-08-17"
---

# 🟢 A2.2 — Self-Grounding

**Mode:** `AX_CORE` | **Atlas Type:** `Axiom` | **Status:** `stance`
**Domain:** Ontological | **Legacy ID:** `AX-009`

---

## Statement

The substrate must be self-grounding — it cannot depend on something outside itself for existence.

## Formal Expression

```
∃S: Grounds(S, S) — self-referential substrate
```

$$
∃S: Grounds(S, S) — self-referential substrate
$$

## Maximum Defensible Position

Framework stance: The substrate is self-grounding.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_CORE` |
| **Atlas Object Type** | `Axiom` |
| **Workbook Role** | `AXIOM` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `stance` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.1
**Chain Tier:** 2 — Ontological Substrate

## References

Aristotle (unmoved mover); Spinoza (causa sui); Aquinas (necessary being)

## Defeat Conditions

Show infinite regress of substrates is coherent. Demonstrate external grounding without circularity.

## Limitations (from workbook)

This is a stance, not a self-evident truth. Everything downstream inherits this commitment status.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::60f04141-adf9-4e94-9b79-6a095b128770::"The substrate must be self-grounding and cannot depend on something outside itself for existence"::null::@Engineering,Product%%
%%tag::Decision::f9a671ed-881a-4c6c-9b8f-ae734f1e4431::"Adopt the stance that the substrate is self-grounding"::null::@Product%%
%%tag::Dependency::9a678fac-2f09-4029-89b2-ea98024ce0a3::"Depends on A2.1 Substrate Requirement"::null::@Engineering%%
%%tag::Risk::e165435f-e5a7-4f2b-865f-877a87e9f508::"Risk that infinite regress of substrates is coherent, undermining self-grounding"::null::@Engineering,Product%%
%%tag::Risk::5585036e-c154-4c9e-b06e-427e0cc3656e::"Risk that external grounding without circularity can be demonstrated, invalidating the stance"::null::@Engineering,Product%%
%%tag::Assumption::31a3bcc4-fa66-4e9c-9a18-f35f8e1b45a2::"Assumes self-grounding is a coherent and necessary property for the substrate"::null::@Product%%
%%tag::Assumption::67f2c7d7-88ea-4db9-8b53-112f64e74a2b::"Assumes the stance is not self-evident and downstream commitments inherit this status"::null::@Product%%
%%tag::OpenQuestion::0fc26b49-abc8-4d5e-b06b-95b5b3c0619b::"Whether infinite regress of substrates is coherent"::null::@Engineering,Product%%
%%tag::OpenQuestion::6ed33e49-9694-4a1e-9ba5-d54f1801669a::"Whether external grounding without circularity can be demonstrated"::null::@Engineering,Product%%
%%tag::Stakeholder::72156aca-b2b7-41ba-b131-9f416f404ad3::"Aristotle (unmoved mover) - philosophical reference for self-grounding"::null::@Product%%
%%tag::Stakeholder::39fc33c4-9d81-491f-84d9-048dedc5414b::"Spinoza (causa sui) - philosophical reference for self-grounding"::null::@Product%%
%%tag::Stakeholder::e1d15e46-fe3e-4111-a496-3ecad2ec6f68::"Aquinas (necessary being) - philosophical reference for self-grounding"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
