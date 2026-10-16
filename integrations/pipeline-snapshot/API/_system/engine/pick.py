"""Pick list: choose which notes of a folder go to the API, the same way in every folder.

    <any folder>/_PICK.md          one line per note:  - [ ] [[note name]]   (date · title)
                                   tick the ones you want ( - [x] ), in Obsidian or any editor
    <Channel>/_PICK.md             a channel folder (it has Clean MD/, Channel Summary/, Prompts/): the list sits in
                                   the channel folder and lists the notes in Clean MD/
    @list.txt                      an item starting with @ is a file of note paths, one per line (always run)

Rules every API station follows (resolve() below):
  - a file named directly is always run (you chose it);
  - a folder with a _PICK.md runs only the ticked notes;
  - a folder without one runs everything if it holds at most PICK_FREE notes; above that it writes a _PICK.md,
    runs nothing from that folder and says so, so a 300-video channel never goes to the API by accident;
  - "_PICK.md" with a line "ALL" (on its own) runs the whole folder.

Re-running write_pick() adds new notes unticked and keeps every tick. Standard titling renames notes and
updates [[links]] in the same folder, so ticks survive the rename.

    python -m engine.pick <folder>            write/refresh the list and print it
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

PICK = "_PICK.md"
PICK_FREE = 10                    # folders this small run whole without a pick list
LINE = re.compile(r"^\s*-\s*\[([ xX])\]\s*\[\[([^\]|]+)(?:\|[^\]]*)?\]\]")


CLEAN = "Clean MD"                # a channel's converted-to-markdown notes (see channel-folder layout)


def source_dir(folder: Path) -> Path:
    """Where a folder's notes are: <Channel>/Clean MD when the folder is a channel folder, else the folder."""
    return folder / CLEAN if (folder / CLEAN).is_dir() else folder


def notes(folder: Path) -> list[Path]:
    """The source notes of a folder: .md files, not index/scorecard/underscore files."""
    folder = source_dir(folder)
    return sorted(n for n in folder.glob("*.md") if not n.name.startswith(("_", ".")) and " - 000 " not in n.name)


def pick_file_for(note: Path) -> Path | None:
    """The _PICK.md that lists this note, if any (same folder, or the channel folder above Clean MD)."""
    for f in (note.parent / PICK, note.parent.parent / PICK if note.parent.name == CLEAN else None):
        if f and f.is_file(): return f
    return None


def by_date(found: list[Path]) -> list[Path]:
    """Newest first (upload_date / date in the YAML); undated notes after them, A to Z."""
    rows = [(_describe(n)[0], n.name.lower(), n) for n in found]
    dated = sorted((r for r in rows if r[0]), key=lambda r: r[0], reverse=True)
    return [r[2] for r in dated + sorted(r for r in rows if not r[0])]


def read_list(listfile: Path) -> list[Path]:
    return [Path(l.strip().strip('"')) for l in listfile.read_text(encoding="utf-8-sig").splitlines() if l.strip()]


def _field(text: str, key: str) -> str:
    m = re.search(rf'^{key}:\s*"?(.*?)"?\s*$', text[:3000], re.M)
    return m.group(1).strip() if m else ""


def _describe(note: Path) -> tuple[str, str, int]:
    """(date, title, words): words from the YAML, else counted; it is what a note costs."""
    try:
        text = note.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return "", note.stem, 0
    head = text[:3000]
    date = next((_field(head, k)[:10] for k in ("upload_date", "date", "published", "created")
                 if re.match(r"\d{4}-\d{2}-\d{2}", _field(head, k))), "")
    words = int(_field(head, "words")) if _field(head, "words").isdigit() else len(text.split())
    return date, _field(head, "title") or note.stem, words


def _same(a: str, b: str) -> bool:
    squash = lambda t: re.sub(r"[^a-z0-9]", "", t.lower())
    return squash(a) == squash(b)


def read_pick(folder: Path) -> tuple[set[str], bool] | None:
    """(ticked note names, run-all flag), or None when the folder has no pick list."""
    f = folder / PICK
    if not f.is_file():
        return None
    ticked, run_all = set(), False
    for line in f.read_text(encoding="utf-8").splitlines():
        if line.strip().upper() == "ALL":
            run_all = True
        m = LINE.match(line)
        if m and m.group(1).lower() == "x":
            ticked.add(m.group(2).strip())
    return ticked, run_all


def write_pick(folder: Path) -> Path:
    """Write or refresh <folder>/_PICK.md, newest first, keeping existing ticks."""
    old = read_pick(folder)
    ticked, run_all = old if old else (set(), False)
    rows = [(*_describe(n), n.stem) for n in by_date(notes(folder))]
    lines = ["# Pick list", "",
             "Tick ( `- [x]` ) the notes the API stations should run. Unticked notes are skipped, so they cost nothing.",
             "Put a line with just `ALL` here to run the whole folder. New notes are added unticked when this list is refreshed.", ""]
    if run_all:
        lines += ["ALL", ""]
    total = sum(r[2] for r in rows)
    lines.insert(2, f"{len(rows)} notes · {total / 1000:,.0f}k words in all.")
    lines.insert(3, "")
    for date, title, words, stem in rows:
        info = " · ".join(x for x in (date, f"{words / 1000:.1f}k words", "" if _same(title, stem) else title[:80]) if x)
        lines.append(f"- [{'x' if stem in ticked else ' '}] [[{stem}]]  ({info})")
    out = folder / PICK
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return out


def resolve(items: list[str], default: Path | None = None, limit: int | None = None, quiet: bool = False) -> list[Path]:
    """The notes a station should run, after the pick-list rules. Folders are read one level deep, except a
    station INBOX (lanes 00_PRIORITY/01_SERIES/02_GENERAL and their sub-folders), which is read recursively."""
    say = (lambda *a: None) if quiet else (lambda *a: print(*a, file=sys.stderr))
    out: list[Path] = []
    for raw in items or ([str(default)] if default else []):
        if raw.startswith("@"):
            out += read_list(Path(raw[1:]))
            continue
        p = Path(raw).expanduser()
        if p.is_file():
            out.append(p)
            continue
        if not p.is_dir():
            say(f"not found: {p}")
            continue
        folders = [p, *sorted(d for d in p.rglob("*") if d.is_dir() and not d.name.startswith(("_", ".")))] \
            if p.name == "INBOX" else [p]
        for folder in folders:
            found = notes(folder)
            if not found:
                continue
            if "00_PRIORITY" in folder.parts:          # dropped in priority = ticked; it sorts first, so runs first
                say(f"PRIORITY  {folder.name}: {len(found)} note(s)")
                out += found
                continue
            pick = read_pick(folder)
            if pick is None and len(found) > PICK_FREE:
                f = write_pick(folder)
                say(f"PICK  {folder.name}: {len(found)} notes and no pick list. Nothing sent to the API from this folder.\n"
                    f"      Tick what you want in {f} (or add a line ALL), then run again.")
                continue
            if pick is not None and not pick[1]:
                chosen = [n for n in found if n.stem in pick[0]]
                say(f"PICK  {folder.name}: {len(chosen)} of {len(found)} ticked")
                found = chosen
            out += found
    return out[:limit] if limit else out


if __name__ == "__main__":
    for raw in sys.argv[1:]:
        f = write_pick(Path(raw))
        print(f)
        print(f.read_text(encoding="utf-8"))
