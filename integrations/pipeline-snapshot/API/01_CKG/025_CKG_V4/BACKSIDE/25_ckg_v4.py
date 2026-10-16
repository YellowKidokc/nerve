r"""25_CKG_V4: readable CKG review. Eight API calls per paper, in order.

The calls are declared in ckg.json next to this script (same format as the split CKG):
each call is a folder under calls/ with prompt.md (what the model is told) and output.md
(the shape it fills). Switch a call off with "on": false. Calls run in run_order; each one
sees the earlier answers. One finished page per paper lands in the OUTBOX:
    <note> · CKG.md        the four sections, then the original
    <note> · CKG.run.json  receipt: hashes, model, per-call time and errors

Run it from the buttons (1 RUN HERE / 2 RUN ON FOLDER), or directly:
    python 25_ckg_v4.py                      everything in ../INBOX (00_PRIORITY first)
    python 25_ckg_v4.py note.md folder\      those notes
    python 25_ckg_v4.py @list.txt            the notes listed in a file (what the buttons pass)
    --outbox DIR  --workers N  --focus "text"  --limit N  --dry-run (builds prompts, no API)
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
import time
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent            # BACKSIDE
FRONT = HERE.parent                               # 025_CKG_V4
INBOX = FRONT / "INBOX"
OUTBOX = FRONT / "OUTBOX"
LANES = ["00_PRIORITY", "01_SERIES", "02_GENERAL", "02_GROUP"]
EXTS = {".md", ".txt"}
API_URL = "https://api.deepseek.com/chat/completions"


def sha(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def load_calls() -> dict:
    m = json.loads((HERE / "ckg.json").read_text(encoding="utf-8"))
    for c in m["calls"]:
        f = HERE / c["folder"]
        c["prompt"] = (f / "prompt.md").read_text(encoding="utf-8") if (f / "prompt.md").exists() else ""
        c["template"] = (f / "output.md").read_text(encoding="utf-8") if (f / "output.md").exists() else ""
    return m


def focus_lines(extra: str) -> str:
    lines = []
    fm = HERE / "FOCUS.md"
    if fm.exists():
        lines += [l.strip() for l in fm.read_text(encoding="utf-8").splitlines()
                  if l.strip() and not l.lstrip().startswith("#")]
    if extra:
        lines += [x.strip() for x in extra.split(";") if x.strip()]
    return "\n".join(f"- {l}" for l in lines)


def build_prompt(c: dict, source: str, prior: dict, focus: str) -> str:
    p = c["prompt"].split("\n---\n", 1)[-1]          # drop the human note above the first ---
    prior_txt = "\n\n".join(f"## {k}\n{v}" for k, v in prior.items()) or "(none yet)"
    p = (p.replace("{{output_template}}", c["template"])
          .replace("{{prior_results}}", prior_txt)
          .replace("{{source}}", source))
    if focus:
        p += "\n\nSTANDING FOCUS (also look for):\n" + focus
    return p


def call_model(model: str, prompt: str) -> str:
    key = os.environ.get("DEEPSEEK_API_KEY")
    if not key:
        raise RuntimeError("DEEPSEEK_API_KEY is not set")
    body = json.dumps({"model": model, "messages": [{"role": "user", "content": prompt}],
                       "response_format": {"type": "json_object"}}).encode()
    req = urllib.request.Request(API_URL, data=body, headers={"Authorization": f"Bearer {key}",
                                                              "Content-Type": "application/json"})
    last = None
    for attempt in range(3):                          # retry transient failures
        try:
            with urllib.request.urlopen(req, timeout=300) as r:
                reply = json.loads(r.read())["choices"][0]["message"]["content"]
            data = json.loads(reply)
            return data.get("markdown") or json.dumps(data, ensure_ascii=False, indent=2)
        except Exception as e:                        # network, rate limit, bad JSON
            last = e
            time.sleep(5 * (attempt + 1))
    raise RuntimeError(f"model call failed after 3 tries: {last!r}")


def run_note(note: Path, m: dict, out: Path, focus: str, dry: bool) -> dict:
    t0 = time.time()
    source = note.read_text(encoding="utf-8", errors="replace")
    receipt = {"station": "25_CKG_V4", "note": str(note), "source_sha256": sha(source),
               "model": m.get("model", "deepseek-chat"), "dry_run": dry,
               "started": datetime.now().isoformat(timespec="seconds"), "calls": []}
    prior, outs = {}, {}
    calls = sorted((c for c in m["calls"] if c.get("on") and c.get("run_order")), key=lambda c: c["run_order"])
    for c in calls:
        p = build_prompt(c, source, prior, focus)
        rec = {"id": c["id"], "prompt_sha256": sha(p), "chars": len(p)}
        t = time.time()
        try:
            if dry:
                outs[c["id"]] = f"<!-- DRY RUN: {c['id']} prompt, {len(p):,} chars -->\n" + p
                prior[c["id"]] = "(dry run)"
            else:
                outs[c["id"]] = call_model(receipt["model"], p)
                prior[c["id"]] = outs[c["id"]]
            rec["ok"] = True
        except Exception as e:                        # one failed call never stops the others
            outs[c["id"]] = f"## {c['title']}\n\n**CALL FAILED:** {e}\n"
            prior[c["id"]] = "(this call failed)"
            rec["ok"], rec["error"] = False, str(e)
        rec["seconds"] = round(time.time() - t, 1)
        receipt["calls"].append(rec)
    page = [f"# {note.stem} · CKG\n",
            f"*CKG V4 · {datetime.now():%Y-%m-%d %H:%M} · "
            f"{receipt['model']} · AI_PROPOSED, nothing ruled or compiled*\n"]
    for c in sorted((c for c in m["calls"] if c.get("page_order")), key=lambda c: c["page_order"]):
        if c["kind"] == "paste_source":
            page.append("# ORIGINAL ARTICLE\n\n" + source + "\n")
        elif c["id"] in outs:
            page.append(outs[c["id"]].rstrip() + "\n")
    out.mkdir(parents=True, exist_ok=True)
    tag = " · CKG_V4_DRY_RUN" if dry else " · CKG"
    dest = out / f"{note.stem}{tag}.md"
    dest.write_text("\n".join(page), encoding="utf-8")
    receipt["output"] = str(dest)
    receipt["seconds"] = round(time.time() - t0, 1)
    receipt["failed_calls"] = [r["id"] for r in receipt["calls"] if not r["ok"]]
    (out / f"{note.stem}{tag}.run.json").write_text(json.dumps(receipt, indent=2, ensure_ascii=False), encoding="utf-8")
    return receipt


def gather(items: list[str]) -> list[Path]:
    notes: list[Path] = []
    if not items:                                     # default: own INBOX, priority lane first
        for lane in LANES:
            d = INBOX / lane
            if d.is_dir():
                notes += sorted(p for p in d.rglob("*") if p.is_file() and p.suffix.lower() in EXTS
                                and not p.name.startswith(("_", ".")))
        return notes
    for raw in items:
        if raw.startswith("@"):                       # a list file from the buttons
            lines = Path(raw[1:]).read_text(encoding="utf-8-sig").splitlines()
            notes += [Path(l.strip()) for l in lines if l.strip()]
            continue
        p = Path(raw)
        if p.is_dir():
            notes += sorted(q for q in p.rglob("*") if q.is_file() and q.suffix.lower() in EXTS
                            and not q.name.startswith(("_", ".")))
        elif p.is_file():
            notes.append(p)
        else:
            print(f"  ! not found, skipped: {raw}")
    return list(dict.fromkeys(notes))


def main() -> int:
    ap = argparse.ArgumentParser(prog="25_CKG_V4", description="Claims, Proofs, Evidence, Lean")
    ap.add_argument("items", nargs="*")
    ap.add_argument("--outbox", default=str(OUTBOX))
    ap.add_argument("--workers", type=int, default=4)
    ap.add_argument("--focus", default="")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args()

    m = load_calls()
    notes = gather(a.items)
    if a.limit:
        notes = notes[:a.limit]
    on = [c["id"] for c in sorted(m["calls"], key=lambda c: c.get("run_order") or 99) if c.get("on") and c.get("run_order")]
    print(f"[{datetime.now():%H:%M:%S}] 25_CKG_V4: {len(notes)} note(s), calls {' > '.join(on)}, "
          f"{max(1, a.workers)} at once{' (DRY RUN)' if a.dry_run else ''}")
    if not notes:
        print("  nothing to run: put notes in INBOX\\00_PRIORITY, 01_SERIES or 02_GENERAL")
        return 0
    if not a.dry_run and not os.environ.get("DEEPSEEK_API_KEY"):
        print("  DEEPSEEK_API_KEY is not set")
        return 2
    out, focus, failed = Path(a.outbox), focus_lines(a.focus), 0

    def one(n: Path):
        r = run_note(n, m, out, focus, a.dry_run)
        bad = r["failed_calls"]
        print(f"[{datetime.now():%H:%M:%S}] {'FAILED ' + ','.join(bad) if bad else 'done'}  {n.name}  ({r['seconds']}s)")
        return bool(bad)

    with ThreadPoolExecutor(max(1, min(20, a.workers))) as ex:
        failed = sum(ex.map(one, notes))
    print(f"[{datetime.now():%H:%M:%S}] finished: {len(notes) - failed} ok, {failed} with failed calls -> {out}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
