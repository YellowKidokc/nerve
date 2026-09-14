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
  "title": "Meaningful Configuration Necessity",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0065_LN3.1_LN3_1_Meaningful_Configuration_Necessity.md",
  "source_sha256": "e783a382bbbb5622d787368865dff818fc9e77b6297b32b1543a1289b23d62be",
  "captured_at": "2026-09-07T00:00:09.985040+00:00",
  "word_count": 301,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0065: LN3.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN3.1 \u2014 Meaningful Configuration Necessity",
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

# Canonical Sequential Record 0065: LN3.1

| Field | Value |
|---|---|
| Registry position | 65 of 191 |
| Canonical ID | `LN3.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN3_1_Meaningful_Configuration_Necessity.md` |
| Source SHA-256 | `7188ac7b3bf2471c1b789fa6c8ff5ca51a5ecb905c304a902a7f67e6dea14365` |

## Sequence Navigation

Previous: [0064 - P3.2 - Coherence Conservation](0064_P3.2_P3_2_Coherence_Conservation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0066 - A4.1 - Parsimony](0066_A4.1_A4_1_Parsimony.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "LN3.1"
legacy_id: "AX-026"
title: "Meaningful Configuration Necessity"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 LN3.1 — Meaningful Configuration Necessity

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-026`

---

## Statement

Not all configurations are meaningful — meaning requires coherent structure.

## Formal Expression

```
Meaningful(config) → Coherent(config)
```

$$
Meaningful(config) → Coherent(config)
$$

## Maximum Defensible Position

Framework claim: Not all configurations are meaningful — meaning requires coherent structure.

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

**Anchor:** A2.2
**Depends On:** A2.2

## References

Kolmogorov complexity; Shannon capacity

## Defeat Conditions

Show random configurations carry meaning.

## Limitations (from workbook)

Supported by information theory: random noise carries no message.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::5b90f24e-952a-4411-ad8b-5716937850e0::"Meaning requires coherent structure"::null::@Engineering,Product%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::f7d24c6c-ae40-414c-89f1-6e623cde6069::"Random configurations could carry meaning"::null::@Engineering,Product%%
%%tag::Assumption::66e3757c-75bd-440b-98b9-3cd0c4078211::"Random noise carries no message"::null::@Engineering%%
%%tag::Metric::54cc5188-f2cc-4141-97fc-5fd8d4d6a530::"Kolmogorov complexity reference"::null::@Engineering%%
%%tag::Metric::72429c4f-64b4-405d-9d5b-e26873b5f31e::"Shannon capacity reference"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
