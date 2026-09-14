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
  "title": "Collapse Rate Gamma",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0029_D6.1_D6_1_Collapse_Rate_Gamma.md",
  "source_sha256": "8bfe4f2ddd74136008bc69123df1e587c2d392d3390d858f4c91caa602e9c0a9",
  "captured_at": "2026-09-06T23:28:01.711755+00:00",
  "word_count": 410,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0029: D6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D6.1 \u2014 Collapse Rate Gamma",
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

# Canonical Sequential Record 0029: D6.1

| Field | Value |
|---|---|
| Registry position | 29 of 191 |
| Canonical ID | `D6.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D6_1_Collapse_Rate_Gamma.md` |
| Source SHA-256 | `47cfdf2dbcb9ca18c614a771830bfb695941a88f9201dac0d099fa95de4b984f` |

## Sequence Navigation

Previous: [0028 - D5.3 - Witness Field Operator](0028_D5.3_D5_3_Witness_Field_Operator.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0030 - D6.2 - Projection Operator](0030_D6.2_D6_2_Projection_Operator.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md); [A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "D6.1"
legacy_id: "AX-048"
title: "Collapse Rate Gamma"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A5.1 (Observation Requirement)"
depends_on: "A5.1, D5.2"
date: "2026-08-17"
---

# 📐 D6.1 — Collapse Rate Gamma

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-048`

---

## Statement

Collapse rate Γ defined — depends on observer's integrated information.

## Formal Expression

```
Γ = collapse rate; Γ ∝ f(Φ)
```

$$
Γ = collapse rate; Γ ∝ f(Φ)
$$

## Maximum Defensible Position

Definition: Γ = collapse rate function.

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

**Anchor:** A5.1 (Observation Requirement)
**Depends On:** A5.1, D5.2
**Chain Tier:** 7 — Collapse Dynamics

## References

Objective collapse theories (GRW, Penrose); Measurement problem

## Defeat Conditions

Show collapse rate is observer-independent. Demonstrate Γ ≠ f(Φ).

## Limitations (from workbook)

The Φ-dependence of Γ is the framework's distinctive prediction. Standard QM has Γ = constant.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::e8356950-89eb-4e4e-a174-74296725ec4e::"Collapse rate Γ must be defined as a function of observer's integrated information Φ"::null::@Engineering%%
%%tag::Requirement::75f6f931-a403-4c84-ab92-e74d600e9cf3::"Collapse rate Γ must depend on observer's integrated information Φ"::null::@Engineering%%
%%tag::Decision::3377c15e-8e92-40c4-afa3-4c14658148d3::"Defined collapse rate Γ as proportional to f(Φ)"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 (Observation Requirement)"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 (Integrated Information Phi)"::null::@Engineering%%
%%tag::Risk::e5b2becb-f024-4bc8-a721-fa61afbe51ec::"Risk that collapse rate is observer-independent, defeating the definition"::null::@Engineering%%
%%tag::Risk::f4f0d8ea-1aa6-4fad-b349-2e89f23a0ad6::"Risk that Γ ≠ f(Φ), invalidating the framework's distinctive prediction"::null::@Engineering%%
%%tag::Assumption::2c39b6a3-b0e8-4758-bfa5-ecdc1da4d63e::"Assumes collapse rate Γ is observer-dependent"::null::@Engineering%%
%%tag::Assumption::704938a7-e4b4-4aa2-a1d5-642f2e6d05b5::"Assumes Γ is a function of Φ"::null::@Engineering%%
%%tag::Metric::7effa161-1534-45a2-9200-75a7d64892ce::"Collapse rate Γ (defined as proportional to f(Φ))"::null::@Engineering%%
%%tag::Stakeholder::37b4d580-c603-4b96-b3d6-12c2ad186172::"Observer (whose integrated information Φ determines collapse rate)"::null::@Engineering%%
%%tag::OpenQuestion::29727fbf-e58a-4e16-885d-1277f8a86315::"Whether collapse rate is observer-independent (defeat condition)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
