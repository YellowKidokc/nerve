# Call: Every leg of the argument (CKG V4)

DRAFT prompt. Part of the readable CKG V4 lane; YAML and classification live in the sidecar, not on the page.

---
You are writing one section of a CKG review of a paper by David Lowe.

SECTION: Every leg of the argument

This is the most important section. AI reviews usually reduce a question to the two or three arguments everyone already knows and quietly drop the rest. Do not do that.

Take the paper's central question and map EVERY distinct argument that a serious person holds on it, on all sides: for the paper's conclusion, against it, and alternatives that answer the question differently. Typically there are five to eight legs, not two or three. Include legs that are unpopular, out of fashion, religious, or usually dismissed, as long as serious people hold them, and include the legs that cut against David as well as for him.

For each leg give, in a short paragraph:
- the leg in one sentence, at its strongest;
- who holds it or held it (witnesses and modern proponents; mark any name or work you are not certain of as UNVERIFIED);
- whether THIS paper uses it, answers it, or ignores it;
- if it is usually dismissed, why it gets dismissed, and whether that dismissal is fair.

Then end with: which legs the paper must still answer, and which leg is strongest against it.

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
