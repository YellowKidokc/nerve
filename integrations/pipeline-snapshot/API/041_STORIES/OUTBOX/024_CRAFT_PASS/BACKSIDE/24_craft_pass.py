r"""CRAFT PASS: one paper -> per-section craft options (the loop) -> DeepSeek's best full version.

    python craft_pass.py "path\to\paper.md"            run it
    python craft_pass.py "path\to\paper.md" --dry-run  build the prompts, send nothing

How it works:
  1. The paper is split into sections (each ## heading starts one; the title and opening are S00).
     Each section is split into units (a line or paragraph), IDs like S03.U02.
     Callout blocks (lines starting with >) and any "has and has not done" section are PROTECTED:
     copied exactly, never given options. That keeps the claim boxes and boundary sections intact.
  2. One call per section, in order. Each call gets the menu (David's cards first, then the craft
     library) plus whatever earlier sections left unused. It picks 2 fundamentals + up to 3 extras.
     Unused cards carry forward. The counts are kept here in code, not trusted to the model.
  3. One final call writes the best full version, every unit tagged with the source unit IDs it
     came from, so the writer can line it up against the original.

Output in OUTBOX\05_CRAFT_PASS\<paper>\ :
  <paper>.craftpass.json     every section's options, memorable line, analogy, loop ledger
  <paper>.craftpass.md       the same, readable
  <paper>.bestversion.json   the full rewrite, unit by unit, with source IDs and techniques
  <paper>.bestversion.md     the full rewrite as a plain page
  <paper>.receipt.json       hashes, model, tokens, per-call time
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
from collections import Counter
from datetime import datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent                 # BACKSIDE
FRONT = HERE.parent                                    # 024_CRAFT_PASS
OUT_ROOT = FRONT / "OUTBOX"
INBOX = FRONT / "INBOX"
LANES = ["00_PRIORITY", "01_SERIES", "02_GENERAL", "02_GROUP"]
LIBRARY = HERE / "craft_library.json"
URL = "https://api.deepseek.com/chat/completions"
MODEL = "deepseek-chat"
PROTECTED_SECTION = re.compile(r"has and has not done|analysis\s*·\s*layer|preserved long-form source", re.I)
FOCUS = ""                                             # set from --focus / FOCUS.md


def parse(text: str) -> tuple[str, list[dict]]:
    """Paper -> title + sections of units. Front matter is dropped (it is metadata, not prose)."""
    text = text.replace("\r\n", "\n")
    text = re.sub(r"\A---\n.*?\n---\n", "", text, count=1, flags=re.S)
    title, sections, cur = "", [], {"heading": None, "lines": []}
    for line in text.split("\n"):
        if line.startswith("# ") and not title:
            title = line[2:].strip()
        elif line.startswith("## "):
            sections.append(cur)
            cur = {"heading": line[3:].strip(), "lines": []}
        else:
            cur["lines"].append(line)
    sections.append(cur)
    out = []
    for s in sections:
        blocks, buf = [], []
        for line in s["lines"] + [""]:
            if line.strip():
                buf.append(line)
            elif buf:
                blocks.append("\n".join(buf))
                buf = []
        if not blocks and s["heading"] is None:
            continue
        sid = f"S{len(out):02d}"
        prot_sec = bool(s["heading"] and PROTECTED_SECTION.search(s["heading"]))
        units = [{"id": f"{sid}.U{i:02d}", "text": b,
                  "protected": prot_sec or b.lstrip().startswith((">", "|", "```", "<!--", "!["))} for i, b in enumerate(blocks, 1)]
        if not units or all(u["protected"] for u in units):
            prot_sec = True                              # nothing but tables / callouts / code: leave it alone
        out.append({"id": sid, "heading": s["heading"], "units": units, "protected": prot_sec})
    return title, out


def show_section(s: dict) -> str:
    head = f"## {s['heading']}\n" if s["heading"] else "(opening section, before the first heading)\n"
    return head + "\n".join(f"[{u['id']}{'|PROTECTED-copy-exactly' if u['protected'] else ''}] {u['text']}" for u in s["units"])


def repair_section(s: dict, got: dict) -> dict:
    """Code, not the model, guarantees the original survives:
    protected units come back word for word; any original unit the model dropped is put back
    in its original place, marked kept=True; stray 'PROTECTED' labels are stripped."""
    order = {u["id"]: i for i, u in enumerate(s["units"])}
    orig = {u["id"]: u for u in s["units"]}
    out = []
    for u in got.get("units") or []:
        t = re.sub(r"^\s*\[?PROTECTED[^\]\s]*\]?\s*", "", str(u.get("text", "")))
        srcs = [x for x in (u.get("source_ids") or []) if x in orig]
        prot = [x for x in srcs if orig[x]["protected"]]
        if srcs and len(prot) == len(srcs):
            t, u["changed"], u["techniques"] = "\n\n".join(orig[x]["text"] for x in srcs), False, []
        elif prot:                                       # mixed: the protected text goes back in front, intact
            t = "\n\n".join(orig[x]["text"] for x in prot) + "\n\n" + t
        u["text"] = t
        out.append(u)
    seen = {x for u in out for x in (u.get("source_ids") or [])}
    for uid in [k for k in orig if k not in seen]:          # put dropped originals back where they belong
        pos = 0
        for i, u in enumerate(out):
            ids = [order[x] for x in (u.get("source_ids") or []) if x in order]
            if ids and max(ids) < order[uid]:
                pos = i + 1
        out.insert(pos, {"source_ids": [uid], "text": orig[uid]["text"], "changed": False,
                         "techniques": [], "kept": not orig[uid]["protected"]})
    got["units"] = out
    return got


def menu() -> tuple[list[dict], list[dict]]:
    david = json.loads((HERE / "david_cards.json").read_text(encoding="utf-8"))
    lib = json.loads(LIBRARY.read_text(encoding="utf-8")) if LIBRARY.exists() else []
    lib = [c for c in lib if not str(c.get("for_written_prose", "")).lower().startswith("no")]
    return david, lib


def card_lines(cards: list[dict]) -> str:
    return "\n".join(f"- {c['card_id']} [{c.get('serves')}] {c.get('name')}: {c.get('what_it_does')}" for c in cards)


def api_key() -> str:
    return (os.environ.get("DEEPSEEK_API_KEY") or "").strip()


def call(prompt: str, max_tokens: int = 8000) -> tuple[dict, int]:
    body = json.dumps({"model": MODEL, "messages": [{"role": "user", "content": prompt}],
                       "response_format": {"type": "json_object"},
                       "max_tokens": max_tokens, "temperature": 0.7}).encode()
    last = None
    for attempt in range(4):
        req = urllib.request.Request(URL, body, {"Authorization": f"Bearer {api_key()}", "Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(req, timeout=300) as r:
                data = json.loads(r.read())
            ch = data["choices"][0]
            if ch.get("finish_reason") == "length":
                raise RuntimeError("reply hit the token limit; nothing saved for this call")
            return json.loads(ch["message"]["content"]), int(data.get("usage", {}).get("total_tokens", 0))
        except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as e:
            last = e
            time.sleep(2 ** (attempt + 1))
    raise RuntimeError(f"call failed after 4 tries: {last!r}")


def ids_in(xs) -> list[str]:
    """Pull card IDs out of whatever the model returned ('CRAFT-058 power word' -> 'CRAFT-058')."""
    out = []
    for x in xs or []:
        m = re.match(r"\s*((?:CRAFT|DAVID)-\d{3})", str(x))
        if m:
            out.append(m.group(1))
    return out


def run(paper: Path, dry: bool, out: Path) -> int:
    src = paper.read_text(encoding="utf-8")
    title, sections = parse(src)
    david, lib = menu()
    names = {c["card_id"]: c.get("name") for c in david + lib}
    sec_tmpl = (HERE / "section_prompt.txt").read_text(encoding="utf-8")
    if FOCUS:
        sec_tmpl += "\n\nSTANDING FOCUS FROM DAVID (also look for):\n" + FOCUS
    best_tmpl = (HERE / "best_version_prompt.txt").read_text(encoding="utf-8")
    out.mkdir(parents=True, exist_ok=True)
    receipt = {"paper": str(paper), "source_sha256": hashlib.sha256(src.encode()).hexdigest(), "model": MODEL,
               "started": datetime.now().isoformat(timespec="seconds"), "library_cards": len(lib), "calls": []}
    print(f"[{datetime.now():%H:%M:%S}] {paper.name}: {len(sections)} sections, "
          f"{sum(len(s['units']) for s in sections)} units, menu {len(david)} DAVID + {len(lib)} library cards")
    if not dry and not api_key():
        print("  DEEPSEEK_API_KEY is not set")
        return 2

    passes, counts, carried, domains = [], Counter(), [], []
    live = [s for s in sections if not s["protected"]]
    for n, s in enumerate(live, 1):
        pos = "This is the OPENING: invite the reader in (DAVID-002)." if n == 1 else (
              "This is the CLOSING section: land it." if n == len(live) else "A middle section.")
        prompt = (sec_tmpl.replace("{{section}}", show_section(s)).replace("{{index}}", str(n))
                  .replace("{{total}}", str(len(live))).replace("{{position_note}}", pos)
                  .replace("{{david_cards}}", card_lines(david)).replace("{{library}}", card_lines(lib))
                  .replace("{{carried}}", "\n".join(f"- {c} {names.get(c, '')}" for c in carried) or "(none yet)")
                  .replace("{{analogy_domains}}", ", ".join(domains) or "(none yet)")
                  .replace("{{section_id}}", s["id"]))
        if dry:
            (out / f"_dry_{paper.stem}_{s['id']}.txt").write_text(prompt, encoding="utf-8")
            continue
        t = time.time()
        try:
            res, tok = call(prompt)
        except Exception as e:
            print(f"[{datetime.now():%H:%M:%S}] {s['id']} FAILED: {e}")
            receipt["calls"].append({"section": s["id"], "ok": False, "error": str(e)})
            continue
        used = ids_in(res.get("fundamentals")) + ids_in(res.get("extras"))
        counts.update(used)
        carried = [c for c in dict.fromkeys(carried + ids_in(res.get("candidates"))) if c not in used]
        res["loop"] = {"used_here": used, "carried_forward": carried, "running_counts": dict(counts)}
        dom = ((res.get("analogy") or {}).get("domain") or "").strip()
        if dom and dom.lower() in {d.lower() for d in domains}:
            res["analogy"]["fit"] = "repeat"            # same world as an earlier section: not used in BEST
        if dom:
            domains.append(dom)
            res["loop"]["analogy_domains_so_far"] = list(domains)
        passes.append(res)
        receipt["calls"].append({"section": s["id"], "ok": True, "tokens": tok, "seconds": round(time.time() - t, 1)})
        print(f"[{datetime.now():%H:%M:%S}] {paper.stem[:28]} {s['id']}: used {', '.join(used)}  "
              f"| {len(carried)} carried | {tok:,} tokens")
    if dry:
        print(f"  dry run: prompts written to {out}")
        return 0

    # Best version, one section at a time: a LONG paper never has to fit in one reply.
    # Protected sections are copied word for word with no call at all.
    by_sec = {p.get("section_id"): p for p in passes}
    best = {"title": title, "sections": []}
    for s in sections:
        if s["protected"] or s["id"] not in by_sec:
            best["sections"].append({"section_id": s["id"], "heading": s["heading"], "units": [
                {"source_ids": [u["id"]], "text": u["text"], "changed": False, "techniques": []} for u in s["units"]]})
            continue
        prompt = (best_tmpl.replace("{{original}}", f"(Title of the whole piece: {title})\n\n" + show_section(s))
                  .replace("{{passes}}", json.dumps([by_sec[s["id"]]], ensure_ascii=False)))
        t = time.time()
        try:
            part, tok = call(prompt)
            secs = part.get("sections") or []
            got = secs[0] if secs else {}
            got["section_id"], got["heading"] = s["id"], s["heading"]       # headings are David's
            best["sections"].append(repair_section(s, got))
            receipt["calls"].append({"section": f"BEST_{s['id']}", "ok": True, "tokens": tok, "seconds": round(time.time() - t, 1)})
        except Exception as e:                       # a failed section falls back to the original, marked in the receipt
            best["sections"].append({"section_id": s["id"], "heading": s["heading"], "units": [
                {"source_ids": [u["id"]], "text": u["text"], "changed": False, "techniques": []} for u in s["units"]]})
            receipt["calls"].append({"section": f"BEST_{s['id']}", "ok": False, "error": str(e)})
            print(f"[{datetime.now():%H:%M:%S}] {paper.stem[:28]} best {s['id']} FAILED (original kept): {e}")
    print(f"[{datetime.now():%H:%M:%S}] {paper.stem[:28]} best version assembled from {len(best['sections'])} sections")

    write_outputs(out, paper.stem, title, sections, passes, best, counts, names)
    receipt["finished"] = datetime.now().isoformat(timespec="seconds")
    receipt["total_tokens"] = sum(c.get("tokens", 0) for c in receipt["calls"])
    (out / f"{paper.stem} · CRAFT.receipt.json").write_text(json.dumps(receipt, indent=2), encoding="utf-8")
    print(f"[{datetime.now():%H:%M:%S}] done: {receipt['total_tokens']:,} tokens -> {out}")
    return 0 if all(c.get("ok") for c in receipt["calls"]) else 1



# ---------------------------------------------------------------- output
# Markers. Unmarked lines are David's original text. Everything the pass adds is a marked line
# placed right after the line it belongs to, so a script can split them apart again.
LEGEND = """<!-- CRAFT MARKERS (unmarked lines are the original)
((R1)) ((R2)) ((R3))   rewrite options for the line above; ((R1*)) is the pass's pick
((...!))               any marker ending in ! claims more than the original (DAVID-005 gate)
((ML))                 the section's memorable line, new; ((ML=S03.U04)) = an existing line already is it
((AN strong|weak))     a "never in the same room" analogy; ((MAP)) its mapping; ((BRK)) where it breaks
((USED)) ((CARRY))     techniques used in this section / carried forward to the next
In the BEST file: ((C)) changed line, ((NEW)) added line, ((KEPT)) a line the model dropped that code put back,
unmarked = kept as written. ((AN repeat)) = analogy from a world already used earlier in the paper
-->"""


def write_outputs(out: Path, stem: str, title: str, sections: list, passes: list, best, counts: Counter, names: dict) -> None:
    all_units = {u["id"]: u for s in sections for u in s["units"]}
    by_sec = {p.get("section_id"): p for p in passes}
    doc = {"title": title, "sections": sections, "passes": passes,
           "card_counts": {k: {"name": names.get(k), "uses": v} for k, v in counts.most_common()}}
    (out / f"{stem} · CRAFT.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False), encoding="utf-8")

    md = [LEGEND, "", f"# {title}", ""]
    for s in sections:
        p = by_sec.get(s["id"], {})
        opts = {o.get("unit_id"): o for o in p.get("options") or []}
        if s["heading"]:
            md += [f"## {s['heading']}", ""]
        for u in s["units"]:
            md.append(u["text"])
            for i, c in enumerate((opts.get(u["id"]) or {}).get("choices") or [], 1):
                mark = f"R{i}" + ("*" if c.get("best") else "") + ("!" if c.get("claims_more_than_original") else "")
                tech = ", ".join(c.get("techniques") or [])
                md.append(f"(({mark})) {c.get('text')}" + (f"  <!-- {tech} -->" if tech else ""))
            md.append("")
        ml = p.get("memorable_line") or {}
        if ml.get("text"):
            tag = "ML" + (f"={ml['existing_unit']}" if ml.get("existing_unit") else "") + ("!" if ml.get("claims_more_than_original") else "")
            md += [f"(({tag})) {ml['text']}", ""]
        an = p.get("analogy") or {}
        if an.get("line"):
            md += [f"((AN {an.get('fit', '?')})) {an['line']}  <!-- {an.get('domain', '')} -->",
                   f"((MAP)) {an.get('mapping', '')}", f"((BRK)) {an.get('breaks_at', '')}", ""]
        if p.get("loop"):
            md += [f"((USED)) {', '.join(p['loop']['used_here'])}",
                   f"((CARRY)) {', '.join(p['loop']['carried_forward']) or 'none'}", ""]
    md += ["<!-- technique counts: " + "; ".join(f"{k} {names.get(k, '')} x{v}" for k, v in counts.most_common()) + " -->"]
    (out / f"{stem} · CRAFT.md").write_text("\n".join(md), encoding="utf-8")

    if not best:
        return
    if best.get("title") != title:                      # the title is David's; the model never changes it
        best["title_proposed"] = best.get("title")
        best["title"] = title
    checks = {"protected_changed": [], "originals_not_used": []}
    used_src = set()
    for s in best.get("sections", []):
        for u in s.get("units", []):
            for sid in u.get("source_ids") or []:
                used_src.add(sid)
                orig = all_units.get(sid)
                if orig and orig["protected"] and u.get("text", "").strip() != orig["text"].strip():
                    checks["protected_changed"].append(sid)
    checks["originals_not_used"] = [k for k in all_units if k not in used_src]
    best["checks"] = checks
    (out / f"{stem} · BEST.json").write_text(json.dumps(best, indent=1, ensure_ascii=False), encoding="utf-8")
    page = [LEGEND, "", f"# {title}", ""]
    for s in best.get("sections", []):
        if s.get("heading"):
            page += [f"## {s['heading']}", ""]
        for u in s.get("units", []):
            src_ids = u.get("source_ids") or []
            tag = "((NEW)) " if src_ids == ["NEW"] else ("((KEPT)) " if u.get("kept") else ("((C)) " if u.get("changed") else ""))
            page += [tag + u.get("text", ""), ""]
    (out / f"{stem} · BEST.md").write_text("\n".join(page), encoding="utf-8")
    if checks["protected_changed"]:
        print(f"  ! {stem}: protected units changed by the model: {checks['protected_changed']}")


def gather(items: list[str]) -> list[Path]:
    def scan(d: Path) -> list[Path]:
        return sorted(p for p in d.rglob("*.md") if p.is_file() and not p.name.startswith(("_", ".")))
    if not items:
        files: list[Path] = []
        for lane in LANES:
            if (INBOX / lane).is_dir():
                files += scan(INBOX / lane)
        return files
    files = []
    for raw in items:
        if raw.startswith("@"):
            files += [Path(l.strip()) for l in Path(raw[1:]).read_text(encoding="utf-8-sig").splitlines() if l.strip()]
        elif Path(raw).is_dir():
            files += scan(Path(raw))
        elif Path(raw).is_file():
            files.append(Path(raw))
        else:
            print(f"  ! not found, skipped: {raw}")
    return [f for f in dict.fromkeys(files) if f.suffix.lower() == ".md"]


def repair_outputs(out: Path) -> int:
    """Re-apply the code-side guarantees to papers already done. No API calls."""
    david, lib = menu()
    names = {c["card_id"]: c.get("name") for c in david + lib}
    n = 0
    for cj in sorted(out.glob("* · CRAFT.json")):
        stem = cj.name[: -len(" · CRAFT.json")]
        bj = out / f"{stem} · BEST.json"
        if not bj.exists():
            continue
        craft = json.loads(cj.read_text(encoding="utf-8"))
        best = json.loads(bj.read_text(encoding="utf-8"))
        secs = {s["id"]: s for s in craft["sections"]}
        fixed = []
        for b in best.get("sections", []):
            s = secs.get(b.get("section_id"))
            if not s:
                fixed.append(b)
            elif s["protected"]:
                fixed.append({"section_id": s["id"], "heading": s["heading"], "units": [
                    {"source_ids": [u["id"]], "text": u["text"], "changed": False, "techniques": []} for u in s["units"]]})
            else:
                fixed.append(repair_section(s, b))
        best["sections"] = fixed
        seen_doms = set()
        for p in craft["passes"]:
            a = p.get("analogy") or {}
            d = (a.get("domain") or "").strip().lower()
            if d and d in seen_doms:
                a["fit"] = "repeat"
            if d:
                seen_doms.add(d)
        counts = Counter({k: v["uses"] for k, v in (craft.get("card_counts") or {}).items()})
        write_outputs(out, stem, craft["title"], craft["sections"], craft["passes"], best, counts, names)
        ch = best.get("checks", {})
        print(f"  repaired {stem[:60]}: protected_changed {len(ch.get('protected_changed', []))}, "
              f"originals_not_used {len(ch.get('originals_not_used', []))}")
        n += 1
    print(f"repaired {n} paper(s), no API calls")
    return 0


def main() -> int:
    global FOCUS
    ap = argparse.ArgumentParser(prog="24_CRAFT_PASS")
    ap.add_argument("items", nargs="*")
    ap.add_argument("--outbox", default=str(OUT_ROOT))
    ap.add_argument("--workers", type=int, default=3)
    ap.add_argument("--focus", default="")
    ap.add_argument("--redo", action="store_true", help="run papers that already have results")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--repair", action="store_true", help="re-apply protection/dropped-line/repeat fixes to finished papers, no API")
    a = ap.parse_args()
    if a.repair:
        return repair_outputs(Path(a.outbox))
    fm = HERE / "FOCUS.md"
    lines = [l.strip() for l in fm.read_text(encoding="utf-8").splitlines() if l.strip() and not l.startswith("#")] if fm.exists() else []
    lines += [x.strip() for x in a.focus.split(";") if x.strip()]
    FOCUS = "\n".join(f"- {l}" for l in lines)

    out = Path(a.outbox)
    papers = gather(a.items)
    todo = [p for p in papers if a.redo or a.dry_run or not (out / f"{p.stem} · CRAFT.json").exists()]
    print(f"[{datetime.now():%H:%M:%S}] 24_CRAFT_PASS: {len(papers)} paper(s), {len(papers) - len(todo)} already done "
          f"(skipped), {len(todo)} to run, {max(1, a.workers)} at once{' (DRY RUN)' if a.dry_run else ''}")
    if not todo:
        return 0
    try:                                                 # every input lands in ORIGINALS once (engine/originals.py)
        if not a.dry_run:
            sys.path.insert(0, str(next(p for p in HERE.parents if (p / "_system").is_dir()) / "_system"))
            from engine.originals import archive
            archive(todo, "24_CRAFT_PASS")
    except Exception as e:
        print(f"  (originals archive skipped: {e})")
    from concurrent.futures import ThreadPoolExecutor
    with ThreadPoolExecutor(max(1, min(10, a.workers))) as ex:
        codes = list(ex.map(lambda p: run(p, a.dry_run, out), todo))
    return max(codes) if codes else 0


if __name__ == "__main__":
    sys.exit(main())
