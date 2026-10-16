"""The one script behind every front-folder button. Each .bat passes the folder it sits in:

    1 RUN HERE.bat       python button.py here   "<its folder>"
    2 RUN ON FOLDER.bat  python button.py folder "<its folder>"

The folder says the rest: its BACKSIDE/station.json names the station, and a station that sits inside another
station's OUTBOX is a LAYER (010 theology inside 020_CKG/OUTBOX).

Base station (020_CKG), in order:
  1 where    here = its INBOX (every channel / folder dropped in it, and loose notes); folder = asks (inside or outside)
  2 prepare  a YouTube channel folder gets Clean MD / Prompts / Channel Summary; raw transcripts are cleaned
             (clean_library.py, local); the X list (_PICK.md) is written or refreshed
  3 which    ticked notes, or "how many?" (engine/ask.choose)
  4 title    notes that do not carry their standard title yet get one (a titled note is recognised and skipped)
  5 run      the station on those notes; for 020 the deep CKG companion. Results go FLAT into its OUTBOX,
             answers onto each note (publish_analysis), scriptures into the note's YAML
  6 layers   offers the layer stations in its OUTBOX, run on the same notes
Layer station: here = the notes its parent just ran (<parent>/OUTBOX/_last_run.txt); folder = asks. Its results go
into its own OUTBOX.
"""
from __future__ import annotations

import importlib.util
import json
import re
import shutil
import subprocess
import sys
from datetime import datetime
from pathlib import Path

SYSTEM = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SYSTEM))
from engine import note as N, pick                                  # noqa: E402
from engine.ask import ask, choose, run_guarded                                   # noqa: E402
from engine.paths import station_dir, station_rows                   # noqa: E402
from engine.publish import publish_on_note                           # noqa: E402

MAIN = SYSTEM.parent
ACTIONS = MAIN / "_ACTIONS" / "actions"
LAST = "_last_run.txt"
YT_RAW = (".srt", ".vtt")
SUBFOLDERS = ("Clean MD", "Prompts", "Channel Summary")
LANES = {"00_PRIORITY", "01_SERIES", "02_GENERAL", "02_GROUP"}
PRIORITY = "00_PRIORITY"


