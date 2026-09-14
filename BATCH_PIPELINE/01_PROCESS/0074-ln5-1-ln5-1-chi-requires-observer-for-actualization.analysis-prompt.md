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
  "title": "Chi Requires Observer For Actualization",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0074_LN5.1_LN5_1_Chi_Requires_Observer_For_Actualization.md",
  "source_sha256": "0f131010b3f4584c323ab82313dff40dddb667d84e37e10ce399d0bc7332ae13",
  "captured_at": "2026-09-07T00:06:04.357137+00:00",
  "word_count": 353,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0074: LN5.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN5.1 \u2014 Chi Requires Observer For Actualization",
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

# Canonical Sequential Record 0074: LN5.1

| Field | Value |
|---|---|
| Registry position | 74 of 191 |
| Canonical ID | `LN5.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN5_1_Chi_Requires_Observer_For_Actualization.md` |
| Source SHA-256 | `6bd7513a740f4a66b559f4c79b257bcb4a48829610b6d9eea8a98a5a0c0c5c2a` |

## Sequence Navigation

Previous: [0073 - P5.2 - Observer Effect Proportional To Phi](0073_P5.2_P5_2_Observer_Effect_Proportional_To_Phi.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0075 - A6.1 - Superposition](0075_A6.1_A6_1_Superposition.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "LN5.1"
legacy_id: "AX-044"
title: "Chi Requires Observer For Actualization"
mode: "FW_EXTENDED"
atlas_type: "Logical Necessity"
status: "logical_necessity"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1"
date: "2026-08-17"
---

# 🔶 LN5.1 — Chi Requires Observer For Actualization

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Logical Necessity` | **Status:** `logical_necessity`
**Domain:** Ontological | **Legacy ID:** `AX-044`

---

## Statement

χ-field exists in superposition until observed — actualization requires observer.

## Formal Expression

```
Actual(χ) → ∃O: Observes(O, χ)
```

$$
Actual(χ) → ∃O: Observes(O, χ)
$$

## Maximum Defensible Position

Framework claim: χ-field exists in superposition until observed — actualization requires observer.

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

**Anchor:** A5.1
**Depends On:** A5.1

## References

Copenhagen; Wheeler; Von Neumann

## Defeat Conditions

Show χ self-actualizes.

## Limitations (from workbook)

Consistent with Copenhagen but goes further — applies to the substrate itself.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::42cfaab8-69dc-489b-85ae-22a5c288a833::"χ-field must exist in superposition until observed"::null::@Engineering%%
%%tag::Requirement::d86ca608-2c94-4f13-acd6-68bec165f82c::"Actualization of χ-field requires an observer"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::d3eba006-a541-4285-9f2a-84193198893c::"Risk of χ self-actualizing, defeating the necessity claim"::null::@Engineering%%
%%tag::Assumption::bf2a22eb-9b60-4bbe-9220-443bc419d4df::"Assumes Copenhagen interpretation and observer role in quantum actualization"::null::@Engineering%%
%%tag::Decision::03a0b53a-6ffa-4c56-9618-4b611e36fbdd::"Adopt FW_EXTENDED mode for this logical necessity"::null::@Engineering%%
%%tag::Decision::a1642b89-03aa-45c7-af69-d38f43f05b2d::"Classify as Logical Necessity in Ontological domain"::null::@Engineering%%
%%tag::Stakeholder::2e394285-aa73-4b34-a832-bd1e196dddc4::"Framework maintainers (AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, AX Truth Kernel)"::null::@Engineering%%
%%tag::OpenQuestion::c3a73518-6131-4997-88a0-1da198c614c4::"Whether χ-field can self-actualize without an observer"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
