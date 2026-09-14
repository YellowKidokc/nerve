---
type: axiom_companion
title: "ckg_evaluation: Information Definition"
status: CANDIDATE_DRAFT
canon_status: candidate_draft
semantic_status: ai_analyzed_pending_review
template_version: CKG_RECORD_V2.0
source_file: "D:\\GitHub\\nerve\\BATCH_PIPELINE\\00_INBOX\\0013_D1.1_D1_1_Information_Definition.md"
source_sha256: 3c655539e3e28695e2d18378823279769c33ca2211bb146985ab93eae97af91d
captured_at: 2026-09-06T23:12:39.750560+00:00
tags: ["information-theory", "systems"]
semantic_provider: "openrouter"
semantic_model: "deepseek/deepseek-v3.2"
semantic_analyzed_at: "2026-09-06T23:12:39.750560+00:00"
usage_tokens: {"prompt_tokens": 2360, "completion_tokens": 2595, "total_tokens": 4955, "cost": 0.003495776427, "is_byok": false, "prompt_tokens_details": {"cached_tokens": 768, "cache_write_tokens": 0, "audio_tokens": 0, "video_tokens": 0}, "cost_details": {"upstream_inference_cost": 0.0035310873, "upstream_inference_prompt_cost": 0.0006467448, "upstream_inference_completions_cost": 0.0028843425}, "completion_tokens_details": {"reasoning_tokens": 0, "image_tokens": 0, "audio_tokens": 0}}
---

# ckg_evaluation: Information Definition

> **CANDIDATE_DRAFT — NOT ADMITTED.** The source is preserved below.
> Evaluated autonomously by Consilience Atlas Large-Batch CKG Pipeline.

## Intake assessment

- Inferred type: `axiom_companion`
- Routing confidence: `0.98`
- Signals: canonical axiom record, sealed v2.3 authority, axiom companion layout
- Original words: `365`
- Original SHA-256: `3c655539e3e28695e2d18378823279769c33ca2211bb146985ab93eae97af91d`

## Source outline

- Canonical Sequential Record 0013: D1.1
- Sequence Navigation
- Registered Dependency Links
- Canonical Source Text
- 📐 D1.1 — Information Definition
- Statement
- Formal Expression
- Maximum Defensible Position
- Classification
- Dependency Chain
- References
- Defeat Conditions
- Limitations (from workbook)

## At a glance
This is a canonical definition record (D1.1) establishing a foundational axiom for a formal system: information is defined as resolved uncertainty, formalized as Shannon entropy. It matters because it provides the syntactic, probabilistic bedrock for a chain of subsequent definitions and theorems. The one-sentence finding is: The system axiomatically adopts Shannon's information measure as its primitive definition of information.

## Central claim
Information is defined as resolved uncertainty, formally expressed by Shannon entropy: \( I(x) = \log_2(1/p(x)) \) for a specific event, and \( H(X) = -\sum p \log p \) for an ensemble.

## Best concise argument
1.  To build a formal system, a precise, operational definition of "information" is required.
2.  Shannon's 1948 mathematical theory of communication provides a widely accepted, quantifiable measure of information as entropy.
3.  Therefore, adopting Shannon entropy as the definition ("information = resolved uncertainty") provides a rigorous, formal foundation for the system's subsequent development.

## System or model
This record is a definitional axiom within a larger, structured axiom chain. It posits a single primitive entity: "information," characterized by its relationship to probability and uncertainty.

### Extracted Truth Predicates (Irreducible Assertions)
| # | Truth Predicate | Source Role | Formal / Mathematical Formulation |
|---|---|---|---|
| P1 | Information is defined as resolved uncertainty. | Core Definition | `I(x) = log₂(1/p(x))` |
| P2 | Shannon entropy is the measure of information for an ensemble. | Formal Extension | `H(X) = -Σ p(x) log₂ p(x)` |
| P3 | This definition depends on the concepts of Existence (A1.1) and Distinction (A1.2). | Dependency Assertion | `D1.1 ⊢ (A1.1 ∧ A1.2)` |
| P4 | Shannon information is syntactic, not semantic. | Limiting Property | N/A (descriptive property) |
| P5 | The framework requires a definition for semantic information in addition to this syntactic one. | Framework Requirement | N/A (system requirement) |
| P6 | The definition is formalized using the Shannon entropy formula. | Implementation Rule | Formula must be `I(x) = log₂(1/p(x)); H(X) = -Σp·log p` |
| P7 | This record has the status of a `definition` within the system. | Epistemic Status | `status: "definition"` |
| P8 | This record belongs to the `Primordial` domain of the system. | Domain Classification | `domain: "Primordial"` |
| P9 | The definition can be defeated by showing information is better defined another way. | Defeat Condition | N/A (metacondition) |
| P10 | The definition can be defeated by demonstrating Shannon entropy is the wrong measure. | Defeat Condition | N/A (metacondition) |
| P11 | Adopting Shannon entropy as the definition of information is a formal decision. | System Decision | `tag::Decision::..."Adopt Shannon entropy..."` |
| P12 | Shannon entropy may be the wrong measure of information. | Acknowledged Risk | `tag::Risk::..."Shannon entropy may be the wrong measure..."` |

