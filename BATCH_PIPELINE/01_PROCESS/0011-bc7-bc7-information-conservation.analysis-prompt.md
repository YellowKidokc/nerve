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
  "title": "Information Conservation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0011_BC7_BC7_Information_Conservation.md",
  "source_sha256": "1d9d60446486c30bbfc365d000ab78d08c6cc0b8b28c86e86c8578a2205407d2",
  "captured_at": "2026-09-06T23:10:20.164604+00:00",
  "word_count": 356,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0011: BC7",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd37 BC7 \u2014 Information Conservation",
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

# Canonical Sequential Record 0011: BC7

| Field | Value |
|---|---|
| Registry position | 11 of 191 |
| Canonical ID | `BC7` |
| Mode | `AX_DERIVED` |
| Canonical source filename | `BC7_Information_Conservation.md` |
| Source SHA-256 | `77730a90b67bd9b4d78d075b10f52c67b0c2dffc8a01b5dbedac38ad44bf6cb1` |

## Sequence Navigation

Previous: [0010 - BC2 - Grace External To System](0010_BC2_BC2_Grace_External_To_System.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0012 - BC8 - Voluntary Coupling](0012_BC8_BC8_Voluntary_Coupling.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "BC7"
legacy_id: "AX-064"
title: "Information Conservation"
mode: "AX_DERIVED"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 🔷 BC7 — Information Conservation

**Mode:** `AX_DERIVED` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-064`

---

## Statement

Total information is conserved — the terminal observer preserves all information.

## Formal Expression

```
dI_total/dt = 0 (information neither created nor destroyed)
```

$$
dI_total/dt = 0 (information neither created nor destroyed)
$$

## Maximum Defensible Position

Derived from BC6: Infinite energy source preserves information.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_DERIVED` |
| **Atlas Object Type** | `Boundary Condition` |
| **Workbook Role** | `BOUNDARY_CONDITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `boundary` |
| **Domain** | Divine |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 8 — Boundary Conditions

## References

Black hole information paradox; Unitarity in QM; Susskind/Maldacena

## Defeat Conditions

Show information destruction. Demonstrate permanent information loss in any process.

## Limitations (from workbook)

The black hole information paradox is the strongest test case.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::404506b0-1a6c-42da-a63b-6db46f25732d::"Total information must be conserved"::null::@Engineering%%
%%tag::Requirement::2e9c34b0-6efa-4e2b-9283-f819732681d7::"Terminal observer must preserve all information"::null::@Engineering%%
%%tag::Decision::18bfb670-7008-4c6f-a15c-e33bd6bd6409::"Adopt information conservation as a boundary condition"::null::@Product%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 Infinite Energy Source"::null::@Engineering%%
%%tag::Risk::c166d307-31ba-4e92-9ad0-d561530b9199::"Black hole information paradox could invalidate conservation"::null::@Engineering%%
%%tag::Metric::56916cff-4739-43b8-9a39-df50cde6c4a6::"dI_total/dt = 0"::null::@Engineering%%
%%tag::Stakeholder::7dfeeeb1-363e-460f-a063-5010a00cf3ee::"Terminal observer"::null::@Engineering%%
%%tag::OpenQuestion::5af62650-647a-4978-9223-209c85716f0b::"How to resolve the black hole information paradox"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
