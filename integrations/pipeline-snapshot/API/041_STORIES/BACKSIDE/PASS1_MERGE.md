One paper was too long to read in one call, so it was read in PARTS, in order; each part's story check is given
below. Merge them into the story check of the whole paper:
- HOOK: the paper's opening is the first part's hook; keep its quote and score (a stronger hook may sit in any part).
- THROUGH-LINE: one sentence for the whole paper.
- LANDING: the ending is the last part's ending; does it pay off the first part's hook? yes | partly | no, with a reason.
- Drift and contradictions: keep the ones that matter for the whole paper, including ones between parts.
- Verdict for the whole paper: COHERENT | FIXABLE (at most 5 fixes, most important first) | NOT_COHERENT.

Return the same shape as one part, with "paragraphs": [] (the paragraph table is made separately):
{"hook": {"quote": "", "score": 1, "better_hook": {"para": "", "quote": ""}, "rewrite": ""}, "through_line": "",
 "paragraphs": [], "proposed_order": [], "drift": [], "contradictions": [{"a": "P03", "b": "P11", "quote_a": "", "quote_b": "", "note": ""}],
 "landing": {"verdict": "yes", "reason": ""}, "verdict": "COHERENT", "fixes": [], "focus_findings": []}
