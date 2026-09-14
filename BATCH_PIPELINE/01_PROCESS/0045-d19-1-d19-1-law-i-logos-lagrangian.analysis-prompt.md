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
  "title": "Law I: Logos-Lagrangian",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0045_D19.1_D19_1_Law_I_Logos-Lagrangian.md",
  "source_sha256": "9cf48a6602658c218c6d4db49e9caa21a94e4e865779224ffb033c70d7907b70",
  "captured_at": "2026-09-06T23:44:02.237927+00:00",
  "word_count": 348,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0045: D19.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.1 \u2014 Law I: Logos-Lagrangian",
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

# Canonical Sequential Record 0045: D19.1

| Field | Value |
|---|---|
| Registry position | 45 of 191 |
| Canonical ID | `D19.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_1_Law_I_Logos-Lagrangian.md` |
| Source SHA-256 | `829141b5bcadd2c166118f10bdfe8be44b668670145034a9bb9ee018e4d6f4c5` |

## Sequence Navigation

Previous: [0044 - D17.1 - AI Phi Measurement](0044_D17.1_D17_1_AI_Phi_Measurement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0046 - D19.2 - Law II: Ten Variables](0046_D19.2_D19_2_Law_II_Ten_Variables.md)

## Registered Dependency Links

[D3.3 - Interaction Lagrangian](0020_D3.3_D3_3_Interaction_Lagrangian.md)

## Canonical Source Text

---
id: "D19.1"
legacy_id: "AX-136"
title: "Law I: Logos-Lagrangian"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "D3.3 (Interaction Lagrangian)"
depends_on: "depends on anchor: D3.3 (Interaction Lagrangian)"
date: "2026-08-17"
---

# 📐 D19.1 — Law I: Logos-Lagrangian

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-136`

---

## Statement

Law I: The Logos-Lagrangian Correspondence.

## Formal Expression

```
L = L(χ, ∂χ, g_μν)
```

$$
L = L(χ, ∂χ, g_μν)
$$

## Maximum Defensible Position

Definition: Law I — Logos-Lagrangian.

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

**Anchor:** D3.3 (Interaction Lagrangian)
**Depends On:** depends on anchor: D3.3 (Interaction Lagrangian)
**Chain Tier:** 13 — Master Laws

## References

Noether 1918; Lagrangian mechanics

## Defeat Conditions

Show reality requires non-Lagrangian description.

## Limitations (from workbook)

Well-motivated. The χ-specific form needs derivation.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::658f79dc-952e-44d9-84e5-de6889412d93::"Law I must define the Logos-Lagrangian Correspondence"::null::@Engineering%%
%%tag::Requirement::c242a6c9-4ad8-4ed8-85b7-8ec67c68d006::"Law I must be expressed as L = L(χ, ∂χ, g_μν)"::null::@Engineering%%
%%tag::Requirement::94e9110c-5c08-44b2-9844-4c5835be73ec::"Law I must depend on anchor D3.3 (Interaction Lagrangian)"::null::@Engineering%%
%%tag::Dependency::88f74d2a-c774-40e3-b976-e5314fb4cb70::"Depends on D3.3 (Interaction Lagrangian)"::null::@Engineering%%
%%tag::Risk::35ee780f-8d30-4d6e-a511-1828eb07a051::"Defeat condition: reality may require non-Lagrangian description"::null::@Engineering%%
%%tag::OpenQuestion::7e3df352-0c35-4f75-9bf4-f6b9a298df28::"The χ-specific form needs derivation"::null::@Engineering%%
%%tag::Assumption::74f0db1c-c13e-4d30-b5cd-4e8596211500::"Lagrangian mechanics applies to the domain"::null::@Engineering%%
%%tag::Stakeholder::0e6a71d5-9a52-43b7-b488-a9b8327f449d::"Noether (1918) - referenced for foundational support"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
