"""The "2 RUN ON FOLDER" front door every station shares. It asks, in order:

  1  Where are the notes?      drag a channel folder, a folder or one note in (Enter = this station's INBOX)
  2  Which ones?               a pick list with ticks -> "122 ticked of 390, run those?"
                               no pick list          -> "390 notes. How many?" (newest N / all / p = write a pick
                                                        list to tick first and stop)
  3  Where do answers go?      Enter = onto each note (publish_analysis); or type a folder
  4  Confirm, then run         papers in parallel; the station's own call limits apply
  5  Anything else?            up to 5 things to look for (they go in as --focus), then which extra passes to
                               run on the same notes (config/followups.json: theology, physics, argument grade...)

    python _system/engine/ask.py 57
"""
from __future__ import annotations

import json
import subprocess
import sys
from datetime import datetime
from pathlib import Path

SYSTEM = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SYSTEM))
from engine import pick                                      # noqa: E402

MAIN = SYSTEM.parent
MAX_FOCUS = 5


def ask(question: str, default: str = "") -> str:
    while True:
        try:
            answer = input(f"{question} ").strip().strip('"')
        except EOFError:
            answer = ""
        except KeyboardInterrupt:           # a stray Ctrl+C (dictation sends "copy") never answers a question
            print("\n  (Ctrl+C ignored: type your answer)")
            continue
        return answer or default


def run_guarded(cmd: list[str], cwd) -> int:
    """Run a long step so a stray Ctrl+C cannot kill it: the child gets its own console group (Ctrl+C does not reach
    it), and a Ctrl+C here asks before stopping. Only typing `stop` stops the run."""
    flags = getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0)
    proc = subprocess.Popen(cmd, cwd=cwd, creationflags=flags)
    while True:
        try:
            return proc.wait()
        except KeyboardInterrupt:
            if ask("\nCtrl+C received. Type stop to stop this run, or press Enter to keep going:").lower() == "stop":
                proc.terminate()
                return 130
            print("  keeping going", flush=True)


def stations() -> dict[str, dict]:
    return {s["number"]: s for s in json.loads((SYSTEM / "config" / "stations.json").read_text(encoding="utf-8"))}


def script_of(s: dict) -> Path:
    from engine.paths import station_dir                    # finds a station even after it was moved by hand
    return station_dir(s["label"]) / s["script"]


def choose(source: Path) -> list[Path] | None:
    """Step 2. None = stop (a pick list was written for ticking)."""
    if source.is_file():
        return [source]
    found = pick.by_date(pick.notes(source))
    if not found:
        print(f"No notes in {pick.source_dir(source)}")
        return None
    listing = pick.read_pick(source)
    if listing:
        ticked, run_all = listing
        chosen = found if run_all else [n for n in found if n.stem in ticked]
        if chosen:
            label = "ALL" if run_all else f"{len(chosen)} ticked"
            if ask(f"Pick list: {label} of {len(found)}. Run those? [Y/n]", "y").lower() in ("y", "yes"):
                return chosen
    default = "all" if len(found) <= pick.PICK_FREE else "p"
    answer = ask(f"{len(found)} notes in {source.name}. How many? [a number = newest N · all · p = make a pick list to tick]"
                 f" ({default})", default).lower()
    if answer in ("all", "a"):
        return found
    if answer.isdigit():
        return found[:int(answer)]
    f = pick.write_pick(source)
    print(f"Pick list written: {f}\nTick the notes you want (or add a line ALL), then run this again.")
    subprocess.run(["cmd", "/c", "start", "", str(f)], check=False) if sys.platform == "win32" else None
    return None


def run(number: str) -> int:
    reg = stations()
    s = reg[number]
    front = script_of(s).parent.parent
    print(f"\n{s['label']}\n{'=' * len(s['label'])}")
    raw = ask(f"1 Where are the notes? (drag a folder or note here; Enter = {front.name}\\INBOX)", str(front / "INBOX"))
    source = Path(raw)
    if not source.exists():
        print(f"Not found: {source}"); return 2
    if source.name == "INBOX":                                 # the station's own inbox: its lanes, read recursively
        notes = pick.resolve([str(source)])
    else:
        notes = choose(source)
    if not notes:
        return 0
    dest = ask("3 Where do the answers go? [Enter = onto each note · or type a folder]", "")
    focus: list[str] = []
    stamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    listfile = SYSTEM / "STATE" / f"ask_{s['label']}_{stamp}.txt"
    listfile.parent.mkdir(parents=True, exist_ok=True)
    listfile.write_text("\n".join(map(str, notes)) + "\n", encoding="utf-8")
    where = "onto each note" if not dest else dest
    if ask(f"4 Run {s['label']} on {len(notes)} note(s), answers {where}? [Y/n]", "y").lower() not in ("y", "yes"):
        return 1
    code = station(s, listfile, dest, focus)
    # 5 follow-ups
    for i in range(MAX_FOCUS):
        extra = ask(f"5 Anything else to look for in these notes? ({i + 1}/{MAX_FOCUS}, Enter = done)")
        if not extra:
            break
        focus.append(extra)
    offers = followups(number, reg)
    if offers:
        menu = "  ".join(f"{n} {reg[n]['label'].split('_', 1)[1]}" for n in offers)
        picked = ask(f"  Run another pass on the same notes? {menu}  (numbers, Enter = none)")
        for n in picked.replace(",", " ").split():
            if n in reg:
                code |= station(reg[n], listfile, dest, focus)
    elif focus:
        code |= station(s, listfile, dest, focus)             # focus only: rerun this station with it
    return code


def followups(number: str, reg: dict) -> list[str]:
    f = SYSTEM / "config" / "followups.json"
    table = json.loads(f.read_text(encoding="utf-8")) if f.exists() else {}
    return [n for n in table.get(number, table.get("default", [])) if n in reg and n != number]


def station(s: dict, listfile: Path, dest: str, focus: list[str]) -> int:
    cmd = [sys.executable, str(script_of(s)), f"@{listfile}"]
    opts = set(s.get("options") or [])
    if focus and "focus" in opts:                             # only flags the station lists in stations.json
        cmd += ["--focus", "; ".join(focus)]
    if dest and "out" in opts:
        cmd += ["--out", dest]
    elif not dest and "publish" in opts:
        cmd.append("--publish")
    print(f"\n$ {' '.join(cmd)}")
    return subprocess.run(cmd, cwd=MAIN).returncode


if __name__ == "__main__":
    raise SystemExit(run(sys.argv[1] if len(sys.argv) > 1 else ask("Station number?")))
