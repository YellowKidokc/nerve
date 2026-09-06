#!/usr/bin/env python3
"""
batch_papers_to_atoms.py
========================
High-speed batch processor that takes 1 to 20+ papers (individually or as a series),
runs DeepSeek (or Ollama) in parallel or fast queue, and writes out a single,
perfect 'nerve-editable-draft-*.json' file that imports into ATOM Builder with ONE click.

Usage:
  python batch_papers_to_atoms.py --folder "Z:\Theophysics_Vault\01_CANON\Chapter_1"
  python batch_papers_to_atoms.py --files paper1.md paper2.md paper3.md
  python batch_papers_to_atoms.py --folder "path" --provider deepseek --workers 4
"""

import os
import sys
import json
import uuid
import argparse
from pathlib import Path
from datetime import datetime
from concurrent.futures import ThreadPoolExecutor, as_completed
import urllib.request
import urllib.parse

try:
    import json_repair
except ImportError:
    json_repair = None

DEEPSEEK_API_KEY = os.environ.get("DEEPSEEK_API_KEY", "")
NAS_OLLAMA_URL = "https://ollama.dlowehomelab.com"

SYSTEM_PROMPT = """You are the governed Claim Atom Builder for Theophysics.
Analyze the provided source paper and extract candidate contents for the 4 core native objects:
1. CLAIM (the narrowest technical assertion, statement, scope, claim mode)
2. EVIDENCE (verbatim quote anchor, source coordinates, discriminating power)
3. PROOF (premises, derivation steps, conclusion, formal boundaries)
4. PROCESS (step-by-step reproducible method, inputs, outputs, verification)

Return ONLY valid JSON matching this structure:
{
  "title": "Paper Title / Theme",
  "statement": "Narrow technical statement of the primary claim",
  "claim_mode": "LOGICAL | MATHEMATICAL | EMPIRICAL | HISTORICAL | PHILOSOPHICAL | THEOLOGICAL | BRIDGE | CONJECTURE",
  "scope": "UNIVERSAL | DOMAIN_SPECIFIC | LOCAL | CONJECTURAL",
  "quote_anchor": "Exact verbatim quote from the text",
  "evidence_description": "What alternative rival hypothesis this discriminates against",
  "premises": ["Premise 1", "Premise 2"],
  "derivation_steps": ["Step 1", "Step 2", "Conclusion"],
  "process_steps": ["Step 1", "Step 2"],
  "math_equation": "E.g. mathematical relation if present",
  "open_questions": ["Any unresolved tension or open question"]
}
Do NOT wrap in markdown fences. Output clean JSON only."""

def call_deepseek(paper_text: str, filename: str) -> dict:
    if not DEEPSEEK_API_KEY:
        raise RuntimeError("DEEPSEEK_API_KEY environment variable is not set.")
    url = "https://api.deepseek.com/chat/completions"
    user_prompt = f"SOURCE FILE: {filename}\n\n=== TEXT ===\n{paper_text[:20000]}\n=== END TEXT ==="
    payload = {
        "model": "deepseek-chat",
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user_prompt}
        ],
        "max_tokens": 4000,
        "temperature": 0.2
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url, data=data,
        headers={"Authorization": f"Bearer {DEEPSEEK_API_KEY}", "Content-Type": "application/json"},
        method="POST"
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        body = json.loads(resp.read().decode("utf-8"))
        raw = body["choices"][0]["message"]["content"].strip()
    return parse_json(raw)

def call_ollama(paper_text: str, filename: str, model: str = "llama3.2:latest") -> dict:
    url = f"{NAS_OLLAMA_URL}/api/generate"
    user_prompt = f"SOURCE FILE: {filename}\n\n=== TEXT ===\n{paper_text[:16000]}\n=== END TEXT ==="
    payload = {
        "model": model,
        "prompt": f"System: {SYSTEM_PROMPT}\n\nUser: {user_prompt}",
        "format": "json",
        "stream": False,
        "options": {"num_predict": 4000, "temperature": 0.1}
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url, data=data,
        headers={"Content-Type": "application/json", "User-Agent": "Nerve-Batch-Atoms/1.0"},
        method="POST"
    )
    with urllib.request.urlopen(req, timeout=180) as resp:
        body = json.loads(resp.read().decode("utf-8"))
        raw = body.get("response", "").strip()
    return parse_json(raw)

def parse_json(raw: str) -> dict:
    text = raw.strip()
    if text.startswith("```"):
        lines = text.split("\n")
        if lines[0].startswith("```"):
            lines = lines[1:]
        if lines and lines[-1].startswith("```"):
            lines = lines[:-1]
        text = "\n".join(lines).strip()
    try:
        return json.loads(text)
    except Exception:
        if json_repair:
            return json_repair.loads(text)
        start, end = text.find("{"), text.rfind("}")
        if start >= 0 and end > start:
            return json.loads(text[start:end+1])
        raise

