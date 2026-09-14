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
  "title": "Law VI: Coherence Non-Increase",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0050_D19.6_D19_6_Law_VI_Coherence_Non-Increase.md",
  "source_sha256": "cd78360842f6c5892cbd9071ea59aaacb4494d3f5b5f9fc165a857e588fbb89a",
  "captured_at": "2026-09-06T23:48:19.709352+00:00",
  "word_count": 367,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0050: D19.6",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.6 \u2014 Law VI: Coherence Non-Increase",
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

# Canonical Sequential Record 0050: D19.6

| Field | Value |
|---|---|
| Registry position | 50 of 191 |
| Canonical ID | `D19.6` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_6_Law_VI_Coherence_Non-Increase.md` |
| Source SHA-256 | `c2dcb71ce222899c13d363f7f9707fa4ad5bf23768afd6ae0d2c35f84cb2b7b3` |

## Sequence Navigation

Previous: [0049 - D19.5 - Law V: Conservation Symmetry](0049_D19.5_D19_5_Law_V_Conservation_Symmetry.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0051 - D19.7 - Law VII: Actualization Requirement](0051_D19.7_D19_7_Law_VII_Actualization_Requirement.md)

## Registered Dependency Links

[T3.1 - Coherence Cannot Self-Increase](0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md)

## Canonical Source Text

---
id: "D19.6"
legacy_id: "AX-141"
title: "Law VI: Coherence Non-Increase"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "T3.1 (Coherence Cannot Self-Increase)"
depends_on: "depends on anchor: T3.1 (Coherence Cannot Self-Increase)"
date: "2026-08-17"
---

# 📐 D19.6 — Law VI: Coherence Non-Increase

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-141`

---

## Statement

Law VI: Closed systems cannot increase own coherence.

## Formal Expression

```
dC/dt ≤ 0 (closed system)
```

$$
\frac{dC}{dt} \le 0 (closed system)
$$

## Maximum Defensible Position

Definition: Law VI — Coherence Non-Increase.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Unified |

## Dependency Chain

**Anchor:** T3.1 (Coherence Cannot Self-Increase)
**Depends On:** depends on anchor: T3.1 (Coherence Cannot Self-Increase)
**Chain Tier:** 13 — Master Laws

## References

2nd Law (Clausius/Boltzmann)

## Defeat Conditions

Show closed system coherence increase.

## Limitations (from workbook)

Restatement of T3.1.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::2252e9c3-e015-4153-a428-12b27a84d3a5::"Closed systems cannot increase own coherence"::null::@Engineering%%
%%tag::Requirement::55cd1eb6-9c29-42ef-a23b-12dad44dc5b0::"Coherence change rate must be non-positive for closed systems (dC/dt ≤ 0)"::null::@Engineering%%
%%tag::Dependency::db39c1ab-b0f6-4433-a170-5c8ea1457a51::"Depends on anchor T3.1 (Coherence Cannot Self-Increase)"::null::@Engineering%%
%%tag::Dependency::8935b0cf-2d33-4025-a947-8181e292b402::"Depends on 2nd Law (Clausius/Boltzmann)"::null::@Engineering%%
%%tag::Risk::bbb5cada-7342-48cd-af7e-c56566cbe9b2::"Risk of showing closed system coherence increase (defeat condition)"::null::@Engineering%%
%%tag::Assumption::402b3016-55cf-4fdb-ae8b-e76f9861dd1e::"Assumes the system is closed"::null::@Engineering%%
%%tag::Decision::7d8da446-4f52-4743-ad6a-decaa0848335::"Adopted Law VI as a formal definition (not an empirical claim)"::null::@Engineering%%
%%tag::Decision::7b2b36be-754f-45e2-b881-e6dfa8a180dd::"Classified as restatement of T3.1"::null::@Engineering%%
%%tag::Stakeholder::dcccf6ff-042b-4050-9b7f-724997f3a347::"AX Truth Kernel classification system (defines epistemic status)"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
