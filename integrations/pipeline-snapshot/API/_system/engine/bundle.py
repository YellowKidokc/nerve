"""Bundle stations: one front folder that runs several passes (two or three API calls) on every note.

A bundle replaces a run of small legacy stations (EVIDENCE: intake + three dials; ATOMS: claim atoms + axiom nodes).
It is declared in station.json:

  "kind": "bundle",
  "out_label": "30_EVIDENCE",                    results land flat in the output folder as '<note> · 30_EVIDENCE.md'
  "steps": ["turbo", "dials"]                    the passes, in page order; each is a small adapter below

How a run works (David, 2026-10-01: "you pick the input folder and the output folder, and all of it runs in parallel"):
  * the folder the button was pointed at and the output folder are honoured; no hidden evidence_root / atoms_workspace;
  * notes run `workers` at a time; inside one note its steps run side by side (they only read the source);
  * every note gets its own scratch folder (BACKSIDE/_work/<slug>/<step>), so nothing is shared between workers, and the
    vendored scripts keep working unchanged: each is pointed at that folder through the environment or its own flags;
  * all calls go through the one relay (engine/gateway.py): one limiter, one retry policy, receipts;
  * a note is FINISHED the moment its steps return: file in the output folder, then the answer on the note. An
    interruption loses nothing finished, and a note already in the output folder is skipped on rerun.
"""
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path

from . import focus as focus_lib
from .gateway import Gateway
from .legacy import path_env
from .paths import API_HOME, inside

VENDOR = inside("vendor")
ANALYSIS_BLOCKS = re.compile(r"<!-- (analysis|analysis-detail|scorecard):start -->.*?<!-- \1:end -->\n?", re.S)


@dataclass
class StepResult:
    step: str
    ok: bool
    heading: str
    body: str = ""
    json_files: dict[str, str] = field(default_factory=dict)   # file name -> text, kept beside the markdown
    error: str = ""


@dataclass
class Job:
    note: Path
    slug: str
    text: str               # the source only: our own analysis blocks are stripped
    work: Path
    provider: str
    focus: list[str]
    env: dict[str, str]


# ------------------------------------------------------------------ small helpers

def slug_of(name: str) -> str:
    s = re.sub(r"[^\w\-]", "_", Path(name).stem)
    return re.sub(r"_+", "_", s).strip("_")[:60] or "untitled"


SOURCE_MARK = re.compile(r"^(<!-- ===== ORIGINAL ARTICLE BELOW[^\n]*-->|#{1,4} Exact source[^\n]*)\n", re.M)


def source_text(note: Path) -> str:
    """The source only. Our own analysis blocks are stripped; and an evidence companion (C1/C2/C3 file) carries the exact original
    below its analysis, so for a companion the passes read that original, never the companion's own analysis."""
    text = ANALYSIS_BLOCKS.sub("", note.read_text(encoding="utf-8", errors="replace"))
    m = SOURCE_MARK.search(text)
    if m and re.search(r"^(type: axiom_companion|paper_id:|source_sha256:)", text[:3000], re.M):
        return text[m.end():].lstrip("\n")
    return text


def demote(text: str, by: int = 2) -> str:
    """Push every markdown heading down `by` levels so a step's page nests under the bundle's own headings."""
    return re.sub(r"^(#{1,4}) ", lambda m: "#" * (len(m.group(1)) + by) + " ", text, flags=re.M)


def companion_body(text: str) -> str:
    """The turbo companion's analysis only: after its YAML block, before its own copy of the source."""
    m = re.match(r"﻿?---\s*\n.*?\n---\s*\n", text, re.S)
    if m:
        text = text[m.end():]
    cut = re.search(r"^(#{1,4} Exact source|<!-- ===== ORIGINAL ARTICLE BELOW)", text, re.M)
    return (text[:cut.start()] if cut else text).strip()


