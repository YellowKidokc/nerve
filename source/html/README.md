# Nerve HTML surfaces

The runtime copies this tree recursively. Keep each page in the folder for the
capability it presents; do not return new pages to the root.

| Folder | Responsibility |
|---|---|
| `atoms/` | Atom authoring, candidate review, reconciliation, capsules, and governed question engines |
| `stratum/` | Stratum actions, shortcuts, and the selection toolbar |
| `clipboard/` | Clipboard interfaces and retained variants |
| `prompts/` | Prompt selection, prompt surfaces, and chat |
| `research/` | Research and research-link surfaces |
| `tasks/` | Calendar and dated task-merger interfaces |
| `hubs/` | General, Nexus, and Theophysics dashboards/hubs |
| `system/` | Settings and TTS engine |
| `data/` | Page data loaded by HTML surfaces |

## Authority boundary

`atoms/atom-builder.html` is the authoritative Nerve Atom Builder interface.
The Obsidian plugin receives a byte-identical generated copy through
`integrations/obsidian/canonization-workbench/sync-nerve-interface.ps1`.
Moving a page does not alter its canon or admission authority.

## Runtime paths

Panel paths are registered relative to the installed `html/` directory, for
example `atoms/atom-builder.html` and `stratum/shortcuts.html`. Installation
scripts preserve this directory tree.