def action(name: str):
    spec = importlib.util.spec_from_file_location(f"action_{name}", ACTIONS / f"{name}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def station_of(front: Path) -> dict:
    return json.loads((front / "BACKSIDE" / "station.json").read_text(encoding="utf-8"))


def parent_station(front: Path) -> Path | None:
    """The base station whose OUTBOX holds this one (for a layer), else None."""
    box = front.parent
    return box.parent if box.name == "OUTBOX" and (box.parent / "BACKSIDE" / "station.json").is_file() else None


# ------------------------------------------------------------------ 2 prepare

def raw_transcripts(folder: Path) -> list[Path]:
    """Downloaded transcripts not cleaned yet: .srt/.vtt, or .md with a video id and no `cleaned:` / `converted:` field
    (`converted:` = the Conversion Station already made it readable prose; the old cleaner would undo that)."""
    raw = []
    for f in folder.iterdir():
        if not f.is_file() or f.name.startswith("_"):
            continue
        if f.suffix.lower() in YT_RAW:
            raw.append(f)
        elif f.suffix.lower() == ".md":
            head = f.read_text(encoding="utf-8", errors="replace")[:1500]
            if ("video_id:" in head or "**Video ID:**" in head) and not re.search(r"^(cleaned|converted):", head, re.M):
                raw.append(f)
    return raw


def prepare(folder: Path) -> None:
    """A YouTube channel folder: make Clean MD / Prompts / Channel Summary and clean what is still raw (local)."""
    raw = raw_transcripts(folder)
    if not raw and not (folder / "Clean MD").is_dir():
        return                                         # a folder of clean notes or papers: nothing to prepare
    for sub in SUBFOLDERS:
        (folder / sub).mkdir(exist_ok=True)
    if raw:
        print(f"  prepare {folder.name}: {len(raw)} raw transcript(s): {action('clean').run_folder(folder).get('say', '')}")


def units(root: Path) -> list[Path]:
    """The folders under an INBOX that hold something to run: channel folders, groups, lanes with loose notes."""
    found = [root] if any(root.glob("*.md")) else []
    for d in sorted(p for p in root.rglob("*") if p.is_dir()):
        rel = d.relative_to(root).parts
        if any(part in SUBFOLDERS or part.startswith(("_", ".")) for part in rel):
            continue
        if any(d.glob("*.md")) or any(d.glob("*.srt")) or any(d.glob("*.vtt")) or (d / "Clean MD").is_dir():
            found.append(d)
    return found


# ------------------------------------------------------------------ 3-4 which, title

def gather(sources: list[Path]) -> list[Path]:
    notes: list[Path] = []
    for src in sources:
        if src.is_file():
            notes.append(src)
            continue
        prepare(src)
        chosen = choose(src)
        if chosen:
            notes += chosen
    return list(dict.fromkeys(notes))


def priority_notes(sources: list[Path]) -> list[Path]:
    """Everything in INBOX/00_PRIORITY: putting it there is the tick, so no pick list and no "how many?"."""
    notes: list[Path] = []
    for src in sources:
        prepare(src)
        notes += pick.by_date(pick.notes(src))
    return list(dict.fromkeys(notes))


def titled(note: Path) -> bool:
    return N.fields(note.read_text(encoding="utf-8", errors="replace")).get("std_title") == note.stem


def title_first(notes: list[Path]) -> list[Path]:
    todo = [n for n in notes if not titled(n)]
    if not todo:
        return notes
    print(f"\n  title: {len(todo)} of {len(notes)} note(s) not titled yet (~1.5k tokens each)")
    t = action("title")
    renamed = {}
    for n in todo:
        r = t.run(n, n.read_text(encoding="utf-8"))
        print(f"    {r.get('say', '')[:120]}")
        if r.get("note"):
            renamed[n] = Path(r["note"])
    return [renamed.get(n, n) for n in notes]


# ------------------------------------------------------------------ 5 run

def deep_ckg(front: Path, notes: list[Path], out: Path, workers: int, focus: list[str]) -> int:
    """The deep CKG companion on these notes, one engine run per note, `workers` at a time. Each note is FINISHED the
    moment its CKG is done (flat '<note> · CKG.md' in `out`, scriptures in its YAML, the answer on the note), so an
    interruption loses nothing that was finished. A note whose CKG is already in `out` is skipped (unless there are
    new questions)."""
    import threading
    from concurrent.futures import ThreadPoolExecutor, as_completed
    runner = SYSTEM / "vendor" / "evidence" / "SCRIPTS" / "turbo_pipeline_runner.py"
    todo = [n for n in notes if focus or not (out / "CKG" / f"{n.stem} · CKG.md").is_file()]
    for n in notes:
        if n not in todo:
            print(f"    already done, skipped: {n.name[:100]}")
    if not todo:
        return 0
    flags = getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0)      # a stray Ctrl+C never reaches the engine
    procs: list[subprocess.Popen] = []
    lock = threading.Lock()
    scripture = action("scripture")
    counter = {"done": 0}

    def one(n: Path) -> int:
        work = front / "BACKSIDE" / "_deep" / _slug(n.name)[:40]
        (work / "INBOX").mkdir(parents=True, exist_ok=True)
        text = n.read_text(encoding="utf-8", errors="replace")      # the source only: our analysis block stays out
        text = re.sub(r"<!-- (analysis|analysis-detail|scorecard):start -->.*?<!-- \1:end -->\n?", "", text, flags=re.S)
        (work / "INBOX" / n.name).write_text(text, encoding="utf-8")
        started = datetime.now().timestamp()
        proc = subprocess.Popen([sys.executable, "-u", str(runner), "--root", str(work), "--workers", "1",
                                 "--provider", "deepseek", *[a for q in focus for a in ("--focus", q)]],
                                cwd=runner.parent, creationflags=flags)
        with lock:
            procs.append(proc)
        code = proc.wait()
        hits = sorted((p for p in (work / "OUTBOX").rglob("*_C1_*.md") if p.stat().st_mtime >= started - 5),
                      key=lambda p: p.stat().st_mtime)
        with lock:
            counter["done"] += 1
            k = counter["done"]
        if not hits:
            say(f"[{k}/{len(todo)}] no CKG came back for {n.name[:90]}")
            return code or 1
        dest = out / "CKG" / f"{n.stem} · CKG.md"                    # dump it now, in order: file, YAML, note, latest
        archive(dest, out)
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(hits[-1], dest)
        r = scripture.run(n, n.read_text(encoding="utf-8"))
        n.write_text(N.set_fields(n.read_text(encoding="utf-8"), r["yaml"]), encoding="utf-8")
        publish_on_note([n], [out, out / "CKG"])
        write_latest(n, out)
        say(f"[{k}/{len(todo)}] finished and on the note: {n.name[:90]}")
        return code

    code = 0
    pool = ThreadPoolExecutor(max_workers=max(1, min(workers, len(todo))))
    futures = [pool.submit(one, n) for n in todo]
    pending = set(futures)
    while pending:
        try:
            for f in as_completed(pending):
                pending.discard(f)
                code |= f.result() or 0
        except KeyboardInterrupt:
            if ask("\nCtrl+C received. Type stop to stop this run, or press Enter to keep going:").lower() == "stop":
                for proc in procs:
                    if proc.poll() is None:
                        proc.terminate()
                pool.shutdown(wait=False, cancel_futures=True)
                say("stopped. Notes already finished keep their results; run again to do the rest.")
                return 130
            print("  keeping going", flush=True)
    pool.shutdown()
    return code


