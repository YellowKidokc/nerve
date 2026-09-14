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
  "title": "Master Coherence Equation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0021_E3.1_E3_1_Master_Coherence_Equation.md",
  "source_sha256": "308eed3ae83fa337b50aaea37952b4b859546f7a722e78235b0da50edaaa76bd",
  "captured_at": "2026-09-06T23:21:20.639700+00:00",
  "word_count": 388,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0021: E3.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E3.1 \u2014 Master Coherence Equation",
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

# Canonical Sequential Record 0021: E3.1

| Field | Value |
|---|---|
| Registry position | 21 of 191 |
| Canonical ID | `E3.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E3_1_Master_Coherence_Equation.md` |
| Source SHA-256 | `968bac893d2bdeab397eb5e503a7dd06b8af3c357290fcb7907f3bae2a3dc71f` |

## Sequence Navigation

Previous: [0020 - D3.3 - Interaction Lagrangian](0020_D3.3_D3_3_Interaction_Lagrangian.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0022 - E3.2 - Universal Coherence Definition](0022_E3.2_E3_2_Universal_Coherence_Definition.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Canonical Source Text

---
id: "E3.1"
legacy_id: "AX-021"
title: "Master Coherence Equation"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2, D3.1"
date: "2026-08-17"
---

# 📐 E3.1 — Master Coherence Equation

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-021`

---

## Statement

Master coherence equation — coherence dynamics with decay and source terms.

## Formal Expression

```
dC/dt = -Γ_decay·C + Γ_source·J(χ)
```

$$
\frac{dC}{dt} = -Γ_decay·C + Γ_source·J(χ)
$$

## Maximum Defensible Position

Model: Coherence dynamics equation.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2, D3.1
**Chain Tier:** 4 — Coherence System

## References

Rate equations; Open systems dynamics

## Defeat Conditions

Show coherence dynamics don't follow this form. Demonstrate missing terms.

## Limitations (from workbook)

Linear approximation. Nonlinear effects may be important at high coherence.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6fc0476b-8817-4097-b620-382f162df99f::"Coherence dynamics must follow the master coherence equation form dC/dt = -Γ_decay·C + Γ_source·J(χ)"::null::@Engineering%%
%%tag::Requirement::cd838863-da8c-4c9d-b676-a7516553c37b::"Coherence dynamics must include decay and source terms"::null::@Engineering%%
%%tag::Decision::8ed94528-473c-490a-8c88-a22f630beacc::"Adopt linear approximation for coherence dynamics equation"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Dependency::57615bda-6e3b-482c-a1b6-64989354bcc5::"Depends on D3.1 (Coherence Functional Definition)"::null::@Engineering%%
%%tag::Risk::98188149-6efb-442a-a9f0-757dd0df57ca::"Nonlinear effects may be important at high coherence, potentially invalidating the linear approximation"::null::@Engineering%%
%%tag::Risk::3f23ef31-cb35-4b1d-842b-4aa71f3f5010::"Coherence dynamics may not follow the specified form, or missing terms may exist"::null::@Engineering%%
%%tag::Assumption::be413678-224b-47da-989d-4ec7d6c50ed4::"Coherence dynamics can be adequately modeled with a linear approximation"::null::@Engineering%%
%%tag::OpenQuestion::7cd2695d-d7ea-4d59-b2eb-7bc524b56629::"Whether coherence dynamics follow the specified equation form"::null::@Engineering%%
%%tag::OpenQuestion::671a0a56-4d4e-4e24-8b16-7888c9b41540::"Whether there are missing terms in the coherence dynamics equation"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