def run(cmd: list[str], cwd: Path, env: dict[str, str], log: Path) -> int:
    """One script as a child process with its own console group (a stray Ctrl+C never reaches it); output to a log."""
    flags = getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0)
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("w", encoding="utf-8", errors="replace") as fh:
        fh.write("$ " + " ".join(cmd) + "\n\n")
        fh.flush()
        return subprocess.run(cmd, cwd=cwd, env=env, stdout=fh, stderr=subprocess.STDOUT, creationflags=flags).returncode


def tail(log: Path, n: int = 6) -> str:
    try:
        return " | ".join(x.strip() for x in log.read_text(encoding="utf-8", errors="replace").splitlines()[-n:] if x.strip())
    except OSError:
        return ""


# ------------------------------------------------------------------ steps (one adapter per vendored script)

def step_turbo(job: Job) -> StepResult:
    """Evidence intake, one call: the v0.4.1 companion (support, claims, predicates, standing)."""
    w = job.work / "turbo"
    (w / "INBOX").mkdir(parents=True, exist_ok=True)
    (w / "INBOX" / f"{job.slug}.md").write_text(job.text, encoding="utf-8")
    script = VENDOR / "evidence" / "SCRIPTS" / "turbo_pipeline_runner.py"
    env = {**job.env, "ONE_MENU_PATH_EVIDENCE_ROOT": str(w), "ONE_MENU_PATH_SIDECAR_MIRROR": str(w / "_sidecar")}
    started = datetime.now().timestamp()
    cmd = [sys.executable, "-u", str(script), "--root", str(w), "--workers", "1", "--provider", "deepseek",
           *[a for q in job.focus for a in ("--focus", q)]]
    code = run(cmd, script.parent, env, w / "run.log")
    hits = sorted((p for p in (w / "OUTBOX").rglob("*_C1_*.md") if p.stat().st_mtime >= started - 5),
                  key=lambda p: p.stat().st_mtime) if (w / "OUTBOX").is_dir() else []
    if not hits:
        return StepResult("turbo", False, "Evidence companion", error=f"no companion came back (exit {code}): {tail(w / 'run.log')}")
    return StepResult("turbo", True, "Evidence companion", companion_body(hits[-1].read_text(encoding="utf-8", errors="replace")))


def step_dials(job: Job) -> StepResult:
    """The three dials, one call: what kind each load-bearing statement is, and claimed vs earned strength."""
    w = job.work / "dials"
    w.mkdir(parents=True, exist_ok=True)
    src = w / f"{job.slug}.md"
    src.write_text(job.text, encoding="utf-8")
    script = VENDOR / "evidence" / "SCRIPTS" / "three_dials_annotate.py"
    env = {**job.env, "ONE_MENU_PATH_EVIDENCE_ROOT": str(w), "ONE_MENU_PATH_SIDECAR_MIRROR": str(w / "_sidecar")}
    code = run([sys.executable, "-u", str(script), str(src)], w, env, w / "run.log")
    made = sorted((w / "OUTBOX" / "ANNOTATED_THREE_DIALS").glob("*.annotated.md")) if (w / "OUTBOX").is_dir() else []
    if code or not made:
        return StepResult("dials", False, "Three dials", error=f"no annotation came back (exit {code}): {tail(w / 'run.log')}")
    raw = made[-1].read_bytes()
    cut = raw.find(b"<!-- ===== ORIGINAL ARTICLE BELOW")
    header = (raw[:cut] if cut >= 0 else raw).decode("utf-8", errors="replace").strip()
    header = re.sub(r"^---\s*\n.*?\n---\s*\n", "", header, count=1, flags=re.S).strip()
    files = {p.name: p.read_text(encoding="utf-8", errors="replace")
             for p in (w / "OUTBOX" / "ANNOTATED_THREE_DIALS").glob("*.ckg.json")}
    return StepResult("dials", True, "Three dials", header, files)


