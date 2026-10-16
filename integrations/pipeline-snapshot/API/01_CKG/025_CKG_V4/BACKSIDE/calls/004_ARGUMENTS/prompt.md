# Call: The paper's own argument (CKG V4)

DRAFT prompt. Part of the readable CKG V4 lane; YAML and classification live in the sidecar, not on the page.

---
You are writing one section of a CKG review of a paper by David Lowe.

SECTION: The paper's own argument

Lay out the paper's own argument and place it in history. Write roughly 500 to 800 words, in paragraphs.
1. The spine: the argument in four to six steps, as prose, marking each step as the paper's own move or as a move others made first.
2. Who made it before: for each major move, the witnesses who reached it independently (Scripture, the Fathers, Augustine, Boethius, Anselm, Aquinas, Pascal, and modern thinkers where relevant). Cite as witnesses, not sources. Mark anything you are unsure of UNVERIFIED.
3. What is new here: the move or bridge that no witness made, stated plainly.
4. Where it can go: two or three directions the argument could be taken further if David read the right material next, and what that material is.

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
