"""Series sheet: every file of a series (evidence companions, the series notebook, the grand-synthesis master paper) read
in one run, one JSON and one markdown per file, one master HTML for the whole series. Local only: no API call.

    parse(path) -> record               what is in one file: front matter, headline numbers, sections, tables, ids
    sheet_md(record) -> str             the per-file markdown (card, outline, extracted tables)
    series_html(series, records, link)  the master page: sortable, filterable, every file one row

The source is read, never changed. The projected original at the bottom of a companion ("Exact source", the
"ORIGINAL ARTICLE BELOW" marker) is counted but not parsed, so a table inside the source never reads as analysis.
"""
from __future__ import annotations

import hashlib
import html
import json
import re
from datetime import datetime
from pathlib import Path

# the columns of the master index (vendor/evidence/SCRIPTS/master_index_parser.py), in the order the page shows them
HEADLINE = ["paper_rating", "evidence_status", "formal_status", "coherence", "total_claims", "claims_with_falsifiers",
            "hidden_premises", "predictions_logged", "bridges_registered", "lean_targets_queued", "evd_state"]
ID_RE = re.compile(r"\b[A-Za-z0-9_]+-(?:C|B|P|F|HP)\d{3}\b")
SOURCE_CUT = re.compile(r"^(#{1,4} Exact source|<!-- ===== ORIGINAL ARTICLE BELOW)", re.M)
FRONT = re.compile(r"\A\ufeff?\s*(?:```ya?ml\s*)?---\s*\n(.*?)\n---[ \t]*\n(?:```[ \t]*\n)?", re.S)


def front_matter(text: str) -> tuple[dict, str]:
    """Top-level `key: value` lines of the YAML block (JSON values stay lists / numbers); the rest of the text."""
    m = FRONT.match(text)
    if not m:
        return {}, text
    out: dict = {}
    for line in m.group(1).splitlines():
        if not line.strip() or line.lstrip().startswith("#") or line[0] in " \t-":
            continue
        k, sep, v = line.partition(":")
        if not sep or not re.fullmatch(r"[A-Za-z_][\w-]*", k.strip()):
            continue
        v = v.strip()
        try:
            out[k.strip()] = json.loads(v)
        except ValueError:
            out[k.strip()] = v.strip("\"'") if v not in ("null", "~") else None
    return out, text[m.end():]


def words(text: str) -> int:
    return len(re.findall(r"\w+", text))


def sections(body: str) -> list[dict]:
    """Headings with their word counts (a section runs to the next heading of any level). Fenced code is skipped."""
    marks, in_code = [], False
    for n, line in enumerate(body.splitlines(), 1):
        if line.lstrip().startswith("```"):
            in_code = not in_code
        m = None if in_code else re.match(r"(#{1,6})\s+(.*?)\s*#*\s*$", line)
        if m:
            marks.append((n, len(m.group(1)), m.group(2)))
    lines = body.splitlines()
    out = []
    for i, (n, level, title) in enumerate(marks):
        end = marks[i + 1][0] - 1 if i + 1 < len(marks) else len(lines)
        out.append({"level": level, "title": title, "line": n, "words": words("\n".join(lines[n:end]))})
    return out


def tables(body: str) -> list[dict]:
    """Every markdown pipe table, with the heading it sits under: columns, and rows as {column: cell}."""
    found, heading, lines, i, in_code = [], "", body.splitlines(), 0, False
    while i < len(lines):
        line = lines[i]
        if line.lstrip().startswith("```"):
            in_code = not in_code
        m = re.match(r"#{1,6}\s+(.*?)\s*#*\s*$", line)
        if m and not in_code:
            heading = m.group(1)
        if (not in_code and line.strip().startswith("|") and i + 1 < len(lines)
                and re.fullmatch(r"\s*\|?\s*:?-{2,}:?\s*(\|\s*:?-{2,}:?\s*)*\|?\s*", lines[i + 1])):
            cols = [c.strip() for c in line.strip().strip("|").split("|")]
            rows, i = [], i + 2
            while i < len(lines) and lines[i].strip().startswith("|"):
                cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
                rows.append({(cols[j] or f"col{j + 1}"): (cells[j] if j < len(cells) else "") for j in range(len(cols))})
                i += 1
            found.append({"section": heading, "columns": cols, "rows": rows})
            continue
        i += 1
    return found