def archive(path: Path, out: Path) -> None:
    """A file about to be replaced moves to <out>/_older/<date>/ (never deleted)."""
    if not path.is_file():
        return
    folder = out / "_older" / datetime.now().strftime("%Y-%m-%d")
    folder.mkdir(parents=True, exist_ok=True)
    target = folder / path.name
    if target.exists():
        target = folder / f"{path.stem} · {datetime.now():%H%M%S}{path.suffix}"
    shutil.move(str(path), str(target))


def write_latest(note: Path, out: Path) -> None:
    """<out>/<note> · ANALYSIS.md: the newest full analysis of the note (CKG + every layer), the same as the block on
    the note, rewritten after every run. The version it replaces goes to _older/."""
    text = note.read_text(encoding="utf-8", errors="replace")
    m = re.search(r"<!-- analysis:start -->(.*?)<!-- analysis:end -->", text, re.S)
    if not m:
        return
    dest = out / f"{note.stem} · ANALYSIS.md"
    archive(dest, out)
    head = (f"---\nnote: \"[[{note.stem}]]\"\nsource_path: {json.dumps(str(note))}\n"
            f"updated: {datetime.now():%Y-%m-%d %H:%M}\n---\n\n# {note.stem}\n\n")
    d = re.search(r"<!-- analysis-detail:start -->(.*?)<!-- analysis-detail:end -->", text, re.S)   # bottom-of-page cards
    dest.write_text(head + m.group(1).strip() + "\n" + (f"\n{d.group(1).strip()}\n" if d else ""), encoding="utf-8")


def _slug(name: str) -> str:
    s = re.sub(r"[^\w\-]", "_", Path(name).stem)
    s = re.sub(r"_+", "_", s).strip("_")
    return s[:80] if s else "untitled_paper"


