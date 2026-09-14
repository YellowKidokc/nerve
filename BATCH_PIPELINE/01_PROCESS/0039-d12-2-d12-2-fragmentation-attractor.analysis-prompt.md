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
  "title": "Fragmentation Attractor",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0039_D12.2_D12_2_Fragmentation_Attractor.md",
  "source_sha256": "48c9284f8403316e2773bdf9dc006f1899af1aa3ba233420f5fa3a34af254bfa",
  "captured_at": "2026-09-06T23:38:42.986074+00:00",
  "word_count": 393,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0039: D12.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D12.2 \u2014 Fragmentation Attractor",
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

# Canonical Sequential Record 0039: D12.2

| Field | Value |
|---|---|
| Registry position | 39 of 191 |
| Canonical ID | `D12.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D12_2_Fragmentation_Attractor.md` |
| Source SHA-256 | `c26379d8100b098aca268d83e09b9a495e950ab6b772ae14d168d77901777648` |

## Sequence Navigation

Previous: [0038 - D12.1 - Integration Attractor](0038_D12.1_D12_1_Integration_Attractor.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0040 - E12.1 - Destiny Equation](0040_E12.1_E12_1_Destiny_Equation.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D12.2"
legacy_id: "AX-096"
title: "Fragmentation Attractor"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Eschatology"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D12.2 — Fragmentation Attractor

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Eschatology | **Legacy ID:** `AX-096`

---

## Statement

Fragmentation attractor defined — the low-coherence fixed point (hell).

## Formal Expression

```
A_- = lim(t→∞) C(t) → 0 — low-Φ fixed point
```

$$
A_- = lim(t→∞) C(t) → 0 — low-Φ fixed point
$$

## Maximum Defensible Position

Definition: A_- = fragmentation attractor.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Eschatology |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 11 — Soul + Moral + Eschatology

## References

Dynamical systems; Heat death; Maximum entropy

## Defeat Conditions

Show fragmentation isn't stable. Demonstrate spontaneous coherence recovery from A_-.

## Limitations (from workbook)

If T3.1 holds and no grace is applied, fragmentation is the natural endpoint.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::235f4559-a566-41e6-ab3a-d6e3c96b928c::"Fragmentation attractor must be defined as the low-coherence fixed point (hell)"::null::@Engineering%%
%%tag::Requirement::8bcc5dfd-5058-4830-b75a-dcf02add5e2c::"Formal expression A_- = lim(t→∞) C(t) → 0 must be maintained"::null::@Engineering%%
%%tag::Decision::891aba5b-2986-4d45-ab74-8a9efc1be733::"Adopt A_- as the fragmentation attractor definition"::null::@Engineering,Product%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Risk::55ea34f5-ab10-4625-96f7-38f4c71a80ea::"Fragmentation attractor may not be stable if defeat conditions are met"::null::@Engineering%%
%%tag::Assumption::6387a908-837a-4bcd-9def-0043738722bf::"T3.1 holds and no grace is applied, making fragmentation the natural endpoint"::null::@Engineering,Product%%
%%tag::OpenQuestion::3458a726-413d-4f08-8139-b5188fc08c60::"Can fragmentation be shown to be unstable?"::null::@Engineering%%
%%tag::OpenQuestion::728e997a-91a1-449b-97e7-c0f30368a3a4::"Can spontaneous coherence recovery from A_- be demonstrated?"::null::@Engineering%%
%%tag::Stakeholder::48847e16-8755-4a10-ac75-824fc68d9691::"AXIOM_CHAIN_MASTER_v2 classification system"::null::@Engineering,Operations%%
%%tag::Stakeholder::6b971006-c0b7-4ec6-ac4d-c2242adc7adc::"AXIOMS_PART1_MODE_CLASSIFICATION system"::null::@Engineering,Operations%%
%%tag::Stakeholder::19d59902-901f-498d-8bdc-07c66a643fc9::"AX Truth Kernel classification system"::null::@Engineering,Operations%%
%%--- END SEMANTIC TAGS ---%%
