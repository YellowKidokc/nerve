# Lean Floor Template Contract — V2

## Purpose
Develop the strongest defensible version of the corpus. Credit supported contributions, improve incomplete arguments, and retire failed formulations with their history preserved. An unresolved claim is not a false claim. Formal validity, encoding fidelity, cross-domain interpretation, and editorial approval are separate judgments.

## Fixed presentation
Use all numbered headings and table columns in `00_FULL_HUMAN_COMPANION_TEMPLATE.md`, unchanged and in order. Include its final Lean Translation Layer. Paper-specific content changes throughout; the structure stays fixed. Never fill gaps with generic claims, invented proofs, or automatic canonical labels. Use Not checked, Not measured, or Not applicable with a reason.

## Fixed verification record
Use `00_FULL_LEAN_TEMPLATE.md` for each selected formal object. Record the actual project toolchain and dependency lock hashes; never substitute a version printed in a template. Different projects retain their own compatible pins until a tested migration is approved. The final run must disclose any mixed versions.

Preserve premise and axiom audits, proof-placeholder checks, applicable negative controls, and non-vacuity witnesses. Record each control as PASS, FAIL, NOT_RUN, or NOT_APPLICABLE with a reason. A passed build is not a completed audit. A failed check does not establish that the mathematical statement is false.

## Selection before execution
Inventory before calling an AI or rebuilding. Assign each source one disposition: SELECTED, EXACT_DUPLICATE, SUPERSEDED, SUPPORTING_ONLY, or REVIEW_REQUIRED. Record hashes, related claim IDs, reason, and retained counterpart where applicable. Hash duplicates can be identified mechanically; semantic equivalence and supersession require review. Never exclude a file just because its result is unfavorable. Report selected and excluded totals together.

## One-command final run
Use a versioned selection manifest and one orchestrator, not one enormous concatenated Lean file. Preserve projects and imports. The run must verify manifest hashes, record toolchains/locks, build each selected module explicitly, execute its audit and control commands, save logs, and produce per-object receipts plus a corpus summary. Ordinary `lake build` alone may not include every source file. Missing selected objects, stale hashes, failed checks, or pending required controls make the final run incomplete or failed, never silently successful.

## Persistent research notebook
Save the corpus map, claim register, evidence notes, open questions, next-work plan, and completed checkpoints between API calls. The model may improve the notebook organization. Source hashes, toolchain, template version, model settings, and question identify reusable work; changes invalidate affected checkpoints. Publish reports in the fixed structure regardless of notebook organization. Preserve revisions and their reasons. Never substitute AI interpretation for a compiler receipt.

## Completion and authority
Archive sources only after output integrity and recorded disposition are verified. Failed items remain recoverable. Reports are drafts until explicit editorial approval. Never overwrite historical verification receipts; create a new version linked to the prior result. V1 receipts keep their original contract and are not automatically upgraded to V2.
