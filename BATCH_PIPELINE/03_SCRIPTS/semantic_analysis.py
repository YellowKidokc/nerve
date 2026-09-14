from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

# Local validator
from validate_ckg import validate_ckg

SCRIPTS_DIR = Path(__file__).resolve().parent
PIPELINE_DIR = SCRIPTS_DIR.parent
INBOX = PIPELINE_DIR / "00_INBOX"
ARCHIVED_INBOX = INBOX / "ARCHIVED_INPUTS"
PROCESS = PIPELINE_DIR / "01_PROCESS"
REVISIONS = PROCESS / "REVISIONS"
OUTBOX = PIPELINE_DIR / "02_OUTBOX"
LOGS = PIPELINE_DIR / "LOGS"

# Mirror backup directory for Synology sync
SYNOLOGY_MIRROR = Path(r"C:\Users\David\Documents\faiththruphysics.com\999_API_CALLS_OUTPUT")


def log_message(msg: str) -> None:
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    formatted = f"[{timestamp}] {msg}"
    print(formatted)
    try:
        LOGS.mkdir(parents=True, exist_ok=True)
        log_file = LOGS / f"batch_pipeline_{datetime.now().strftime('%Y%m%d')}.log"
        with log_file.open("a", encoding="utf-8") as f:
            f.write(formatted + "\n")
    except Exception:
        pass


def get_openrouter_key() -> str:
    key = os.environ.get("OPENROUTER_API_KEY", "").strip()
    if key:
        return key
    if os.name == "nt":
        try:
            import winreg
            with winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Environment") as rkey:
                val, _ = winreg.QueryValueEx(rkey, "OPENROUTER_API_KEY")
                return str(val).strip()
        except Exception:
            pass
    raise RuntimeError(
        "OPENROUTER_API_KEY is not configured in environment or user registry. "
        "Please set OPENROUTER_API_KEY."
    )


def clean_llm_response(text: str) -> str:
    text = text.strip()
    # Strip markdown fence if the model wrapped the entire output in ```markdown ... ```
    fenced = re.fullmatch(r"```(?:markdown|md)?\s*(.*?)\s*```", text, re.DOTALL | re.IGNORECASE)
    if fenced:
        text = fenced.group(1).strip()
    return text


def call_openrouter(
    prompt: str,
    primary_model: str = "deepseek/deepseek-v3.2",
    fallback_model: str = "deepseek/deepseek-r1",
    timeout: int = 180,
    retries: int = 3,
) -> tuple[str, str, dict]:
    api_key = get_openrouter_key()
    models_to_try = [primary_model]
    if fallback_model and fallback_model != primary_model:
        models_to_try.append(fallback_model)

    last_err = None
    for model in models_to_try:
        is_fallback = (model != primary_model)
        if is_fallback:
            log_message(f"Primary model rate-limited or failed; switching to paid fallback: {model}")

        body = json.dumps({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": (
                        "You are a strict first-principles CKG research and epistemics engine. "
                        "Follow the required 14-heading markdown contract exactly. "
                        "Extract all irreducible truth predicates into a table under 'System or model'. "
                        "Separate formal theorems, empirical facts, and theological interpretations strictly. "
                        "Never write generic filler in 'Central claim' and never include sorry or admit in formal results."
                    )
                },
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.1,
            "max_tokens": 7000
        }, ensure_ascii=False).encode("utf-8")

        headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://consilience-atlas.theophysics.org",
            "X-Title": "Consilience Atlas Autonomous Batch Runner"
        }

        req = urllib.request.Request(
            "https://openrouter.ai/api/v1/chat/completions",
            data=body,
            method="POST",
            headers=headers
        )

        for attempt in range(1, retries + 1):
            try:
                log_message(f"Calling OpenRouter ({model}) [Attempt {attempt}/{retries}]...")
                with urllib.request.urlopen(req, timeout=timeout) as resp:
                    data = json.loads(resp.read().decode("utf-8"))
                
                content = clean_llm_response(data["choices"][0]["message"]["content"])
                usage = data.get("usage", {})
                
                # Immediate gate check
                validate_ckg(content)
                log_message(f"Gate validation passed for model {model}.")
                return content, model, usage
            except urllib.error.HTTPError as http_err:
                last_err = http_err
                status_code = http_err.code
                err_body = ""
                try:
                    err_body = http_err.read().decode("utf-8")
                except Exception:
                    pass
                log_message(f"HTTP {status_code} on {model}: {err_body[:200]}")
                if status_code in (429, 503, 502) and not is_fallback:
                    # Break out of attempts and fall back to paid model
                    break
                time.sleep(2 ** attempt)
            except Exception as e:
                last_err = e
                log_message(f"Validation or network error on {model} attempt {attempt}: {e}")
                time.sleep(2 ** attempt)

    raise RuntimeError(f"All models failed for prompt: {last_err}")


