"""Scripture references in a source, found in code (no API, never invented).

Finds written and spoken forms, as they appear in papers and auto-captioned transcripts:
    John 3:16 · Jn 3:16-18 · 1 Cor 15:3-8 · First Corinthians 15 · 1st Peter 3:15 · I John 4
    John chapter 3 verse 16 · Romans chapter eight · Isaiah 53 · Psalm 22:1 · Acts 1 through 11
Each hit: {"ref": "1 Corinthians 15:3-8", "book": "1 Corinthians", "chapter": 15, "verses": "3-8",
           "where": "[12:31]" or "line 88", "context": "... the words around it ..."}
A bare book name ("in Genesis") is recorded once per book with chapter None, as a mention.

    python -m engine.scripture <file.md>          prints the table
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

# canonical name: spoken/written forms (lower case). Numbered books take their number from the prefix.
BOOKS = {
    "Genesis": ["genesis", "gen"], "Exodus": ["exodus", "exod", "ex"], "Leviticus": ["leviticus", "lev"],
    "Numbers": ["numbers", "num"], "Deuteronomy": ["deuteronomy", "deut", "dt"], "Joshua": ["joshua", "josh"],
    "Judges": ["judges", "judg"], "Ruth": ["ruth"], "Samuel": ["samuel", "sam"], "Kings": ["kings", "kgs"],
    "Chronicles": ["chronicles", "chron", "chr"], "Ezra": ["ezra"], "Nehemiah": ["nehemiah", "neh"],
    "Esther": ["esther", "esth"], "Job": ["job"], "Psalms": ["psalms", "psalm", "ps", "psa"],
    "Proverbs": ["proverbs", "prov"], "Ecclesiastes": ["ecclesiastes", "eccl", "qoheleth"],
    "Song of Songs": ["song of songs", "song of solomon", "song"], "Isaiah": ["isaiah", "isa"],
    "Jeremiah": ["jeremiah", "jer"], "Lamentations": ["lamentations", "lam"], "Ezekiel": ["ezekiel", "ezek"],
    "Daniel": ["daniel", "dan"], "Hosea": ["hosea", "hos"], "Joel": ["joel"], "Amos": ["amos"],
    "Obadiah": ["obadiah", "obad"], "Jonah": ["jonah"], "Micah": ["micah", "mic"], "Nahum": ["nahum", "nah"],
    "Habakkuk": ["habakkuk", "hab"], "Zephaniah": ["zephaniah", "zeph"], "Haggai": ["haggai", "hag"],
    "Zechariah": ["zechariah", "zech"], "Malachi": ["malachi", "mal"],
    "Matthew": ["matthew", "matt", "mt"], "Mark": ["mark", "mk"], "Luke": ["luke", "lk"], "John": ["john", "jn"],
    "Acts": ["acts"], "Romans": ["romans", "rom"], "Corinthians": ["corinthians", "cor"],
    "Galatians": ["galatians", "gal"], "Ephesians": ["ephesians", "eph"], "Philippians": ["philippians", "phil"],
    "Colossians": ["colossians", "col"], "Thessalonians": ["thessalonians", "thess"], "Timothy": ["timothy", "tim"],
    "Titus": ["titus"], "Philemon": ["philemon", "philem"], "Hebrews": ["hebrews", "heb"], "James": ["james", "jas"],
    "Peter": ["peter", "pet"], "Jude": ["jude"], "Revelation": ["revelation", "revelations", "rev"],
}
CHAPTERS = {"Genesis": 50, "Exodus": 40, "Leviticus": 27, "Numbers": 36, "Deuteronomy": 34, "Joshua": 24, "Judges": 21,
            "Ruth": 4, "1 Samuel": 31, "2 Samuel": 24, "1 Kings": 22, "2 Kings": 25, "1 Chronicles": 29,
            "2 Chronicles": 36, "Ezra": 10, "Nehemiah": 13, "Esther": 10, "Job": 42, "Psalms": 150, "Proverbs": 31,
            "Ecclesiastes": 12, "Song of Songs": 8, "Isaiah": 66, "Jeremiah": 52, "Lamentations": 5, "Ezekiel": 48,
            "Daniel": 12, "Hosea": 14, "Joel": 3, "Amos": 9, "Obadiah": 1, "Jonah": 4, "Micah": 7, "Nahum": 3,
            "Habakkuk": 3, "Zephaniah": 3, "Haggai": 2, "Zechariah": 14, "Malachi": 4, "Matthew": 28, "Mark": 16,
            "Luke": 24, "John": 21, "Acts": 28, "Romans": 16, "1 Corinthians": 16, "2 Corinthians": 13, "Galatians": 6,
            "Ephesians": 6, "Philippians": 4, "Colossians": 4, "1 Thessalonians": 5, "2 Thessalonians": 3,
            "1 Timothy": 6, "2 Timothy": 4, "Titus": 3, "Philemon": 1, "Hebrews": 13, "James": 5, "1 Peter": 5,
            "2 Peter": 3, "1 John": 5, "2 John": 1, "3 John": 1, "Jude": 1, "Revelation": 22}
NUMBERED = {"Samuel": 2, "Kings": 2, "Chronicles": 2, "Corinthians": 2, "Thessalonians": 2, "Timothy": 2,
            "Peter": 2, "John": 3}
# words that are also common English: only count them with a chapter number, never as a bare mention
AMBIGUOUS = {"job", "mark", "acts", "numbers", "song", "ex", "col", "phil", "mic", "hab", "lam", "hos", "am",
             "jude", "james", "john", "luke", "peter", "titus", "ruth", "joel", "amos", "jonah", "daniel", "timothy",
             "ps", "gen", "rev", "rom", "gal", "eph", "heb", "tim", "pet", "dan", "jer", "isa", "sam", "num", "lev",
             "mt", "mk", "lk", "jn", "dt", "kings", "judges", "revelations"}
ORDINAL = {"1": 1, "first": 1, "1st": 1, "i": 1, "2": 2, "second": 2, "2nd": 2, "ii": 2, "3": 3, "third": 3,
           "3rd": 3, "iii": 3}
UNITS = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
         "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]
TENS = {"twenty": 20, "thirty": 30, "forty": 40, "fifty": 50, "sixty": 60, "seventy": 70, "eighty": 80, "ninety": 90}
NUMWORD = r"(?:(?:twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)(?:[- ](?:one|two|three|four|five|six|seven|eight|nine))?|" \
          + "|".join(sorted(UNITS[1:], key=len, reverse=True)) + r")"
NUM = rf"(\d{{1,3}}|{NUMWORD})"
FORMS = sorted({f: b for b, fs in BOOKS.items() for f in fs}.items(), key=lambda kv: -len(kv[0]))
FORM_TO_BOOK = dict(FORMS)
BOOK_RE = "|".join(re.escape(f).replace(r"\ ", r"\s+") for f, _ in FORMS)
PATTERN = re.compile(
    rf"\b(?:(?P<ord>1st|2nd|3rd|first|second|third|iii|ii|i|[123])\s*)?(?P<book>{BOOK_RE})\.?"
    rf"(?:\s+(?:chapter\s+)?{NUM.replace('(', '(?P<ch>', 1)}"
    rf"(?:\s*(?::|\.(?=\d)|,?\s*verses?\s+|\s+v\.?\s*){NUM.replace('(', '(?P<v1>', 1)}"
    rf"(?:\s*(?:-|–|to|through|thru|and|&)\s*{NUM.replace('(', '(?P<v2>', 1)})?)?"
    rf"(?:\s*(?:-|–|through|thru)\s*{NUM.replace('(', '(?P<ch2>', 1)}(?!\s*(?::|verse)))?)?\b",
    re.IGNORECASE)
TIMESTAMP = re.compile(r"\[(\d{1,2}:\d{2}(?::\d{2})?)\]")


def number(token: str | None) -> int | None:
    if not token:
        return None
    t = token.lower().replace("-", " ").strip()
    if t.isdigit():
        return int(t)
    parts = t.split()
    total = TENS.get(parts[0], 0) + (UNITS.index(parts[-1]) if parts[-1] in UNITS else 0)
    return total or (UNITS.index(t) if t in UNITS else None)


def _where(text: str, pos: int) -> str:
    stamps = list(TIMESTAMP.finditer(text, 0, pos))
    return f"[{stamps[-1].group(1)}]" if stamps else f"line {text.count(chr(10), 0, pos) + 1}"


def source_part(text: str) -> tuple[str, int]:
    """(the source itself, its offset in text): the part from "## Transcript" on when there is one; otherwise the
    text without YAML. Our own scorecard and analysis blocks (publish_analysis) are blanked either way, so our
    reports are never read as the source."""
    if "\n## Transcript" in text:
        offset = text.index("\n## Transcript")
    elif text.startswith("---") and "\n---" in text[3:]:
        offset = text.index("\n---", 3) + 4
    else:
        offset = 0
    body = text[offset:]
    for a, b in (("<!-- analysis:start -->", "<!-- analysis:end -->"), ("<!-- scorecard:start -->", "<!-- scorecard:end -->")):
        body = re.sub(re.escape(a) + ".*?" + re.escape(b), lambda m: " " * len(m.group(0)), body, flags=re.S)
    return body, offset


def find(text: str) -> list[dict]:
    """Every reference, in order of first appearance; a reference said twice is listed once with all places."""
    body, offset = source_part(text)
    seen: dict[str, dict] = {}
    for m in PATTERN.finditer(body):
        form = re.sub(r"\s+", " ", m.group("book").lower())
        book = FORM_TO_BOOK.get(form)
        if not book:
            continue
        ch = number(m.group("ch"))
        if ch is None and (form in AMBIGUOUS or m.group("ord") is None and book in NUMBERED):
            continue                                        # "the job", "Mark said", bare "John": not a reference
        if ch is None and not m.group(0)[0].isupper():
            continue                                        # bare mentions only when capitalised ("Genesis")
        o = m.group("ord")
        if book in NUMBERED:
            n = ORDINAL.get(o.lower()) if o else None
            if n is None or n > NUMBERED[book]:
                if book == "John" and not o:
                    n = None                                # the Gospel
                else:
                    continue
            book = f"{n} {book}" if n else book
        elif o and o.lower() not in ("i",):                 # "2 Romans" is not a book
            continue
        if CHAPTERS.get(book) == 1 and ch and ch > 1 and not m.group("v1"):   # "Jude 14 (and 15)" = Jude 1:14-15
            v1, v2, ch2 = ch, number(m.group("ch2")), None
            ch = 1
        else:
            v1, v2, ch2 = number(m.group("v1")), number(m.group("v2")), number(m.group("ch2"))
        if ch is not None and ch > CHAPTERS.get(book, 150):
            continue                                        # no such chapter: a number, not a reference
        verses = f"{v1}-{v2}" if v1 and v2 else (str(v1) if v1 else "")
        ref = book + (f" {ch}" if ch else "") + (f":{verses}" if verses else "") + (f"-{ch2}" if ch2 and not verses else "")
        pos = m.start() + offset
        s = max(0, m.start() - 90); e = min(len(body), m.end() + 90)
        context = re.sub(r"\s+", " ", TIMESTAMP.sub("", re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", body[s:e]))).strip()
        hit = seen.setdefault(ref, {"ref": ref, "book": book, "chapter": ch, "verses": verses, "where": [],
                                    "context": context, "kind": "cited" if ch else "mentioned"})
        w = _where(text, pos)
        if w not in hit["where"]:
            hit["where"].append(w)
    # a bare book mention is dropped when the same book is also cited with a chapter
    cited_books = {h["book"] for h in seen.values() if h["chapter"]}
    return [h for h in seen.values() if h["chapter"] or h["book"] not in cited_books]


def table(hits: list[dict]) -> str:
    rows = ["| Reference | How | Where | Context |", "|---|---|---|---|"]
    for h in hits:
        where = ", ".join(h["where"][:6]) + (" …" if len(h["where"]) > 6 else "")
        rows.append(f"| {h['ref']} | {h['kind']} | {where} | {h['context'][:160].replace('|', '/')} |")
    return "\n".join(rows)


def prompt_block(hits: list[dict]) -> str:
    """What the CKG call is told: the detected list, every one of which must appear in its Scriptures table."""
    if not hits:
        return ("SCRIPTURE REFERENCES DETECTED IN CODE: none. Still list any passage the source quotes or paraphrases "
                "without naming it (kind: alluded), with the words that show it.")
    lines = [f"- {h['ref']} ({h['kind']}; {', '.join(h['where'][:3])})" for h in hits]
    return ("SCRIPTURE REFERENCES DETECTED IN CODE (verbatim from the source; every one MUST appear in the Scriptures "
            "table, with what the speaker says about it). Then ADD any passage the source quotes or paraphrases without "
            "naming it (kind: alluded) and quote the words that show it. Never add a passage the source does not use.\n"
            + "\n".join(lines))


HEADING = "## Scriptures"
TABLE_HEAD = "| Reference | How | Where | What is said about it |\n|---|---|---|---|"


def ensure_section(markdown: str, hits: list[dict], after: str = "## Definitions") -> str:
    """Make the report's Scriptures table complete: add the section (after the `after` section, else at the end)
    if the model left it out, and add a row for every detected reference the model's table does not name."""
    if HEADING not in markdown:
        block = f"\n{HEADING}\n\n{TABLE_HEAD}\n"
        i = markdown.find(after)
        if i >= 0:
            nxt = re.search(r"^#{1,2} ", markdown[i + len(after):], re.M)
            j = i + len(after) + nxt.start() if nxt else len(markdown)
            markdown = markdown[:j].rstrip() + "\n" + block + "\n" + markdown[j:]
        else:
            markdown = markdown.rstrip() + "\n" + block
    start = markdown.index(HEADING)
    nxt = re.search(r"^#{1,2} ", markdown[start + len(HEADING):], re.M)
    end = start + len(HEADING) + nxt.start() if nxt else len(markdown)
    section = markdown[start:end]
    said = re.sub(r"\s+", " ", section.lower())
    missing = [h for h in hits if h["ref"].lower() not in said and _short(h["ref"]).lower() not in said]
    if not missing:
        return markdown
    if "|---" not in section:
        section = section.rstrip() + "\n\n" + TABLE_HEAD
    rows = "\n".join(f"| {h['ref']} | {h['kind']} (found in code) | {', '.join(h['where'][:4])} | "
                     f"{h['context'][:160].replace('|', '/')} |" for h in missing)
    return markdown[:start] + section.rstrip() + "\n" + rows + "\n\n" + markdown[end:].lstrip("\n")


