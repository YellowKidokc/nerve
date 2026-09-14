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
  "title": "Moral Coherence Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0037_D11.1_D11_1_Moral_Coherence_Definition.md",
  "source_sha256": "bf2fb4a31379a744ed842ef7ca88fac6332c23216413db32cd410235ad2c0610",
  "captured_at": "2026-09-06T23:36:55.711763+00:00",
  "word_count": 479,
  "tags": [],
  "source_outline": [
    "Canonical Sequential Record 0037: D11.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D11.1 \u2014 Moral Coherence Definition",
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

# Canonical Sequential Record 0037: D11.1

| Field | Value |
|---|---|
| Registry position | 37 of 191 |
| Canonical ID | `D11.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D11_1_Moral_Coherence_Definition.md` |
| Source SHA-256 | `012668a6a5c7aee213d2201745c307ea2dee1003eab24d5e5677c4d25f35dec2` |

## Sequence Navigation

Previous: [0036 - E10.1 - Soul Field Equation](0036_E10.1_E10_1_Soul_Field_Equation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0038 - D12.1 - Integration Attractor](0038_D12.1_D12_1_Integration_Attractor.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "D11.1"
legacy_id: "AX-090"
title: "Moral Coherence Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Moral"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 📐 D11.1 — Moral Coherence Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Moral | **Legacy ID:** `AX-090`

---

## Statement

Moral coherence defined — function of integrated information, sign, and behavior.

## Formal Expression

```
C_moral = f(Φ, σ, behavior_vector)
```

$$
C_moral = f(Φ, σ, behavior_vector)
$$

## Maximum Defensible Position

Definition: C_moral = moral coherence measure.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Moral |

## Dependency Chain

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 11 — Soul + Moral + Eschatology

## References

Virtue ethics (Aristotle); IIT; Moral psychology

## Defeat Conditions

Show morality without coherence structure. Demonstrate coherence without moral content.

## Limitations (from workbook)

The identification of morality with coherence is a bridge claim, not a necessary truth.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::9f1e4325-caf1-4bd1-a832-a35c96864480::"Moral coherence must be defined as a function of integrated information, sign, and behavior"::null::@Engineering%%
%%tag::Requirement::5dd60472-e384-45a7-aefe-82bf68639fe6::"Moral coherence must be formally expressed as C_moral = f(Φ, σ, behavior_vector)"::null::@Engineering%%
%%tag::Requirement::02adbca1-c5be-447f-8cb2-c9cf92d6d1ac::"Moral coherence must be defined as a measure (C_moral)"::null::@Engineering%%
%%tag::Decision::f9ede272-82de-45a0-94f4-5af3a32efe64::"Moral coherence is defined as a function of integrated information, sign, and behavior"::null::@Engineering%%
%%tag::Decision::8f809fbd-b5e4-48a0-8eb9-ba4fe67b60c5::"Moral coherence is formally expressed as C_moral = f(Φ, σ, behavior_vector)"::null::@Engineering%%
%%tag::Decision::bdfe8d20-8b73-4827-9241-e0c85c93604e::"Moral coherence is classified as a formal definition / primitive formalization"::null::@Engineering%%
%%tag::Dependency::6e6b9a78-2ce2-4ade-a315-f9fcb325248c::"D11.1 depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Dependency::cee082b0-dd7f-46fe-a8f8-b817b101d66a::"D11.1 is anchored to BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Risk::c625310d-a429-4868-a394-ea3dbab48369::"Risk: The identification of morality with coherence is a bridge claim, not a necessary truth"::null::@Engineering%%
%%tag::Risk::6fbbb6f9-190f-497e-8700-a1887b1e47d1::"Risk: Defeat condition - showing morality without coherence structure"::null::@Engineering%%
%%tag::Risk::05dd89ad-e3f7-4708-825d-20286a5a6eee::"Risk: Defeat condition - demonstrating coherence without moral content"::null::@Engineering%%
%%tag::Assumption::8330d3e3-2f31-4711-8f4f-cb7202a44e0f::"Assumption: Morality has a coherence structure"::null::@Engineering%%
%%tag::Assumption::51a8ebdf-5a06-4f24-9355-f0837a939765::"Assumption: Coherence can contain moral content"::null::@Engineering%%
%%tag::Stakeholder::3d58b952-94af-4ae7-94a2-eef302ea4133::"Stakeholder: Aristotle (virtue ethics) - referenced for moral coherence definition"::null::@Engineering%%
%%tag::Stakeholder::1f806f3b-b324-44ee-a322-8691775ce08c::"Stakeholder: Integrated Information Theory (IIT) - referenced for moral coherence definition"::null::@Engineering%%
%%tag::Stakeholder::1ec7f7ea-69a2-4b95-9ca3-fb7e60a7353e::"Stakeholder: Moral psychology - referenced for moral coherence definition"::null::@Engineering%%
%%tag::OpenQuestion::0dbe7039-6ab0-43b3-b9e3-c3ff7c2ee5d1::"Open question: Is the identification of morality with coherence a necessary truth?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
