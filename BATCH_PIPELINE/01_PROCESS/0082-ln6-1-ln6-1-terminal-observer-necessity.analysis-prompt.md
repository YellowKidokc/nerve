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
  "title": "Terminal Observer Necessity",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0082_LN6.1_LN6_1_Terminal_Observer_Necessity.md",
  "source_sha256": "72f6dbc44b589782950447a48e4adcce13a840c52539b5c8bfaff4d10ea8c6de",
  "captured_at": "2026-09-07T00:13:20.331345+00:00",
  "word_count": 347,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0082: LN6.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 LN6.1 \u2014 Terminal Observer Necessity",
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

# Canonical Sequential Record 0082: LN6.1

| Field | Value |
|---|---|
| Registry position | 82 of 191 |
| Canonical ID | `LN6.1` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `LN6_1_Terminal_Observer_Necessity.md` |
| Source SHA-256 | `6953d52b3db50d155c60765bc935ef5ab053ec9ae7e860a565189e50831d7890` |

## Sequence Navigation

Previous: [0081 - T6.1 - Von Neumann Chain Termination](0081_T6.1_T6_1_Von_Neumann_Chain_Termination.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0083 - A7.1 - Closure Requirement](0083_A7.1_A7_1_Closure_Requirement.md)

## Registered Dependency Links

[T6.1 - Von Neumann Chain Termination](0081_T6.1_T6_1_Von_Neumann_Chain_Termination.md)

## Canonical Source Text

---
id: "LN6.1"
legacy_id: "AX-055"
title: "Terminal Observer Necessity"
mode: "FW_EXTENDED"
atlas_type: "Claim"
status: "framework_commitment"
domain: "Ontological"
anchor: "T6.1"
depends_on: "T6.1"
date: "2026-08-17"
---

# 🔶 LN6.1 — Terminal Observer Necessity

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Claim` | **Status:** `framework_commitment`
**Domain:** Ontological | **Legacy ID:** `AX-055`

---

## Statement

Von Neumann chain termination requires a terminal observer — one that doesn't need further observation.

## Formal Expression

```
∃O_terminal: ¬∃O': Observes(O', O_terminal) required
```

$$
∃O_terminal: ¬∃O': Observes(O', O_terminal) required
$$

## Maximum Defensible Position

Framework claim: Von Neumann chain termination requires a terminal observer — one that doesn't need further observation.

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

**Anchor:** T6.1
**Depends On:** T6.1

## References

Von Neumann 1932; Wigner 1961

## Defeat Conditions

Show terminal observer is unnecessary.

## Limitations (from workbook)

The leap from 'chain terminates' to 'terminal observer' is contested.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::7583d79a-0c8b-460f-b11c-358bbcc0fb02::"Von Neumann chain termination must require a terminal observer"::null::@Engineering%%
%%tag::Requirement::3904f9cb-4003-400d-9bcf-62e9d8d7a46c::"Terminal observer must not require further observation"::null::@Engineering%%
%%tag::Dependency::cb7a11e2-0fa5-4536-9cb5-0c1d9752b326::"Depends on T6.1 - Von Neumann Chain Termination"::null::@Engineering%%
%%tag::Risk::60cebd8e-bf1a-4633-ba0a-63026eba7e98::"The leap from 'chain terminates' to 'terminal observer' is contested"::null::@Engineering%%
%%tag::OpenQuestion::206855c8-43a0-4055-ad44-ff145da0a5d2::"Whether terminal observer is truly necessary"::null::@Engineering%%
%%tag::Stakeholder::2666a12f-3593-4282-9661-1becfb167113::"Von Neumann (1932) - foundational reference"::null::@Engineering%%
%%tag::Stakeholder::cef5a32e-e10c-4512-88fb-8886fcad8906::"Wigner (1961) - foundational reference"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
