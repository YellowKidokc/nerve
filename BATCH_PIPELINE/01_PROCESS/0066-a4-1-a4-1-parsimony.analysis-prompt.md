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
  "title": "Parsimony",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0066_A4.1_A4_1_Parsimony.md",
  "source_sha256": "9cd17db621f0b7f7df81f4f978a1b260eb5b27c13ea991dc61743757e03e1beb",
  "captured_at": "2026-09-07T00:00:57.533897+00:00",
  "word_count": 322,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0066: A4.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A4.1 \u2014 Parsimony",
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

# Canonical Sequential Record 0066: A4.1

| Field | Value |
|---|---|
| Registry position | 66 of 191 |
| Canonical ID | `A4.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A4_1_Parsimony.md` |
| Source SHA-256 | `d0cb6df7060fb65fc4f23fea8eb52f0400519e9bee9a14cc3e78e410827d7cf4` |

## Sequence Navigation

Previous: [0065 - LN3.1 - Meaningful Configuration Necessity](0065_LN3.1_LN3_1_Meaningful_Configuration_Necessity.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0067 - A4.2 - Algorithmic Depth](0067_A4.2_A4_2_Algorithmic_Depth.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "A4.1"
legacy_id: "AX-027"
title: "Parsimony"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 A4.1 — Parsimony

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-027`

---

## Statement

Nature prefers simpler descriptions — Occam's razor formalized.

## Formal Expression

```
P(theory) ∝ 2^(-K(theory)) — Solomonoff prior
```

$$
P(theory) ∝ 2^(-K(theory)) — Solomonoff prior
$$

## Maximum Defensible Position

Framework claim: Nature prefers simpler descriptions — Occam's razor formalized.

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

**Anchor:** A2.2
**Depends On:** A2.2

## References

Occam's Razor; Solomonoff 1964

## Defeat Conditions

Show nature prefers complex descriptions.

## Limitations (from workbook)

Solomonoff's universal prior formalizes this. Well-established in AI and philosophy of science.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::e06911c6-6e34-43e8-95c0-9223d73849b9::"Nature must prefer simpler descriptions"::null::@Engineering,Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Decision::cea17287-9f5d-4625-a544-79d869b532ce::"Adopt Solomonoff prior as formalization of parsimony"::null::@Engineering%%
%%tag::Risk::b3c3603d-37c9-4e60-a1a1-08e476e45d0c::"Risk that nature prefers complex descriptions, defeating the claim"::null::@Engineering,Product%%
%%tag::Assumption::f5fc148d-b150-418c-bb87-6b2e37081782::"Assumes Solomonoff's universal prior is well-established and applicable"::null::@Engineering%%
%%tag::Metric::6bac4129-37b8-4398-ace9-c08a3708a7b9::"P(theory) ∝ 2^(-K(theory)) - Solomonoff prior"::null::@Engineering%%
%%tag::Stakeholder::c05fdff1-496d-4cfd-bfff-47590a3bf490::"AI and philosophy of science communities"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
