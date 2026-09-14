import re
import sys

EXPECTED_HEADINGS = [
    "At a glance",
    "Central claim",
    "Best concise argument",
    "System or model",
    "Evidence chain",
    "Best evidence and sources",
    "Strongest objection and negative controls",
    "What survives",
    "What this does not establish",
    "Corrections and revisions",
    "Implications",
    "Formal or testable path",
    "Open questions and next actions",
    "Recommended classification and relationships",
]

def validate_ckg(markdown: str) -> None:
    # If validating a full container document, extract the analytical portion starting from ## At a glance
    if "## At a glance" in markdown:
        if "## Complete preserved source" in markdown:
            analysis_part = markdown[markdown.find("## At a glance"):markdown.find("## Complete preserved source")].strip()
        else:
            analysis_part = markdown[markdown.find("## At a glance"):].strip()
    else:
        analysis_part = markdown
        if "## Complete preserved source" in markdown or "BEGIN EXACT SOURCE" in markdown:
            raise ValueError("The semantic analysis response must not reproduce the preserved source.")

    found = re.findall(r"(?m)^##\s+(.+?)\s*$", analysis_part)

    if found != EXPECTED_HEADINGS:
        raise ValueError(
            "Wrong CKG heading contract.\nExpected:\n- "
            + "\n- ".join(EXPECTED_HEADINGS)
            + "\n\nFound:\n- "
            + "\n- ".join(found)
        )

    # Validate sections
    sections = dict(re.findall(r"(?ms)^##\s+(.+?)\s*$\n(.*?)(?=^##\s+|\Z)", analysis_part))
    
    # 1. Central claim check - reject generic filler
    central_claim = sections.get("Central claim", "").strip()
    if re.search(r"explains meaning, math, dependencies|this paper presents a framework|comprehensive overview", central_claim, re.IGNORECASE):
        raise ValueError("Central claim contains generic filler instead of the exact, narrow substantive proposition.")

    # 2. Formal results in Best evidence and sources - disallow sorry or admit
    best_evidence = sections.get("Best evidence and sources", "")
    if re.search(r"\b(?:sorry|admit)\b", best_evidence, re.IGNORECASE):
        raise ValueError("Formal results in 'Best evidence and sources' contains 'sorry' or 'admit'. An unproven Lean stub is not a formal result!")

    # 3. Substantive ledger checks
    for heading in ("What survives", "What this does not establish", "Formal or testable path"):
        body = sections.get(heading, "").strip()
        if len(re.sub(r"\s+", "", body)) < 80:
            raise ValueError(f"{heading!r} is too thin; it requires a substantive ledger entry (>= 80 non-whitespace characters).")
        if re.search(r"\[(?:open|not run)\]|\b(?:open|not run)\b", body, re.IGNORECASE):
            raise ValueError(f"{heading!r} contains an unresolved placeholder ('[OPEN]' or 'not run').")

    # 4. Formal or testable path subsections
    formal = sections.get("Formal or testable path", "")
    for subheading in ("Lean / formal checks", "Empirical / literature checks", "Adversarial checks"):
        if not re.search(rf"(?mi)^###\s+{re.escape(subheading)}\s*$", formal):
            raise ValueError(f"'Formal or testable path' is missing required subsection: '### {subheading}'")

    # 5. System or model subsections (Bridge and Truth Predicates)
    system = sections.get("System or model", "")
    if not re.search(r"(?mi)^###\s+.*(?:Terms and plain-language bridge|Bridge).*", system):
        raise ValueError("'System or model' is missing required subsection: '### Terms and plain-language bridge'")
    if not re.search(r"(?mi)^###\s+.*Truth Predicates.*", system) and not re.search(r"\|\s*#\s*\|\s*Truth Predicate", system):
        raise ValueError("'System or model' is missing extracted truth predicates table.")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        with open(sys.argv[1], "r", encoding="utf-8") as f:
            content = f.read()
        validate_ckg(content)
        print("CKG Evaluation validation PASSED: 100% compliant with CKG quality gate.")
    else:
        print("Usage: python validate_ckg.py <case_file.md>")
