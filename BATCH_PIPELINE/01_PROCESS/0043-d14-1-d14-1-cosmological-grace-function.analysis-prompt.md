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
  "title": "Cosmological Grace Function",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0043_D14.1_D14_1_Cosmological_Grace_Function.md",
  "source_sha256": "af47b79a2d3baa31f65b098dd697897512f627322bf89849ede330e8042adadd",
  "captured_at": "2026-09-06T23:42:30.576954+00:00",
  "word_count": 358,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0043: D14.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D14.1 \u2014 Cosmological Grace Function",
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

# Canonical Sequential Record 0043: D14.1

| Field | Value |
|---|---|
| Registry position | 43 of 191 |
| Canonical ID | `D14.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D14_1_Cosmological_Grace_Function.md` |
| Source SHA-256 | `d9d14229f4a3754147c0f272c3a96da154760d32f598a423826eb04d5b22782f` |

## Sequence Navigation

Previous: [0042 - E13.1 - GR-QM Bridge Equation](0042_E13.1_E13_1_GR-QM_Bridge_Equation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0044 - D17.1 - AI Phi Measurement](0044_D17.1_D17_1_AI_Phi_Measurement.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D14.1"
legacy_id: "AX-107"
title: "Cosmological Grace Function"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Cosmology"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D14.1 — Cosmological Grace Function

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Cosmology | **Legacy ID:** `AX-107`

---

## Statement

G(t, Ψ_collective) defined — replaces static Λ in Friedmann equations.

## Formal Expression

```
G(t, Ψ_collective): dynamic grace-cosmology function
```

$$
G(t, Ψ_collective): dynamic grace-cosmology function
$$

## Maximum Defensible Position

Definition: G(t,Ψ) = cosmological grace function.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Cosmology |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 12 — Physics Unification

## References

Friedmann 1922; Dark Energy surveys

## Defeat Conditions

Show G(t,Ψ) gives wrong expansion history.

## Limitations (from workbook)

Functional form is TBD. Without specifying G(t,Ψ), predictions remain qualitative.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::25409c5a-17f1-4501-aa19-4f7ce4d77c91::"G(t, Ψ_collective) must replace static Λ in Friedmann equations"::null::@Engineering%%
%%tag::Requirement::a0feb821-72b3-48c0-8480-ee2874a7bbca::"G(t, Ψ_collective) must be defined as the cosmological grace function"::null::@Engineering%%
%%tag::Requirement::b261b7c9-e1ca-4ba5-9cf8-2e4bb0170974::"G(t, Ψ) must not give wrong expansion history"::null::@Engineering%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::OpenQuestion::4c46299c-e2be-4bb3-bb0f-ebd9ce8be9f4::"Functional form of G(t, Ψ) is TBD"::null::@Engineering%%
%%tag::Risk::8bac4a03-0584-49e9-925d-90c1562d16af::"Without specifying G(t, Ψ), predictions remain qualitative"::null::@Engineering%%
%%tag::Decision::6a03e0e1-feb5-41e1-8ad3-8743187d927d::"Adopt G(t, Ψ) as dynamic grace-cosmology function replacing static Λ"::null::@Engineering%%
%%tag::Metric::c7895492-fe20-4a42-8b61-b0fbf7ec83ee::"Chain Tier: 12 — Physics Unification"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
