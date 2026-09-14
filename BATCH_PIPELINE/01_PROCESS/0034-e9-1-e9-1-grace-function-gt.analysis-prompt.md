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
  "title": "Grace Function G(t)",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0034_E9.1_E9_1_Grace_Function_Gt.md",
  "source_sha256": "0067cc92945ef638334cd62b2e66ba76958e3f5f68ded86d5e2da895c91c0146",
  "captured_at": "2026-09-06T23:34:24.576263+00:00",
  "word_count": 459,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0034: E9.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E9.1 \u2014 Grace Function G(t)",
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

# Canonical Sequential Record 0034: E9.1

| Field | Value |
|---|---|
| Registry position | 34 of 191 |
| Canonical ID | `E9.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E9_1_Grace_Function_Gt.md` |
| Source SHA-256 | `8a1747ed524c6406e98d89d079744de033c7a2e607ad113214acc56bf84ca44e` |

## Sequence Navigation

Previous: [0033 - D9.1 - Grace Operator Definition](0033_D9.1_D9_1_Grace_Operator_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0035 - D10.1 - Soul Field Psi_S](0035_D10.1_D10_1_Soul_Field_Psi_S.md)

## Registered Dependency Links

[D9.1 - Grace Operator Definition](0033_D9.1_D9_1_Grace_Operator_Definition.md); [BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "E9.1"
legacy_id: "AX-081"
title: "Grace Function G(t)"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Moral"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6, D9.1"
date: "2026-08-17"
---

# 📐 E9.1 — Grace Function G(t)

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Moral | **Legacy ID:** `AX-081`

---

## Statement

Grace function G(t) — time-dependent grace with growth rate and resistance.

## Formal Expression

```
G(t) = G₀ · e^(∫r(t')dt') · (1-R(t))
```

$$
G(t) = G₀ · e^(∫r(t')dt') · (1-R(t))
$$

## Maximum Defensible Position

Model: G(t) = grace dynamics equation.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Moral |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6, D9.1
**Chain Tier:** 10 — Grace Dynamics

## References

Growth models; Logistic equation; Sola Gratia

## Defeat Conditions

Show grace is constant (not time-dependent). Demonstrate resistance term is wrong.

## Limitations (from workbook)

Functional form is proposed, not derived. The resistance term R(t) is a free function.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::d2241c24-2ad1-4c37-a777-fd3722ca9595::"Grace function G(t) must be time-dependent with growth rate and resistance"::null::@Engineering%%
%%tag::Requirement::be7f9625-164e-477f-9c02-00784607cff9::"Grace function G(t) must follow the formal expression G(t) = G₀ · e^(∫r(t')dt') · (1-R(t))"::null::@Engineering%%
%%tag::Requirement::322badae-8dd5-490e-8146-8360d434941d::"Grace function must be anchored to BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Requirement::d33a55a5-a5f0-4f11-bd6e-569adc50079d::"Grace function must depend on D9.1 (Grace Operator Definition)"::null::@Engineering%%
%%tag::Decision::d6113fab-d520-448f-ab41-1a91cc92ab9b::"Adopt G(t) = G₀ · e^(∫r(t')dt') · (1-R(t)) as the grace dynamics equation"::null::@Engineering%%
%%tag::Decision::615f4e93-7c33-4d45-9fcc-82b978ec4a7e::"Classify E9.1 as an equation with epistemic status 'equation'"::null::@Engineering%%
%%tag::Decision::4cd036cc-d84f-45c3-88c5-17853d3662ca::"Assign E9.1 to the Moral domain"::null::@Product%%
%%tag::Decision::ade60d9d-b305-400b-8521-80dd21a277fd::"Place E9.1 in Chain Tier 10 - Grace Dynamics"::null::@Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Dependency::e7b4af08-7f45-4bd3-8999-dc15347fa933::"Depends on D9.1 (Grace Operator Definition)"::null::@Engineering%%
%%tag::Risk::caab08b7-4e5a-4e44-8d98-3b16ab22b8a4::"Risk that grace is constant (not time-dependent), invalidating the model"::null::@Engineering%%
%%tag::Risk::a372c6f9-364e-4cbb-a3e0-a794e94ad7b2::"Risk that the resistance term R(t) is wrong"::null::@Engineering%%
%%tag::Assumption::98a25ecb-d3b3-4bb3-b1d9-22bf2e92a8a8::"Assumption that the functional form is proposed, not derived"::null::@Engineering%%
%%tag::Assumption::4c202c63-3810-481f-b9bc-3305dc83e8a7::"Assumption that the resistance term R(t) is a free function"::null::@Engineering%%
%%tag::OpenQuestion::3ebc7016-16b6-4cee-b0d6-9c7e5d3b511b::"Whether grace is constant or time-dependent"::null::@Engineering%%
%%tag::OpenQuestion::304e9420-da31-4dc0-8817-085bba8c5876::"Whether the resistance term R(t) is correctly specified"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
