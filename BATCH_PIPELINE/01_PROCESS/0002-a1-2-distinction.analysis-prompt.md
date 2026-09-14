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
  "title": "Distinction",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0002_A1_2_Distinction.md",
  "source_sha256": "330e8d12b694889c3d96c91c3a716a9af1d61f04e1316f7db0078b463aee39ea",
  "captured_at": "2026-09-06T22:59:02.548956+00:00",
  "word_count": 372,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0002: A1.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 A1.2 \u2014 Distinction",
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

# Canonical Sequential Record 0002: A1.2

| Field | Value |
|---|---|
| Registry position | 2 of 191 |
| Canonical ID | `A1.2` |
| Mode | `AX_CORE` |
| Canonical source filename | `A1_2_Distinction.md` |
| Source SHA-256 | `dc8b2f94ee3fdbe4e9d5280cfd3b693610efc77faa2e9c400f415862a0255777` |

## Sequence Navigation

Previous: [0001 - A1.1 - Existence](0001_A1.1_A1_1_Existence.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0003 - A2.1 - Substrate Requirement](0003_A2.1_A2_1_Substrate_Requirement.md)

## Registered Dependency Links

[A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "A1.2"
legacy_id: "AX-002"
title: "Distinction"
mode: "AX_CORE"
atlas_type: "Axiom"
status: "primitive"
domain: "Primordial"
anchor: "A1.2 (Distinction) — ROOT"
depends_on: "A1.1"
date: "2026-08-17"
---

# 🟢 A1.2 — Distinction

**Mode:** `AX_CORE` | **Atlas Type:** `Axiom` | **Status:** `primitive`
**Domain:** Primordial | **Legacy ID:** `AX-002`

---

## Statement

For anything to be describable or knowable, it must be distinguishable from something else.

## Formal Expression

```
∀x∀y(x≠y → Distinguishable(x,y))
```

$$
∀x∀y(x\ney → Distinguishable(x,y))
$$

## Maximum Defensible Position

The framework presently assumes or requires: For anything to be describable or knowable, it must be distinguishable from something else.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_CORE` |
| **Atlas Object Type** | `Axiom` |
| **Workbook Role** | `AXIOM` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `primitive` |
| **Domain** | Primordial |

## Dependency Chain

**Anchor:** A1.2 (Distinction) — ROOT
**Depends On:** A1.1
**Chain Tier:** 0 — Primordial Foundation

## References

Spencer-Brown 1969 (Laws of Form); Bateson 1972

## Defeat Conditions

Show a property of reality that requires no distinction. Show existence without distinguishability is coherent.

## Limitations (from workbook)

Advaita Vedanta's non-dual challenge is genuine. Needs steelman of Śaṅkara's position.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::3990d2ba-d232-45fd-9c84-0c1a2b66be18::"Anything describable or knowable must be distinguishable from something else"::null::@Product%%
%%tag::Dependency::a7cd912c-e842-496c-a857-6197e0964246::"Depends on A1.1 Existence"::null::@Product%%
%%tag::Risk::75b08d2a-ca1e-4c85-b8f7-07cf3a08f8da::"Advaita Vedanta's non-dual challenge could invalidate the axiom"::null::@Product%%
%%tag::OpenQuestion::35359a6d-d22e-4f52-b633-e2466236238d::"How to steelman Śaṅkara's non-dual position against the axiom"::null::@Product%%
%%tag::Assumption::f73917fc-39c8-42b0-ace0-03929f49aa06::"The framework assumes distinguishability is required for describability or knowability"::null::@Product%%
%%tag::Requirement::99fa2d00-f78e-4da0-8b65-e8865d5c0886::"Defeat condition: Show a property of reality that requires no distinction"::null::@Product%%
%%tag::Requirement::394fa504-ecdb-4022-ade8-935f37a69abe::"Defeat condition: Show existence without distinguishability is coherent"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
