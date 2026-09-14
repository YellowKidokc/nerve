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
  "title": "Coherence Conservation",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0064_P3.2_P3_2_Coherence_Conservation.md",
  "source_sha256": "3fc16ce0a6ab3a2e045983883c52df75b24a3560b643db64a36ef07e9285eb0d",
  "captured_at": "2026-09-06T23:59:55.350566+00:00",
  "word_count": 378,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0064: P3.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 P3.2 \u2014 Coherence Conservation",
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

# Canonical Sequential Record 0064: P3.2

| Field | Value |
|---|---|
| Registry position | 64 of 191 |
| Canonical ID | `P3.2` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `P3_2_Coherence_Conservation.md` |
| Source SHA-256 | `7c9a977d322d381d1f748c8c9c71b61a0057665cc3e690e3d0e5872548a1ef52` |

## Sequence Navigation

Previous: [0063 - P3.1 - Coherence Non-Negativity](0063_P3.1_P3_1_Coherence_Non-Negativity.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0065 - LN3.1 - Meaningful Configuration Necessity](0065_LN3.1_LN3_1_Meaningful_Configuration_Necessity.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md); [D3.1 - Coherence Functional Definition](0018_D3.1_D3_1_Coherence_Functional_Definition.md)

## Canonical Source Text

---
id: "P3.2"
legacy_id: "AX-024"
title: "Coherence Conservation"
mode: "FW_EXTENDED"
atlas_type: "Property"
status: "property"
domain: "Ontological"
anchor: "A2.2"
depends_on: "A2.2, D3.1"
date: "2026-08-17"
---

# 🔶 P3.2 — Coherence Conservation

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Property` | **Status:** `property`
**Domain:** Ontological | **Legacy ID:** `AX-024`

---

## Statement

Total coherence is conserved in closed systems — it redistributes but doesn't vanish.

## Formal Expression

```
d(C_total)/dt = 0 (closed system)
```

$$
d(C_total)/dt = 0 (closed system)
$$

## Maximum Defensible Position

Framework claim: Total coherence is conserved in closed systems — it redistributes but doesn't vanish.

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

**Anchor:** A2.2
**Depends On:** A2.2, D3.1

## References

Conservation laws; Noether's theorem

## Defeat Conditions

Show coherence destruction or creation from nothing.

## Limitations (from workbook)

Tension with T3.1 — coherence can't self-increase but is conserved? Resolution: it redistributes.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::5c2f0815-4c4b-42d8-819f-49c79498537b::"Total coherence must be conserved in closed systems"::null::@Engineering%%
%%tag::Requirement::04947657-6d9a-4dcc-bab8-a4c500b3c5e8::"Coherence must redistribute rather than vanish in closed systems"::null::@Engineering%%
%%tag::Decision::ff5c0325-7e89-4fad-8c94-15029b8341d2::"Adopt the principle of coherence conservation in closed systems"::null::@Engineering%%
%%tag::Risk::f4445b21-bea2-467e-a51a-5c7dc3296bbf::"Risk of coherence destruction or creation from nothing invalidating the conservation principle"::null::@Engineering%%
%%tag::Assumption::d0450a8e-33fe-4635-b3c6-dea04cedb2e8::"Assumes the system is closed for coherence conservation to hold"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 Self-Grounding"::null::@Engineering%%
%%tag::Dependency::57615bda-6e3b-482c-a1b6-64989354bcc5::"Depends on D3.1 Coherence Functional Definition"::null::@Engineering%%
%%tag::Metric::5633d669-bbf6-4131-9789-99fc6d0a65c4::"Total coherence (C_total) rate of change equals zero in closed systems"::null::@Engineering%%
%%tag::OpenQuestion::b22fe415-48cd-48ee-bb73-627224c232b1::"How to resolve the tension between P3.2 and T3.1 regarding coherence self-increase"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
