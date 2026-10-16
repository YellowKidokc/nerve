"""Read and write a note's YAML front matter, so every action writes its small results the same way.

    fields(text) -> dict of the simple `key: value` lines (strings, JSON lists/numbers where they parse)
    set_fields(text, {"scriptures": [...], "words": 1234}) -> text with those keys replaced or added
Values are written as JSON (a list stays a list Obsidian can search). A note without front matter gets one.
"""
from __future__ import annotations

import json
import re


def _split(text: str) -> tuple[str, str]:
    if text.startswith("---") and "\n---" in text[3:]:
        end = text.index("\n---", 3)
        return text[:end], text[end:]
    return "---", "\n---\n" + text


def fields(text: str) -> dict:
    head, _ = _split(text)
    out = {}
    for m in re.finditer(r"^([A-Za-z_][\w-]*):\s*(.*)$", head, re.M):
        raw = m.group(2).strip()
        try:
            out[m.group(1)] = json.loads(raw)
        except ValueError:
            out[m.group(1)] = raw.strip('"')
    return out


def set_fields(text: str, values: dict) -> str:
    head, rest = _split(text)
    for key, value in values.items():
        line = f"{key}: {json.dumps(value, ensure_ascii=False)}"
        if re.search(rf"^{re.escape(key)}:", head, re.M):
            head = re.sub(rf"^{re.escape(key)}:.*$", lambda _: line, head, count=1, flags=re.M)
        else:
            head += "\n" + line
    return head + rest
