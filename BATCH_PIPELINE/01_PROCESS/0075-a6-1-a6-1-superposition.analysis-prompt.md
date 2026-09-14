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
  "title": "Superposition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0075_A6.1_A6_1_Superposition.md",
  "source_sha256": "3b9ae6af99859becdb9d0d7dc3b64dd3d74c69523debe8dec537eceee6171348",
  "captured_at": "2026-09-07T00:06:52.901153+00:00",
  "word_count": 322,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0075: A6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A6.1 \u2014 Superposition",
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

# Canonical Sequential Record 0075: A6.1

| Field | Value |
|---|---|
| Registry position | 75 of 191 |
| Canonical ID | `A6.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A6_1_Superposition.md` |
| Source SHA-256 | `f5bc7e9e65f1ded24562c786c847de7579830776eb0e6240488d19ba59b8bd4e` |

## Sequence Navigation

Previous: [0074 - LN5.1 - Chi Requires Observer For Actualization](0074_LN5.1_LN5_1_Chi_Requires_Observer_For_Actualization.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0076 - A6.2 - Collapse](0076_A6.2_A6_2_Collapse.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "A6.1"
legacy_id: "AX-045"
title: "Superposition"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1"
date: "2026-08-17"
---

# 🔶 A6.1 — Superposition

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-045`

---

## Statement

Unobserved systems exist in superposition of possible states.

## Formal Expression

```
|ψ⟩ = Σ c_n|n⟩ (before measurement)
```

$$
|ψ⟩ = Σ c_n|n⟩ (before measurement)
$$

## Maximum Defensible Position

Framework claim: Unobserved systems exist in superposition of possible states.

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

Standard QM; Schrödinger 1926

## Defeat Conditions

Show definite states without observation.

## Limitations (from workbook)

Standard QM assumption. Framework extends it to χ-field.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::cce61e44-2fb3-413b-b3fa-deec18fdd498::"Unobserved systems must exist in superposition of possible states"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::9126560f-124d-48a7-9c46-63ff5c4c900d::"Defeat condition: showing definite states without observation would invalidate the claim"::null::@Engineering%%
%%tag::Assumption::d70a4317-04e2-4d4d-9ac8-29d47539d756::"Standard QM assumption is applied"::null::@Engineering%%
%%tag::Decision::5e405689-2ebd-4d0b-b03d-6f755c5afa39::"Framework extends superposition to χ-field"::null::@Engineering%%
%%tag::Metric::10f87d78-1d59-48e1-a6c2-2a4222bad036::"Registry position 75 of 191"::null::@Operations%%
%%tag::Stakeholder::2e394285-aa73-4b34-a832-bd1e196dddc4::"Framework maintainers (AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, AX Truth Kernel)"::null::@Operations%%
%%--- END SEMANTIC TAGS ---%%
