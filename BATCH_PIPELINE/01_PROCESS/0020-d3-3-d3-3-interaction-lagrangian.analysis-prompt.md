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
  "title": "Interaction Lagrangian",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0020_D3.3_D3_3_Interaction_Lagrangian.md",
  "source_sha256": "cf5985710ac4a8fe88a514700a1cee4987589ea8e369a4dfe8b6cfb1594fd106",
  "captured_at": "2026-09-06T23:20:36.922472+00:00",
  "word_count": 380,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0020: D3.3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D3.3 \u2014 Interaction Lagrangian",
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

# Canonical Sequential Record 0020: D3.3

| Field | Value |
|---|---|
| Registry position | 20 of 191 |
| Canonical ID | `D3.3` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D3_3_Interaction_Lagrangian.md` |
| Source SHA-256 | `55fdabe7d6e48925aeaf4c1097b92623c0fdeb26e5a6b237c7b789918e3dd1e3` |

## Sequence Navigation

Previous: [0019 - D3.2 - Self-Interaction Potential](0019_D3.2_D3_2_Self-Interaction_Potential.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0021 - E3.1 - Master Coherence Equation](0021_E3.1_E3_1_Master_Coherence_Equation.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D3.3"
legacy_id: "AX-020"
title: "Interaction Lagrangian"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D3.3 — Interaction Lagrangian

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-020`

---

## Statement

Interaction Lagrangian — how χ couples to other fields.

## Formal Expression

```
L_int = g·χ·ψ̄ψ — Yukawa-type coupling
```

$$
L_int = g·χ·ψ̄ψ — Yukawa-type coupling
$$

## Maximum Defensible Position

Definition: L_int = Yukawa coupling of χ to matter.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 4 — Coherence System

## References

Yukawa theory; Standard Model couplings

## Defeat Conditions

Show χ doesn't couple to matter. Demonstrate wrong coupling structure.

## Limitations (from workbook)

Yukawa form is the simplest. Real coupling may be more complex.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::de00c7f3-0959-488e-97d3-4b7a211730c6::"χ must couple to matter via Yukawa-type coupling"::null::@Engineering%%
%%tag::Requirement::9da9cffa-eba6-4179-b86b-9f51c006390d::"The coupling structure must be Yukawa-type"::null::@Engineering%%
%%tag::Decision::2e906b85-dd2e-4bef-bedf-2968ec0227c7::"Adopt Yukawa-type coupling for χ-matter interaction"::null::@Engineering%%
%%tag::Decision::d66ecb79-a0b6-43cb-b473-309a710a30dc::"Use simplest coupling form (Yukawa) for initial definition"::null::@Engineering%%
%%tag::Risk::0791ae8c-40a5-4aa1-ad73-39780ae3579b::"Real coupling may be more complex than Yukawa form"::null::@Engineering%%
%%tag::Risk::b0d955f0-1154-4506-ac8c-b4c322ae21e8::"χ might not couple to matter, invalidating the definition"::null::@Engineering%%
%%tag::Assumption::2db47eca-0c49-4eb0-bc3a-f75482a40437::"Yukawa form is the simplest and sufficient for definition"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Dependency::93d5d717-0130-45e1-9457-00d61aef7d66::"Depends on Yukawa theory and Standard Model couplings"::null::@Engineering%%
%%tag::Metric::30ecb167-8dce-4ca1-b4e8-8fce16a010f0::"Coupling strength g (in L_int = g·χ·ψ̄ψ)"::null::@Engineering%%
%%tag::Stakeholder::e6ce0973-e3de-4a1f-81bc-b72cfec76f48::"Ontological domain researchers"::null::@Engineering%%
%%tag::OpenQuestion::67d103fa-6e6f-4432-b695-445c3fb2e253::"Is the Yukawa coupling structure correct for χ?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
