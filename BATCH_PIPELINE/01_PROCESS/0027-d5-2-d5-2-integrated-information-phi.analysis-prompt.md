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
  "title": "Integrated Information Phi",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0027_D5.2_D5_2_Integrated_Information_Phi.md",
  "source_sha256": "515bac2e74592d9bb99bb0251403645bcd8dfec89b0bd5eac92e32ef6d43a160",
  "captured_at": "2026-09-06T23:26:25.499951+00:00",
  "word_count": 359,
  "tags": [
    "consciousness",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0027: D5.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D5.2 \u2014 Integrated Information Phi",
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

# Canonical Sequential Record 0027: D5.2

| Field | Value |
|---|---|
| Registry position | 27 of 191 |
| Canonical ID | `D5.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D5_2_Integrated_Information_Phi.md` |
| Source SHA-256 | `f792ea725aff6f1d5d3fb4d554b34aa1b058ae90004bac0ccf9c42538eda1c07` |

## Sequence Navigation

Previous: [0026 - D5.1 - Observer Definition](0026_D5.1_D5_1_Observer_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0028 - D5.3 - Witness Field Operator](0028_D5.3_D5_3_Witness_Field_Operator.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "D5.2"
legacy_id: "AX-038"
title: "Integrated Information Phi"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A5.1 (Observation Requirement)"
depends_on: "A5.1"
date: "2026-08-17"
---

# 📐 D5.2 — Integrated Information Phi

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-038`

---

## Statement

Integrated information Φ defined per IIT — measures consciousness degree.

## Formal Expression

```
Φ = integrated information (IIT measure)
```

$$
Φ = integrated information (IIT measure)
$$

## Maximum Defensible Position

Definition: Φ = integrated information.

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
**Depends On:** A5.1
**Chain Tier:** 6 — Observer System

## References

Tononi 2004; IIT 3.0; Oizumi et al. 2014

## Defeat Conditions

Show Φ doesn't measure consciousness. Demonstrate better consciousness measure.

## Limitations (from workbook)

Computing Φ is intractable for real systems. Proxy measures exist but approximate.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::3675a9f7-3479-4ba0-8c5f-574c2bbee3a3::"Φ must measure consciousness degree"::null::@Engineering,Product%%
%%tag::Decision::f28267de-0061-41f1-b181-79a756eaa165::"Adopt IIT definition of Φ for consciousness measurement"::null::@Engineering,Product%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::0e72d15c-856f-4277-a68a-554bcb4ffb84::"Computing Φ is intractable for real systems"::null::@Engineering%%
%%tag::Risk::f23ffaee-35c7-4588-ab30-327ebad14f33::"Proxy measures only approximate Φ"::null::@Engineering%%
%%tag::OpenQuestion::1f9bf9f5-fb5d-4a07-a433-31c89c63168a::"Show Φ doesn't measure consciousness"::null::@Engineering,Product%%
%%tag::OpenQuestion::e3ebfa7a-a411-4c98-961e-049b71c750a3::"Demonstrate better consciousness measure"::null::@Engineering,Product%%
%%tag::Metric::4b3d2d85-cd64-414f-9680-b730906bdd7c::"Φ = integrated information (IIT measure)"::null::@Engineering%%
%%tag::Stakeholder::9e3f6c71-7ae9-476e-8edb-160f244bec53::"IIT research community (Tononi, Oizumi et al.)"::null::@Engineering,Product%%
%%tag::Assumption::fdf97f63-2b04-4ec2-9b9e-3e8e4e63cbdf::"Φ is a valid measure of consciousness"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