def run_station(front: Path, st: dict, notes: list[Path], out: Path, workers: int, focus: list[str]) -> int:
    (front / "OUTBOX").mkdir(exist_ok=True)
    out.mkdir(parents=True, exist_ok=True)
    listfile = front / "OUTBOX" / LAST                 # every station keeps the list of notes it last ran
    listfile.write_text("\n".join(map(str, notes)) + "\n", encoding="utf-8")
    try:                                               # every input lands in ORIGINALS once
        from engine.originals import archive
        archive(notes, st.get("label", ""))
    except Exception as e:
        print(f"  originals: skipped ({e})")
    say(f"{st['label']}: {len(notes)} note(s), {workers} at once, results -> {out}"
        + (f", looking for: {' | '.join(focus)}" if focus else ""))
    if st["number"] == "20":
        return deep_ckg(front, notes, out, workers, focus)
    if st.get("kind") == "chain":                       # one set of papers through several stations, in order, one OUTBOX
        code = 0
        for i, number in enumerate(st["chain"], 1):
            row = next(r for r in station_rows() if r["number"] == number)
            step = station_dir(row["label"]).parent      # that station's front folder, wherever it was moved
            say(f"step {i}/{len(st['chain'])}: {row['label']}")
            code |= run_station(step, station_of(step), notes, out, workers, focus)
        return code
    if st.get("kind") == "bundle":                      # several passes per note, notes in parallel, folders as chosen here
        from engine.bundle import run_bundle
        return run_bundle(front, st, notes, out, workers, focus, lambda n, dirs: publish_on_note([n], dirs),
                          lambda n, o: None)             # the ANALYSIS.md rewrite happens once, in main(), in the right folder
    row = next(r for r in station_rows() if r["number"] == st["number"])
    if st.get("kind") == "legacy":                      # a wrapped older tool: it reads its own inputs, run as before
        cmd = [sys.executable, str(SYSTEM / "engine" / "menu.py"), st["number"], "--yes", "--workers", str(workers)]
        if st.get("items"):
            cmd += [a for n in notes for a in ("--item", str(n))]
        else:
            say("legacy station: its configured evidence/data root is used; selected notes are not forwarded")
    else:                                               # engine station: these notes, flat copies into `out`
        cmd = [sys.executable, "-u", str(station_dir(row["label"]) / row["script"]), f"@{listfile}",
               "--outbox", str(out), "--workers", str(workers)] + (["--focus", "; ".join(focus)] if focus else [])
        if st["number"] in ("48", "49"):
            topic = ask("What topic should this station synthesize? ")
            if not topic:
                print("A topic is required.")
                return 2
            cmd += ["--topic", topic]
    return run_guarded(cmd, MAIN)


def say(line: str) -> None:
    print(f"\n[{datetime.now():%H:%M:%S}] {line}", flush=True)


def ask_int(question: str, default: int, low: int, high: int) -> int:
    raw = ask(f"{question} ({low}-{high}, Enter = {default}) ")
    return max(low, min(high, int(raw))) if raw.isdigit() else default


def ask_focus() -> list[str]:
    focus = []
    for i in range(5):
        extra = ask(f"  Anything else to look for in this pass? ({i + 1}/5, Enter = done) ")
        if not extra:
            break
        focus.append(extra)
    return focus


def lean_here(front: Path, st: dict) -> int:
    """A Lean station reads its own INBOX (00_PRIORITY, 01_SERIES, 02_GENERAL; .lean as well as .md), so the notes are not
    gathered here: it runs on its INBOX, then offers the layers in its OUTBOX (050 congruence matrix, 051, 052...)."""
    workers = ask_int("How many at once?", 8, 1, 30)
    focus = ask_focus()
    if ask(f"\nRun {st['label']} on its INBOX, {workers} at once? [Y/n] ").lower() in ("n", "no"):
        return 1
    row = next(r for r in station_rows() if r["number"] == st["number"])
    say(f"{st['label']}: its INBOX, {workers} at once")
    code = run_guarded([sys.executable, "-u", str(station_dir(row["label"]) / row["script"]), "--workers", str(workers)]
                       + (["--focus", "; ".join(focus)] if focus else []), MAIN)
    layers = sorted(d for d in (front / "OUTBOX").iterdir() if d.is_dir() and (d / "BACKSIDE" / "station.json").is_file())
    if layers:
        menu = "  ".join(f"{i} {d.name.split('_', 1)[1].replace('_', ' ').title()}" for i, d in enumerate(layers, 1))
        picked = ask(f"\nAlso run a layer? {menu}  (numbers, Enter = none) ").replace(",", " ").split()
        for i, d in enumerate(layers, 1):
            if str(i) in picked:
                print(f"\n{d.name}")
                code |= run_station(d, station_of(d), [], d / "OUTBOX", workers, ask_focus())
    say(f"done. Results: {front / 'OUTBOX'}")
    return code


# ------------------------------------------------------------------ main

