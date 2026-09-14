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
  "title": "Integration Attractor",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0038_D12.1_D12_1_Integration_Attractor.md",
  "source_sha256": "a032e80cfc819674275ede5655f5eab7b082464a446ddc74955c2d573ff1059a",
  "captured_at": "2026-09-06T23:37:52.629934+00:00",
  "word_count": 397,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0038: D12.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D12.1 \u2014 Integration Attractor",
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

# Canonical Sequential Record 0038: D12.1

| Field | Value |
|---|---|
| Registry position | 38 of 191 |
| Canonical ID | `D12.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D12_1_Integration_Attractor.md` |
| Source SHA-256 | `eadb82182573dd722c5ef3d4184507ce649b96b2b78401e024b460de8bbd0c67` |

## Sequence Navigation

Previous: [0037 - D11.1 - Moral Coherence Definition](0037_D11.1_D11_1_Moral_Coherence_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0039 - D12.2 - Fragmentation Attractor](0039_D12.2_D12_2_Fragmentation_Attractor.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D12.1"
legacy_id: "AX-095"
title: "Integration Attractor"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Eschatology"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D12.1 — Integration Attractor

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Eschatology | **Legacy ID:** `AX-095`

---

## Statement

Integration attractor defined — the high-coherence fixed point (heaven).

## Formal Expression

```
A_+ = lim(t→∞) C(t) → C_max — high-Φ fixed point
```

$$
A_+ = lim(t→∞) C(t) → C_max — high-Φ fixed point
$$

## Maximum Defensible Position

Definition: A_+ = integration attractor.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Eschatology |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 11 — Soul + Moral + Eschatology

## References

Dynamical systems (attractors); Teilhard de Chardin (Omega Point)

## Defeat Conditions

Show no stable high-coherence attractor exists. Demonstrate coherence always decays.

## Limitations (from workbook)

The attractor structure is borrowed from dynamical systems. Whether moral systems have attractors is assumed.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::4683850d-0770-451f-b735-8d5ae7c1c18b::"Integration attractor must be defined as the high-coherence fixed point (heaven)"::null::@Product%%
%%tag::Requirement::64c2785c-a400-49e8-92a3-05f5a228b570::"Formal expression A_+ = lim(t→∞) C(t) → C_max must be used as the definition"::null::@Engineering%%
%%tag::Decision::6e5c2ad5-4a17-4d01-a812-5a129020c343::"Adopt dynamical systems attractor concept for moral/eschatological definition"::null::@Engineering,Product%%
%%tag::Decision::c6068af5-5e78-47f9-aba0-1c4ad3664cfe::"Define integration attractor as A_+ (high-coherence fixed point)"::null::@Product%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Assumption::44b59882-ab64-440e-b53b-ace3bb3b89a8::"Moral systems have attractors (borrowed from dynamical systems)"::null::@Engineering,Product%%
%%tag::Risk::24f69516-c977-4311-8700-712b671332bc::"No stable high-coherence attractor exists (defeat condition)"::null::@Engineering,Product%%
%%tag::Risk::a58c476a-7fb7-4b58-b3af-17ccd45f61de::"Coherence always decays (defeat condition)"::null::@Engineering,Product%%
%%tag::Stakeholder::b998e7e4-74de-4a21-be12-9f742b013736::"Teilhard de Chardin (Omega Point reference)"::null::@Product%%
%%tag::OpenQuestion::c9471a5f-9e7e-472b-908f-3929b92d2dc3::"Whether moral systems have attractors is assumed but unproven"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