### Terms and plain-language bridge
*   **Shannon Entropy (H(X)):** A measure of the average "surprise" or uncertainty inherent in a set of possible outcomes. *Analogy:* The amount of "news" contained in a message, where a highly predictable message (e.g., "the sun rose") carries little news, while a very unlikely message (e.g., "it snowed in the Sahara") carries a lot.
*   **Information (I(x)):** The reduction in that surprise when a specific outcome is observed. *Analogy:* The "answer" to a question that resolves your uncertainty. If you're unsure which card will be drawn from a deck, learning it's the Ace of Spades gives you a specific amount of information.
*   **Syntactic vs. Semantic Information:** Syntactic information is about the *form* and probability structure (e.g., the bits in a message), while semantic information is about the *meaning* or interpretation (e.g., what those bits signify). This definition covers only the syntactic part.

## Evidence chain
1.  **Claim:** A formal system needs a precise definition of information.
    *   **Support:** The record is structured as a canonical definition within an axiom chain (`mode: AX_SCAFFOLD`, `atlas_type: Definition`).
    *   **Inference:** The authors are constructing a formal framework requiring foundational primitives.
    *   **Confidence:** High, based on the document's explicit structure and classification.
    *   **Boundary:** This is a methodological choice, not a proven necessity.

2.  **Claim:** Shannon's 1948 formulation is a viable candidate for this definition.
    *   **Support:** The source cites "Shannon 1948" and encodes the entropy formulas.
    *   **Inference:** Shannon's theory is a standard, mathematically rigorous foundation in information theory.
    *   **Confidence:** High, given its canonical status in the field.
    *   **Boundary:** Viability does not imply uniqueness or ultimate correctness.

3.  **Claim:** Therefore, the system adopts "information = resolved uncertainty (Shannon entropy)" as its definition.
    *   **Support:** The "Statement" and "Maximum Defensible Position" sections state this unequivocally.
    *   **Inference:** This is the axiomatic starting point chosen for the system.
    *   **Confidence:** Certain, as it is the explicit content of the record.
    *   **Boundary:** This establishes a definition within the system, not a universal truth.

## Best evidence and sources
*   **Primary Evidence/Formal Result:** The formal expressions `I(x) = log₂(1/p(x))` and `H(X) = -Σp·log p` are given as the definition. This is a mathematical formulation, not a proven theorem.
*   **Secondary Interpretation:** The source's own classification and tags frame this as a `Decision` and `Requirement` for engineering a framework.
*   **Analogy:** The concept of "resolved uncertainty" serves as an intuitive bridge to the formal math.
*   **Citations:** "Shannon 1948" is referenced, pointing to the historical and intellectual origin of the formulas.

## Strongest objection and negative controls
The strongest objection is that Shannon information is purely syntactic and probabilistic, stripping away meaning, context, and the qualitative nature of information as it is often understood in philosophy, biology, or everyday life. Competing interpretations include:
*   **Floridi's Semantic Information:** Information as well-formed, meaningful, and truthful data.
*   **Biological/Functional Information:** Information as that which has a causal effect in a biological system (e.g., genetic information, neural signals).
*   **Algorithmic Information Theory (Kolmogorov Complexity):** Information as the minimal description length of an object, independent of probability distributions.
A negative control would be to apply this definition to a meaningful but highly probable statement (e.g., a profound philosophical truth expressed in common words). The syntactic information content would be low, contradicting the intuitive sense that it carries significant information. This highlights the definition's limitation to the statistical aspect of communication.