def _workspace_step(job: Job, name: str, folder: str, script_name: str, heading: str, marker: str) -> StepResult:
    """The atoms / axiom-nodes runners read WORKSPACE/01_PRIORITY and append their section to the paper: give each its
    own workspace holding this one note, then lift the appended section back out."""
    w = job.work / name
    lane = w / "01_PRIORITY"
    lane.mkdir(parents=True, exist_ok=True)
    src = lane / f"{job.slug}.md"
    src.write_text(job.text, encoding="utf-8")
    script = VENDOR / "api_deep" / folder / "SCRIPTS" / script_name
    env = {**job.env, "ONE_MENU_PATH_ATOMS_WORKSPACE": str(w)}
    code = run([sys.executable, "-u", str(script), "--workspace", str(w), "--provider", "deepseek"], script.parent, env, w / "run.log")
    done = src.read_text(encoding="utf-8", errors="replace") if src.is_file() else ""
    cut = done.rfind(marker)             # the last one: the source may have a similar heading of its own
    if code or cut < 0:
        return StepResult(name, False, heading, error=f"no result came back (exit {code}): {tail(w / 'run.log')}")
    section = done[cut:].strip()
    files = {p.name: p.read_text(encoding="utf-8", errors="replace") for p in (w / "06_JSON_RECORDS").glob("*.json")}
    return StepResult(name, True, heading, section, files)


def step_atoms(job: Job) -> StepResult:
    return _workspace_step(job, "atoms", "ATOMS", "run_atoms.py", "Claim atoms", "## Atom Classification (Axiom API)")


def step_axioms(job: Job) -> StepResult:
    return _workspace_step(job, "axioms", "AXIOM_NODES", "run_axiom_nodes.py", "Axiom nodes", "## Axiom Node Mapping (Axiom Nodes API)")


def step_chi(job: Job) -> StepResult:
    """chi-Evaluator v2: the Master Equation's ten channels scored per claim, then four synthesized statements.
    Two calls per claim (evaluate, synthesize), DeepSeek only. The vendored script runs unchanged in a scratch copy."""
    w = job.work / "chi"
    (w / "INBOX").mkdir(parents=True, exist_ok=True)
    for f in (VENDOR / "chi_evaluator").glob("*.py"):
        shutil.copy2(f, w / f.name)
    (w / "INBOX" / f"{job.slug}.txt").write_text(job.text, encoding="utf-8")
    key = os.environ.get("DEEPSEEK_API_KEY", "") or "no-key"
    (w / "config.txt").write_text(f"RUN_OPENAI=FALSE\nRUN_DEEPSEEK=TRUE\nDEEPSEEK_API_KEY={key}\nDEEPSEEK_MODEL=deepseek-chat\n"
                                  "DEEPSEEK_MAX_TOKENS=4096\nTEMPERATURE=0.3\nFRUIT_BETA=4.0\nFRUIT_CHI_C=0.30\nARCHIVE_INBOX=TRUE\n", encoding="utf-8")
    code = run([sys.executable, "-u", str(w / "run_evaluator.py")], w, job.env, w / "run.log")
    mds = sorted((w / "OUTBOX").rglob("*.md")) if (w / "OUTBOX").is_dir() else []
    if code or not mds:
        return StepResult("chi", False, "chi-Evaluator", error=f"no evaluation came back (exit {code}): {tail(w / 'run.log')}")
    body = "\n\n".join(f"### {m.stem.replace(job.slug + '_', '')}\n\n{demote(m.read_text(encoding='utf-8', errors='replace'), 1)}" for m in mds)
    files = {p.name: p.read_text(encoding="utf-8", errors="replace") for p in (w / "OUTBOX").rglob("*.json")}
    return StepResult("chi", True, "chi-Evaluator", body, files)


STEPS = {"turbo": step_turbo, "dials": step_dials, "atoms": step_atoms, "axioms": step_axioms, "chi": step_chi}


# ------------------------------------------------------------------ one note, then all notes

def say(line: str) -> None:
    print(f"[{datetime.now():%H:%M:%S}] {line}", flush=True)


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


