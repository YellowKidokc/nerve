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
  "title": "Law VIII: Sign Algebra",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0052_D19.8_D19_8_Law_VIII_Sign_Algebra.md",
  "source_sha256": "45d09f50ad2fbf6e29c84bdeb0506e2415d782de944a8b9a11cf77d1abce42da",
  "captured_at": "2026-09-06T23:50:05.498523+00:00",
  "word_count": 362,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0052: D19.8",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.8 \u2014 Law VIII: Sign Algebra",
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

# Canonical Sequential Record 0052: D19.8

| Field | Value |
|---|---|
| Registry position | 52 of 191 |
| Canonical ID | `D19.8` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_8_Law_VIII_Sign_Algebra.md` |
| Source SHA-256 | `2c2dd7c6babc8488580cb46e4c2368a171aa4551343e11158e4220f6b63d0bfb` |

## Sequence Navigation

Previous: [0051 - D19.7 - Law VII: Actualization Requirement](0051_D19.7_D19_7_Law_VII_Actualization_Requirement.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0053 - D19.9 - Law IX: Grace Non-Unitarity](0053_D19.9_D19_9_Law_IX_Grace_Non-Unitarity.md)

## Registered Dependency Links

[D8.1 - Sign Operator](0032_D8.1_D8_1_Sign_Operator.md)

## Canonical Source Text

---
id: "D19.8"
legacy_id: "AX-143"
title: "Law VIII: Sign Algebra"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "D8.1 (Sign Operator)"
depends_on: "depends on anchor: D8.1 (Sign Operator)"
date: "2026-08-17"
---

# 📐 D19.8 — Law VIII: Sign Algebra

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-143`

---

## Statement

Law VIII: Moral orientation is binary, preserved under self-transformation.

## Formal Expression

```
σ ∈ {±1}; [σ̂,Û]=0; ∃Ĝ(ext): Ĝ|-1⟩=|+1⟩
```

$$
σ ∈ {±1}; [σ̂,Û]=0; ∃Ĝ(ext): Ĝ|-1⟩=|+1⟩
$$

## Maximum Defensible Position

Definition: Law VIII — Sign Algebra.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Unified |

## Dependency Chain

**Anchor:** D8.1 (Sign Operator)
**Depends On:** depends on anchor: D8.1 (Sign Operator)
**Chain Tier:** 13 — Master Laws

## References

Operator Algebra; Sola Gratia

## Defeat Conditions

Disprove binary sign, self-conservation, or external flip requirement.

## Limitations (from workbook)

Vulnerability concentrated in parent axioms.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::17888f9c-35f6-4503-923d-275623174f5d::"Moral orientation must be binary"::null::@Engineering%%
%%tag::Requirement::54832a57-392d-4ffe-b2ce-ac0e44b2f26a::"Moral orientation must be preserved under self-transformation"::null::@Engineering%%
%%tag::Requirement::6767ca6e-c111-4b72-9f6d-424233edb33d::"External flip operator must exist to map -1 to +1"::null::@Engineering%%
%%tag::Dependency::ccfe3840-ae90-4849-ac63-c0db28024377::"Depends on D8.1 Sign Operator"::null::@Engineering%%
%%tag::Risk::dc9786e6-5047-4012-9c49-2520b22e3e17::"Disproving binary sign, self-conservation, or external flip requirement defeats the law"::null::@Engineering%%
%%tag::Risk::d0d13f40-15dd-4095-9595-a579b3a7187a::"Vulnerability concentrated in parent axioms"::null::@Engineering%%
%%tag::Decision::1783ac7a-17c3-4a62-a695-c78e71635a24::"Defined Law VIII as Sign Algebra with binary sign and self-conservation"::null::@Engineering%%
%%tag::Metric::a3cafe46-690b-4a0b-9674-618b55082e6c::"Sign value σ ∈ {±1}"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