## What survives
The definition survives as a rigorous, operational, and mathematically tractable foundation for the *syntactic* and *probabilistic* layer of a formal system. It remains defensible as a chosen axiom within a constructed framework, providing a clear, quantitative handle on one aspect of information. Its utility in deriving further formal results about communication channels, data compression, and uncertainty reduction is untouched by objections about semantics.

## What this does not establish
This does not establish that Shannon entropy is the only or universally correct definition of information. It does not establish that semantic meaning is reducible to or derivable from this syntactic definition. It does not prove any physical facts about the universe; it is a formal, definitional choice. It certainly does not establish any theological conclusions about the nature of reality or consciousness. The definition is confined to the context of the axiomatic system being built.

## Corrections and revisions
The source itself is presented as a sealed canonical record ("v2.3 authority"), suggesting it is a fixed point in the system. The "Limitations" section explicitly notes the need for a semantic information component, indicating this definition is knowingly incomplete and may be supplemented later, not replaced. The "Defeat Conditions" show awareness of potential challenges but do not record past revisions.

## Implications
*   **Formal:** Provides the primitive for deriving subsequent definitions (e.g., D1.2 Bit Definition) and theorems within the system.
*   **Philosophical/Semantic:** Formally separates the syntactic (statistical) from semantic (meaningful) aspects of information, acknowledging the need to address the latter separately.
*   **Empirical:** None directly. The definition is a formal tool, though it can be applied to model empirical systems (e.g., communication networks).
*   **Theological:** None.

## Formal or testable path
### Lean / formal checks
| Proposed theorem or invariant | Definitions & premises to encode | Status | What a passing result establishes | Reader meaning / analogy |
|---|---|---|---|---|
| The bit definition (D1.2) is derivable from D1.1 plus probability axioms. | Encode `I(x)`, `H(X)`, define `bit` as unit of information (`I(x)=1` when `p(x)=1/2`). | Not Started | Consistency of the axiom chain's first derivational step. | Checking that the first building block fits solidly on the foundation. |
| Shannon entropy `H(X)` is non-negative for any discrete probability distribution. | Encode probability distribution axioms and the `log` function properties. | Not Started | A basic, well-known property of the defined measure holds in the formal system. | Verifying the tool works as expected in simple cases. |
| `H(X)` is maximized for a uniform distribution. | Encode probability distributions, summation, and inequality constraints. | Not Started | A key optimality property of the defined measure is formally verified. | Confirming a core behavioral feature of the definition. |

### Empirical / literature checks
Compare the framework's derivations using this definition against standard textbook results in information theory (e.g., source coding theorem). Survey philosophical literature (e.g., Floridi) to explicitly map where the syntactic definition ends and semantic extensions must begin.

### Adversarial checks
Attempt to derive a contradiction by applying the definition to self-referential or paradoxical statements. Try to model a system where "meaning" clearly drives function but syntactic information content is low (e.g., a simple gene regulatory switch) and see if the framework's tools become inadequate, validating the noted limitation.

## Open questions and next actions
**Open Questions:**
1.  How will semantic information be formally defined and integrated into the framework without contradicting or confusing the syntactic foundation?
2.  What specific phenomena or use cases are excluded by committing to this syntactic definition?
3.  Are the defeat conditions ("better defined another way," "wrong measure") operationalizable for future review?

**Next Action:** Proceed to formalize the immediate dependent definition, D1.2 (Bit Definition), to test the derivational utility of D1.1.

## Recommended classification and relationships
*   **Record Type:** Axiom / Definition
*   **Domain:** Formal Systems, Information Theory
*   **Tags:** `axiom`, `definition`, `information-theory`, `shannon-entropy`, `syntactic-information`
*   **Relationships:**
    *   `depends_on`: A1.1 (Existence), A1.2 (Distinction)
    *   `refines`: (None. It is a primitive.)
    *   `supports`: D1.2 (Bit Definition), and all subsequent theorems relying on information measures.
    *   `contradicts`: Philosophical positions defining information primarily by meaning or function.
    *   `tests`: Formal verification of its basic properties (non-negativity, concavity).

## Complete preserved source

<!-- BEGIN EXACT SOURCE PROJECTION; authoritative original bytes are permanently preserved -->

<!-- GENERATED FROM AXIOMS_CANONICAL_ONLY.md; do not edit this projection by hand. -->



# Canonical Sequential Record 0013: D1.1



| Field | Value |

|---|---|

