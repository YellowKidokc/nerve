# Canonization interface contract

## Governing direction

```text
Nerve Atom Builder HTML
→ editable Nerve draft JSON
→ validated candidate JSON
→ generated Obsidian Properties and Markdown projection
→ later index/database consumers
```

## Authority

- The Nerve Atom Builder is the authoring interface and schema compiler.
- Validated JSON is the full-fidelity candidate interchange record.
- Obsidian Properties and Markdown are readable generated projections.
- Neither the interface, JSON export, nor an Obsidian projection creates canonical admission.
- A separate signed human admission event remains mandatory.

## Round trip

- Editable Nerve draft JSON can be exported and imported back into the Nerve interface.
- Validated candidate JSON is preserved in Obsidian and projected into Properties.
- A validated packet is not reverse-hydrated into editable fields because doing so would guess field ownership and could overwrite provenance.
- A future governed amendment operation may round-trip human-owned projection changes after it has explicit ownership, conflict, version, and receipt rules.

## Runtime copy

Obsidian loads a byte-identical generated copy of `D:\GitHub\nerve\source\html\atom-builder.html`. Run `sync-nerve-interface.ps1` after changing the authoritative Nerve source. The synchronization fails unless SHA-256 hashes match.
