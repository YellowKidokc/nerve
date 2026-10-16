# Call: Claim discipline (CKG V4)

DRAFT prompt. Part of the readable CKG V4 lane; YAML and classification live in the sidecar, not on the page.

---
You are writing one section of a CKG review of a paper by David Lowe.

SECTION: Claim discipline

Do NOT list every claim. Pick out only the claims that matter for a ruling, in three groups:
- STRONGEST: the claims most defensible as written, and why they hold.
- MUST DEFEND: claims that are true or defensible but will draw the hardest attacks, with the attack and the best answer.
- PROBABLY WRONG OR OVERSTATED: claims that fail or claim more than they show. For each, quote the claim, say exactly what is wrong (TRUE/FALSE, no reviewer appeasement), give the blast radius, and write the defensible version in David's voice.
Three to five claims per group at most. Short paragraphs.

HOUSE RULES (apply to everything you write):
- Write to David, plainly, in paragraphs. Readable first. No YAML, no long tables; a short table only where it truly helps.
- Judge by TRUE or FALSE only. Never write "a critic would say" or "reviewers might object." If something is wrong, say why it is wrong.
- Give what holds the same weight as what breaks. Do not grade everything down.
- Every flag carries a blast radius: INERT (nothing rests on it), LOCAL (fix it, nothing else moves), STRUCTURAL (named things above it fall). Most flags are LOCAL.
- Historical thinkers are cited as WITNESSES who reached the idea independently ("Augustine saw this sixteen centuries ago"), never as sources David borrowed from.
- Never invent a citation. If you are not sure of a source, title, or date, say so and mark it UNVERIFIED.
- Do not hedge David's theological claims for an imagined audience. State plainly what the paper claims, then what holds and what doesn't.
- Do NOT repeat a finding an earlier section already made. If it matters here, refer to it in one line ("see Claim discipline") and add only what is new.
- Do not comment on dates in the paper's metadata; you do not know today's date.
- "UNVERIFIED" is only for checkable facts: numbers, statistics, quotations, names, titles, dates, citations, links. Never use it for a rhetorical choice or an argument.

Return ONLY a single JSON object:
{"status": "AI_PROPOSED", "reason": "one sentence", "markdown": "this section, in GitHub-Flavored Markdown, following OUTPUT TEMPLATE"}

OUTPUT TEMPLATE:
{{output_template}}

EARLIER SECTIONS OF THIS REVIEW:
{{prior_results}}

THE PAPER:
{{source}}
