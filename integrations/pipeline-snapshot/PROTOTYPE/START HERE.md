# API PIPELINE — project handoff

Intended Windows location: D:\GitHub\API PIPELINE

Open index.html in a browser. It is one self-contained HTML prototype with embedded styles, scripts, and sample data. No installation or server is required. This prototype is stored in the API PIPELINE folder of YellowKidokc/pipeline-workflows.

## Prompt for the next session

Continue David's API PIPELINE from this HTML prototype. Preserve its black-and-gold appearance and the agreed navigation and responsibilities below. First read the prototype and this handoff. David will supply the next one or two steps, involving the combination of two other systems; those systems and their integration requirements have not yet been supplied. Record and clarify those requirements before implementing them. Do not invent the missing systems or treat the sample activity as evidence of working API execution.

The current goal is to make the organization intuitive. Backend implementation can come later. Preserve the original source notes, keep credentials outside project files, and explain any proposed change to the agreed behavior.

## Agreed structure

- API Calls: each API group is its own workspace and has a home page.
- Python: a separate top-level area, not designed yet. Examples include collecting definitions into a definitions folder, breaking material into parts, and comparing parts to decide what to use.
- Workflows: named, ordered sequences of API groups and HTML assembly steps. They use each group's settings instead of imposing one global model on every group.
- HTML Review: open assembled HTML, inspect it, and pass or fail it. Normally later processing may continue while review waits. A workflow may instead wait for approval.
- Settings: application defaults; individual API groups own their model and run settings.

Each API group has Run, Settings, Results, and Backside views.

Run shows the selected folder and notes, a large green Run button, a red Stop button, visible stages, and a verbose scrolling activity log. Show which note and call are active, what is happening, checks, failures, and files saved. The completion summary can include word counts and cost when real usage and pricing exist. Do not invent progress percentages or costs.

Settings owns the group's model, concurrency, call order, output format, folder layout, and completed-result handling. Each group has a consistent output destination. Save each finished result immediately, one at a time; do not wait for the batch to finish.

Backside starts with the whole template and breaks it into proposed individual calls. Each call pairs a prompt with an editable output template. David can change tables to bullet points and edit the actual output requirements. The supplied CKG Atom Record v3.2 has 26 S/A sections used as a draft breakdown; these are not necessarily 26 final API calls. Calls can be split or combined.

Backside supports moving a call up, down, or directly to a numbered position. Keep the prompt and output definition together. Call execution order is distinct from assembled document order and stable section IDs. Keep structured facts consistent across Markdown, JSON, and HTML presentations.

## API groups supplied by David

01_CKG
02_C_P_E_L
02_YOUTUBE
03_EVIDENCE
04_ACADEMIA
05_LEAN
06_ATOMS
07_ARGUMENTS
08_API_DEEP
037_EVIDENCE_SQLITE_SYNC
038_EVIDENCE_SIDECARS
041_STORIES
047_NEW_PAPER
048_TOPIC_SYNTHESIS
049_GAP_MAP
060_OPENAI_STATIONS

Source workbench: D:\GitHub\pipeline-workflows\API
Some mockup call names are illustrative; inventory the real groups before wiring execution. HTML assembly companions such as 05B Lean are planned, not confirmed existing tools.

## Current prototype boundaries and future problems to resolve

- API calls, Python execution, filesystem access, downloads of generated answers, background jobs, and HTML assembly are not implemented. The prototype can download example configuration, edited template, and log files.
- Most edits are kept only in memory and reset on reload. Download edited templates/settings to retain them. A future application needs durable storage and versioned run snapshots.
- Folder paths and note lists are examples, not actual disk discovery. Browsers cannot silently write to arbitrary Windows folders. Choose a local Python service or desktop wrapper if automatic immediate saves are required.
- Sample log playback is not a real run, global run manager, or durable progress history. A real application needs separate run state so switching groups, reloading, or closing a view does not stop work.
- Reordering calls must account for dependencies. Preserve call identity and settings separately from display position.
- Define which records each step reads: original note, earlier outputs, or both. Workflow handoffs shown now are proposals.
- Define what failing HTML review does: mark for correction, rerun a step, or block later publication. Review approval must not silently publish.
- Independent calls share one concurrency limit; avoid multiplying limits across notes, calls, and groups. Decide retry, cancellation, restart, and duplicate-save behavior.
- Keep API credentials secure. Validate output completeness and schemas; do not present truncated model responses as successful results.
- Table/bullet conversion in this prototype is illustrative, not a guaranteed lossless transformation. Check all original requirements before using edited templates in production.
- The CKG-specific template editor is currently a demonstration; other groups need their own template definitions and independent saved state.

## Next additions — waiting for David

1. First additional system/step: not supplied yet.
2. Second additional system/step: not supplied yet.
3. Desired connection between them: not supplied yet.

When supplied, record each system's purpose, inputs, outputs, existing files or location, what to preserve, and where it fits in the pipeline. Document open questions and proposed behavior first. No implementation is requested tonight for those additions.

## Received: classifier and research discussion

The supplied Auto Paper Classifier Design DOCX is preserved as Auto Paper Classifier Design - ORIGINAL.docx. Read CLASSIFIER AND RESEARCH - REVIEW.md for corrections, integration proposals, and the next-session prompt. It covers classification, research modes, and a writing-choice interface, rather than just one classifier. Website details remain pending. No backend was implemented.

## HTML assembly update

Read HTML ASSEMBLIES.md and assembly-definitions.json. The prototype now contains seven designated group/assembly pairs and editable assembly definitions, with distinct layout directions for C_P_E_L, Academia, Atoms, and API Deep.
