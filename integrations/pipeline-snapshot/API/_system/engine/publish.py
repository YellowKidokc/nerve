"""Where answers go: onto the note they came from (David: "all the analysis always goes on the original").

publish_on_note(notes) runs tools/publish_analysis.py on each note, which rewrites the one block between
<!-- analysis:start --> and <!-- analysis:end --> (deep CKG, layers, argument grades, Fruits, YouTube catalogue),
so every station can call it after its run and the note always shows everything done so far. `look_in` names
output folders chosen at a button, so results written outside the usual OUTBOXes are found too.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

TOOL = Path(__file__).resolve().parents[1] / "tools" / "publish_analysis.py"


def publish_on_note(notes: list[Path], look_in: list[Path] | None = None) -> int:
    notes = [n for n in notes if n.suffix == ".md" and n.is_file()]
    if not notes:
        return 0
    print(f"\npublishing onto {len(notes)} note(s)", flush=True)
    extra = [a for d in (look_in or []) for a in ("--look-in", str(d))]
    return subprocess.run([sys.executable, str(TOOL), *map(str, notes), *extra]).returncode