def kind_of(path: Path, fm: dict) -> str:
    name = path.name.upper()
    if "NOTEBOOK" in name:
        return "series_notebook"
    if "GRAND_SYNTHESIS" in name or "MASTER_PAPER" in name and not fm.get("paper_id"):
        return "series_master"
    if fm.get("paper_id") or fm.get("type") == "axiom_companion" or re.search(r"_C\d_[0-9a-f]{6,}", path.stem):
        return "companion"
    return "other"


def number(v):
    try:
        return float(v) if isinstance(v, (int, float, str)) and str(v).strip() not in ("", "null") else None
    except ValueError:
        return None


def parse(path: Path, series: str | None = None) -> dict:
    raw = path.read_bytes()
    text = raw.decode("utf-8", errors="replace").replace("\r\n", "\n")   # Windows line endings read the same
    fm, rest = front_matter(text)
    cut = SOURCE_CUT.search(rest)
    analysis, source = (rest[:cut.start()], rest[cut.start():]) if cut else (rest, "")
    secs = sections(analysis)
    title = (next((s["title"] for s in secs if s["level"] == 1), "") or str(fm.get("clean_title") or fm.get("title") or path.stem))
    ids = sorted(set(ID_RE.findall(analysis)))
    head = {k: fm[k] for k in HEADLINE if k in fm}
    tabs = tables(analysis)
    head["sections"], head["tables"], head["words"] = len(secs), len(tabs), words(analysis)
    if "total_claims" not in head:                       # a companion that does not carry the flat key: count the ids
        claims = [i for i in ids if re.search(r"-C\d{3}$", i)]
        if claims:
            head["total_claims"] = len(claims)
    return {"file": path.name, "stem": path.stem, "path": str(path), "series": series or path.parent.name,
            "kind": kind_of(path, fm), "title": title, "sha256": hashlib.sha256(raw).hexdigest(),
            "analysis_words": words(analysis), "source_words": words(source), "front_matter": fm, "headline": head,
            "sections": secs, "tables": tabs, "ids": {"claims": [i for i in ids if re.search(r"-C\d{3}$", i)], "bridges": [i for i in ids if re.search(r"-B\d{3}$", i)],
             "predictions": [i for i in ids if re.search(r"-P\d{3}$", i)], "falsifiers": [i for i in ids if re.search(r"-F\d{3}$", i)],
             "hidden_premises": [i for i in ids if re.search(r"-HP\d{3}$", i)]}}


def cell(v) -> str:
    return "" if v is None else str(v).replace("|", "\\|").replace("\n", " ")


def sheet_md(rec: dict) -> str:
    h = rec["headline"]
    out = [f"---\nsource: {json.dumps(rec['path'], ensure_ascii=False)}\nseries: {json.dumps(rec['series'], ensure_ascii=False)}\n"
           f"kind: {rec['kind']}\nsha256: {rec['sha256']}\nupdated: {datetime.now():%Y-%m-%d %H:%M}\n---\n",
           f"# {rec['title']}\n", f"**{rec['kind'].replace('_', ' ')}** in series `{rec['series']}` · `{rec['file']}` · "
           f"{rec['analysis_words']:,} analysis words" + (f" · {rec['source_words']:,} source words" if rec["source_words"] else "") + "\n"]
    if h:
        out += ["## Headline\n", "| Field | Value |", "|---|---|", *[f"| {k.replace('_', ' ')} | {cell(v)} |" for k, v in h.items()], ""]
    if rec["ids"]["claims"] or rec["ids"]["bridges"] or rec["ids"]["predictions"]:
        out += ["## Stable ids\n"] + [f"- **{k.replace('_', ' ')}** ({len(v)}): " + ", ".join(v[:40]) + (" …" if len(v) > 40 else "")
                                      for k, v in rec["ids"].items() if v] + [""]
    out += ["## Outline\n"] + [f"{'  ' * (s['level'] - 1)}- {s['title']} ({s['words']:,} w)" for s in rec["sections"]] + [""]
    for t in rec["tables"]:
        out += [f"## Table · {t['section'] or 'untitled'}\n", "| " + " | ".join(cell(c) for c in t["columns"]) + " |",
                "|" + "---|" * len(t["columns"]), *["| " + " | ".join(cell(r.get(c if c else f'col{j + 1}')) for j, c in enumerate(t["columns"])) + " |"
                                                    for r in t["rows"]], ""]
    return "\n".join(out)


