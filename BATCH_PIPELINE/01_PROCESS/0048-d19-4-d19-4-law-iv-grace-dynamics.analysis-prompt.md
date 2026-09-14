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
  "title": "Law IV: Grace Dynamics",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0048_D19.4_D19_4_Law_IV_Grace_Dynamics.md",
  "source_sha256": "58be96b61e7ce6ffd65348ae46a07c452c3b8d01ea24c5777394e105986f4c69",
  "captured_at": "2026-09-06T23:46:37.686103+00:00",
  "word_count": 464,
  "tags": [
    "information-theory",
    "systems",
    "theology"
  ],
  "source_outline": [
    "Canonical Sequential Record 0048: D19.4",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.4 \u2014 Law IV: Grace Dynamics",
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

# Canonical Sequential Record 0048: D19.4

| Field | Value |
|---|---|
| Registry position | 48 of 191 |
| Canonical ID | `D19.4` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_4_Law_IV_Grace_Dynamics.md` |
| Source SHA-256 | `33708b4e5273f982ad23810c266edf0351b11e2c25bb3e8b383d703a3301a036` |

## Sequence Navigation

Previous: [0047 - D19.3 - Law III: Entropy-Sin](0047_D19.3_D19_3_Law_III_Entropy-Sin.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0049 - D19.5 - Law V: Conservation Symmetry](0049_D19.5_D19_5_Law_V_Conservation_Symmetry.md)

## Registered Dependency Links

[D9.1 - Grace Operator Definition](0033_D9.1_D9_1_Grace_Operator_Definition.md)

## Canonical Source Text

---
id: "D19.4"
legacy_id: "AX-139"
title: "Law IV: Grace Dynamics"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "D9.1 (Grace Operator Definition)"
depends_on: "depends on anchor: D9.1 (Grace Operator Definition)"
date: "2026-08-17"
---

# 📐 D19.4 — Law IV: Grace Dynamics

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-139`

---

## Statement

Law IV: The Grace Dynamics.

## Formal Expression

```
Ĝ: σ(-1) → σ(+1); non-unitary
```

$$
Ĝ: σ(-1) → σ(+1); non-unitary
$$

## Maximum Defensible Position

Definition: Law IV — Grace Dynamics.

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

**Anchor:** D9.1 (Grace Operator Definition)
**Depends On:** depends on anchor: D9.1 (Grace Operator Definition)
**Chain Tier:** 13 — Master Laws

## References

Sola Gratia; 2nd Law (external input)

## Defeat Conditions

Show grace dynamics are inconsistent.

## Limitations (from workbook)

Law IV is the theological law. Central claim.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::336326e7-e177-4d59-a75a-d5c35acf2a5d::"Law IV must define Grace Dynamics as a non-unitary transformation from σ(-1) to σ(+1)"::null::@Engineering%%
%%tag::Requirement::e6d4d003-3877-4d31-be7b-39b752220e12::"Law IV must depend on the Grace Operator Definition (D9.1)"::null::@Engineering%%
%%tag::Requirement::1d27d753-79ca-43b6-91d0-300a32a19830::"Law IV must be classified as a formal definition with primitive formalization"::null::@Engineering%%
%%tag::Requirement::5e6906bf-24e6-493f-b8fb-80aa21056582::"Law IV must be defeated if grace dynamics are shown to be inconsistent"::null::@Engineering%%
%%tag::Decision::5c591ef0-5a27-4775-967e-fd7b9e8f261f::"Adopt the formal expression Ĝ: σ(-1) → σ(+1); non-unitary for Grace Dynamics"::null::@Engineering%%
%%tag::Decision::c1318982-7dae-43fc-b64f-1119b7e59a2f::"Classify Law IV as a theological law and central claim"::null::@Product%%
%%tag::Dependency::07409173-524a-458f-8705-82fdb1bd3559::"Law IV depends on D9.1 Grace Operator Definition"::null::@Engineering%%
%%tag::Dependency::703b4c65-f4e4-4ded-a55b-a6859f2d3d77::"Law IV references Sola Gratia and 2nd Law (external input)"::null::@Legal%%
%%tag::Risk::cc38519a-d856-4de9-b21a-13fe796883ba::"Risk that grace dynamics are shown to be inconsistent, defeating the law"::null::@Engineering%%
%%tag::Assumption::3080bae2-2114-4333-8b48-b2401411018a::"Assumption that the Grace Operator Definition (D9.1) is valid and sufficient as an anchor"::null::@Engineering%%
%%tag::Assumption::b3fdf270-7e4c-4b4b-a547-cd67528298a8::"Assumption that the formal expression is a valid representation of Grace Dynamics"::null::@Engineering%%
%%tag::Metric::d11c4049-c6ec-42dc-bb08-ae5fbe429737::"Registry position: 48 of 191"::null::@Operations%%
%%tag::Metric::681082f1-6e67-493b-924c-f3580191316f::"Chain Tier: 13 — Master Laws"::null::@Operations%%
%%tag::Stakeholder::c240f5e1-5897-431f-81c3-ebf115fc2c3c::"AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications as sources"::null::@Operations%%
%%tag::OpenQuestion::cc048dd0-ad66-4fd3-81c4-c5a2f61a3564::"Unresolved: What are the specific implications of the non-unitary transformation for the broader system?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
