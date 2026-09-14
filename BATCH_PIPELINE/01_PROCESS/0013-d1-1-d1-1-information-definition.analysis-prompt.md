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
  "title": "Information Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0013_D1.1_D1_1_Information_Definition.md",
  "source_sha256": "3c655539e3e28695e2d18378823279769c33ca2211bb146985ab93eae97af91d",
  "captured_at": "2026-09-06T23:11:50.253228+00:00",
  "word_count": 365,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0013: D1.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D1.1 \u2014 Information Definition",
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

# Canonical Sequential Record 0013: D1.1

| Field | Value |
|---|---|
| Registry position | 13 of 191 |
| Canonical ID | `D1.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D1_1_Information_Definition.md` |
| Source SHA-256 | `7daeed0ef4232a77bd68f030980ca07f80197d81c4f388a39ac0a817c92ee8b3` |

## Sequence Navigation

Previous: [0012 - BC8 - Voluntary Coupling](0012_BC8_BC8_Voluntary_Coupling.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0014 - D1.2 - Bit Definition](0014_D1.2_D1_2_Bit_Definition.md)

## Registered Dependency Links

[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "D1.1"
legacy_id: "AX-004"
title: "Information Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Primordial"
anchor: "A1.1, A1.2"
depends_on: "A1.1, A1.2"
date: "2026-08-17"
---

# 📐 D1.1 — Information Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Primordial | **Legacy ID:** `AX-004`

---

## Statement

Information is defined as resolved uncertainty — Shannon entropy.

## Formal Expression

```
I(x) = log₂(1/p(x)); H(X) = -Σp·log p
```

$$
I(x) = log₂(1/p(x)); H(X) = -Σp·log p
$$

## Maximum Defensible Position

Definition: Information = resolved uncertainty.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Primordial |

## Dependency Chain

**Anchor:** A1.1, A1.2
**Depends On:** A1.1, A1.2
**Chain Tier:** 1 — Primordial Foundation

## References

Shannon 1948

## Defeat Conditions

Show information is better defined another way. Demonstrate Shannon entropy is wrong measure.

## Limitations (from workbook)

Shannon information is syntactic, not semantic. Framework needs semantic information too.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::608a546e-e0d1-4c1e-9542-db2e3d2e625f::"Information must be defined as resolved uncertainty (Shannon entropy)"::null::@Engineering%%
%%tag::Requirement::574524a7-47a8-4e82-82e7-d8896b874ead::"The formal expression must use Shannon entropy formula"::null::@Engineering%%
%%tag::Decision::d5c32d45-4fb2-4b00-af2c-cf351883701b::"Adopt Shannon entropy as the definition of information"::null::@Engineering%%
%%tag::Dependency::407195e2-4751-411e-8740-390a792945d3::"Depends on A1.1 Existence and A1.2 Distinction"::null::@Engineering%%
%%tag::Risk::d5eb94c2-9d07-4233-9bf7-e4980b05ff66::"Shannon entropy may be the wrong measure of information"::null::@Engineering%%
%%tag::Risk::934d8c1e-643c-4e91-ae3d-42781107c2ea::"Shannon information is syntactic, not semantic, limiting framework"::null::@Engineering,Product%%
%%tag::OpenQuestion::bd5b2e68-d4bf-4c2b-90e3-95809f6ed4ca::"How to incorporate semantic information into the framework"::null::@Engineering,Product%%
%%tag::Metric::3e3c8198-81c1-420a-9441-af8312efe644::"Shannon entropy H(X) = -Σp·log p"::null::@Engineering%%
%%tag::Stakeholder::b334a870-efcc-47f1-a9ca-64d3c2319b5c::"Framework users needing semantic information"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
