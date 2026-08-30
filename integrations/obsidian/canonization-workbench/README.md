# Canonization Workbench for Obsidian

This integration embeds Nerve's Atom Builder as the canonization authoring interface in an Obsidian right-side pane.

It provides focused Semantic AI passes for:

- nondiscriminatory structural discovery;
- classification and burden;
- reconciliation and translation;
- claims;
- definitions and boundaries;
- mathematics and notation;
- theology and Scripture;
- bridges and translations;
- truth predicates and Why-Closure;
- full canonization.

The **Run the three-stage candidate pipeline** choice makes three distinct API calls in order: discovery, classification, then reconciliation. Each later stage receives the prior candidate output as non-authoritative context. The same passes are available for the current note and from the Obsidian file/folder context menu. Folder work is bounded to two concurrent calls and produces candidate-only receipts.

## Install

Copy this folder into:

`.obsidian/plugins/canonization-workbench`

Then run `sync-nerve-interface.ps1` to generate the byte-identical embedded copy of `source/html/atoms/atom-builder.html`. Enable or reload the plugin in Obsidian. The Nerve sidebar supports editable-draft JSON export/import; validated candidate packets are preserved as JSON and projected into readable Obsidian notes, but are never reverse-hydrated automatically.

Do not commit `data.json`; it contains vault-local settings. The generated `nerve/atom-builder.html` is also intentionally excluded from the integration source because the authoritative file is already tracked at `source/html/atoms/atom-builder.html`.

## Authority boundary

All outputs are candidate drafts. The plugin cannot create a canonical admission event.
