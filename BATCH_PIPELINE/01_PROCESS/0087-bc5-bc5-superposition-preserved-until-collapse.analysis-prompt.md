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
  "title": "Superposition Preserved Until Collapse",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0087_BC5_BC5_Superposition_Preserved_Until_Collapse.md",
  "source_sha256": "e1cc6f0f6273cc949a9d56846f3cf16999ecb11d7615aee16e3919b4085ec2ec",
  "captured_at": "2026-09-07T00:18:37.321109+00:00",
  "word_count": 379,
  "tags": [
    "quantum-foundations",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0087: BC5",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd36 BC5 \u2014 Superposition Preserved Until Collapse",
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

# Canonical Sequential Record 0087: BC5

| Field | Value |
|---|---|
| Registry position | 87 of 191 |
| Canonical ID | `BC5` |
| Mode | `FW_EXTENDED` |
| Canonical source filename | `BC5_Superposition_Preserved_Until_Collapse.md` |
| Source SHA-256 | `442e838df119f1a9c7da932696711a69e6921c49fc1393e050a2eb109d1eac2d` |

## Sequence Navigation

Previous: [0086 - BC3 - Measurement Orthogonality](0086_BC3_BC3_Measurement_Orthogonality.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0088 - BC9 - Opacity Requirement](0088_BC9_BC9_Opacity_Requirement.md)

## Registered Dependency Links

[BC4 - Three Observers Required](0006_BC4_BC4_Three_Observers_Required.md)

## Canonical Source Text

---
id: "BC5"
legacy_id: "AX-062"
title: "Superposition Preserved Until Collapse"
mode: "FW_EXTENDED"
atlas_type: "Boundary Condition"
status: "boundary"
domain: "Divine"
anchor: "BC4"
depends_on: "BC4"
date: "2026-08-17"
---

# 🔶 BC5 — Superposition Preserved Until Collapse

**Mode:** `FW_EXTENDED` | **Atlas Type:** `Boundary Condition` | **Status:** `boundary`
**Domain:** Divine | **Legacy ID:** `AX-062`

---

## Statement

Superposition is preserved until the moment of collapse — no premature decoherence by the terminal observer.

## Formal Expression

```
O_T preserves |ψ⟩ until voluntary collapse
```

$$
O_T preserves |ψ⟩ until voluntary collapse
$$

## Maximum Defensible Position

Framework claim: Superposition is preserved until the moment of collapse — no premature decoherence by the terminal observer.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `FW_EXTENDED` |
| **Atlas Object Type** | `Boundary Condition` |
| **Workbook Role** | `FW_EXTENDED` |
| **Mathematical Use** | Framework commitment |
| **Epistemic Status** | `boundary` |
| **Domain** | Divine |

## Dependency Chain

**Anchor:** BC4
**Depends On:** BC4

## References

Quantum error correction; Decoherence theory

## Defeat Conditions

Show terminal observer causes premature decoherence.

## Limitations (from workbook)

Requires terminal observer to NOT collapse by default — only on voluntary coupling.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::9bd9dc33-1b6c-4f59-ae43-936c00bd1279::"Superposition must be preserved until voluntary collapse by the terminal observer"::null::@Engineering%%
%%tag::Requirement::f8daac98-4ab0-4ea8-97c4-bfb33eeab279::"Terminal observer must not cause premature decoherence"::null::@Engineering%%
%%tag::Decision::d82329f2-02db-4866-8497-baf9d9acd2aa::"Adopt FW_EXTENDED mode for this boundary condition"::null::@Product%%
%%tag::Decision::0a50515d-8bfb-44a2-99a2-c3777414d6cf::"Classify BC5 as a boundary condition in the Divine domain"::null::@Product%%
%%tag::Dependency::94f12723-4b03-4840-91f6-762e6f102528::"Depends on BC4 - Three Observers Required"::null::@Engineering%%
%%tag::Assumption::111cfe7e-0d41-4fd3-89b9-148ecaddc663::"Terminal observer does not collapse by default"::null::@Engineering%%
%%tag::Assumption::2bf44a54-91a0-48f5-8bf4-2ad448713cda::"Collapse occurs only on voluntary coupling"::null::@Engineering%%
%%tag::Risk::b5c87736-afb0-464e-aeb6-a0af56fee964::"Terminal observer could cause premature decoherence, invalidating the boundary condition"::null::@Engineering%%
%%tag::OpenQuestion::11751f44-cf23-4039-b802-337c585d7f2a::"What constitutes voluntary coupling by the terminal observer?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
