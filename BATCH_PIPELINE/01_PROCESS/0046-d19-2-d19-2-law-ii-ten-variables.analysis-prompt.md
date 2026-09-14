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
  "title": "Law II: Ten Variables",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0046_D19.2_D19_2_Law_II_Ten_Variables.md",
  "source_sha256": "450fd496321e79976c92917459869f3eff420b6975dda9b48fc294e57e069041",
  "captured_at": "2026-09-06T23:44:50.343462+00:00",
  "word_count": 388,
  "tags": [
    "information-theory",
    "lean4"
  ],
  "source_outline": [
    "Canonical Sequential Record 0046: D19.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D19.2 \u2014 Law II: Ten Variables",
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

# Canonical Sequential Record 0046: D19.2

| Field | Value |
|---|---|
| Registry position | 46 of 191 |
| Canonical ID | `D19.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D19_2_Law_II_Ten_Variables.md` |
| Source SHA-256 | `cdf707cc1a40c1e055031fae6d00cba9fcdd1a8d7e05841b9c0b6d87d55e891c` |

## Sequence Navigation

Previous: [0045 - D19.1 - Law I: Logos-Lagrangian](0045_D19.1_D19_1_Law_I_Logos-Lagrangian.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0047 - D19.3 - Law III: Entropy-Sin](0047_D19.3_D19_3_Law_III_Entropy-Sin.md)

## Registered Dependency Links

[E2.1 - Master Equation First Form](0017_E2.1_E2_1_Master_Equation_First_Form.md)

## Canonical Source Text

---
id: "D19.2"
legacy_id: "AX-137"
title: "Law II: Ten Variables"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Unified"
anchor: "E2.1 (Master Equation First Form)"
depends_on: "depends on anchor: E2.1 (Master Equation First Form)"
date: "2026-08-17"
---

# 📐 D19.2 — Law II: Ten Variables

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Unified | **Legacy ID:** `AX-137`

---

## Statement

Law II: The Ten Variables.

## Formal Expression

```
{G, M, E, S, T, K, R, Q, F, C} = complete variable set
```

$$
{G, M, E, S, T, K, R, Q, F, C} = complete variable set
$$

## Maximum Defensible Position

Definition: Law II — Ten Variables.

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

**Anchor:** E2.1 (Master Equation First Form)
**Depends On:** depends on anchor: E2.1 (Master Equation First Form)
**Chain Tier:** 13 — Master Laws

## References

Shannon; IIT; GR; QM; Moral theory

## Defeat Conditions

Show fewer or more variables are needed.

## Limitations (from workbook)

Completeness proof needed.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::4ae566ea-487b-406a-a78a-63fb7bb66320::"Law II must define the complete variable set of ten variables"::null::@Engineering%%
%%tag::Requirement::377eb0ee-f11a-4420-8b76-5d77ea797b3a::"The variable set must be complete"::null::@Engineering%%
%%tag::Dependency::b2391b21-a152-4df8-b4c4-13488139cdab::"Depends on anchor E2.1 (Master Equation First Form)"::null::@Engineering%%
%%tag::Risk::9c0f4488-8ff6-4283-ad26-b8876887c952::"Completeness proof needed for the variable set"::null::@Engineering%%
%%tag::Risk::17e4cd66-c99b-4d51-a152-e006da6e8c2d::"Defeat condition: showing fewer or more variables are needed"::null::@Engineering%%
%%tag::OpenQuestion::d50dc5a8-8050-4f61-8a26-1f503575887d::"Is the ten-variable set complete?"::null::@Engineering%%
%%tag::Decision::1be7cf21-1167-45db-8920-a95e860601d7::"Adopt the ten-variable set as the complete variable set"::null::@Engineering%%
%%tag::Assumption::789c4737-c746-42d4-ad23-df9b95773e32::"The ten variables are sufficient and necessary for the unified domain"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
