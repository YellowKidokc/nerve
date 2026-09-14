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
  "title": "Coherence Cannot Self-Increase",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md",
  "source_sha256": "64d71b9f9f5a360d19b6ab7a0aac04b54283bb403b2cf0b001bb29c3187e461c",
  "captured_at": "2026-09-06T23:05:51.641844+00:00",
  "word_count": 369,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0009: T3.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd37 T3.1 \u2014 Coherence Cannot Self-Increase",
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

# Canonical Sequential Record 0009: T3.1

| Field | Value |
|---|---|
| Registry position | 9 of 191 |
| Canonical ID | `T3.1` |
| Mode | `AX_DERIVED` |
| Canonical source filename | `T3_1_Coherence_Cannot_Self-Increase.md` |
| Source SHA-256 | `a8ab20258d81a80ef9c18053240b55d1f6fdf837c69aa38983cc9aa3f18d51d4` |

## Sequence Navigation

Previous: [0008 - A1.3 - Information Primacy](0008_A1.3_A1_3_Information_Primacy.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0010 - BC2 - Grace External To System](0010_BC2_BC2_Grace_External_To_System.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Canonical Source Text

---
id: "T3.1"
legacy_id: "AX-025"
title: "Coherence Cannot Self-Increase"
mode: "AX_DERIVED"
atlas_type: "Theorem"
status: "theorem"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2, D3.1"
date: "2026-08-17"
---

# 🔷 T3.1 — Coherence Cannot Self-Increase

**Mode:** `AX_DERIVED` | **Atlas Type:** `Theorem` | **Status:** `theorem`
**Domain:** Ontological | **Legacy ID:** `AX-025`

---

## Statement

A closed system cannot increase its own coherence — informational Second Law.

## Formal Expression

```
dC/dt ≤ 0 (closed system, no external input)
```

$$
\frac{dC}{dt} \le 0 (closed system, no external input)
$$

## Maximum Defensible Position

Derived theorem: Closed systems cannot self-increase coherence.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_DERIVED` |
| **Atlas Object Type** | `Theorem` |
| **Workbook Role** | `THEOREM` |
| **Mathematical Use** | Formal derivation candidate |
| **Epistemic Status** | `theorem` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2, D3.1
**Chain Tier:** 4 — Coherence System

## References

2nd Law (Clausius/Boltzmann); Landauer's principle

## Defeat Conditions

Show closed system coherence increase. Find a perpetual motion machine of the second kind.

## Limitations (from workbook)

THE load-bearing theorem. If this falls, grace becomes optional. The entire sign-flip argument depends on it.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::20f34173-e465-4f05-a884-0ebaad3e3dbf::"Closed system must not increase its own coherence"::null::@Engineering%%
%%tag::Decision::4c2cc3ce-08fc-479d-b168-6d7cff941e48::"Adopt theorem T3.1 as load-bearing for sign-flip argument"::null::@Product%%
%%tag::Risk::8e9e9a01-d3fd-436d-94c5-30103e5fae54::"If theorem falls, grace becomes optional and sign-flip argument fails"::null::@Product%%
%%tag::Dependency::5be62e35-a353-43d1-a656-a552d31eaf73::"Depends on A2.2 Self-Grounding and D3.1 Coherence Functional Definition"::null::@Engineering%%
%%tag::Metric::fb23051f-9b02-43d4-89a2-790f7d0624f4::"Coherence change rate dC/dt ≤ 0 for closed system"::null::@Engineering%%
%%tag::OpenQuestion::d41bf29b-e958-422d-b2eb-1e1de12e0d95::"Can a closed system ever show coherence increase?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
