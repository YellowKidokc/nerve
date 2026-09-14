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
  "title": "It From Bit",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0057_LN1.2_LN1_2_It_From_Bit.md",
  "source_sha256": "451447bf2da91f2c97b5ac99381185fb505a606c489692f0b37d34d6e970437d",
  "captured_at": "2026-09-06T23:54:25.878454+00:00",
  "word_count": 353,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0057: LN1.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN1.2 \u2014 It From Bit",
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

# Canonical Sequential Record 0057: LN1.2

| Field | Value |
|---|---|
| Registry position | 57 of 191 |
| Canonical ID | `LN1.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN1_2_It_From_Bit.md` |
| Source SHA-256 | `92a7c113a7b1e9c9296246b38eba9f09b8d7154265c342e2227ccad0f11d1151` |

## Sequence Navigation

Previous: [0056 - LN1.1 - Matter-Energy Derivative](0056_LN1.1_LN1_1_Matter-Energy_Derivative.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0058 - P2.1 - Chi Ontological Priority](0058_P2.1_P2_1_Chi_Ontological_Priority.md)

## Registered Dependency Links

[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "LN1.2"
legacy_id: "AX-007"
title: "It From Bit"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A1.1, A1.2"
depends_on: "A1.1, A1.2"
date: "2026-08-17"
---

# 🔶 LN1.2 — It From Bit

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-007`

---

## Statement

Physical reality emerges from informational processes.

## Formal Expression

```
Physics ← Information (ontological priority)
```

$$
Physics ← Information (ontological priority)
$$

## Maximum Defensible Position

Framework claim: Physical reality emerges from informational processes.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Logical Necessity` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `logical_necessity` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A1.1, A1.2
**Depends On:** A1.1, A1.2

## References

Wheeler 1990; Digital physics (Zuse, Wolfram)

## Defeat Conditions

Show physical processes without informational substrate.

## Limitations (from workbook)

Wheeler's slogan. Whether it's literally true is debated.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::02647a4f-be4d-410c-a507-6ae3751d0a80::"Physical reality must emerge from informational processes"::null::@Engineering,Product%%
%%tag::Decision::fe44ab08-9d97-4b9c-aaf4-3b342b6a6af8::"Adopt 'It From Bit' as a logical necessity in FW_EXTENDED mode"::null::@Product%%
%%tag::Dependency::407195e2-4751-411e-8740-390a792945d3::"Depends on A1.1 Existence and A1.2 Distinction"::null::@Engineering%%
%%tag::Risk::a849420b-56a0-4e74-8d51-ed41989208dd::"Debate over literal truth of Wheeler's slogan could undermine the claim"::null::@Product%%
%%tag::Assumption::c3dca7fc-234f-41be-868a-0df3fd47e9e7::"Assumes informational processes are ontologically prior to physical reality"::null::@Engineering,Product%%
%%tag::OpenQuestion::3b255de0-9546-423e-ac03-c0c943a4db24::"Whether physical processes can exist without informational substrate"::null::@Engineering,Product%%
%%tag::Stakeholder::b8227d83-7e70-4c2d-9e44-ee00d0a26aea::"Wheeler (referenced as originator of the concept)"::null::@Product%%
%%tag::Stakeholder::db019eff-f956-4db2-82f2-51b9ca517fac::"Digital physics community (Zuse, Wolfram) referenced as supporting tradition"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
