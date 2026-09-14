from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
from datetime import datetime, timezone
from pathlib import Path

import sys

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

SCRIPTS_DIR = Path(__file__).resolve().parent
PIPELINE_DIR = SCRIPTS_DIR.parent
INBOX = PIPELINE_DIR / "00_INBOX"
PROCESS = PIPELINE_DIR / "01_PROCESS"
PROMPT_TEMPLATE = SCRIPTS_DIR / "PROMPTS" / "knowledge_record.md"

def sha256_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()

def slug(value: str) -> str:
    clean = re.sub(r"[^a-zA-Z0-9]+", "-", value).strip("-").lower()
    return clean[:100] or "untitled"

def infer_title(path: Path, text: str) -> str:
    # 1. Direct frontmatter title match
    fm_match = re.search(r'(?m)^title:\s*["\']?([^"\'\r\n]+)["\']?\s*$', text)
    if fm_match:
        t = fm_match.group(1).strip()
        if t and not t.lower().startswith("canonical sequential"):
            return t

    # 2. Main Markdown heading
    for line in text.splitlines():
        candidate = line.strip().lstrip("#").strip()
        if candidate and len(candidate) <= 180:
            if candidate.startswith(("<", "!--", "---", "status:", "title:", "type:")):
                continue
            if candidate.lower().startswith("canonical sequential record"):
                continue
            # Strip emoji if any (e.g. 🟢 A1.1 — Existence -> A1.1 — Existence)
            clean_cand = re.sub(r"^[^\w\s\d\-—–]+", "", candidate).strip()
            if clean_cand:
                return clean_cand
    return path.stem.replace("_", " ").replace("-", " ").strip().title()

def infer_type(text: str) -> tuple[str, float, list[str]]:
    lower = text.lower()
    signals: list[str] = []
    
    # 1. Immediate Axiom Companion detection
    if any(m in text for m in ("Canonical Sequential Record", "atlas_type: \"Axiom\"", "atlas_type: 'Axiom'", "Mode | `AX_CORE`", "Mode | AX_CORE", "Sealed chain authority", "Front Identity Layer", "Production-Confirmed Identity", "Atom Meaning Block", "Mode: AX_CORE")):
        return "axiom_companion", 0.98, ["canonical axiom record", "sealed v2.3 authority", "axiom companion layout"]

    scores = {"conversation": 0, "article": 0, "system": 0, "research_synthesis": 0}
    if re.search(r"(?m)^(user|assistant|human|codex|claude):", lower):
        scores["conversation"] += 4
        signals.append("speaker labels")
    if lower.count("## ") >= 3:
        scores["article"] += 2
        signals.append("sectioned text")
    if any(term in lower for term in ("what the evidence", "original paper", "negative control")):
        scores["research_synthesis"] += 4
        signals.append("evidence analysis")
    if any(term in lower for term in ("system model", "architecture", "framework", "ontology")):
        scores["system"] += 3
        signals.append("system language")

    ranked = sorted(scores.items(), key=lambda item: item[1], reverse=True)
    best_type, best_score = ranked[0]
    return best_type if best_score else "article", 0.85, signals

def infer_tags(text: str) -> list[str]:
    vocabulary = {
        "consciousness": ("conscious", "sentience", "subjective"),
        "lean4": ("lean", "formal theorem", "typeclass", "proof"),
        "theology": ("theological", "christian", "god", "trinity", "perichoresis"),
        "systems": ("system", "closed-loop", "framework", "substrate"),
        "quantum-foundations": ("quantum", "gleason", "born rule", "collapse", "wigner"),
        "information-theory": ("shannon", "kolmogorov", "entropy", "bit")
    }
    padded = f" {text.lower()} "
    return sorted(tag for tag, terms in vocabulary.items() if any(term in padded for term in terms))

def section_outline(text: str) -> list[str]:
    headings = []
    for line in text.splitlines():
        match = re.match(r"^#{1,4}\s+(.+?)\s*$", line)
        if match:
            headings.append(match.group(1))
    return headings[:30]

