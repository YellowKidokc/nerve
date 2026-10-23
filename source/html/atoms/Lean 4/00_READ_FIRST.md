# Lean Floor — V2

Start with `00_COMPLETE_LEAN_FLOOR_TEMPLATE.md`. The formal packet records checks; the human companion explains the work using a fixed structure. The same report structure applies to individual papers and the corpus overview.

## Intended workflow
1. Inventory the dropped collection, preserving originals and project structure.
2. Link Lean sources to adjacent summaries, papers, and supporting code.
3. Record selection decisions: active candidates, exact duplicates, superseded versions, supporting material, and unresolved items. Do not guess which half to omit.
4. Read selected work in stages, maintaining persistent evidence notes and checkpoints.
5. Verify selected declarations in their actual pinned projects, then perform the applicable audits and controls.
6. Generate a formal packet and fixed-format human companion per result.
7. Generate one corpus report explaining foundations, strongest supported results, connections, corrections, and unresolved work, with explicit accounting of exclusions.
8. Archive successfully processed intake only after integrity checks; retain recoverable failures. Open the completed results folder.

## Current implementation boundary
These files define the V2 format and workflow. Replacing templates does not implement selection, automatic resume, source archiving, or a final corpus verification orchestrator. Existing launchers must be audited and connected to this contract before they can be described as enforcing it. Existing reports are not rewritten by this update.

## Final run design
One command should drive a manifest of separate Lean modules and projects. This preserves imports and reusable proofs while producing one overall result. Do not concatenate the corpus or force differing projects onto one toolchain without testing compatibility. Root project currently inspected: Lean v4.30.0; record each project's actual pin when executing.
