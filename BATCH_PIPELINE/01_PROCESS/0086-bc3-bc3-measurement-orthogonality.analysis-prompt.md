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
  "title": "Measurement Orthogonality",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0086_BC3_BC3_Measurement_Orthogonality.md",
  "source_sha256": "7248ef3830fe37185aeb938937364e96a93005d64134610ed9e522bcc38c0fa0",
  "captured_at": "2026-09-07T00:17:47.914175+00:00",
  "word_count": 307,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0086: BC3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 BC3 \u2014 Measurement Orthogonality",
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

# Canonical Sequential Record 0086: BC3

| Field | Value |
|---|---|
| Registry position | 86 of 191 |
| Canonical ID | `BC3` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `BC3_Measurement_Orthogonality.md` |
| Source SHA-256 | `ad93daa7f6999b59c305b1b421db8ef73bd98f2a47fb5f466042895850c66fb7` |

## Sequence Navigation

Previous: [0085 - BC1 - Terminal Observer Exists](0085_BC1_BC1_Terminal_Observer_Exists.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0087 - BC5 - Superposition Preserved Until Collapse](0087_BC5_BC5_Superposition_Preserved_Until_Collapse.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [A6.2 - Collapse](0076_A6.2_A6_2_Collapse.md)

## Canonical Source Text

---
id: "BC3"
legacy_id: "AX-060"
title: "Measurement Orthogonality"
mode: "FW_EXTENDED"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "A5.1 → A6.2"
depends_on: "A5.1, A6.2"
date: "2026-08-17"
---

# 🔶 BC3 — Measurement Orthogonality

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-060`

---

## Statement

Measurements must be orthogonal — independent measurement axes.

## Formal Expression

```
⟨m_i|m_j⟩ = δ_ij
```

$$
⟨m_i|m_j⟩ = δ_ij
$$

## Maximum Defensible Position

Framework claim: Measurements must be orthogonal — independent measurement axes.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Boundary Condition` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `boundary` |
| **Domain** | Divine |

## Dependency Chain

**Anchor:** A5.1 → A6.2
**Depends On:** A5.1, A6.2

## References

QM measurement theory; Stern-Gerlach

## Defeat Conditions

Show non-orthogonal measurement suffices.

## Limitations (from workbook)

Standard QM constraint applied to the BC system.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::e9a4de23-8e52-4514-86a4-3250bcbc2294::"Measurements must be orthogonal"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::c4dd9de9-8a27-4f23-97ec-8bb922793aa6::"Depends on A6.2 Collapse"::null::@Engineering%%
%%tag::Risk::4fed990c-4687-4bbb-b5df-101f5c906f15::"Non-orthogonal measurement could invalidate the boundary condition"::null::@Engineering%%
%%tag::OpenQuestion::4095ed36-08cd-43c1-b61c-60e91062b2a7::"Whether non-orthogonal measurement suffices"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
