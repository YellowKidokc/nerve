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
  "title": "Logos Field Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0015_D2.1_D2_1_Logos_Field_Definition.md",
  "source_sha256": "2beeca3c8324615accb27c1955bf5a1a601309d2540a9a46faada919f2e2bc4d",
  "captured_at": "2026-09-06T23:15:22.036341+00:00",
  "word_count": 410,
  "tags": [
    "information-theory",
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0015: D2.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D2.1 \u2014 Logos Field Definition",
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

# Canonical Sequential Record 0015: D2.1

| Field | Value |
|---|---|
| Registry position | 15 of 191 |
| Canonical ID | `D2.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D2_1_Logos_Field_Definition.md` |
| Source SHA-256 | `01207bebca23710e4c0ba27a53ebd4df9b6cfef1b3ade9a4671fa6f68e679b42` |

## Sequence Navigation

Previous: [0014 - D1.2 - Bit Definition](0014_D1.2_D1_2_Bit_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0016 - D2.2 - Chi Field Properties](0016_D2.2_D2_2_Chi_Field_Properties.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D2.1"
legacy_id: "AX-010"
title: "Logos Field Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D2.1 — Logos Field Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-010`

---

## Statement

The Logos Field χ is defined as the self-grounding informational substrate.

## Formal Expression

```
χ: M → ℝ (scalar field over manifold M)
```

$$
χ: M → ℝ (scalar field over manifold M)
$$

## Maximum Defensible Position

Definition: χ = self-grounding informational field.

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
**Chain Tier:** 3 — Field Definitions

## References

Quantum Field Theory; Higgs mechanism; Wheeler 1990

## Defeat Conditions

Show the χ-field is not needed. Demonstrate substrate without field structure.

## Limitations (from workbook)

The identification χ = Logos is a bridge claim, not part of the definition.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::f7276569-b5ca-4e9e-a2e1-cc3f6b2d76b7::"The Logos Field χ must be defined as the self-grounding informational substrate"::null::@Engineering%%
%%tag::Requirement::1b8fc526-cf73-4b77-8f0b-8644111442a6::"The χ-field must be a scalar field over manifold M"::null::@Engineering%%
%%tag::Decision::3aab3f36-2501-4d94-a76f-28dd8f905cf9::"Adopt the formal expression χ: M → ℝ for the Logos Field"::null::@Engineering%%
%%tag::Decision::01ad9200-1e66-4674-97d7-8b2fa9590658::"Classify D2.1 as a Definition in the AX_SCAFFOLD mode"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::21d5c129-e7a1-4606-ac82-4e6ffc763c83::"Risk that the χ-field is not needed, undermining the definition"::null::@Engineering%%
%%tag::Risk::a67d793c-eb8d-4fae-9b52-4e597ff2e305::"Risk that a substrate without field structure can be demonstrated"::null::@Engineering%%
%%tag::Assumption::76e7fd05-1579-49b1-bd91-85492097f538::"Assumes the identification χ = Logos is a bridge claim, not part of the definition"::null::@Engineering%%
%%tag::Metric::dbe4c2a4-2899-4637-8af3-52b9de79b3fe::"Registry position 15 of 191"::null::@Operations%%
%%tag::Metric::f26e9323-eafe-4cdd-89b3-17145c3c5018::"Chain Tier 3 — Field Definitions"::null::@Engineering%%
%%tag::Stakeholder::53088d28-3f6c-4b3d-964a-8d1ea1dfc92b::"Workbook role DEFINITION"::null::@Engineering%%
%%tag::OpenQuestion::a79e2c07-8814-4dab-9144-80a8e2b231a5::"Whether the χ-field is actually needed"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
