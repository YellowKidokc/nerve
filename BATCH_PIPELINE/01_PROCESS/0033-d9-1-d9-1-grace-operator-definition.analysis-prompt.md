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
  "title": "Grace Operator Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0033_D9.1_D9_1_Grace_Operator_Definition.md",
  "source_sha256": "870ac0531834b8358a0e6dd6a8883f6ef138a48216c5f7aa3e483e98e07a5d4d",
  "captured_at": "2026-09-06T23:31:26.983489+00:00",
  "word_count": 407,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0033: D9.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D9.1 \u2014 Grace Operator Definition",
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

# Canonical Sequential Record 0033: D9.1

| Field | Value |
|---|---|
| Registry position | 33 of 191 |
| Canonical ID | `D9.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D9_1_Grace_Operator_Definition.md` |
| Source SHA-256 | `3af5842f5b05a332cd842f7d8ed9d69a0ecd89f5997dad621152f43315bc6d73` |

## Sequence Navigation

Previous: [0032 - D8.1 - Sign Operator](0032_D8.1_D8_1_Sign_Operator.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0034 - E9.1 - Grace Function G(t)](0034_E9.1_E9_1_Grace_Function_Gt.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D9.1"
legacy_id: "AX-075"
title: "Grace Operator Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Moral"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D9.1 — Grace Operator Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Moral | **Legacy ID:** `AX-075`

---

## Statement

Grace operator defined — non-unitary operator that flips sign state.

## Formal Expression

```
Ĝ: σ(-1) → σ(+1); Ĝ†Ĝ ≠ I (non-unitary)
```

$$
Ĝ: σ(-1) → σ(+1); Ĝ†Ĝ \ne I (non-unitary)
$$

## Maximum Defensible Position

Definition: Ĝ = grace operator (non-unitary sign flip).

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Moral |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 10 — Grace Dynamics

## References

Open quantum systems; Lindblad equation; Sola Gratia

## Defeat Conditions

Show sign flip without external operator. Demonstrate unitary grace.

## Limitations (from workbook)

The non-unitarity is the key physics claim. Unitary operators can't flip eigenvalues.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::e70a6b61-e4f9-48df-a4e3-71f26771908f::"Grace operator must be non-unitary"::null::@Engineering%%
%%tag::Requirement::7000a29c-1d26-4ede-a1d8-fd60a4f9098c::"Grace operator must flip sign state from -1 to +1"::null::@Engineering%%
%%tag::Requirement::6acc57a1-e8bc-46a3-890c-57faea748c08::"Grace operator must satisfy Ĝ†Ĝ ≠ I"::null::@Engineering%%
%%tag::Decision::9434db32-2385-4d10-8450-c69d670fb279::"Defined grace operator as non-unitary sign flip"::null::@Engineering%%
%%tag::Decision::bab5e0c2-db8c-49fb-8f0f-833f14a56c25::"Chose non-unitary formulation over unitary alternative"::null::@Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 Infinite Energy Source"::null::@Engineering%%
%%tag::Risk::a63babd4-a249-4449-9064-d59757c867a3::"Risk that unitary grace could be demonstrated, defeating the definition"::null::@Engineering%%
%%tag::Risk::ea566424-cb3f-4cd9-8c98-2a93f07b5def::"Risk that sign flip could be shown without external operator"::null::@Engineering%%
%%tag::Assumption::41b54b95-b6e4-4ce4-9d61-8457c4b4ea82::"Assumes non-unitary operators cannot flip eigenvalues"::null::@Engineering%%
%%tag::Assumption::5b2c4bfa-667e-4a12-82e9-32734bd14adc::"Assumes the formal expression is a valid primitive formalization"::null::@Engineering%%
%%tag::OpenQuestion::c9edaa9a-ccc1-43b6-a325-82b40468357d::"Whether unitary grace is possible"::null::@Engineering%%
%%tag::Stakeholder::50ec9f9e-3509-4294-8faf-52a3f858ccf0::"Atlas system users interested in moral domain definitions"::null::@Product%%
%%tag::Metric::343f325c-4146-4dfb-be31-737ae9818a7e::"Chain tier value: 10"::null::@Engineering%%
%%tag::Metric::89a0403a-6722-4b2a-9f64-2ca4eb602fbd::"Registry position: 33 of 191"::null::@Operations%%
%%--- END SEMANTIC TAGS ---%%
