"""The ORIGINALS gate: every input a station runs on is copied ONCE into one originals folder.

Where: the `originals_root` key in config/paths.json (now \\\\192.168.2.50\\h_hp\\Desktop\\Youtube API CKG\\ORIGINALS).
Move the folder later by changing that one key.

How "once" works: each file's SHA-256 fingerprint is recorded in _ORIGINALS_LEDGER.jsonl inside that folder.
  * Same bytes seen before (any name, any station)  -> skipped, only a 'seen again' line is added.
  * New bytes                                        -> copied in under its own name; if that name is
                                                        taken by different bytes, it is saved as
                                                        <name>__<first 8 of hash><ext>. Nothing is overwritten.
The first time it runs, it fingerprints whatever is already in the folder, so files put there by
hand are recognized too.

It never stops a run: if the folder can't be reached, it prints one line and the station carries on.
"""
from __future__ import annotations

import hashlib
import json
import shutil
import threading
from datetime import datetime
from pathlib import Path

LEDGER = "_ORIGINALS_LEDGER.jsonl"
_LOCK = threading.Lock()


def _root() -> Path | None:
    from engine.paths import raw_value, expand
    raw = raw_value("originals_root")
    return expand(raw) if raw else None


def _sha(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def _load(root: Path) -> dict[str, str]:
    """sha -> stored name. Seeds the ledger from the folder's existing files the first time."""
    led = root / LEDGER
    known: dict[str, str] = {}
    if led.exists():
        for line in led.read_text(encoding="utf-8").splitlines():
            try:
                r = json.loads(line)
                if r.get("event") in ("stored", "seeded"):
                    known.setdefault(r["sha256"], r["name"])
            except Exception:
                continue
        return known
    rows = []
    for p in sorted(root.iterdir()):
        if p.is_file() and p.name != LEDGER:
            s = _sha(p)
            known.setdefault(s, p.name)
            rows.append(json.dumps({"event": "seeded", "sha256": s, "name": p.name,
                                    "at": datetime.now().isoformat(timespec="seconds")}, ensure_ascii=False))
    led.write_text("\n".join(rows) + ("\n" if rows else ""), encoding="utf-8")
    return known


def archive(files, station: str = "") -> dict:
    """Copy each file into ORIGINALS unless its exact bytes are already there. Returns counts."""
    res = {"stored": 0, "already": 0, "failed": 0}
    try:
        root = _root()
        if not root:
            print("  originals: no originals_root set in config/paths.json; skipped")
            return res
        if not root.exists():
            print(f"  originals: {root} not reachable; skipped (the run continues)")
            return res
    except Exception as e:
        print(f"  originals: skipped ({e})")
        return res
    with _LOCK:
        known = _load(root)
        rows = []
        for f in files:
            f = Path(f)
            try:
                if not f.is_file():
                    continue
                s = _sha(f)
                now = datetime.now().isoformat(timespec="seconds")
                if s in known:
                    res["already"] += 1
                    rows.append({"event": "seen_again", "sha256": s, "name": known[s], "source": str(f),
                                 "station": station, "at": now})
                    continue
                name = f.name
                if (root / name).exists():
                    name = f"{f.stem}__{s[:8]}{f.suffix}"
                shutil.copy2(f, root / name)
                known[s] = name
                res["stored"] += 1
                rows.append({"event": "stored", "sha256": s, "name": name, "source": str(f),
                             "station": station, "at": now})
            except Exception as e:
                res["failed"] += 1
                rows.append({"event": "failed", "source": str(f), "error": str(e)[:200], "station": station})
        if rows:
            with open(root / LEDGER, "a", encoding="utf-8") as fh:
                fh.write("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows))
    print(f"  originals: {res['stored']} new copied to ORIGINALS, {res['already']} already there"
          + (f", {res['failed']} failed" if res["failed"] else ""))
    return res
