from __future__ import annotations

import argparse
import sys
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
if hasattr(sys.stderr, "reconfigure"):
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

# Local pipeline modules
from run_packet import prepare_packet
from semantic_analysis import process_packet, log_message

SCRIPTS_DIR = Path(__file__).resolve().parent
PIPELINE_DIR = SCRIPTS_DIR.parent
INBOX = PIPELINE_DIR / "00_INBOX"
PROCESS = PIPELINE_DIR / "01_PROCESS"
OUTBOX = PIPELINE_DIR / "02_OUTBOX"


def main():
    parser = argparse.ArgumentParser(description="End-to-End Batch Pipeline Runner for Consilience Atlas.")
    parser.add_argument("--limit", type=int, default=0, help="Maximum number of files to process from INBOX (0 = all).")
    parser.add_argument("--primary-model", default="deepseek/deepseek-v3.2", help="Primary OpenRouter model.")
    parser.add_argument("--fallback-model", default="deepseek/deepseek-r1", help="Fallback OpenRouter model.")
    args = parser.parse_args()

    # 1. Discover all files in INBOX (excluding subfolders like ARCHIVED_INPUTS)
    inbox_files = sorted([
        f for f in INBOX.iterdir()
        if f.is_file() and f.suffix.lower() in {".md", ".txt", ".html", ".json"}
    ])

    if not inbox_files:
        log_message(f"No pending files found in {INBOX}. Please place source files into 00_INBOX.")
        return 0

    if args.limit > 0:
        inbox_files = inbox_files[:args.limit]

    total = len(inbox_files)
    log_message(f"Found {total} pending file(s) in 00_INBOX.")
    log_message(f"Primary model: {args.primary_model} | Fallback: {args.fallback_model}")

    successes = 0
    failures = 0

    for idx, source_file in enumerate(inbox_files, 1):
        log_message(f"\n[{idx}/{total}] Processing: {source_file.name}...")
        try:
            # Step A: Prepare packet (source hashing, intake metadata, prompt)
            prompt_path = prepare_packet(source_file)
            log_message(f"[{idx}/{total}] Prepared working packet: {prompt_path.name}")

            # Step B: Run AI analysis, validate strict CKG contract, assemble master record
            master_md_path = process_packet(
                prompt_path=prompt_path,
                primary_model=args.primary_model,
                fallback_model=args.fallback_model
            )
            successes += 1
            log_message(f"[{idx}/{total}] Completed successfully -> {master_md_path.name}")
        except Exception as e:
            failures += 1
            log_message(f"[{idx}/{total}] FAILED {source_file.name}: {e}")

    log_message(f"\n======================================================")
    log_message(f"Batch Execution Finished.")
    log_message(f"Total: {total} | Succeeded: {successes} | Failed: {failures}")
    log_message(f"Protected Master Records in: {OUTBOX}")
    log_message(f"======================================================")

    return 0 if failures == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
