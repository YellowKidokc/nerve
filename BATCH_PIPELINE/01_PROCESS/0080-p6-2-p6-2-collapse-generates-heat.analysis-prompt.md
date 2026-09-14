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
  "title": "Collapse Generates Heat",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0080_P6.2_P6_2_Collapse_Generates_Heat.md",
  "source_sha256": "6d9cc83d093e27d9473da0f65f55100857a8abbb437948258c4e490fce0a3c87",
  "captured_at": "2026-09-07T00:11:24.546606+00:00",
  "word_count": 342,
  "tags": [
    "information-theory",
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0080: P6.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P6.2 \u2014 Collapse Generates Heat",
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

# Canonical Sequential Record 0080: P6.2

| Field | Value |
|---|---|
| Registry position | 80 of 191 |
| Canonical ID | `P6.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P6_2_Collapse_Generates_Heat.md` |
| Source SHA-256 | `526deb3a1f42d78ff798c20e8aaad025519f6bd1e8f8cbee225fb9b6d3b7cf01` |

## Sequence Navigation

Previous: [0079 - P6.1 - Collapse Rate Proportional To Phi](0079_P6.1_P6_1_Collapse_Rate_Proportional_To_Phi.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0081 - T6.1 - Von Neumann Chain Termination](0081_T6.1_T6_1_Von_Neumann_Chain_Termination.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [A6.3 - Irreversibility](0077_A6.3_A6_3_Irreversibility.md)

## Canonical Source Text

---
id: "P6.2"
legacy_id: "AX-053"
title: "Collapse Generates Heat"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A5.1 → A6.3"
depends_on: "A5.1, A6.3"
date: "2026-08-17"
---

# 🔶 P6.2 — Collapse Generates Heat

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-053`

---

## Statement

Wavefunction collapse generates heat — Landauer bound applies.

## Formal Expression

```
Q_collapse ≥ kT·ln2 per bit of information gained
```

$$
Q_collapse \ge kT·ln2 per bit of information gained
$$

## Maximum Defensible Position

Framework claim: Wavefunction collapse generates heat — Landauer bound applies.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Property` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `property` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1 → A6.3
**Depends On:** A5.1, A6.3

## References

Landauer 1961; Bennett 1982

## Defeat Conditions

Show collapse without heat generation.

## Limitations (from workbook)

Follows from Landauer's principle if collapse is information gain.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::b6e00b72-9b6c-4e84-bd86-1d1c3968daaa::"Wavefunction collapse must generate heat"::null::@Engineering%%
%%tag::Requirement::5061ab86-a3d7-45d8-a6ed-2e34e2f6723a::"Landauer bound must apply to collapse"::null::@Engineering%%
%%tag::Metric::3aa822c5-e3ae-440e-bed1-828378d10f33::"Heat generated per bit of information gained (Q_collapse ≥ kT·ln2)"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::2f165a18-32d9-4f90-9d40-955f029ec409::"Depends on A6.3 Irreversibility"::null::@Engineering%%
%%tag::Assumption::58eb5c3d-588d-4702-8196-9c7f3f5d4928::"Collapse is information gain"::null::@Engineering%%
%%tag::Risk::e4ce42e1-9828-4fc9-ab3f-fe8c8fb0482d::"Collapse without heat generation would defeat the property"::null::@Engineering%%
%%tag::Reference::86f3354f-c918-4130-ab0b-2447821c520e::"Landauer 1961; Bennett 1982"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