| Registry position | 13 of 191 |

| Canonical ID | `D1.1` |

| Mode | `AX_SCAFFOLD` |

| Canonical source filename | `D1_1_Information_Definition.md` |

| Source SHA-256 | `7daeed0ef4232a77bd68f030980ca07f80197d81c4f388a39ac0a817c92ee8b3` |



## Sequence Navigation



Previous: [0012 - BC8 - Voluntary Coupling](0012_BC8_BC8_Voluntary_Coupling.md)  

Index: [All Canonical Records](FIS.html)  

Next: [0014 - D1.2 - Bit Definition](0014_D1.2_D1_2_Bit_Definition.md)



## Registered Dependency Links



[A1.2 - Distinction](0002_A1.2_A1_2_Distinction.md); [A1.1 - Existence](0001_A1.1_A1_1_Existence.md)



## Canonical Source Text



---

id: "D1.1"

legacy_id: "AX-004"

title: "Information Definition"

mode: "AX_SCAFFOLD"

atlas_type: "Definition"

status: "definition"

domain: "Primordial"

anchor: "A1.1, A1.2"

depends_on: "A1.1, A1.2"

date: "2026-08-17"

---



# 📐 D1.1 — Information Definition



**Mode:** `AX_SCAFFOLD` | **Atlas Type:** `Definition` | **Status:** `definition`

**Domain:** Primordial | **Legacy ID:** `AX-004`



---



## Statement



Information is defined as resolved uncertainty — Shannon entropy.



## Formal Expression



```

I(x) = log₂(1/p(x)); H(X) = -Σp·log p

```



$$

I(x) = log₂(1/p(x)); H(X) = -Σp·log p

$$



## Maximum Defensible Position



Definition: Information = resolved uncertainty.



## Classification



| Field | Value |

|-------|-------|

| **Mode** | `AX_SCAFFOLD` |

| **Atlas Object Type** | `Definition` |

| **Workbook Role** | `DEFINITION` |

| **Mathematical Use** | Formal definition / primitive formalization |

| **Epistemic Status** | `definition` |

| **Domain** | Primordial |



## Dependency Chain



**Anchor:** A1.1, A1.2

**Depends On:** A1.1, A1.2

**Chain Tier:** 1 — Primordial Foundation



## References



Shannon 1948



## Defeat Conditions



Show information is better defined another way. Demonstrate Shannon entropy is wrong measure.



## Limitations (from workbook)



Shannon information is syntactic, not semantic. Framework needs semantic information too.



---



*Generated 2026-08-17 from AXIOM_CHAIN_MASTER_v2, AXIOMS_PART1_MODE_CLASSIFICATION, and AX Truth Kernel classifications.*



%%--- SEMANTIC TAGS ---%%

%%tag::Requirement::608a546e-e0d1-4c1e-9542-db2e3d2e625f::"Information must be defined as resolved uncertainty (Shannon entropy)"::null::@Engineering%%

%%tag::Requirement::574524a7-47a8-4e82-82e7-d8896b874ead::"The formal expression must use Shannon entropy formula"::null::@Engineering%%

%%tag::Decision::d5c32d45-4fb2-4b00-af2c-cf351883701b::"Adopt Shannon entropy as the definition of information"::null::@Engineering%%

%%tag::Dependency::407195e2-4751-411e-8740-390a792945d3::"Depends on A1.1 Existence and A1.2 Distinction"::null::@Engineering%%

%%tag::Risk::d5eb94c2-9d07-4233-9bf7-e4980b05ff66::"Shannon entropy may be the wrong measure of information"::null::@Engineering%%

%%tag::Risk::934d8c1e-643c-4e91-ae3d-42781107c2ea::"Shannon information is syntactic, not semantic, limiting framework"::null::@Engineering,Product%%

%%tag::OpenQuestion::bd5b2e68-d4bf-4c2b-90e3-95809f6ed4ca::"How to incorporate semantic information into the framework"::null::@Engineering,Product%%

%%tag::Metric::3e3c8198-81c1-420a-9441-af8312efe644::"Shannon entropy H(X) = -Σp·log p"::null::@Engineering%%

%%tag::Stakeholder::b334a870-efcc-47f1-a9ca-64d3c2319b5c::"Framework users needing semantic information"::null::@Product%%

%%--- END SEMANTIC TAGS ---%%

<!-- END EXACT SOURCE PROJECTION -->
