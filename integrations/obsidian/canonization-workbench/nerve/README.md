# Nerve interface projection

`atom-builder.html` and its local JavaScript/CSS dependencies in this folder are generated, byte-identical runtime copies of files beside:

`D:\GitHub\nerve\source\html\atoms\atom-builder.html`

The Nerve source remains authoritative. This copy exists only so Obsidian can render the same interface in a right-side pane without Docker, an HTTP server, or the private `nerve://` desktop protocol.

Run `sync-nerve-interface.ps1` after changing the Nerve source. The script copies the interface bundle and refuses success unless every SHA-256 hash matches.

JSON is the full-fidelity interchange record. Obsidian Properties and Markdown notes are generated projections, not a second source of truth and not admission events.
