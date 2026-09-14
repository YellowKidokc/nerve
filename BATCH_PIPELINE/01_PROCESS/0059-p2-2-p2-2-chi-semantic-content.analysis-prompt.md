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
  "title": "Chi Semantic Content",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0059_P2.2_P2_2_Chi_Semantic_Content.md",
  "source_sha256": "d6c1bcdf10823b49eace5840888199349fc5c44dc505fbcbb5103c10e214a9cc",
  "captured_at": "2026-09-06T23:56:03.185668+00:00",
  "word_count": 340,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0059: P2.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P2.2 \u2014 Chi Semantic Content",
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

# Canonical Sequential Record 0059: P2.2

| Field | Value |
|---|---|
| Registry position | 59 of 191 |
| Canonical ID | `P2.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P2_2_Chi_Semantic_Content.md` |
| Source SHA-256 | `da474743420cbe4b0232428aa44c9d68af17331ae4a72fe09c203d1ad96ee228` |

## Sequence Navigation

Previous: [0058 - P2.1 - Chi Ontological Priority](0058_P2.1_P2_1_Chi_Ontological_Priority.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0060 - LN2.1 - Information Anchor Necessity](0060_LN2.1_LN2_1_Information_Anchor_Necessity.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "P2.2"
legacy_id: "AX-014"
title: "Chi Semantic Content"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2"
date: "2026-08-17"
---

# 🔶 P2.2 — Chi Semantic Content

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-014`

---

## Statement

χ-field carries semantic content, not just syntactic information.

## Formal Expression

```
χ = semantic_information ⊃ syntactic_information
```

$$
χ = semantic_information ⊃ syntactic_information
$$

## Maximum Defensible Position

Framework claim: χ-field carries semantic content, not just syntactic information.

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

**Anchor:** A2.2
**Depends On:** A2.2

## References

Floridi 2011; Dretske 1981

## Defeat Conditions

Show information without semantic content at fundamental level.

## Limitations (from workbook)

Whether fundamental physics has 'meaning' is deeply contested.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::5edeea37-4ddc-4aba-a82a-338fdbf6550f::"χ-field must carry semantic content, not just syntactic information"::null::@Engineering%%
%%tag::Decision::5b3ba758-24cc-4347-9e5c-e985e2120085::"Adopt FW_EXTENDED mode for P2.2"::null::@Engineering%%
%%tag::Decision::8fabd1fe-435c-4b36-93ba-6bd7a3a5471e::"Classify P2.2 as a Property in the Ontological domain"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Risk::6650e7b9-6e48-4a8f-9e2b-698ac9db2b2d::"Fundamental physics may not have 'meaning', challenging the claim"::null::@Engineering%%
%%tag::Assumption::b5765bf9-8619-4f50-a734-f2cfb2593254::"Semantic information is a superset of syntactic information"::null::@Engineering%%
%%tag::OpenQuestion::63e92eff-d3c6-4d5a-a731-0118a2772c95::"Whether fundamental physics has 'meaning' is deeply contested"::null::@Engineering%%
%%tag::Stakeholder::3188f69d-4c14-4241-aa85-8a9906e2324a::"Floridi and Dretske as referenced authorities on semantic content"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
