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
  "title": "Observation Requirement",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0005_A5.1_A5_1_Observation_Requirement.md",
  "source_sha256": "a61fe45f6e7447a54b251ab8582edce891be52426182160239f139e6d0ebab84",
  "captured_at": "2026-09-06T23:02:40.059443+00:00",
  "word_count": 365,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0005: A5.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 A5.1 \u2014 Observation Requirement",
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

# Canonical Sequential Record 0005: A5.1

| Field | Value |
|---|---|
| Registry position | 5 of 191 |
| Canonical ID | `A5.1` |
| Mode | `AX_CORE` |
| Canonical source filename | `A5_1_Observation_Requirement.md` |
| Source SHA-256 | `7b2fa943673bb40cf2b9eb00f24487c37899e41b11ba136b534d17b5294d2c28` |

## Sequence Navigation

Previous: [0004 - A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0006 - BC4 - Three Observers Required](0006_BC4_BC4_Three_Observers_Required.md)

## Registered Dependency Links

_No registered canonical dependency link extracted._

## Canonical Source Text

---
id: "A5.1"
legacy_id: "AX-035"
title: "Observation Requirement"
mode: "AX_CORE"
atlas_type: "Axiom"
status: "stance"
domain: "Ontological"
anchor: "A5.1 (Observation Requirement) — ROOT"
depends_on: "∅ (core root)"
date: "2026-08-17"
---

# 🟢 A5.1 — Observation Requirement

**Mode:** `AX_CORE` | **Atlas Type:** `Axiom` | **Status:** `stance`
**Domain:** Ontological | **Legacy ID:** `AX-035`

---

## Statement

Nothing is actual without being observed. Observation is constitutive of actuality.

## Formal Expression

```
∀x: Actual(x) → ∃O: Observes(O, x)
```

$$
∀x: Actual(x) → ∃O: Observes(O, x)
$$

## Maximum Defensible Position

Framework stance: Observation is constitutive of actuality.

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

**Anchor:** A5.1 (Observation Requirement) — ROOT
**Depends On:** ∅ (core root)
**Chain Tier:** 6 — Observer System

## References

Wheeler 1983 (participatory universe); Copenhagen interpretation; Von Neumann 1932

## Defeat Conditions

Show actuality without observation. Demonstrate objective reality independent of all observers.

## Limitations (from workbook)

This is a stance. MWI and Bohmian mechanics deny it. Framework-dependent.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::a253d55d-c209-4bce-9888-6f3d85e49ddd::"Nothing is actual without being observed"::null::@Engineering%%
%%tag::Requirement::1230fa33-cfbc-4226-9077-ddcb7148ba0e::"Observation is constitutive of actuality"::null::@Engineering%%
%%tag::Decision::eaabbae9-e085-476e-836c-3a826bbba557::"Adopt stance that observation is constitutive of actuality"::null::@Engineering%%
%%tag::Risk::96cce9ac-dbbf-45a2-a9db-4da72f89af77::"MWI and Bohmian mechanics deny the stance"::null::@Engineering%%
%%tag::Assumption::c85b5e20-5822-4126-8465-1079fc91aea3::"Assumes observation is required for actuality"::null::@Engineering%%
%%tag::Dependency::826646d7-f2b3-4482-a292-6b9ef6e494a9::"Depends on core root (no external dependencies)"::null::@Engineering%%
%%tag::OpenQuestion::4d487f54-913c-4905-a31b-8cd72b2f171c::"Can actuality exist without observation?"::null::@Engineering%%
%%tag::OpenQuestion::7b9b96f5-23f9-42f8-a811-8fe8bab98f99::"Can objective reality be demonstrated independent of all observers?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