def main(mode: str, front: Path) -> int:
    st = station_of(front)
    base = parent_station(front)
    title = f"{st['label']}" + (f"  (layer of {station_of(base)['label']})" if base else "")
    print(f"\n{title}\n{'=' * len(title)}")
    if st.get("items") == "lean" and mode == "here" and (not base or st.get("own_inbox")):
        return lean_here(front, st)
    if base and mode == "here" and not st.get("own_inbox"):
        listed = base / "OUTBOX" / LAST
        notes = [p for p in pick.read_list(listed) if p.is_file()] if listed.is_file() else []
        if not notes:
            print(f"No notes yet: run {station_of(base)['label']} first (its list is {listed}).")
            return 0
        if ask(f"Run on the {len(notes)} note(s) {station_of(base)['label']} just did? [Y/n] ").lower() in ("n", "no"):
            return 1
    else:
        if mode == "here":
            sources = units(front / "INBOX")
            if not sources:
                print(f"Nothing in {front / 'INBOX'}. Drop a channel folder, a folder of notes, or notes in it.")
                return 0
        else:
            raw = ask("Where is the folder? (drag a folder or a note here) ")
            sources = [Path(raw)] if raw else []
            if not sources or not sources[0].exists():
                print(f"Not found: {raw}")
                return 2
        say("scanning")
        first = [s for s in sources if PRIORITY in s.parts]      # dropped in 00_PRIORITY = ticked: runs first, no questions
        notes = priority_notes(first)
        rest = [s for s in sources if s not in first]
        if notes:
            say(f"priority: {len(notes)} note(s) from {PRIORITY}, these run first")
            if rest and ask(f"Also go through the rest of the INBOX ({len(rest)} folder(s))? [y/N] ").lower() in ("y", "yes"):
                notes += [n for n in gather(rest) if n not in notes]
        else:
            notes = gather(rest)
        if not notes:
            return 0
    where = ask(f"Where do you want the output? (Enter = {front / 'OUTBOX'}; answers always also go on each note) ")
    out = Path(where) if where else front / "OUTBOX"
    workers = ask_int("How many at once?", 8, 1, 30)
    focus = ask_focus()                                 # up to 5 extra questions for this pass
    if ask(f"\nRun {st['label']} on {len(notes)} note(s), {workers} at once? [Y/n] ").lower() in ("n", "no"):
        return 1
    on_note = st.get("on_note", True)                  # false: the notes are only read (series sheet): no title, no YAML, no publish
    if not base and on_note:
        say("title: notes without their standard title")
        notes = title_first(notes)
    code = run_station(front, st, notes, out, workers, focus)
    if not on_note:                                   # read-only station: nothing is written onto the source files
        say(f"done. Results: {out}")
        return code
    say("scriptures into each note's YAML")
    scripture = action("scripture")
    for n in notes:
        if n.is_file():
            r = scripture.run(n, n.read_text(encoding="utf-8"))
            n.write_text(N.set_fields(n.read_text(encoding="utf-8"), r["yaml"]), encoding="utf-8")
    layers = sorted(d for d in (front / "OUTBOX").iterdir() if d.is_dir() and (d / "BACKSIDE" / "station.json").is_file())
    looked = [out]
    if layers and not base:
        menu = "  ".join(f"{i} {d.name.split('_', 1)[1].replace('_', ' ').title()}" for i, d in enumerate(layers, 1))
        picked = ask(f"\nPut a second layer on these notes? {menu}  (numbers, Enter = none) ").replace(",", " ").split()
        for i, d in enumerate(layers, 1):
            if str(i) in picked:
                print(f"\n{d.name}")
                layer_out = d / "OUTBOX" if out == front / "OUTBOX" else out / d.name
                code |= run_station(d, station_of(d), notes, layer_out, workers, ask_focus())
                looked.append(layer_out)
    say("onto each note: the CKG, then each layer under it, above the transcript")
    publish_on_note(notes, looked + [out / "CKG"])
    for n in notes:                                   # the newest full version of each note, in the OUTBOX root
        if n.is_file():
            write_latest(n, out if base is None else base / "OUTBOX")
    if not base:                                      # every CKG's Story Bank, gathered into one searchable file
        subprocess.run([sys.executable, str(SYSTEM / "tools" / "story_bank.py"), "--root", str(out)])
    say(f"done. Results: {out}   Answers: on each note.")
    return code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1] if len(sys.argv) > 1 else "here",
                          Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else Path.cwd()))