def assemble(note: Path, st: dict, results: list[StepResult]) -> str:
    ok = [r for r in results if r.ok]
    failed = [r for r in results if not r.ok]
    head = [f"---\nnote: \"[[{note.stem}]]\"\nstation: {st['label']}\nupdated: {datetime.now():%Y-%m-%d %H:%M}\n"
            f"passes_ok: {json.dumps([r.step for r in ok])}\npasses_failed: {json.dumps([r.step for r in failed])}\n---\n",
            f"# {note.stem}\n"]
    for r in failed:
        head.append(f"> **{r.heading} did not come back:** {r.error}\n")
    body = [f"## {r.heading}\n\n{demote(r.body)}\n" for r in ok]
    return "\n".join(head + body)


def run_bundle(front: Path, st: dict, notes: list[Path], out: Path, workers: int, focus: list[str],
               publish, write_latest, redo: bool = False) -> int:
    """Run the station's steps on `notes`, `workers` notes at a time. `publish(note, dirs)` and `write_latest(note, out)`
    are button.py's, passed in so each note is finished (file, then the note itself) as soon as its steps return."""
    steps = [s for s in st["steps"] if s in STEPS]
    label = st["label"]
    out_label = st.get("out_label", label)
    out.mkdir(parents=True, exist_ok=True)
    todo = [n for n in notes if redo or focus or not (out / f"{n.stem} · {out_label}.md").is_file()]
    for n in notes:
        if n not in todo:
            say(f"already done, skipped: {n.name[:100]}")
    if not todo:
        return 0
    settings = json.loads(inside("config", "settings.json").read_text(encoding="utf-8"))
    provider = os.environ.get("ONE_MENU_PROVIDER", "").strip() or settings["default_provider"]
    run_id = datetime.now().strftime("%Y%m%dT%H%M%S")
    gateway = Gateway(workers, run_id, mock=(provider == "mock")).start()
    standing, _ = focus_lib.compose(front / "BACKSIDE", None, focus_lib.resolve_run_focus(focus))
    gateway.focus[label] = standing
    env = {**os.environ, **path_env(), **gateway.env_for(label), "ONE_MENU_RUN_ID": run_id, "PYTHONIOENCODING": "utf-8"}
    if provider == "mock":                         # legacy scripts skip a route without a key; the relay never sends it anywhere
        for key in ("DEEPSEEK_API_KEY", "OPENROUTER_API_KEY", "OPENAI_API_KEY", "MOONSHOT_API_KEY"):
            env.setdefault(key, "mock-key-not-sent")
    lock = threading.Lock()
    counter = {"done": 0, "failed": 0}
    say(f"{label}: {len(todo)} note(s), {workers} at once, passes: {', '.join(steps)} -> {out}")

    def one(n: Path) -> int:
        job = Job(n, slug_of(n.name), source_text(n), front / "BACKSIDE" / "_work" / slug_of(n.name), provider, focus, env)
        if job.work.exists():
            shutil.rmtree(job.work, ignore_errors=True)
        job.work.mkdir(parents=True, exist_ok=True)
        with ThreadPoolExecutor(max_workers=len(steps)) as inner:     # a note's passes side by side
            futures = {s: inner.submit(STEPS[s], job) for s in steps}
            results = []
            for s in steps:
                try:
                    results.append(futures[s].result())
                except Exception as exc:                              # one bad pass never loses the others
                    results.append(StepResult(s, False, s, error=f"{type(exc).__name__}: {exc}"))
        with lock:
            counter["done"] += 1
            k = counter["done"]
        good = [r for r in results if r.ok]
        if not good:
            with lock:
                counter["failed"] += 1
            say(f"[{k}/{len(todo)}] FAILED, nothing came back for {n.name[:80]}: " + "; ".join(r.error for r in results)[:300])
            return 1
        dest = out / f"{n.stem} · {out_label}.md"                     # finish this note now: file, then the note
        archive(dest, out)
        dest.write_text(assemble(n, st, results), encoding="utf-8")
        for r in good:
            for name, text in r.json_files.items():
                (out / "_json").mkdir(exist_ok=True)
                (out / "_json" / f"{n.stem} · {r.step} · {name}").write_text(text, encoding="utf-8")
        publish(n, [out])
        write_latest(n, out)
        bad = [r for r in results if not r.ok]
        if not bad:
            shutil.rmtree(job.work, ignore_errors=True)               # kept only when a pass failed: its run.log says why
        if bad:
            with lock:
                counter["failed"] += 1
        say(f"[{k}/{len(todo)}] finished and on the note: {n.name[:80]}"
            + (f"  (missing: {', '.join(r.step for r in bad)})" if bad else ""))
        return 1 if bad else 0

    code = 0
    try:
        with ThreadPoolExecutor(max_workers=max(1, min(workers, len(todo)))) as pool:
            for f in as_completed([pool.submit(one, n) for n in todo]):
                code |= f.result() or 0
    finally:
        stats = gateway.stats.snapshot()
        gateway.stop()
    say(f"{label}: {len(todo) - counter['failed']} of {len(todo)} complete · calls {stats['calls']} "
        f"(failed {stats['failed']}) · tokens {stats['tokens']:,}")
    return code


