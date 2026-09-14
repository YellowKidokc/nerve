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
  "title": "Von Neumann Chain Termination",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0081_T6.1_T6_1_Von_Neumann_Chain_Termination.md",
  "source_sha256": "46f4f802b6cd5845f22391e2f555dd435a4b363344364f8deaf879c262e2ca72",
  "captured_at": "2026-09-07T00:12:21.793339+00:00",
  "word_count": 379,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0081: T6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 T6.1 \u2014 Von Neumann Chain Termination",
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

# Canonical Sequential Record 0081: T6.1

| Field | Value |
|---|---|
| Registry position | 81 of 191 |
| Canonical ID | `T6.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `T6_1_Von_Neumann_Chain_Termination.md` |
| Source SHA-256 | `804040b52b7d57b42cb9892b0a9958cb00b3977daf45b52faade1c744522e43f` |

## Sequence Navigation

Previous: [0080 - P6.2 - Collapse Generates Heat](0080_P6.2_P6_2_Collapse_Generates_Heat.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0082 - LN6.1 - Terminal Observer Necessity](0082_LN6.1_LN6_1_Terminal_Observer_Necessity.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [A6.2 - Collapse](0076_A6.2_A6_2_Collapse.md)

## Canonical Source Text

---
id: "T6.1"
legacy_id: "AX-054"
title: "Von Neumann Chain Termination"
mode: "FW_EXTENDED"
atlas_type: "Theorem"
status: "theorem"
domain: "Ontological"
anchor: "A5.1 → A6.2"
depends_on: "A5.1, A6.2"
date: "2026-08-17"
---

# 🔶 T6.1 — Von Neumann Chain Termination

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Theorem` | **Status:** `theorem`
**Domain:** Ontological | **Legacy ID:** `AX-054`

---

## Statement

The Von Neumann measurement chain must terminate — infinite regress is unphysical.

## Formal Expression

```
∃N: chain terminates at observer N
```

$$
∃N: chain terminates at observer N
$$

## Maximum Defensible Position

Framework claim: The Von Neumann measurement chain must terminate — infinite regress is unphysical.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Theorem` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `theorem` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1 → A6.2
**Depends On:** A5.1, A6.2

## References

Von Neumann 1932; Wigner's friend

## Defeat Conditions

Show infinite measurement chain is physical.

## Limitations (from workbook)

Standard argument against infinite regress in measurement.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::7ebe1f12-bb4d-47dc-93d4-d23e3599e250::"The Von Neumann measurement chain must terminate"::null::@Engineering%%
%%tag::Requirement::d9949d48-5daa-4045-a2f8-50005c7fc06a::"Infinite regress in measurement must be unphysical"::null::@Engineering%%
%%tag::Decision::bbe2b648-2f7b-40aa-9e79-52fdc7c80274::"Adopt the theorem that the measurement chain terminates at a finite observer N"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 (Observation Requirement)"::null::@Engineering%%
%%tag::Dependency::c4dd9de9-8a27-4f23-97ec-8bb922793aa6::"Depends on A6.2 (Collapse)"::null::@Engineering%%
%%tag::Risk::200b2317-2d32-4eff-86db-46d00cbb03e9::"Risk that an infinite measurement chain is shown to be physical, defeating the theorem"::null::@Engineering%%
%%tag::Assumption::10523cee-2e14-4559-989e-6a4245f7d4b3::"Assumes the standard argument against infinite regress in measurement is valid"::null::@Engineering%%
%%tag::Stakeholder::be11065d-024d-45b6-9a3e-c15bdf203ee3::"Wigner's friend (thought experiment referenced as a stakeholder in the argument)"::null::@Engineering%%
%%tag::OpenQuestion::fd7e4fea-4c36-48d3-8938-5cf358f81bbf::"Whether an infinite measurement chain can be physical"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
