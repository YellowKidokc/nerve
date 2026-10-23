# KIMI PROMPT — Move nerve's AI-calling panels into the Forge workbench (v0.1)
**Date:** 2026-10-23 | **For:** Kimi | **Approved direction:** David (this session)
**Status:** Draft for David's go. Read receipt required before any edit.

## The rule you are carrying out
All AI calls go through Forge (one service, one port). No panel, page, or program talks to an AI
provider itself. Forge's default address is http://127.0.0.1:8790 (setting: FORGE_URL).
Forge speaks the OpenAI format: POST {FORGE_URL}/v1/chat/completions.
Provider keys live only in Forge. Nothing in nerve's HTML may hold a key or a provider URL.

## What to do (HTML side only)
nerve has 29 panels in `D:\GitHub\nerve\source\html\`. Only TWO make real AI calls. Move both.

1. `chat.html` — the AI chat panel. Today it sends `type: 'ai_chat'` to nerve's Rust side
   (`ai.rs`), which calls the provider. Provider list and selector are built from nerve's config.
2. `reconciliation.html` — line ~592 calls Ollama directly at 127.0.0.1:11434/api/generate to
   propose classifications.

For each, in the API workbench (`D:\GitHub\pipeline-workflows\API`, the ONE API folder):
- Create a station with the same shape as the existing ones: `station.json`, `PROMPT.md`,
  `FOCUS.md`, a template under `BACKSIDE\templates\`, and receipts in Runs. Copy the pattern of
  an existing small station; do not invent a new shape.
- Put them in a NEW station group on the API Calls section (see `Rust_AI_API\docs\WORKBENCH_UI_SPEC.md`
  §3). Working title: "Desk Tools" — David will rename it. Do NOT call it "pipeline" anything.
- The model call is Forge's `/v1/chat/completions`. For reconciliation, use Forge's local route
  (Ollama) by route name, not by hitting Ollama directly.
- Keep the page behavior: chat keeps streaming and follow-up messages; reconciliation keeps its
  draft-in-localStorage behavior.

In nerve, leave only a launcher: a hotkey / menu item that opens the Forge page in a panel.
Delete the AI logic from both HTML files. No provider URLs, no keys, no fetch to :11434.

## Rules that must survive the move (from David's Aug 26 canon ruling)
- Reconciliation output stays visibly AI-PROPOSED and editable. Nothing enters the canon graph
  except through the existing **Stage Candidate** button and its receipt. The AI step never stages.
- Nerve's `canon.rs` is NOT an AI call. Do not touch it.

## Do NOT touch
- `task-calendar.html` and `task-merger-*.html`: they only MENTION api.anthropic.com in task
  text. They make no AI calls. (An earlier scan flagged them wrongly.)
- `capsule.html` and `toolbar.html`: the select-text popup. They send `selection_run` to the Rust
  side, which may run an AI workflow. They make no provider calls themselves. Leave them; they
  start going through Forge when the Rust-side prompt lands.
- `7q-engine.html`, `atom-builder.html`, `workbench.html`, the `atoms\` folder (56 files) and the
  `data\` folder (21 files): checked 2026-10-23 (recursive scan: provider URLs, chat/model
  patterns, nerve AI message types) and none make AI calls. They are workbenches, prototypes,
  dashboards and viewers. Do not move or edit them. If you find an AI call the scan missed
  (`data\Lean.html` has 3 fetches with no recognizable host: check it), STOP and report it
  instead of editing.
- `reconciliation.html` also talks to the canon engine at 127.0.0.1:8765. That is the canon
  service, not an AI call. Keep that behavior exactly as it is.
- `ai.rs`, `config.rs` provider defaults, `main.rs` AI handlers: Rust side is a separate prompt.
  Until it lands, nothing in the HTML should still send `ai_chat`.
- hotstrings / hotkeys: separate prompt.

## Where the work lands
- Station files: `D:\GitHub\pipeline-workflows\API`. Page changes (only if the new group needs
  registering): `Rust_AI_API\ui\api-workspace.html`. Check how groups are registered first.
- Rust_AI_API: branch from `claude/magical-shannon-t21l61` (PR #3). The default branch is stale
  and has none of the workbench. Do not commit to the default branch.
- nerve: new branch off the current one. Draft PRs only. Do not push to main/master anywhere.

## Done means
1. `grep` over `nerve\source\html\*.html` finds zero provider URLs, zero ':11434', zero api keys.
2. Both stations appear under the new group, each with Code / Template / Ask / Runs tabs.
3. One real call from each station through Forge shows up as a receipt in Runs.
4. If Forge is down, the page says so plainly. It never falls back to calling a provider.
5. Acceptance on David's machine is PENDING; write that in each PR description.

## Read receipt (first message back, before any edit)
Restate in your own words: the one-route rule, why `canon.rs` and the task panels are untouched,
and what "leave only a launcher" means. If any restatement is wrong, stop and ask.