# ------------------------------------------------------------------ the script behind a bundle (no questions)

def main(label: str) -> int:
    """Run a bundle straight from the command line, no questions (what each bundle's own script calls):

        python 30_evidence_intake.py <folder or note> [...] --out <folder> --workers 8 [--limit N] [--focus "..."]

    Input and output folders are whatever you give it; the output defaults to the station's own OUTBOX."""
    import argparse
    from .paths import station_dir
    from .publish import publish_on_note
    front = station_dir(label).parent
    st = json.loads((station_dir(label) / "station.json").read_text(encoding="utf-8"))
    ap = argparse.ArgumentParser(prog=label, description=st.get("description", ""))
    ap.add_argument("inputs", nargs="*", help="folders (searched for .md notes) or notes; default = this station's INBOX")
    ap.add_argument("--out", help="output folder (default: this station's OUTBOX)")
    ap.add_argument("--workers", type=int, default=8, help="notes at once, and API calls side by side (1-30)")
    ap.add_argument("--limit", type=int, help="only the first N notes")
    ap.add_argument("--focus", action="append", default=[], help="an extra question (repeatable, up to 5)")
    ap.add_argument("--no-publish", action="store_true", help="leave the notes themselves untouched")
    ap.add_argument("--redo", action="store_true", help="run again on notes already in the output folder")
    ap.add_argument("--provider", help="deepseek (default) or mock (fake replies, no API)")
    ap.add_argument("--model", help="accepted for ONE_MENU; the vendored scripts keep their own model")
    ap.add_argument("--dry-run", action="store_true", help="list the notes and the passes, call nothing")
    args = ap.parse_args()
    notes: list[Path] = []
    for raw in args.inputs or [str(front / "INBOX")]:
        p = Path(raw)
        if p.is_file():
            notes.append(p)
        elif p.is_dir():
            notes += sorted(f for f in p.rglob("*.md")
                            if not any(part.startswith(("_", ".")) or part in ("Clean MD", "Prompts", "Channel Summary")
                                       for part in f.relative_to(p).parts))
        else:
            print(f"{label}: not found: {p}", file=sys.stderr)
            return 2
    notes = list(dict.fromkeys(notes))[: args.limit]
    if not notes:
        print(f"{label}: no notes found")
        return 0
    out = Path(args.out) if args.out else front / "OUTBOX"
    if args.provider:
        os.environ["ONE_MENU_PROVIDER"] = args.provider
    if args.dry_run:
        print(f"{label}: would run {', '.join(st['steps'])} on {len(notes)} note(s), results -> {out}")
        for n in notes:
            print(f"  would run {n}")
        return 0
    publish = (lambda n, dirs: None) if args.no_publish else (lambda n, dirs: publish_on_note([n], dirs))
    return run_bundle(front, st, notes, out, max(1, min(30, args.workers)), args.focus[:5], publish, lambda n, o: None, args.redo)
