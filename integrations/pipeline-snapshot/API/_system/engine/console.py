"""What the Station Console (Nerve, Ctrl+Alt+W) shows: every station, its buttons and its folders, as JSON.

    python _system/engine/console.py list        all stations, read from config/stations.json + each station.json

Nerve runs this file and draws what it prints; it never reads the API folders itself. So any pipeline folder (one
per drive) that carries `_system/engine/console.py` shows up in the console the same way.
A button is any .bat in a station's front folder: running it from the console is the same as double-clicking it.
"""
from __future__ import annotations

import json
import sys
from datetime import datetime
from pathlib import Path

SYSTEM = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SYSTEM))
from engine.paths import MAIN, station_dir, station_rows     # noqa: E402

TEXT = ("FOCUS.md", "PROMPT.md", "README.md")


def count_files(folder: Path) -> int:
    if not folder.is_dir():
        return 0
    return sum(1 for p in folder.rglob("*") if p.is_file() and not p.name.startswith(("_", ".")))


def station(row: dict) -> dict:
    back = station_dir(row["label"])
    meta = {}
    if (back / "station.json").is_file():
        meta = json.loads((back / "station.json").read_text(encoding="utf-8"))
    front = back.parent if back.name == "BACKSIDE" else back
    last = front / "OUTBOX" / "_last_run.txt"
    parts = front.relative_to(MAIN).parts
    return {
        "number": row["number"],
        "label": row["label"],
        "name": row.get("name", ""),
        "kind": meta.get("kind", ""),
        "description": meta.get("description", ""),
        "uses_api": bool(meta.get("uses_api")),
        "options": meta.get("options", []),
        "family": parts[0] if len(parts) > 1 else "",
        "front": str(front),
        "path": "\\".join(parts),
        "layer_of": front.parent.parent.name if front.parent.name == "OUTBOX" else "",
        "buttons": [{"name": b.stem, "path": str(b)} for b in sorted(front.glob("*.bat"))],
        "inbox": count_files(front / "INBOX"),
        "outbox": count_files(front / "OUTBOX"),
        "last_run": datetime.fromtimestamp(last.stat().st_mtime).isoformat(timespec="minutes") if last.exists() else "",
        "text": {n: str(back / n) for n in TEXT if (back / n).is_file()},
    }


def listing() -> dict:
    rows = [r for r in station_rows() if not r.get("retired")]
    stations = []
    for row in rows:
        try:
            stations.append(station(row))
        except (OSError, ValueError) as e:
            stations.append({"number": row["number"], "label": row["label"], "error": str(e)})
    stations.sort(key=lambda s: s["number"])
    return {"root": str(MAIN), "system": str(SYSTEM), "stations": stations}


if __name__ == "__main__":
    if (sys.argv[1:] or ["list"])[0] != "list":
        raise SystemExit("usage: console.py list")
    sys.stdout.reconfigure(encoding="utf-8")
    print(json.dumps(listing(), ensure_ascii=False))
