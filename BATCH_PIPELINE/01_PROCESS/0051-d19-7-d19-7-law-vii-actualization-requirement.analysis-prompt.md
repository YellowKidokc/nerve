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
  "title": "Law VII: Actualization Requirement",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0051_D19.7_D19_7_Law_VII_Actualization_Requirement.md",
  "source_sha256": "200ea1cf7fd91a03594c690b110d8d42cf9af773bdd81eda3e2ac0dec60062ef",
  "captured_at": "2026-09-06T23:49:19.632731+00:00",
  "word_count": 354,
  "tags": [
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0051: D19.7",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.7 \u2014 Law VII: Actualization Requirement",
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

# Canonical Sequential Record 0051: D19.7

| Field | Value |
|---|---|
| Registry position | 51 of 191 |
| Canonical ID | `D19.7` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_7_Law_VII_Actualization_Requirement.md` |
| Source SHA-256 | `d578519e1def25038b1cb69a68437cde2b658b610613b4ad3163eeb84a9d8ec4` |

## Sequence Navigation

Previous: [0050 - D19.6 - Law VI: Coherence Non-Increase](0050_D19.6_D19_6_Law_VI_Coherence_Non-Increase.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0052 - D19.8 - Law VIII: Sign Algebra](0052_D19.8_D19_8_Law_VIII_Sign_Algebra.md)

## Registered Dependency Links

[LN5.1 - Chi Requires Observer For Actualization](0074_LN5.1_LN5_1_Chi_Requires_Observer_For_Actualization.md)

## Canonical Source Text

---
id: "D19.7"
legacy_id: "AX-142"
title: "Law VII: Actualization Requirement"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "LN5.1 (Chi Requires Observer)"
depends_on: "depends on anchor: LN5.1 (Chi Requires Observer)"
date: "2026-08-17"
---

# 📐 D19.7 — Law VII: Actualization Requirement

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-142`

---

## Statement

Law VII: Potentiality cannot self-actualize.

## Formal Expression

```
∀x: Actual(x) → ∃y: Actualizes(y,x) ∧ y ≠ x
```

$$
∀x: Actual(x) → ∃y: Actualizes(y,x) ∧ y \ne x
$$

## Maximum Defensible Position

Definition: Law VII — Actualization Requirement.

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

**Anchor:** LN5.1 (Chi Requires Observer)
**Depends On:** depends on anchor: LN5.1 (Chi Requires Observer)
**Chain Tier:** 13 — Master Laws

## References

Aristotle; Aquinas; Von Neumann 1932

## Defeat Conditions

Show potentiality self-actualizes.

## Limitations (from workbook)

Quantum vacuum fluctuations appear spontaneous. Response: they require the quantum field.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::b687b19d-3ccf-49ca-a460-3b2defc68615::"Potentiality cannot self-actualize"::null::@Engineering%%
%%tag::Requirement::b3c52c59-aa63-447e-be15-98f1451ccf8f::"Actual(x) implies existence of a distinct actualizer y"::null::@Engineering%%
%%tag::Dependency::a4f839c0-ce18-4000-b1ff-279c2c74aada::"Depends on anchor LN5.1 (Chi Requires Observer)"::null::@Engineering%%
%%tag::Risk::fe7c292f-fc19-417b-98d8-dac9761957c3::"Quantum vacuum fluctuations appear spontaneous, threatening the law"::null::@Engineering%%
%%tag::Assumption::a5e2810c-b015-43c5-bc3e-a6379bdf81c2::"Quantum vacuum fluctuations require the quantum field"::null::@Engineering%%
%%tag::OpenQuestion::34ee26b5-62e3-4f33-bb95-6c1340517dde::"How to show potentiality self-actualizes (defeat condition)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
