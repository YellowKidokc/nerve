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
  "title": "Collapse",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0076_A6.2_A6_2_Collapse.md",
  "source_sha256": "614ac3d87e2a68630d0a31b07510514550d068fda1c7e0e5a9ceae361929ff77",
  "captured_at": "2026-09-07T00:07:46.470923+00:00",
  "word_count": 325,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0076: A6.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A6.2 \u2014 Collapse",
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

# Canonical Sequential Record 0076: A6.2

| Field | Value |
|---|---|
| Registry position | 76 of 191 |
| Canonical ID | `A6.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A6_2_Collapse.md` |
| Source SHA-256 | `ad185b3470d9670e192b259f1ae2fbc60c932322021c6cd8031fbbf3146a9799` |

## Sequence Navigation

Previous: [0075 - A6.1 - Superposition](0075_A6.1_A6_1_Superposition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0077 - A6.3 - Irreversibility](0077_A6.3_A6_3_Irreversibility.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "A6.2"
legacy_id: "AX-046"
title: "Collapse"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1"
date: "2026-08-17"
---

# 🔶 A6.2 — Collapse

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-046`

---

## Statement

Observation causes superposition to collapse to definite state.

## Formal Expression

```
|ψ⟩ → |n⟩ with probability |c_n|²
```

$$
|ψ⟩ → |n⟩ with probability |c_n|²
$$

## Maximum Defensible Position

Framework claim: Observation causes superposition to collapse to definite state.

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

Copenhagen; Von Neumann 1932; Born rule

## Defeat Conditions

Show collapse doesn't happen (MWI).

## Limitations (from workbook)

Copenhagen assumption. MWI, Bohmian, and QBist alternatives exist.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::820df457-4760-413a-8764-a3102696450d::"Observation must cause superposition to collapse to a definite state"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Decision::620f97f5-bbfe-46b4-a36b-1a928e999a11::"Adopt Copenhagen interpretation for collapse"::null::@Engineering%%
%%tag::Risk::55bab0e6-e518-4dc0-b612-eec645e1f619::"Alternative interpretations (MWI, Bohmian, QBist) challenge collapse"::null::@Engineering%%
%%tag::Assumption::9aa6c83e-215c-4704-95f0-005086035e17::"Copenhagen assumption that collapse occurs upon observation"::null::@Engineering%%
%%tag::Metric::d550d479-bc54-4f68-8ada-6b908538dc46::"Probability of collapse to state n is |c_n|²"::null::@Engineering%%
%%tag::OpenQuestion::7d4e04e2-239e-44de-9628-ef8f6f5c0d71::"Does collapse actually happen or is it only apparent (MWI)?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
