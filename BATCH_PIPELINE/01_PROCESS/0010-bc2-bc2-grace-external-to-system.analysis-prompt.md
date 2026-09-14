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
  "title": "Grace External To System",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0010_BC2_BC2_Grace_External_To_System.md",
  "source_sha256": "0ef7596bc44897cbc6c3ea131c28b26bd43dc274ae457cc34015e0b7e3b9cc2b",
  "captured_at": "2026-09-06T23:08:40.264320+00:00",
  "word_count": 372,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0010: BC2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd37 BC2 \u2014 Grace External To System",
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

# Canonical Sequential Record 0010: BC2

| Field | Value |
|---|---|
| Registry position | 10 of 191 |
| Canonical ID | `BC2` |
| Mode | `AX_DERIVED` |
| Canonical source filename | `BC2_Grace_External_To_System.md` |
| Source SHA-256 | `e017474576a5e487a5b71b66387f4dcb2dce6a8e7e452055a13b1f6364e7d8e4` |

## Sequence Navigation

Previous: [0009 - T3.1 - Coherence Cannot Self-Increase](0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0011 - BC7 - Information Conservation](0011_BC7_BC7_Information_Conservation.md)

## Registered Dependency Links

[T3.1 - Coherence Cannot Self-Increase](0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md)

## Canonical Source Text

---
id: "BC2"
legacy_id: "AX-059"
title: "Grace External To System"
mode: "AX_DERIVED"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "T3.1 (Coherence Cannot Self-Increase)"
depends_on: "T3.1"
date: "2026-08-17"
---

# 🔷 BC2 — Grace External To System

**Mode:** `AX_DERIVED` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-059`

---

## Statement

Grace must be external to the system — coherence recovery requires external source.

## Formal Expression

```
Grace ∉ System; Grace ∈ External
```

$$
Grace ∉ System; Grace ∈ External
$$

## Maximum Defensible Position

Derived: Since coherence can't self-increase, restoration requires external input.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_DERIVED` |
| **Atlas Object Type** | `Boundary Condition` |
| **Workbook Role** | `BOUNDARY_CONDITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `boundary` |
| **Domain** | Divine |

## Dependency Chain

**Anchor:** T3.1 (Coherence Cannot Self-Increase)
**Depends On:** T3.1
**Chain Tier:** 8 — Boundary Conditions

## References

Sola Gratia; Open systems thermodynamics

## Defeat Conditions

Show internal coherence recovery. Demonstrate self-bootstrapping moral improvement.

## Limitations (from workbook)

Derived from T3.1. If the informational 2nd Law holds, BC2 follows necessarily.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::58ee0057-3eed-4d5d-be0d-7c0a09d86259::"Grace must be external to the system"::null::@Engineering,Product%%
%%tag::Requirement::a728c95e-e974-4d57-b780-1bd9ad230b14::"Coherence recovery requires external source"::null::@Engineering%%
%%tag::Dependency::4876c467-b049-492d-a7e4-124b2cdf5787::"Depends on T3.1 (Coherence Cannot Self-Increase)"::null::@Engineering%%
%%tag::Assumption::d7b45d6c-93af-405a-89d0-b79c90d88ffa::"Coherence cannot self-increase"::null::@Engineering%%
%%tag::Assumption::d823822e-dcd6-43d1-8010-cdb773e1b1e7::"Informational 2nd Law holds"::null::@Engineering%%
%%tag::Risk::768b1c6a-5cb6-44ab-918a-66a1a4b4f3ed::"Internal coherence recovery demonstrated"::null::@Engineering%%
%%tag::Risk::9f516263-11b8-4510-b20c-718a130e5961::"Self-bootstrapping moral improvement demonstrated"::null::@Engineering,Product%%
%%tag::Decision::200b463e-77d4-44c1-9233-1de5cbeffa89::"Grace classified as external to system"::null::@Engineering,Product%%
%%tag::Metric::818bc643-ec0c-480b-be59-40ae8788813a::"Registry position 10 of 191"::null::@Operations%%
%%tag::Metric::d702aa8c-a9c9-4ceb-bfcd-7f54fb673363::"Chain Tier 8"::null::@Operations%%
%%tag::Stakeholder::39a2e6ab-39f7-4574-a1c6-94bb230b44b6::"Divine domain"::null::@Product%%
%%tag::OpenQuestion::eb09171b-bd83-4a78-8ba8-481e8d0b2c49::"How to demonstrate internal coherence recovery"::null::@Engineering%%
%%tag::OpenQuestion::1aae1d8e-307b-45c5-bd41-42b93c4625ba::"How to demonstrate self-bootstrapping moral improvement"::null::@Engineering,Product%%
%%--- END SEMANTIC TAGS ---%%
