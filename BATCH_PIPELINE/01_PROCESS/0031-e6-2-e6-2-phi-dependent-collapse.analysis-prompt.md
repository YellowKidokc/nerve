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
  "title": "Phi-Dependent Collapse",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0031_E6.2_E6_2_Phi-Dependent_Collapse.md",
  "source_sha256": "d473b47dff6ffe1f205bb1c0490f49562d57534b843bf81669e2a3f6b699e7b6",
  "captured_at": "2026-09-06T23:29:43.143815+00:00",
  "word_count": 427,
  "tags": [
    "consciousness",
    "quantum-foundations"
  ],
  "source_outline": [
    "Canonical Sequential Record 0031: E6.2",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udcd0 E6.2 \u2014 Phi-Dependent Collapse",
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

# Canonical Sequential Record 0031: E6.2

| Field | Value |
|---|---|
| Registry position | 31 of 191 |
| Canonical ID | `E6.2` |
| Mode | `AX_SCAFFOLD` |
| Canonical source filename | `E6_2_Phi-Dependent_Collapse.md` |
| Source SHA-256 | `b3cf9e9a5c07de8f3a501955ad54fd17e117785f7b99f6e109978e1ac1dbac18` |

## Sequence Navigation

Previous: [0030 - D6.2 - Projection Operator](0030_D6.2_D6_2_Projection_Operator.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0032 - D8.1 - Sign Operator](0032_D8.1_D8_1_Sign_Operator.md)

## Registered Dependency Links

[D5.2 - Integrated Information Phi](0027_D5.2_D5_2_Integrated_Information_Phi.md); [A5.1 - Observation Requirement](0005_A5.1_A5_1_Observation_Requirement.md); [A6.2 - Collapse](0076_A6.2_A6_2_Collapse.md)

## Canonical Source Text

---
id: "E6.2"
legacy_id: "AX-051"
title: "Phi-Dependent Collapse"
mode: "AX_SCAFFOLD"
atlas_type: "Equation"
status: "equation"
domain: "Ontological"
anchor: "A5.1 → D5.2 → A6.2"
depends_on: "A5.1, D5.2, A6.2"
date: "2026-08-17"
---

# 📐 E6.2 — Phi-Dependent Collapse

**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Equation` | **Status:** `equation`
**Domain:** Ontological | **Legacy ID:** `AX-051`

---

## Statement

Phi-dependent collapse equation — collapse rate scales with integrated information.

## Formal Expression

```
Γ(Φ) = Γ₀ · (Φ/Φ_ref)^α — collapse rate scales with Φ
```

$$
Γ(Φ) = Γ₀ · (Φ/Φ_ref)^α — collapse rate scales with Φ
$$

## Maximum Defensible Position

Model: Collapse rate as power law in Φ.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_SCAFFOLD` |
| **Atlas Object Type** | `Equation` |
| **Workbook Role** | `EQUATION/PREDICTION` |
| **Mathematical Use** | Mathematical model |
| **Epistemic Status** | `equation` |
| **Domain** | Ontological |

## Dependency Chain

**Anchor:** A5.1 → D5.2 → A6.2
**Depends On:** A5.1, D5.2, A6.2
**Chain Tier:** 7 — Collapse Dynamics

## References

IIT; Objective collapse (GRW); Penrose-Hameroff

## Defeat Conditions

Show collapse rate is Φ-independent. Measure Γ and find no Φ correlation.

## Limitations (from workbook)

THE testable prediction. If Γ ∝ Φ^α, then higher-consciousness observers collapse faster.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::78ca4289-e80c-4311-b6bc-d08aba600b4a::"Collapse rate must scale with integrated information (Phi)"::null::@Engineering%%
%%tag::Requirement::76ef94a8-edf2-40fa-8704-d27395adaa4d::"Collapse rate must follow a power law in Phi"::null::@Engineering%%
%%tag::Requirement::161ca535-b09e-424f-958a-893ee9b3e402::"Collapse rate must be Phi-dependent"::null::@Engineering%%
%%tag::Decision::c05733a6-3ea1-4730-864d-f109e4b4336b::"Model collapse rate as a power law in Phi"::null::@Engineering%%
%%tag::Metric::7d8b7693-1b15-4403-9b21-bcee14c6c350::"Collapse rate Gamma(Phi) = Gamma_0 * (Phi/Phi_ref)^alpha"::null::@Engineering%%
%%tag::Metric::00f07607-6a04-4bc0-b6af-84370a64c8ba::"Exponent alpha in the power law"::null::@Engineering%%
%%tag::Metric::55b91a0c-84c1-439b-baf2-46c05c2d68a5::"Reference integrated information Phi_ref"::null::@Engineering%%
%%tag::Metric::638dad56-58b7-43cc-92bd-86b360184b0c::"Baseline collapse rate Gamma_0"::null::@Engineering%%
%%tag::Dependency::e494cf1b-0a0a-4b61-9bff-b9498c0cfc0b::"Depends on A5.1 Observation Requirement"::null::@Engineering%%
%%tag::Dependency::cd830cf2-8376-40cc-9d43-4d25f0795b78::"Depends on D5.2 Integrated Information Phi"::null::@Engineering%%
%%tag::Dependency::c4dd9de9-8a27-4f23-97ec-8bb922793aa6::"Depends on A6.2 Collapse"::null::@Engineering%%
%%tag::Dependency::470dcf19-2884-4b3d-90d6-02d91b80575e::"References IIT (Integrated Information Theory)"::null::@Engineering%%
%%tag::Dependency::50ca2061-2d30-4dde-98ca-391025479819::"References Objective collapse (GRW)"::null::@Engineering%%
%%tag::Dependency::cbfa1f3b-9703-45ae-8ee3-8f4b6f4a37b4::"References Penrose-Hameroff"::null::@Engineering%%
%%tag::Risk::37f2df08-72d9-4f81-bc89-0c09b28bd1aa::"Collapse rate may be Phi-independent"::null::@Engineering%%
%%tag::Risk::67bcdd54-2ff6-43ed-92c4-fb77fa942726::"Measurement may find no Phi correlation"::null::@Engineering%%
%%tag::Requirement::6cab06fd-883e-4246-9ddb-d54a1960d9e3::"Higher-consciousness observers must collapse faster"::null::@Engineering%%
%%tag::OpenQuestion::eb949236-fef2-409b-8f1a-7254602bcde6::"Whether collapse rate is Phi-dependent"::null::@Engineering%%
%%tag::OpenQuestion::e69df556-dec3-4894-9f82-c934de21063c::"Whether Gamma correlates with Phi"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
