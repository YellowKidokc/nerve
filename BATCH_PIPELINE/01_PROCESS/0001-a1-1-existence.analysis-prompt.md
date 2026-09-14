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
  "title": "Existence",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0001_A1_1_Existence.md",
  "source_sha256": "f80836ed4da87ae1d91382f73102320ce76195cd740c9540ad5e7ce9257d78df",
  "captured_at": "2026-09-06T22:58:18.050853+00:00",
  "word_count": 351,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0001: A1.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udfe2 A1.1 \u2014 Existence",
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

# Canonical Sequential Record 0001: A1.1

| Field | Value |
|---|---|
| Registry position | 1 of 191 |
| Canonical ID | `A1.1` |
| Mode | `AX_CORE` |
| Canonical source filename | `A1_1_Existence.md` |
| Source SHA-256 | `38fce636723205958d3d6c026708139c71d1c51c535fae76b5404a3d53d29a5e` |

## Sequence Navigation

Previous: _Start of sequence_  
Index: [All Canonical Records](FIS.html)  
Next: [0002 - A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md)

## Registered Dependency Links

_No registered canonical dependency link extracted._

## Canonical Source Text

---
id: "A1.1"
legacy_id: "AX-001"
title: "Existence"
mode: "AX_CORE"
atlas_type: "Axiom"
status: "primitive"
domain: "Primordial"
anchor: "A1.1 (Existence) — ROOT"
depends_on: "∅ (foundational)"
date: "2026-08-17"
---

# 🟢 A1.1 — Existence

**Mode:** `AX_CORE` | **Atlas Type:** `Axiom` | **Status:** `primitive`
**Domain:** Primordial | **Legacy ID:** `AX-001`

---

## Statement

Something exists rather than nothing.

## Formal Expression

```
∃x (existential quantifier — irreducible)
```

$$
∃x (existential quantifier — irreducible)
$$

## Maximum Defensible Position

At least one thing exists; equivalently, the chosen domain of discourse is nonempty.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_CORE` |
| **Atlas Object Type** | `Axiom` |
| **Workbook Role** | `AXIOM` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `primitive` |
| **Domain** | Primordial |

## Dependency Chain

**Anchor:** A1.1 (Existence) — ROOT
**Depends On:** ∅ (foundational)
**Chain Tier:** 0 — Primordial Foundation

## References

Leibniz 1714; Heidegger 1927

## Defeat Conditions

Show absolute nothingness is coherent and stable.

## Limitations (from workbook)

Strongest axiom. Near-universally accepted. Only challenge: radical Buddhist śūnyatā reading.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::8a268bf7-741f-41ca-b754-2863da5ea30e::"Something must exist rather than nothing"::null::@Product%%
%%tag::Requirement::7cb0b902-5026-4d80-b16d-3c99c5e03b20::"The domain of discourse must be nonempty"::null::@Engineering%%
%%tag::Decision::75f9a34a-c4f3-4d36-8d3b-56d30f6ebe95::"Adopt existence as a primitive axiom"::null::@Product%%
%%tag::Decision::23342923-a781-4cc9-bfcc-04ff196e9375::"Use existential quantifier as irreducible formal expression"::null::@Engineering%%
%%tag::Risk::bbee365a-984c-480d-b58b-60c71d08a425::"Challenge from radical Buddhist śūnyatā reading could undermine axiom"::null::@Product%%
%%tag::Assumption::db5d8351-74ca-4c61-9c15-b0b7a65e0f3a::"Absolute nothingness is not coherent or stable"::null::@Product%%
%%tag::Dependency::51fd30a1-7a1f-4f7d-9c5a-81a12c1d7656::"Depends on foundational empty set (∅)"::null::@Engineering%%
%%tag::Stakeholder::3ea48a6b-0fd9-438a-995b-1b98429105d3::"Leibniz and Heidegger as referenced philosophical authorities"::null::@Product%%
%%tag::OpenQuestion::8d16ca4f-3a36-4f25-958c-3bb802109200::"Is absolute nothingness coherent and stable?"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
