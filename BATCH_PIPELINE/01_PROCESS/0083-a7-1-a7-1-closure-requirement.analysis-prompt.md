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
  "title": "Closure Requirement",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0083_A7.1_A7_1_Closure_Requirement.md",
  "source_sha256": "25e01451ddb6b59523001af9311332815c0970f8ed891b7c30c9abfe826e7974",
  "captured_at": "2026-09-07T00:14:13.057762+00:00",
  "word_count": 348,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0083: A7.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 A7.1 \u2014 Closure Requirement",
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

# Canonical Sequential Record 0083: A7.1

| Field | Value |
|---|---|
| Registry position | 83 of 191 |
| Canonical ID | `A7.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `A7_1_Closure_Requirement.md` |
| Source SHA-256 | `18c9abb03f62360793ff6fa04d3313aaf761c7c95c80060f268d7c715c79f7ef` |

## Sequence Navigation

Previous: [0082 - LN6.1 - Terminal Observer Necessity](0082_LN6.1_LN6_1_Terminal_Observer_Necessity.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0084 - A7.2 - Uniqueness From Boundary Conditions](0084_A7.2_A7_2_Uniqueness_From_Boundary_Conditions.md)

## Registered Dependency Links

[LN6.1 - Terminal Observer Necessity](0082_LN6.1_LN6_1_Terminal_Observer_Necessity.md)

## Canonical Source Text

---
id: "A7.1"
legacy_id: "AX-056"
title: "Closure Requirement"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "LN6.1"
depends_on: "LN6.1"
date: "2026-08-17"
---

# 🔶 A7.1 — Closure Requirement

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-056`

---

## Statement

Physical theory requires boundary conditions for closure — underdetermined without them.

## Formal Expression

```
Theory + BCs → unique solution
```

$$
Theory + BCs → unique solution
$$

## Maximum Defensible Position

Framework claim: Physical theory requires boundary conditions for closure — underdetermined without them.

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

**Anchor:** LN6.1
**Depends On:** LN6.1

## References

PDE theory; Dirichlet/Neumann conditions

## Defeat Conditions

Show physical theory without BCs.

## Limitations (from workbook)

Standard in physics. The application to theology is the novel part.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::52f05828-fe1b-46d8-b4f2-6110c85a2933::"Physical theory must include boundary conditions for closure"::null::@Engineering%%
%%tag::Requirement::5ca47dac-0789-44d1-9cab-56fd00ee7ba0::"Physical theory must be underdetermined without boundary conditions"::null::@Engineering%%
%%tag::Decision::8f9d68f8-3f02-4c94-85eb-f4df1c478bce::"Adopt framework commitment that physical theory requires boundary conditions for closure"::null::@Engineering%%
%%tag::Dependency::063388f8-3f1c-412f-b8ee-7c951bc5a434::"Depends on LN6.1 Terminal Observer Necessity"::null::@Engineering%%
%%tag::Risk::7deb331d-73d9-423c-ab41-a68a985ae1c6::"Risk of showing physical theory without boundary conditions"::null::@Engineering%%
%%tag::Assumption::b2108f2a-190d-44b4-be4f-f164eed1895a::"Assumed that standard PDE theory with Dirichlet/Neumann conditions applies"::null::@Engineering%%
%%tag::OpenQuestion::541355e7-771b-4763-a105-20606d4782d2::"Application of closure requirement to theology remains novel and unresolved"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
