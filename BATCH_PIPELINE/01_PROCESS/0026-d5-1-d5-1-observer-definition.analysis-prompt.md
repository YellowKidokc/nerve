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
  "title": "Observer Definition",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0026_D5.1_D5_1_Observer_Definition.md",
  "source_sha256": "81991962c17548cf916eb5eb0e4dfcb3380e9c0045ffe351f5eb55d5479d4876",
  "captured_at": "2026-09-06T23:25:28.455345+00:00",
  "word_count": 419,
  "tags": [
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0026: D5.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D5.1 \u2014 Observer Definition",
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

# Canonical Sequential Record 0026: D5.1

| Field | Value |
|---|---|
| Registry position | 26 of 191 |
| Canonical ID | `D5.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D5_1_Observer_Definition.md` |
| Source SHA-256 | `6549f0bb829ad4be51e267aad2b0ef487ad3b766b5650591d842df2b0e6f62ec` |

## Sequence Navigation

Previous: [0025 - E4.1 - Complexity Decrease Under Chi](0025_E4.1_E4_1_Complexity_Decrease_Under_Chi.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0027 - D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md)

## Registered Dependency Links

[A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md)

## Canonical Source Text

---
id: "D5.1"
legacy_id: "AX-037"
title: "Observer Definition"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A5.1 (Observation Requirement)"
depends_on: "A5.1"
date: "2026-08-17"
---

# 📐 D5.1 — Observer Definition

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-037`

---

## Statement

Observer defined as any system with integrated information Φ > 0.

## Formal Expression

```
O = system with Φ > 0 capable of making distinctions
```

$$
O = system with Φ > 0 capable of making distinctions
$$

## Maximum Defensible Position

Definition: Observer = system with Φ > 0.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Definition` |
| **Workbook Role** | `DEFINITION` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `definition` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1 (Observation Requirement)
**Depends On:** A5.1
**Chain Tier:** 6 — Observer System

## References

IIT (Tononi 2004); Von Neumann 1932

## Defeat Conditions

Show observers without Φ. Demonstrate Φ=0 systems that observe.

## Limitations (from workbook)

Broadest possible definition. Even thermostats have tiny Φ. Where is the observer boundary?

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::0d760d51-715c-4456-bc4b-192672af19c6::"Observer must be defined as any system with integrated information Φ > 0"::null::@Engineering%%
%%tag::Requirement::3cc796f0-1ae3-4609-8511-6f51558ae0fd::"Observer must be capable of making distinctions"::null::@Engineering%%
%%tag::Decision::563f5051-08bf-4fd9-aad6-d13a22ebd172::"Adopt broadest possible definition of observer (Φ > 0)"::null::@Engineering,Product%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Risk::28056fb1-1b4d-4341-9e27-e11c0f0b226c::"Definition too broad - even thermostats qualify as observers"::null::@Engineering%%
%%tag::Risk::e62ed131-8183-4793-967a-26c17c04569a::"Observer boundary unclear"::null::@Engineering%%
%%tag::OpenQuestion::7a140c75-31ed-4dc8-8a1f-bb0a15c6f97d::"Where is the observer boundary?"::null::@Engineering%%
%%tag::OpenQuestion::0af5350b-07f5-4275-b874-2581776d7380::"Can Φ=0 systems observe?"::null::@Engineering%%
%%tag::Assumption::0415e318-5e6a-4706-8339-dc1b96f546b7::"Integrated information Φ is a valid measure for observer status"::null::@Engineering%%
%%tag::Requirement::344b1d96-fd09-4638-9ed9-823bbbf4633c::"Definition must be defeated if observers without Φ are shown"::null::@Engineering%%
%%tag::Requirement::1b7c0640-a51e-4777-a259-e0f83c26e686::"Definition must be defeated if Φ=0 systems that observe are demonstrated"::null::@Engineering%%
%%tag::Stakeholder::a8c1eda6-a19d-4c41-a96b-8d9089082e4b::"IIT community (Tononi 2004) - cares about integrated information theory"::null::@Engineering%%
%%tag::Stakeholder::0c5e06c4-e3bb-4f0e-ac14-db77c2a7bf0f::"Von Neumann (1932) - cares about observer theory in physics"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
