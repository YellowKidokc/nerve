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
  "title": "Voluntary Coupling",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0012_BC8_BC8_Voluntary_Coupling.md",
  "source_sha256": "24ab2d50d7594351177f3c73192f65830ea0fb7f17c4b6c1ade830518c11fcb9",
  "captured_at": "2026-09-06T23:11:03.652304+00:00",
  "word_count": 386,
  "tags": [
    "information-theory"
  ],
  "source_outline": [
    "Canonical Sequential Record 0012: BC8",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd37 BC8 \u2014 Voluntary Coupling",
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

# Canonical Sequential Record 0012: BC8

| Field | Value |
|---|---|
| Registry position | 12 of 191 |
| Canonical ID | `BC8` |
| Mode | `AX_DERIVED` |
| Canonical source filename | `BC8_Voluntary_Coupling.md` |
| Source SHA-256 | `b70d64b969d6fcc2d83d8a3f2f5c622efb2d8d76494063ae8b73f54ebc9fcf1f` |

## Sequence Navigation

Previous: [0011 - BC7 - Information Conservation](0011_BC7_BC7_Information_Conservation.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0013 - D1.1 - Information Definition](0013_D1.1_D1_1_Information_Definition.md)

## Registered Dependency Links

[BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)

## Canonical Source Text

---
id: "BC8"
legacy_id: "AX-065"
title: "Voluntary Coupling"
mode: "AX_DERIVED"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "BC6 (Infinite Energy Source)"
depends_on: "BC6"
date: "2026-08-17"
---

# 🔷 BC8 — Voluntary Coupling

**Mode:** `AX_DERIVED` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-065`

---

## Statement

The coupling between observer and grace must be voluntary — forced coupling violates observer integrity.

## Formal Expression

```
Coupling(Observer, Grace) = Voluntary; ¬Forced
```

$$
Coupling(Observer, Grace) = Voluntary; ¬Forced
$$

## Maximum Defensible Position

Derived from BC6: Grace coupling must be voluntary.

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

**Anchor:** BC6 (Infinite Energy Source)
**Depends On:** BC6
**Chain Tier:** 8 — Boundary Conditions

## References

Sola Fide; Free will theology; Libertarian free will (Kane 1996)

## Defeat Conditions

Show forced coupling that preserves observer integrity. Demonstrate involuntary grace that doesn't destroy personhood.

## Limitations (from workbook)

The 'irresistible grace' debate (Calvinism vs Arminianism) is relevant here.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::63366b44-972f-4813-9d34-0ac3d339579c::"Coupling between observer and grace must be voluntary"::null::@Product%%
%%tag::Requirement::31cfe942-1af0-4a37-89a7-2e9b5cfe6880::"Forced coupling must be prohibited"::null::@Product%%
%%tag::Requirement::ddca1177-aaa1-4ab6-a957-91dae322b27b::"Observer integrity must be preserved"::null::@Product%%
%%tag::Decision::530cf5de-519c-4255-93e6-84639d9422bb::"Adopt voluntary coupling as a boundary condition"::null::@Product%%
%%tag::Dependency::8b129bda-94a0-4196-8689-182d6f5dd85d::"Depends on BC6 (Infinite Energy Source)"::null::@Engineering%%
%%tag::Risk::b60397af-c6ba-431b-b080-a42e96946246::"Forced coupling could violate observer integrity"::null::@Product%%
%%tag::Risk::e8106ddd-63b9-47f4-99b8-12276a5467d5::"Involuntary grace could destroy personhood"::null::@Product%%
%%tag::Assumption::042b2f8a-e599-41b6-aead-18cb2fc624cf::"Grace coupling must be voluntary"::null::@Product%%
%%tag::OpenQuestion::4e0830dd-1123-460e-9a7a-1e41576b774a::"Can forced coupling preserve observer integrity?"::null::@Product%%
%%tag::OpenQuestion::59fc33af-d654-41e5-996f-9de41a0c9527::"Can involuntary grace exist without destroying personhood?"::null::@Product%%
%%tag::Stakeholder::c36864f1-0de7-41af-b05b-55e23a2c5c98::"Observers (entities subject to coupling)"::null::@Product%%
%%tag::Stakeholder::e4db8c4e-d291-475e-804b-48dbb5ec5b51::"Theologians debating irresistible grace (Calvinists vs Arminians)"::null::@Product%%
%%--- END SEMANTIC TAGS ---%%
