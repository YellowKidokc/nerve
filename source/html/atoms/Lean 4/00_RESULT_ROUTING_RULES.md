# Result Routing — V2

## Authoritative records
Keep verified result bundles in `OUTBOX/00_VERIFIED_RECEIPTS/<result_id>/<version>/`. Preserve historical receipts unchanged. Failed, blocked, and unverified candidates belong in `OUTBOX/06_REVIEW_REQUIRED/<result_id>/`; their reports remain useful without being labeled verified. Draft explanations remain drafts until approved.

## Bundle contents
Each result links its formal packet, REPORT.md, preserved Lean project/module, supporting documents, existing summaries, verification logs, source manifest, and routing metadata. The bottom of REPORT.md contains the Lean Translation Layer. Existing summaries are evidence of intent, not verification receipts.

## Browse views
Maintain domain, subject/tag, claim-type, evidence, and formal-status views as generated pointers to the authoritative bundle. Include result ID, version, source hashes, and status. Do not count pointers as independent results or edit them as source records. Multiple classifications are intentional, not duplicate proofs.

## Status and approval
Use the human template's verification labels. Keep editorial canonical approval separate. Historical FORMALLY_HARDENED/FORMALLY_TESTED labels retain their original contracts; do not translate them to V2 verification status without inspecting the underlying evidence.

## Intake completion
Record a disposition for every input. Verify preserved copies and their hashes before moving intake into a dated source archive. Exact duplicate copies may be archived with links to the retained source. Semantic duplicates, revisions, and alternate proofs require reviewed dispositions. Never delete a source as a side effect of generating a report.