def extract_truth_predicates_from_text(markdown: str) -> list[dict]:
    predicates = []
    # Look for table under Extracted Truth Predicates
    table_match = re.search(r"\|\s*#\s*\|\s*Truth Predicate\s*\|\s*Source Role\s*\|\s*Formal / Mathematical Formulation\s*\|(.*?)(?=\n\n|\n##|\Z)", markdown, re.DOTALL | re.IGNORECASE)
    if table_match:
        lines = [l.strip() for l in table_match.group(1).splitlines() if l.strip().startswith("|") and not "---" in l]
        for line in lines:
            parts = [p.strip() for p in line.strip("|").split("|")]
            if len(parts) >= 2:
                predicates.append({
                    "id": parts[0],
                    "statement": parts[1],
                    "role": parts[2] if len(parts) > 2 else "Foundational assertion",
                    "formal": parts[3] if len(parts) > 3 else ""
                })
    return predicates


def assemble_master_record(
    metadata: dict,
    analysis_text: str,
    raw_source_text: str,
    model_used: str,
    usage: dict
) -> tuple[str, dict]:
    captured = datetime.now(timezone.utc).isoformat()
    clean_title = metadata.get("title", "Untitled Record")
    source_sha256 = metadata.get("source_sha256", "")
    inferred_type = metadata.get("inferred_type", "article")
    tags = metadata.get("tags", ["consilience-atlas", "axiom", "lean4", "systems"])
    outline_lines = "\n".join(f"- {h}" for h in metadata.get("source_outline", [])) or "- [No headings extracted]"

    truth_predicates = extract_truth_predicates_from_text(analysis_text)

    # Master Markdown Record
    markdown_master = f"""---
type: {inferred_type}
title: "ckg_evaluation: {clean_title}"
status: CANDIDATE_DRAFT
canon_status: candidate_draft
semantic_status: ai_analyzed_pending_review
template_version: CKG_RECORD_V2.0
source_file: {json.dumps(str(metadata.get('source_file')), ensure_ascii=False)}
source_sha256: {source_sha256}
captured_at: {captured}
tags: {json.dumps(tags)}
semantic_provider: "openrouter"
semantic_model: "{model_used}"
semantic_analyzed_at: "{captured}"
usage_tokens: {json.dumps(usage)}
---

# ckg_evaluation: {clean_title}

> **CANDIDATE_DRAFT — NOT ADMITTED.** The source is preserved below.
> Evaluated autonomously by Consilience Atlas Large-Batch CKG Pipeline.

## Intake assessment

- Inferred type: `{inferred_type}`
- Routing confidence: `{metadata.get('routing_confidence', 0.95)}`
- Signals: {', '.join(metadata.get('routing_signals', []))}
- Original words: `{metadata.get('word_count', 0)}`
- Original SHA-256: `{source_sha256}`

## Source outline

{outline_lines}

{analysis_text.strip()}

## Complete preserved source

<!-- BEGIN EXACT SOURCE PROJECTION; authoritative original bytes are permanently preserved -->

{raw_source_text.rstrip()}

<!-- END EXACT SOURCE PROJECTION -->
"""

    # Master JSON Record
    json_master = {
        "title": clean_title,
        "object_type": "AXIOM" if inferred_type == "axiom_companion" else "ATOM",
        "inferred_type": inferred_type,
        "status": "CANDIDATE_DRAFT",
        "canon_status": "candidate_draft",
        "semantic_status": "ai_analyzed_pending_review",
        "template_version": "CKG_RECORD_V2.0",
        "source_file": metadata.get("source_file"),
        "source_sha256": source_sha256,
        "captured_at": captured,
        "tags": tags,
        "semantic_provider": "openrouter",
        "semantic_model": model_used,
        "usage": usage,
        "truth_predicates": truth_predicates,
        "source_outline": metadata.get("source_outline", [])
    }

    return markdown_master, json_master


