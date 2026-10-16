"""22_CKG_INBOX_CHECK: wrapped legacy station. Everything it runs is declared in station.json."""
import sys
from pathlib import Path
sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / "_system").is_dir()) / "_system"))
from engine.legacy import main

if __name__ == "__main__":
    raise SystemExit(main("22_CKG_INBOX_CHECK"))
