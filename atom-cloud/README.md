# Nerve desktop + Cloudflare jobs

Installed September 8, 2026. All outputs remain candidates.

## Desktop

Reopen the Axiom & Claim Builder panel in Nerve to load the updated interface.

- **Import** accepts multiple completed CKG JSON records or evidence-chain Markdown reports. Records stay separate in the document selector.
- **Predicates** shows imported statements, roles, and formal expressions.
- **Diagnostics** exposes editable generation prompts. Missing JSON answers stay empty.
- **Review** records Accept, Save revision, Reject, and Defer decisions for individual values or report sections.
- **Export** preserves the entire original record alongside edits and decision history.
- **Batch raw documents** opens the existing deep intake pipeline. Its original per-document Markdown/JSON saving code was retained; a real corpus batch was not rerun during this change.

The supplied Distinction JSON contains 12 predicates and no diagnostic answers. The selected Logos evidence-chain Markdown contains 16 analysis sections before its preserved source. Both were tested without editing their originals or accepting their substantive claims.

The old simulated Lean verification and classification buttons no longer announce successful tests. Seeded Lean demonstration results are hidden; no actual Lean verifier was connected by this change.

## Cloud execution

Worker: `https://nerve-atom-jobs.davidokc28.workers.dev`

The local bridge listens only at `127.0.0.1:7861`. The cloud credential is encrypted for David's Windows account in `%LOCALAPPDATA%\NerveAtomCloud\token.xml`; it is not shipped in browser JavaScript.

Start the bridge after a reboot:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File D:\GitHub\nerve\atom-cloud\start-bridge.ps1
```

In Atlas, choose **Cloudflare Workers** to send model calls through the queue. In Atom Builder, **Cloud jobs** can submit saved sources independently using the selected object type and prompt snapshot. Submitted pipeline jobs run both generation and audit remotely, even if the page closes. Results are retained in D1 and can be inspected and opened as new candidates. Single interactive two-pass runs still need the desktop client to submit the second pass; the first pass remains saved if it closes.

The consumer permits up to **100 concurrent jobs**, with five actual inference attempts per job and a longer queue redelivery allowance for lease recovery. This is a configured ceiling, not a promise of 100 simultaneous model inferences. Cloudflare's model rate limits and account billing still apply. No large corpus was submitted in testing.

Each request is limited to 60 KB of UTF-8 JSON. Oversized requests fail explicitly rather than truncating sources. The service is single-user; it does not expose a public unauthenticated mode. Results are saved remotely; the desktop's job index is browser-local, so retain/export receipts if moving between machines or clearing browser storage.

## Verification

11 local tests passed: authentication, input/output boundaries, duplicate and leased deliveries, retries, pipeline source retention, provider response envelopes, exact-file import mapping, review round trips, and malformed inputs. Browser checks confirmed the two supplied record imports and prompt visibility. Python and JavaScript syntax checks passed.

A fresh synthetic pipeline completed on its first attempt after correcting the provider response envelope handling:

`a7701586ad6e5c68bac12ba2b6ca0ec916b3976930111d6f4509f6d7271a2474`

Completed: `2026-09-08T23:42:58.483Z`.

The earlier synthetic request also completed after deployment fixes and retries; it was not a clean first-attempt pass.

Source/installed backups: `D:\GitHub\nerve\tmp\atom-remake-backup-20260908-182034`.

Local preview: `http://127.0.0.1:8768/axiom-builder.html` (preview server must be running). Browser preview storage is separate from Nerve's custom-protocol storage.

## Tests

```powershell
node --test worker.test.mjs ckg-import.test.cjs
python -m py_compile bridge.py
```

The exact-file import tests reference David's supplied local and NAS fixtures.