# ------------------------------------------------------------------ the master page

CSS = """:root{--bg:#fafaf7;--fg:#1d1d1b;--mut:#6b6a63;--line:#dcdad0;--card:#fff;--acc:#2d5a87;--ok:#2f7d4f;--warn:#9a6b12}
@media (prefers-color-scheme:dark){:root:not([data-theme=light]){--bg:#15161a;--fg:#e8e6df;--mut:#9a988f;--line:#2e3038;--card:#1d1f25;--acc:#7fb0e0;--ok:#6fc08e;--warn:#e0b25a}}
:root[data-theme=dark]{--bg:#15161a;--fg:#e8e6df;--mut:#9a988f;--line:#2e3038;--card:#1d1f25;--acc:#7fb0e0;--ok:#6fc08e;--warn:#e0b25a}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font:15px/1.5 system-ui,sans-serif}
main{max-width:1200px;margin:0 auto;padding:24px 16px 64px}h1{font-size:1.5rem;margin:0 0 4px}h2{font-size:1.1rem;margin:28px 0 8px}
.sub{color:var(--mut);margin:0 0 16px}.stats{display:flex;flex-wrap:wrap;gap:10px;margin:12px 0 20px}
.stat{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:10px 14px;min-width:120px}.stat b{display:block;font-size:1.3rem}.stat span{color:var(--mut);font-size:.8rem}
input{width:100%;max-width:360px;padding:8px 10px;border:1px solid var(--line);border-radius:6px;background:var(--card);color:var(--fg);margin-bottom:10px}
.wrap{overflow-x:auto;border:1px solid var(--line);border-radius:8px;background:var(--card)}table{border-collapse:collapse;width:100%;font-size:.88rem}
th,td{padding:7px 10px;border-bottom:1px solid var(--line);text-align:left;vertical-align:top}th{cursor:pointer;white-space:nowrap;position:sticky;top:0;background:var(--card);color:var(--mut);font-weight:600}
td.n{text-align:right;font-variant-numeric:tabular-nums}a{color:var(--acc)}tr.open td{background:color-mix(in srgb,var(--acc) 6%,transparent)}
.tag{display:inline-block;padding:1px 8px;border-radius:99px;border:1px solid var(--line);font-size:.75rem;color:var(--mut)}
.docs{display:grid;grid-template-columns:repeat(auto-fill,minmax(300px,1fr));gap:10px}.doc{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:12px 14px}
.doc h3{margin:0 0 4px;font-size:.95rem}.doc ul{margin:6px 0 0;padding-left:18px;color:var(--mut);font-size:.82rem}tr.detail td{background:var(--bg);font-size:.82rem;color:var(--mut)}
"""

JS = """const t=document.getElementById('t'),q=document.getElementById('q');
function val(td){const v=td.dataset.v;return v===undefined||v===''?null:(isNaN(v)?v.toLowerCase():parseFloat(v))}
t.querySelectorAll('th').forEach((th,i)=>th.addEventListener('click',()=>{const asc=th.dataset.o!=='a';t.querySelectorAll('th').forEach(x=>delete x.dataset.o);th.dataset.o=asc?'a':'d';
const body=t.tBodies[0],rows=[...body.querySelectorAll('tr.row')];rows.sort((a,b)=>{const x=val(a.cells[i]),y=val(b.cells[i]);if(x===y)return 0;if(x===null)return 1;if(y===null)return -1;return(x>y?1:-1)*(asc?1:-1)});
rows.forEach(r=>{body.appendChild(r);const d=document.getElementById('d'+r.dataset.i);if(d)body.appendChild(d)})}));
q.addEventListener('input',()=>{const s=q.value.toLowerCase();t.querySelectorAll('tr.row').forEach(r=>{const hit=r.textContent.toLowerCase().includes(s);r.hidden=!hit;const d=document.getElementById('d'+r.dataset.i);if(d)d.hidden=!hit||!r.classList.contains('open')})});
t.querySelectorAll('tr.row').forEach(r=>r.addEventListener('click',e=>{if(e.target.tagName==='A')return;r.classList.toggle('open');const d=document.getElementById('d'+r.dataset.i);if(d)d.hidden=!r.classList.contains('open')}));"""


