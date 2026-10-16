# CKG V4 (readable)

A separate lane next to the V3.2 CKG. Same runner, same Run page (variant **ckg_v4**).

What a person reads, in order: what the paper is (detailed synopsis) · the question ladder · every leg of the argument (all of them, not the usual two or three) · the paper's own argument (who made it before, what is new, where it can go) · claim discipline (strongest / must defend / probably wrong, not every claim) · evidence (best, weakest, missing, check before publishing) · paragraph walk · audit and build next · the original.

What is NOT on the page: YAML, classification, IDs, full claim lists, scorecards. Those belong in the paper's sidecar (intake's `<uuid>.meta.yaml`) and ledger, tied to the UUID.

Each call is a folder under `calls/` with `prompt.md` and `output.md`; switch any off in `ckg.json`.
