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
  "title": "Information Primacy",
  "inferred_type": "axiom_companion",
  "routing_confidence": 0.98,
  "routing_signals": [
    "canonical axiom record",
    "sealed v2.3 authority",
    "axiom companion layout"
  ],
  "canon_status": "candidate_draft",
  "source_file": "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0008_A1.3_A1_3_Information_Primacy.md",
  "source_sha256": "e9231e7c466b21bf4fc56f968f56867754f0d0db13afb24d6432c9432c960dd0",
  "captured_at": "2026-09-06T23:04:58.157602+00:00",
  "word_count": 391,
  "tags": [
    "information-theory",
    "systems"
  ],
  "source_outline": [
    "Canonical Sequential Record 0008: A1.3",
    "Sequence Navigation",
    "Registered Dependency Links",
    "Canonical Source Text",
    "\ud83d\udd37 A1.3 \u2014 Information Primacy",
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

# Canonical Sequential Record 0008: A1.3

| Field | Value |
|---|---|
| Registry position | 8 of 191 |
| Canonical ID | `A1.3` |
| Mode | `AX_DERIVED` |
| Canonical source filename | `A1_3_Information_Primacy.md` |
| Source SHA-256 | `0dd0f963e8f558723f4645360a2352916b8fc1d94cac915294d4adf29ae8ff94` |

## Sequence Navigation

Previous: [0007 - BC6 - Infinite Energy Source](0007_BC6_BC6_Infinite_Energy_Source.md)  
Index: [All Canonical Records](FIS.html)  
Next: [0009 - T3.1 - Coherence Cannot Self-Increase](0009_T3.1_T3_1_Coherence_Cannot_Self-Increase.md)

## Registered Dependency Links

[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)

## Canonical Source Text

---
id: "A1.3"
legacy_id: "AX-003"
title: "Information Primacy"
mode: "AX_DERIVED"
atlas_type: "Theorem"
status: "derived"
domain: "Primordial"
anchor: "A1.2 (Distinction)"
depends_on: "A1.1, A1.2"
date: "2026-08-17"
---

# 🔷 A1.3 — Information Primacy

**Mode:** `AX_DERIVED` | **Atlas Type:** `Theorem` | **Status:** `derived`
**Domain:** Primordial | **Legacy ID:** `AX-003`

---

## Statement

Distinguishability IS information; information is ontologically primitive.

## Formal Expression

```
I ≡ Distinguishability; H(X) = -Σp·log p
```

$$
I ≡ Distinguishability; H(X) = -Σp·log p
$$

## Maximum Defensible Position

Derived: Distinguishability = Information.

## Classification

| Field | Value |
|-------|-------|
| **Mode** | `AX_DERIVED` |
| **Atlas Object Type** | `Theorem` |
| **Workbook Role** | `AXIOM` |
| **Mathematical Use** | Formal definition / primitive formalization |
| **Epistemic Status** | `derived` |
| **Domain** | Primordial |

## Dependency Chain

**Anchor:** A1.2 (Distinction)
**Depends On:** A1.1, A1.2
**Chain Tier:** 1 — Primordial Foundation

## References

Wheeler 1990 (It from Bit); Shannon 1948; Landauer 1961

## Defeat Conditions

Show information as emergent, not primitive. Demonstrate matter/energy without informational substrate.

## Limitations (from workbook)

Wheeler's 'It from Bit' is suggestive but not proven. Landauer's principle shows info is physical.

---

*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*

%%--- SEMANTIC TAGS ---%%
%%tag::Requirement::eae46ccd-434c-401c-bae8-66c568d15ecf::"Information must be ontologically primitive"::null::@Engineering%%
%%tag::Requirement::39e38360-0458-4196-ac9b-01d65cfdf0e4::"Distinguishability must be equivalent to information"::null::@Engineering%%
%%tag::Decision::97de6938-4f4b-4a29-9686-74d5c7f54962::"Adopt information primacy as a derived theorem"::null::@Engineering%%
%%tag::Dependency::407195e2-4751-411e-8740-390a792945d3::"Depends on A1.1 (Existence) and A1.2 (Distinction)"::null::@Engineering%%
%%tag::Risk::18cdb709-a3fa-408a-a0aa-d6c1e3a16563::"Information may be shown as emergent, not primitive"::null::@Engineering%%
%%tag::Risk::64d0ec2f-edbf-4d38-a498-4d35663d6c71::"Matter/energy may exist without informational substrate"::null::@Engineering%%
%%tag::Assumption::7c3cba58-3dbb-46f7-bd5f-80a01a4513aa::"Wheeler's 'It from Bit' is suggestive but not proven"::null::@Engineering%%
%%tag::Assumption::ca5bb1be-5518-4c6e-8ab2-fb59b69579f1::"Landauer's principle shows information is physical"::null::@Engineering%%
%%tag::Metric::dc16a688-e7f7-404a-b3f5-893769255e65::"H(X) = -Σp·log p (Shannon entropy)"::null::@Engineering%%
%%tag::Stakeholder::a1b846a0-7c79-441c-99a3-1983f242b134::"Wheeler (It from Bit concept)"::null::@Engineering%%
%%tag::Stakeholder::0c73e792-4368-49f2-9da7-0b55753a701e::"Shannon (information theory)"::null::@Engineering%%
%%tag::Stakeholder::fd143224-a05b-4df9-aecf-805a2eed4187::"Landauer (physical information)"::null::@Engineering%%
%%tag::OpenQuestion::ece429ad-33ff-4271-ac5d-da605fc6a445::"Can information be shown as emergent rather than primitive?"::null::@Engineering%%
%%tag::OpenQuestion::26e85e88-a507-4a82-9dbe-859ec6878a82::"Can matter/energy exist without informational substrate?"::null::@Engineering%%
%%--- END SEMANTIC TAGS ---%%