def standalone_prompt(hits: list[dict], source: str) -> str:
    """Its own small call (a long analysis reply plus 60 scripture rows overruns the 8k output cap)."""
    return ("List every Bible passage this source uses, in order of first use, as JSON:\n"
            '{"scriptures": [{"ref": "Book 1:2-3", "how": "cited|mentioned|alluded", "where": "timestamp or section", '
            '"said": "what the speaker does with it, at most 15 words; for alluded, the words that show it"}]}\n'
            "One row per use: a passage used for two different points gets two rows with their own timestamps.\n"
            + prompt_block(hits) + "\n\nSOURCE:\n" + source)


JSON_ASK = ('Also return "scriptures": [{"ref": "Book 1:2-3", "how": "cited|mentioned|alluded", "where": "timestamp", '
            '"said": "what the speaker does with it, one line; for alluded, the words that show it"}] listing every '
            'passage the source uses.')


def merge(hits: list[dict], model_rows: list | None) -> list[dict]:
    """The model's rows (kept as given) plus a row for every detected reference the model did not name."""
    rows = [{"ref": str(r.get("ref", "")).strip(), "how": str(r.get("how", "")).strip(), "where": str(r.get("where", "")),
             "said": str(r.get("said", "")).strip()} for r in (model_rows or []) if isinstance(r, dict) and r.get("ref")]
    named = {_norm(r["ref"]) for r in rows}
    for h in hits:
        if _norm(h["ref"]) not in named:
            rows.append({"ref": h["ref"], "how": f"{h['kind']} (found in code)", "where": ", ".join(h["where"][:4]),
                         "said": h["context"][:160]})
    return rows


def rows_table(rows: list[dict]) -> str:
    return "\n".join([TABLE_HEAD] + [f"| {r['ref']} | {r['how']} | {r['where']} | {r['said'].replace('|', '/')} |" for r in rows])


def _norm(ref: str) -> str:
    """One spelling per reference, so the model's "Ps 22:1" / "Psalm 22:1" / "1 Cor. 15:3" match ours."""
    hits = find(ref)
    return hits[0]["ref"].lower() if hits else re.sub(r"\s+", " ", ref.lower()).strip()


def _short(ref: str) -> str:
    """'1 Corinthians 15:3-8' -> '1 Cor 15:3-8' style, so a model's abbreviation still counts as named."""
    book, _, rest = ref.rpartition(" ") if re.search(r"\d", ref.split()[-1]) else (ref, "", "")
    words = book.split()
    return " ".join(w[:3] if w.isalpha() and len(w) > 4 else w for w in words) + (f" {rest}" if rest else "")


if __name__ == "__main__":
    for raw in sys.argv[1:]:
        hits = find(Path(raw).read_text(encoding="utf-8", errors="replace"))
        print(f"{Path(raw).name}: {len(hits)} reference(s)\n{table(hits)}\n")