def build_draft_snapshot(processed_papers: list[dict]) -> dict:
    """Builds the exact nerve-editable-draft JSON that atom-builder.html imports."""
    atoms = {}
    html = {}
    atom_counter = 1

    for item in processed_papers:
        filename = item["filename"]
        raw_text = item["raw_text"]
        data = item["data"]
        fam_name = Path(filename).stem

        # Create 4 Atoms per paper: CLAIM, EVIDENCE, PROOF, PROCESS
        types = [
            ("CLAIM", {
                "title": data.get("title", fam_name),
                "statement": data.get("statement", ""),
                "st_tech": data.get("statement", ""),
                "purpose": f"Derived from {filename}",
                "cclass": data.get("claim_mode", "LOGICAL"),
                "scope": data.get("scope", "UNIVERSAL"),
                "raw": raw_text,
                "src_uri": filename,
                "src_span": f"Whole file - {len(raw_text)} chars"
            }, {
                "claim_mode": (data.get("claim_mode", "logical")).lower(),
                "epistemic_grade": "asserted"
            }),
            ("EVIDENCE", {
                "title": f"Evidence for {data.get('title', fam_name)}",
                "statement": f"Verbatim anchor evidence supporting {fam_name}",
                "edge_disc": data.get("evidence_description", ""),
                "src_span": data.get("quote_anchor", "")[:250],
                "raw": raw_text,
                "src_uri": filename
            }, {
                "evidence_type": "textual_quote"
            }),
            ("PROOF", {
                "title": f"Proof of {data.get('title', fam_name)}",
                "statement": "Premises and derivation chain",
                "proof_premises": "\n".join(data.get("premises", [])),
                "proof_steps": "\n".join(data.get("derivation_steps", [])),
                "eq": data.get("math_equation", ""),
                "raw": raw_text,
                "src_uri": filename
            }, {}),
            ("PROCESS", {
                "title": f"Process Method for {data.get('title', fam_name)}",
                "statement": "Operational method & reproduction steps",
                "pr_ops": "\n".join(data.get("process_steps", [])),
                "raw": raw_text,
                "src_uri": filename
            }, {})
        ]

        for otype, vals, tg in types:
            aid = f"A{str(atom_counter).zfill(3)}"
            u = str(uuid.uuid4())
            atoms[aid] = {
                "uuid": u,
                "otype": otype,
                "fam": fam_name,
                "reg": "formal",
                "exported": False,
                "uuid_history": []
            }
            html[aid] = {
                "vals": vals,
                "tg": tg,
                "terms": [],
                "ledger": [],
                "lnks": []
            }
            atom_counter += 1

    return {
        "format": "nerve-editable-draft/1.0.0",
        "exported_at": datetime.now().isoformat(),
        "authority": "EDITABLE DRAFT — NOT VALIDATED — NOT ADMITTED",
        "nerve_draft": {
            "n": atom_counter - 1,
            "atoms": atoms,
            "html": html
        }
    }

def process_single_file(path: Path, provider: str, model: str) -> dict:
    print(f"[*] Processing {path.name} via {provider.upper()}...")
    text = path.read_text(encoding="utf-8", errors="replace")
    if provider == "deepseek":
        data = call_deepseek(text, path.name)
    else:
        data = call_ollama(text, path.name, model=model)
    print(f"[✓] Extracted: {path.name} -> '{data.get('title', path.stem)}'")
    return {"filename": path.name, "path": str(path), "raw_text": text, "data": data}

def main():
    parser = argparse.ArgumentParser(description="Batch convert papers to ATOM Builder Draft JSON")
    parser.add_argument("--folder", help="Folder containing .md or .txt papers")
    parser.add_argument("--files", nargs="*", help="Specific list of files")
    parser.add_argument("--provider", choices=["deepseek", "ollama"], default="deepseek", help="AI Provider (default: deepseek)")
    parser.add_argument("--model", default="llama3.2:latest", help="Ollama model name if using ollama")
    parser.add_argument("--workers", type=int, default=4, help="Number of parallel workers (default: 4)")
    parser.add_argument("--output", default="nerve-batch-atoms-draft.json", help="Output JSON path")
    args = parser.parse_args()

    files = []
    if args.folder:
        folder = Path(args.folder)
        if not folder.exists():
            print(f"Error: folder not found: {folder}")
            sys.exit(1)
        files = sorted(list(folder.glob("*.md")) + list(folder.glob("*.txt")))
    elif args.files:
        files = [Path(f) for f in args.files if Path(f).exists()]

    if not files:
        print("No files found. Specify --folder or --files.")
        sys.exit(1)

    print(f"=== Nerve High-Speed ATOM Batch Pipeline ===")
    print(f"Found {len(files)} paper(s). Target Provider: {args.provider.upper()}. Parallel workers: {args.workers}")

    results = []
    with ThreadPoolExecutor(max_workers=args.workers) as executor:
        future_map = {executor.submit(process_single_file, f, args.provider, args.model): f for f in files}
        for future in as_completed(future_map):
            f = future_map[future]
            try:
                res = future.result()
                results.append(res)
            except Exception as exc:
                print(f"[!] Error on {f.name}: {exc}")

    if not results:
        print("No papers were successfully processed.")
        sys.exit(1)

    # Sort results to match original file order
    results.sort(key=lambda x: x["filename"])

    snapshot = build_draft_snapshot(results)
    out_path = Path(args.output).resolve()
    out_path.write_text(json.dumps(snapshot, indent=2), encoding="utf-8")

    print(f"\n========================================================")
    print(f"SUCCESS: Generated draft snapshot for {len(results)} paper(s) ({len(results)*4} ATOMs total)!")
    print(f"Saved to: {out_path}")
    print(f"To view in ATOM Builder:")
    print(f"  1. Open Atom Builder in browser or Obsidian")
    print(f"  2. Click 'Import JSON' at the top")
    print(f"  3. Select: {out_path.name}")
    print(f"  -> All {len(results)*4} Atom cards and boxes will populate instantly!")
    print(f"========================================================")

if __name__ == "__main__":
    main()
