# Nerve interface projection

`atom-builder.html` in this folder is a generated, byte-identical runtime copy of:

`D:\GitHub\nerve\source\html\atom-builder.html`

The Nerve source remains authoritative. This copy exists only so Obsidian can render the same interface in a right-side pane without Docker, an HTTP server, or the private `nerve://` desktop protocol.

Run `sync-nerve-interface.ps1` after changing the Nerve source. The script copies the file and refuses success unless the SHA-256 hashes match.

JSON is the full-fidelity interchange record. Obsidian Properties and Markdown notes are generated projections, not a second source of truth and not admission events.
