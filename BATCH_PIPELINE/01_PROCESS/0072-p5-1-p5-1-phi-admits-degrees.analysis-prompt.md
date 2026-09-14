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
  "title": "Phi Admits Degrees",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0072_P5.1_P5_1_Phi_Admits_Degrees.md",
  "source_sha256": "83a16b0ef6d836970b8b30e780a669d66ce95498d218eb466776368e79fe4795",
  "captured_at": "2026-09-07T00:04:54.525460+00:00",
  "word_count": 352,
  "tags": [
    "consciousness",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0072: P5.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P5.1 \u2014 Phi Admits Degrees",
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

# Canonical Sequential Record 0072: P5.1

| Field | Value |
|---|---|
| Registry position | 72 of 191 |
| Canonical ID | `P5.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P5_1_Phi_Admits_Degrees.md` |
| Source SHA-256 | `d1527b57e4c39715bba3ea4a40b22d2a280ed99b8cfb13754db0cb7ba1aa971e` |

## Sequence Navigation

Previous: [0071 - A5.2 - Participatory Universe](0071_A5.2_A5_2_Participatory_Universe.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0073 - P5.2 - Observer Effect Proportional To Phi](0073_P5.2_P5_2_Observer_Effect_Proportional_To_Phi.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md); [A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "P5.1"
legacy_id: "AX-040"
title: "Phi Admits Degrees"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A5.1"
depends_on: "A5.1, D5.2"
date: "2026-08-17"
---

# 🔶 P5.1 — Phi Admits Degrees

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-040`

---

## Statement

Integrated information Φ admits degrees — consciousness is not all-or-nothing.

## Formal Expression

```
Φ ∈ ℝ⁺ (continuous, not binary)
```

$$
Φ ∈ ℝ⁺ (continuous, not binary)
$$

## Maximum Defensible Position

Framework claim: Integrated information Φ admits degrees — consciousness is not all-or-nothing.

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

**Anchor:** A5.1
**Depends On:** A5.1, D5.2

## References

IIT (Tononi 2004); Gradual consciousness theories

## Defeat Conditions

Show consciousness is binary.

## Limitations (from workbook)

Well-supported by IIT. Matches intuition that awareness has degrees.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::f707740c-6e26-4b04-9a9e-e4bc21df3094::"Integrated information Φ must admit degrees"::null::@Engineering%%
%%tag::Requirement::67481921-d24b-4c96-bcf9-14e2a33b4f23::"Consciousness must not be all-or-nothing"::null::@Engineering%%
%%tag::Decision::14f208c3-db68-43e2-8a48-47a7a279b6b1::"Adopt continuous Φ over binary"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 Integrated Information Phi"::null::@Engineering%%
%%tag::Risk::2bde55d6-6a46-4ba6-9261-44158233179b::"Defeat if consciousness is shown to be binary"::null::@Engineering%%
%%tag::Assumption::c798ad65-b559-41b1-bb06-fd6727e27e2d::"Assumes IIT supports gradual consciousness"::null::@Engineering%%
%%tag::Metric::3da9a1d2-ce75-44f8-aced-ef89596aab2f::"Φ ∈ ℝ⁺ (continuous, not binary)"::null::@Engineering%%
%%tag::Stakeholder::a947b615-d489-48aa-b246-14b499341a7f::"IIT researchers (Tononi 2004)"::null::@Engineering%%
%%tag::Stakeholder::5544fa5d-6a2e-498c-9097-9a6ec996b7b5::"Proponents of gradual consciousness theories"::null::@Engineering%%
%%tag::OpenQuestion::cdd947ae-0cd3-4386-8de5-9ce54b1ee564::"Is consciousness truly gradual?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
