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
  "title": "Irreversibility",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0077_A6.3_A6_3_Irreversibility.md",
  "source_sha256": "a246c9b2afe6a70718f63e139296926b436d67ae8f186606ca95f0a444b98930",
  "captured_at": "2026-09-07T00:08:37.342316+00:00",
  "word_count": 305,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0077: A6.3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A6.3 \u2014 Irreversibility",
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

# Canonical Sequential Record 0077: A6.3

| Field | Value |
|---|---|
| Registry position | 77 of 191 |
| Canonical ID | `A6.3` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A6_3_Irreversibility.md` |
| Source SHA-256 | `de9ede33383acc900341a73258dd770c4a539e0cd79c2ebe8e23cbed7f705364` |

## Sequence Navigation

Previous: [0076 - A6.2 - Collapse](0076_A6.2_A6_2_Collapse.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0078 - E6.1 - Modified Schrödinger With Collapse](0078_E6.1_E6_1_Modified_Schr_dinger_With_Collapse.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "A6.3"
legacy_id: "AX-047"
title: "Irreversibility"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1"
date: "2026-08-17"
---

# 🔶 A6.3 — Irreversibility

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-047`

---

## Statement

Collapse is irreversible — you cannot uncollapse a measurement.

## Formal Expression

```
Collapse: irreversible process
```

$$
Collapse: irreversible process
$$

## Maximum Defensible Position

Framework claim: Collapse is irreversible — you cannot uncollapse a measurement.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Claim` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `framework_commitment` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1
**Depends On:** A5.1

## References

Landauer's principle; 2nd Law

## Defeat Conditions

Show reversible collapse.

## Limitations (from workbook)

Standard assumption. Quantum error correction can partially reverse, but not fully.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::6cd96285-913f-4124-a54a-bebe0d7411ce::"Collapse must be irreversible"::null::@Engineering%%
%%tag::Requirement::97ee0675-c570-43ce-8580-6cea89ebe41c::"Measurement cannot be uncollapsed"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Assumption::aa5a70a5-513a-4314-afc6-937570e3a366::"Assumes collapse is a standard irreversible process"::null::@Engineering%%
%%tag::Risk::7772215e-9f0c-4b87-b6f6-bffc8c7d8814::"Risk that reversible collapse is demonstrated"::null::@Engineering%%
%%tag::OpenQuestion::57c003e7-3a59-4df4-a965-f9c38b25e01d::"Whether quantum error correction can fully reverse collapse"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