def process_packet(
    prompt_path: Path,
    primary_model: str = "deepseek/deepseek-r1:free",
    fallback_model: str = "deepseek/deepseek-v3.2"
) -> Path:
    base_slug = prompt_path.name.removesuffix(".analysis-prompt.md")
    intake_json_path = PROCESS / f"{base_slug}.intake.json"
    knowledge_path = PROCESS / f"{base_slug}.knowledge.md"

    if not intake_json_path.exists():
        raise FileNotFoundError(f"Missing intake metadata: {intake_json_path}")
    if not knowledge_path.exists():
        raise FileNotFoundError(f"Missing knowledge draft: {knowledge_path}")

    metadata = json.loads(intake_json_path.read_text(encoding="utf-8"))
    prompt_text = prompt_path.read_text(encoding="utf-8")

    # Read the original source from knowledge draft
    knowledge_content = knowledge_path.read_text(encoding="utf-8")
    marker = "## Complete preserved source"
    if marker in knowledge_content:
        source_part = knowledge_content.split(marker, 1)[1]
        # Strip comments
        raw_source_text = source_part.replace("<!-- BEGIN EXACT SOURCE PROJECTION; authoritative bytes are preserved in ARCHIVE -->", "").replace("<!-- END EXACT SOURCE PROJECTION -->", "").strip()
    else:
        raw_source_text = ""

    # Call AI
    analysis_text, model_used, usage = call_openrouter(
        prompt=prompt_text,
        primary_model=primary_model,
        fallback_model=fallback_model
    )

    # Assemble master record
    md_master, json_obj = assemble_master_record(
        metadata=metadata,
        analysis_text=analysis_text,
        raw_source_text=raw_source_text,
        model_used=model_used,
        usage=usage
    )

    # If previous version exists in OUTBOX, move to REVISIONS
    OUTBOX.mkdir(parents=True, exist_ok=True)
    master_md_path = OUTBOX / f"{base_slug}_CKG_MASTER.md"
    master_json_path = OUTBOX / f"{base_slug}_CKG_MASTER.json"

    if master_md_path.exists():
        REVISIONS.mkdir(parents=True, exist_ok=True)
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        rev_path = REVISIONS / f"{base_slug}_CKG_MASTER_{timestamp}.md"
        shutil.move(str(master_md_path), str(rev_path))
        log_message(f"Moved prior master to revisions: {rev_path.name}")

    master_md_path.write_text(md_master, encoding="utf-8")
    master_json_path.write_text(json.dumps(json_obj, indent=2), encoding="utf-8")
    log_message(f"[OUTBOX WRITTEN] {master_md_path.name}")

    # Mirror to Synology backup directory
    try:
        SYNOLOGY_MIRROR.mkdir(parents=True, exist_ok=True)
        syn_md = SYNOLOGY_MIRROR / master_md_path.name
        syn_json = SYNOLOGY_MIRROR / master_json_path.name
        shutil.copy2(str(master_md_path), str(syn_md))
        shutil.copy2(str(master_json_path), str(syn_json))
        log_message(f"[SYNOLOGY MIRROR] Synced to {syn_md.name}")
    except Exception as e:
        log_message(f"[WARN] Failed to copy to Synology mirror: {e}")

    # Move source file from INBOX to ARCHIVED_INPUTS
    source_file_str = metadata.get("source_file")
    if source_file_str:
        orig_file = Path(source_file_str)
        if orig_file.exists() and orig_file.parent.resolve() == INBOX.resolve():
            ARCHIVED_INBOX.mkdir(parents=True, exist_ok=True)
            archived_dest = ARCHIVED_INBOX / orig_file.name
            shutil.move(str(orig_file), str(archived_dest))
            log_message(f"[INBOX -> ARCHIVE] Moved original {orig_file.name} to {ARCHIVED_INBOX.name}")

    return master_md_path


def main():
    parser = argparse.ArgumentParser(description="Autonomous CKG Semantic Analysis Engine.")
    parser.add_argument("--packets", nargs="*", default=[], help="Specific .analysis-prompt.md packets to run.")
    parser.add_argument("--primary-model", default="deepseek/deepseek-v3.2", help="Primary OpenRouter model.")
    parser.add_argument("--fallback-model", default="deepseek/deepseek-r1", help="Fallback model.")
    parser.add_argument("--limit", type=int, default=0, help="Max number of packets to process.")
    args = parser.parse_args()

    prompts = []
    if args.packets:
        prompts = [Path(p) for p in args.packets if Path(p).exists()]
    else:
        # Find all .analysis-prompt.md in PROCESS that don't have a matching CKG_MASTER in OUTBOX
        candidate_prompts = sorted(PROCESS.glob("*.analysis-prompt.md"))
        for p in candidate_prompts:
            base_slug = p.name.removesuffix(".analysis-prompt.md")
            out_file = OUTBOX / f"{base_slug}_CKG_MASTER.md"
            if not out_file.exists():
                prompts.append(p)

    if args.limit > 0:
        prompts = prompts[:args.limit]

    log_message(f"Starting batch analysis on {len(prompts)} packet(s)...")
    success_count = 0
    fail_count = 0

    for prompt_path in prompts:
        log_message(f"\n======================================================\nProcessing: {prompt_path.name}\n======================================================")
        try:
            master_path = process_packet(
                prompt_path=prompt_path,
                primary_model=args.primary_model,
                fallback_model=args.fallback_model
            )
            success_count += 1
            log_message(f"[SUCCESS] {prompt_path.stem} -> {master_path.name}")
        except Exception as e:
            fail_count += 1
            log_message(f"[FAILED] {prompt_path.stem}: {e}")

    log_message(f"\nBatch processing complete: {success_count} succeeded, {fail_count} failed.")
    return 0 if fail_count == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
