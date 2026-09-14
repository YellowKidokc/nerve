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
  "title": "Full Master Equation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0055_E19.1_E19_1_Full_Master_Equation.md",
  "source_sha256": "282139bf1ddf2822d558098ef2abcab5d8edf4f2e2683d0ab48b16ae4da22645",
  "captured_at": "2026-09-06T23:52:47.550745+00:00",
  "word_count": 507,
  "tags": [
    "information-theory",
    "theology"
  ],
  "source_outline": [
    "Canonical Sequential Record 0055: E19.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E19.1 \u2014 Full Master Equation",
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

# Canonical Sequential Record 0055: E19.1

| Field | Value |
|---|---|
| Registry position | 55 of 191 |
| Canonical ID | `E19.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E19_1_Full_Master_Equation.md` |
| Source SHA-256 | `3f219a18c6792d43d83909b48255816c260916e96d22acd03d908a7222928e2a` |

## Sequence Navigation

Previous: [0054 - D19.10 - Law X: Trinity Closure](0054_D19.10_D19_10_Law_X_Trinity_Closure.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0056 - LN1.1 - Matter-Energy Derivative](0056_LN1.1_LN1_1_Matter-Energy_Derivative.md)

## Registered Dependency Links

[D19.10 - Law X: Trinity Closure](0054_D19.10_D19_10_Law_X_Trinity_Closure.md); [D19.1 - Law I: Logos-Lagrangian](0045_D19.1_D19_1_Law_I_Logos-Lagrangian.md); [D19.9 - Law IX: Grace Non-Unitarity](0053_D19.9_D19_9_Law_IX_Grace_Non-Unitarity.md); [D19.8 - Law VIII: Sign Algebra](0052_D19.8_D19_8_Law_VIII_Sign_Algebra.md); [D19.6 - Law VI: Coherence Non-Increase](0050_D19.6_D19_6_Law_VI_Coherence_Non-Increase.md); [D19.3 - Law III: Entropy-Sin](0047_D19.3_D19_3_Law_III_Entropy-Sin.md); [D19.4 - Law IV: Grace Dynamics](0048_D19.4_D19_4_Law_IV_Grace_Dynamics.md); [D19.7 - Law VII: Actualization Requirement](0051_D19.7_D19_7_Law_VII_Actualization_Requirement.md); [D19.5 - Law V: Conservation Symmetry](0049_D19.5_D19_5_Law_V_Conservation_Symmetry.md); [D19.2 - Law II: Ten Variables](0046_D19.2_D19_2_Law_II_Ten_Variables.md)

## Canonical Source Text

---
id: "E19.1"
legacy_id: "AX-146"
title: "Full Master Equation"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Unified"
anchor: "D19.1–D19.10"
depends_on: "D19.1, D19.2, D19.3, D19.4, D19.5, D19.6, D19.7, D19.8, D19.9, D19.10"
date: "2026-08-17"
---

# 📐 E19.1 — Full Master Equation

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Unified | **Legacy ID:** `AX-146`

---

## Statement

The Theophysics Master Equation — all ten laws in one expression.

## Formal Expression

```
LLC = χ(t)(d/dt(Σ primitives))² - S·χ(t)
```

$$
LLC = χ(t)(d/dt(Σ primitives))² - S·χ(t)
$$

## Maximum Defensible Position

Model: Master equation integrating all ten laws.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Unified |

## Dependency Chain

**Anchor:** D19.1–D19.10
**Depends On:** D19.1, D19.2, D19.3, D19.4, D19.5, D19.6, D19.7, D19.8, D19.9, D19.10
**Chain Tier:** 13 — Master Laws

## References

All embedded theories

## Defeat Conditions

Show the master equation is internally inconsistent.

## Limitations (from workbook)

Schematic. Each primitive needs precise definition. The square of the time derivative needs justification.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6d7126ab-ff27-4410-a742-0f9f20cee225::"Master equation must integrate all ten laws"::null::@Engineering%%
%%tag::Requirement::fc87fe2c-7b3d-43ac-8d33-34f682a3633e::"Each primitive in the master equation needs precise definition"::null::@Engineering%%
%%tag::Requirement::238b89fa-823c-4440-b430-0a974f8427d4::"The square of the time derivative in the master equation needs justification"::null::@Engineering%%
%%tag::Dependency::2fd3a466-93e3-414f-9bd0-4750a3bb2509::"Depends on Law I: Logos-Lagrangian (D19.1)"::null::@Engineering%%
%%tag::Dependency::010a2e01-56ac-40da-b2d4-45ea91f11ac4::"Depends on Law II: Ten Variables (D19.2)"::null::@Engineering%%
%%tag::Dependency::6da4df3b-66bf-4ff0-aa2a-fb6f64a59016::"Depends on Law III: Entropy-Sin (D19.3)"::null::@Engineering%%
%%tag::Dependency::86d1fe23-2e18-4e68-ae4a-1bde2de00ab1::"Depends on Law IV: Grace Dynamics (D19.4)"::null::@Engineering%%
%%tag::Dependency::2873e682-cde7-4bad-96cc-719399e908eb::"Depends on Law V: Conservation Symmetry (D19.5)"::null::@Engineering%%
%%tag::Dependency::7e26d689-1f48-47a3-8508-1e302d11a985::"Depends on Law VI: Coherence Non-Increase (D19.6)"::null::@Engineering%%
%%tag::Dependency::223af8e2-8a46-4e29-b5b2-9acfb5625552::"Depends on Law VII: Actualization Requirement (D19.7)"::null::@Engineering%%
%%tag::Dependency::b1aac020-ff51-4df3-a679-6b9bc4a4a0c0::"Depends on Law VIII: Sign Algebra (D19.8)"::null::@Engineering%%
%%tag::Dependency::d877996c-d019-45f5-9bed-4dd45a38a27b::"Depends on Law IX: Grace Non-Unitarity (D19.9)"::null::@Engineering%%
%%tag::Dependency::f2dc8c5f-a82b-49c3-af67-e7c3d62b4b3c::"Depends on Law X: Trinity Closure (D19.10)"::null::@Engineering%%
%%tag::Risk::372da212-8430-4432-8056-8d44ab76c571::"Master equation may be internally inconsistent"::null::@Engineering%%
%%tag::OpenQuestion::db46c7c6-64d0-4592-9fd7-5bca186263ef::"Precise definitions of each primitive in the master equation"::null::@Engineering%%
%%tag::OpenQuestion::e62b3dd9-1b86-4b00-9c4a-9c9761019c41::"Justification for the square of the time derivative"::null::@Engineering%%
%%tag::Decision::72c2da88-88ed-428b-b090-081f155bc765::"Adopt the master equation LLC = χ(t)(d/dt(Σ primitives))² - S·χ(t) as the unified expression of all ten laws"::null::@Engineering%%
%%tag::Metric::681082f1-6e67-493b-924c-f3580191316f::"Chain tier: 13 — Master Laws"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