def render_record(metadata: dict, text: str) -> str:
    tags = json.dumps(metadata["tags"])
    outline_lines = "\n".join(f"- {h}" for h in metadata["source_outline"]) or "- [No headings extracted]"
    return f"""---
type: {metadata['inferred_type']}
title: {json.dumps(metadata['title'], ensure_ascii=False)}
status: CANDIDATE_DRAFT
canon_status: candidate_draft
semantic_status: pending_ai_analysis
source_file: {json.dumps(str(metadata['source_file']), ensure_ascii=False)}
source_sha256: {metadata['source_sha256']}
captured_at: {metadata['captured_at']}
tags: {tags}
---

# {metadata['title']}

> **CANDIDATE_DRAFT — NOT ADMITTED.** Source preserved below. Analytical case file pending.

## Intake assessment

- Inferred type: `{metadata['inferred_type']}`
- Routing confidence: `{metadata['routing_confidence']}`
- Signals: {', '.join(metadata['routing_signals'])}
- Original words: `{metadata['word_count']}`
- Original SHA-256: `{metadata['source_sha256']}`

## Source outline

{outline_lines}

## Complete preserved source

<!-- BEGIN EXACT SOURCE PROJECTION; authoritative bytes are preserved in ARCHIVE -->

{text.rstrip()}

<!-- END EXACT SOURCE PROJECTION -->
"""

def prepare_packet(file_path: Path) -> Path:
    raw_bytes = file_path.read_bytes()
    raw_text = raw_bytes.decode("utf-8", errors="replace")
    file_sha256 = sha256_bytes(raw_bytes)
    base_slug = slug(file_path.stem)
    
    inferred_t, confidence, signals = infer_type(raw_text)
    metadata = {
        "title": infer_title(file_path, raw_text),
        "inferred_type": inferred_t,
        "routing_confidence": confidence,
        "routing_signals": signals,
        "canon_status": "candidate_draft",
        "source_file": str(file_path),
        "source_sha256": file_sha256,
        "captured_at": datetime.now(timezone.utc).isoformat(),
        "word_count": len(raw_text.split()),
        "tags": infer_tags(raw_text),
        "source_outline": section_outline(raw_text),
    }
    
    PROCESS.mkdir(parents=True, exist_ok=True)
    
    # 1. Write intake JSON
    intake_json_path = PROCESS / f"{base_slug}.intake.json"
    intake_json_path.write_text(json.dumps(metadata, indent=2), encoding="utf-8")
    
    # 2. Write initial knowledge draft
    knowledge_path = PROCESS / f"{base_slug}.knowledge.md"
    knowledge_path.write_text(render_record(metadata, raw_text), encoding="utf-8")
    
    # 3. Write analysis prompt
    template = PROMPT_TEMPLATE.read_text(encoding="utf-8")
    prompt_text = template.replace("{{METADATA}}", json.dumps(metadata, indent=2)).replace("{{SOURCE}}", raw_text)
    prompt_path = PROCESS / f"{base_slug}.analysis-prompt.md"
    prompt_path.write_text(prompt_text, encoding="utf-8")
    
    return prompt_path

def main():
    parser = argparse.ArgumentParser(description="Prepare INBOX files for CKG Analysis.")
    parser.add_argument("--files", nargs="*", default=[], help="Specific files to prepare.")
    parser.add_argument("--limit", type=int, default=0, help="Maximum number of files to prepare from INBOX (0 = all).")
    args = parser.parse_args()
    
    files_to_process = []
    if args.files:
        files_to_process = [Path(f) for f in args.files if Path(f).exists()]
    else:
        files_to_process = sorted([f for f in INBOX.iterdir() if f.is_file() and f.suffix.lower() in {".md", ".txt", ".html"}])
    
    if args.limit > 0:
        files_to_process = files_to_process[:args.limit]
        
    print(f"[*] Found {len(files_to_process)} file(s) in INBOX to prepare.")
    prepared = []
    for fp in files_to_process:
        try:
            prompt_path = prepare_packet(fp)
            prepared.append(prompt_path)
            print(f"[PREPARED] {fp.name} -> {prompt_path.name}")
        except Exception as e:
            print(f"[ERROR] Failed to prepare {fp.name}: {e}")
            
    print(f"[OK] Packet preparation complete: {len(prepared)} ready in 01_PROCESS.")
    return 0

if __name__ == "__main__":
    main()