def avg(values: list) -> str:
    nums = [x for x in map(number, values) if x is not None]
    return f"{sum(nums) / len(nums):.1f}" if nums else "–"


def series_html(title: str, records: list[dict], link) -> str:
    """`link(record, 'md'|'json')` gives the relative href of that file's sheet."""
    comp = [r for r in records if r["kind"] == "companion"]
    docs = [r for r in records if r["kind"] != "companion"]
    esc = html.escape
    stats = [("files", len(records)), ("companions", len(comp)), ("avg rating", avg([r["headline"].get("paper_rating") for r in comp])),
             ("claims", int(sum(number(r["headline"].get("total_claims")) or 0 for r in comp))),
             ("falsifiers", int(sum(number(r["headline"].get("claims_with_falsifiers")) or 0 for r in comp))),
             ("hidden premises", int(sum(number(r["headline"].get("hidden_premises")) or 0 for r in comp))),
             ("analysis words", f"{sum(r['analysis_words'] for r in records):,}")]
    cols = [("paper_rating", "rating"), ("evidence_status", "evidence"), ("formal_status", "formal"), ("coherence", "coherence"),
            ("total_claims", "claims"), ("claims_with_falsifiers", "falsifiers"), ("hidden_premises", "hidden prem."),
            ("predictions_logged", "predictions"), ("bridges_registered", "bridges"), ("lean_targets_queued", "lean")]
    head = "<th>#</th><th>file</th>" + "".join(f"<th>{c[1]}</th>" for c in cols) + "<th>words</th><th>sections</th><th>tables</th><th>json</th>"
    rows = []
    for i, r in enumerate(comp, 1):
        h = r["headline"]
        tds = "".join(f'<td class="n" data-v="{esc(str(h.get(k, "")))}">{esc(str(h.get(k, "")))}</td>' for k, _ in cols)
        outline = " · ".join(f"{esc(s['title'])} ({s['words']})" for s in r["sections"] if s["level"] <= 2) or "no headings"
        rows.append(f'<tr class="row" data-i="{i}"><td class="n" data-v="{i}">{i}</td><td data-v="{esc(r["title"])}"><a href="{esc(link(r, "md"))}">{esc(r["title"])}</a>'
                    f'<br><span class="tag">{esc(r["file"][:60])}</span></td>{tds}<td class="n" data-v="{r["analysis_words"]}">{r["analysis_words"]:,}</td>'
                    f'<td class="n" data-v="{h["sections"]}">{h["sections"]}</td><td class="n" data-v="{h["tables"]}">{h["tables"]}</td>'
                    f'<td data-v=""><a href="{esc(link(r, "json"))}">json</a></td></tr>'
                    f'<tr class="detail" id="d{i}" hidden><td></td><td colspan="{len(cols) + 5}">{outline}</td></tr>')
    cards = "".join(f'<div class="doc"><h3><a href="{esc(link(r, "md"))}">{esc(r["title"])}</a></h3><span class="tag">{esc(r["kind"].replace("_", " "))}</span> '
                    f'<span class="tag">{r["analysis_words"]:,} words</span><ul>' + "".join(f"<li>{esc(s['title'])}</li>" for s in [x for x in r["sections"] if x["level"] == 2][:8])
                    + "</ul></div>" for r in docs)
    return (f'<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">'
            f'<title>{esc(title)} sheet</title><style>{CSS}</style></head><body><main><h1>{esc(title)}</h1>'
            f'<p class="sub">Series sheet · built {datetime.now():%Y-%m-%d %H:%M} · read from the files as they are, nothing changed · click a row for its outline</p>'
            f'<div class="stats">{"".join(f"<div class=stat><b>{esc(str(v))}</b><span>{esc(k)}</span></div>" for k, v in stats)}</div>'
            + (f'<h2>Series documents</h2><div class="docs">{cards}</div>' if cards else "")
            + f'<h2>Papers</h2><input id="q" placeholder="filter…" aria-label="filter papers"><div class="wrap"><table id="t"><thead><tr>{head}</tr></thead>'
              f'<tbody>{"".join(rows)}</tbody></table></div></main><script>{JS}</script></body></html>')
