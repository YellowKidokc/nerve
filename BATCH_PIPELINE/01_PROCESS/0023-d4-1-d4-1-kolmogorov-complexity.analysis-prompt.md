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
  "title": "Kolmogorov Complexity",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0023_D4.1_D4_1_Kolmogorov_Complexity.md",
  "source_sha256": "6db7cf5f9984034014f25a3c8743218c33a04f5914233948f7cb49f867b3a7b8",
  "captured_at": "2026-09-06T23:23:07.977850+00:00",
  "word_count": 351,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0023: D4.1",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 D4.1 \u2014 Kolmogorov Complexity",
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

# Canonical Sequential Record 0023: D4.1

| Field | Value |
|---|---|
| Registry position | 23 of 191 |
| Canonical ID | `D4.1` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `D4_1_Kolmogorov_Complexity.md` |
| Source SHA-256 | `0856be3e36c3b31be9a2397c52f30a452f3e5b54c1c29fe5f89c907f8725f2bf` |

## Sequence Navigation

Previous: [0022 - E3.2 - Universal Coherence Definition](0022_E3.2_E3_2_Universal_Coherence_Definition.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0024 - D4.2 - Compression Ratio](0024_D4.2_D4_2_Compression_Ratio.md)

## Registered Dependency Links

[A2.2 - Self-Grounding](0004_A2.2_A2_2_Self-Grounding.md)

## Canonical Source Text

---
id: "D4.1"
legacy_id: "AX-029"
title: "Kolmogorov Complexity"
mode: "AX_SCAFFOLD"
atlas_type: "Definition"
status: "definition"
domain: "Ontological"
anchor: "A2.2 (Self-Grounding)"
depends_on: "A2.2"
date: "2026-08-17"
---

# 📐 D4.1 — Kolmogorov Complexity

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`
**Domain:** Ontological | **Legacy ID:** `AX-029`

---

## Statement

Kolmogorov complexity defined — algorithmic information content.

## Formal Expression

```
K(x) = min{|p| : U(p) = x} — shortest program producing x
```

$$
K(x) = min{|p| : U(p) = x} — shortest program producing x
$$

## Maximum Defensible Position

Definition: K(x) = Kolmogorov complexity.

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

**Anchor:** A2.2 (Self-Grounding)
**Depends On:** A2.2
**Chain Tier:** 5 — Compression Theory

## References

Kolmogorov 1963; Chaitin 1966; Solomonoff 1964

## Defeat Conditions

K(x) is well-established. Challenge: it's uncomputable in general.

## Limitations (from workbook)

K(x) is uncomputable (halting problem). Framework uses it as theoretical measure, not practical one.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::fa5c5d5b-fd4a-4fa9-8fe0-b366622abd7c::"Define Kolmogorov complexity as algorithmic information content"::null::@Engineering%%
%%tag::Requirement::3698462a-925a-4680-ab9e-3ee177ff32cb::"Formal expression K(x) = min{|p| : U(p) = x} must be used"::null::@Engineering%%
%%tag::Decision::53e8562d-ef54-4ae2-8142-31cd9410b83c::"Adopt Kolmogorov complexity as a theoretical measure"::null::@Engineering%%
%%tag::Risk::42bc2a63-3a88-479a-84b9-e45fe4f07455::"K(x) is uncomputable in general"::null::@Engineering%%
%%tag::Dependency::010f7277-6ab0-498d-a992-f5cc450b7623::"Depends on A2.2 (Self-Grounding)"::null::@Engineering%%
%%tag::Stakeholder::4ffb36a3-ca39-4df5-8bc4-869a10869c42::"Kolmogorov, Chaitin, Solomonoff as referenced authorities"::null::@Engineering%%
%%tag::OpenQuestion::482499dc-16ec-41e5-a99c-5be34dce1252::"How to handle uncomputability in practical applications"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
