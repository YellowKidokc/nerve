# Atom Metaframing Spec — Claims Ledger + Worldview Reference Index

*David Lowe, POF 2828 · 2026-08-07*
*Companion spec to the Faith-through-physics-atoms repo. Written so a coding AI can implement directly against the canonical node specification. Two infrastructure pieces, both flagged in the Master Outline v3.1 (repair ledger items 13–14).*

---

## GOVERNING DECISION

**The atoms are the source of truth. Both artifacts below are renderings.**

No hand-maintained parallel spreadsheet. The moment an xlsx is edited by hand, it drifts from the atom graph and becomes a second canon — which is exactly the failure this infrastructure exists to prevent. Every ledger row and every index entry regenerates from node JSON. Humans accept or reject nodes and edges through the existing Adversarial Review Gate; machines render.

This follows the repo's own rule: *artifacts enter where they belong.*

---

## PIECE 1 — THE CLAIMS CROSS-REFERENCE LEDGER

**What it is:** a fifth generated view, added to the repo's existing four (Canon / Workbench / Graph / Public). Call it the **Ledger view**.

**Why a view and not a new node type:** the dependency data already lives in `edges[]` (`dependsOn`, `feedsInto`, `bridgesTo`, `challenges`, `expands`, `forksFrom`, `descendsTo`), and the blast-radius logic already lives in the falsification-propagation table. The ledger is those two things, tabulated. Building it as data would duplicate the graph; building it as a view costs only a renderer.

### Ledger columns (each maps to existing schema fields — nothing new invented)

| Column | Source field |
|---|---|
| Claim ID | `claimID` (claims only — spec §CLAIM NODES) |
| Statement | `statementPlain` (hover/full row: `statementTechnical`) |
| Kind | `claimKind` |
| Canon state | `canonicalStatus` + `epistemicStatus` |
| Public grade | rendered from `claimKind` + `verificationStatus` — the [T]/[C]/[D] labels from the writing canon map here: theorem-grade → `mathematical` + `machine-verified`; correspondence → `isomorphism`/`interpretation`; delta → declared `interpretation` with `falsificationCondition` populated |
| Depends on | inbound `dependsOn` edges |
| Feeds | outbound `feedsInto` edges (the "this one touches that one" column) |
| Bridges | `bridgesTo` edges + linked BRIDGE nodes, with `bridgeGrade` |
| Attack surface | linked OBJECTION + KILL nodes, `challengeStatus` |
| Blast radius | `failureScope` on each outbound edge, read against the propagation table |
| Kill condition | `falsificationCondition` |
| Burden state | `claimBurden.supportStatus` |
| Location | `workbenchStage` + human-readable paper/movement reference |

### Sorts and filters the renderer must support

- **By blast radius** — every claim with any `failureScope: global` edge, first. These are the load-bearing walls.
- **By burden gap** — `supportStatus: incomplete | blocked`. This is the repair queue, auto-generated.
- **By touch** — pick any claim, see the full neighborhood: everything it depends on, everything that depends on it, every bridge it sits on.
- **By challenge** — `challenged-open` claims, sorted by how much depends on them. Audit triage.

### The one hard rule

The xlsx/CSV/HTML export is **regenerated, never edited.** Header on every export: *"Rendered from the atom graph on {date}. Do not hand-edit. Changes enter through nodes and edges, via the Gate."*

---

## PIECE 2 — THE WORLDVIEW REFERENCE INDEX

**What it is:** a public HTML page listing every external worldview the framework engages, each steelmanned, each with our response and an external source link (SEP etc.). This is the page David hands out: "referenced from here."

**Node mapping (all existing types — no schema extension required):**

1. **Each worldview = one or more OBJECTION nodes.** The repo already requires steelmanning ("strongest form, not strawmen"). A worldview with several distinctive claims gets several OBJECTION nodes, each with `targetClaim` pointing at the framework claim it presses on.
2. **Each worldview = one EVIDENCE node for its sources.** `sourceType: "external-reference"`, `sourceRef`: the SEP article (or equivalent), `citationStatus: verified` once a human has checked the link and the attribution. Note per spec: EVIDENCE nodes do not require statement fields.
3. **The index page itself = one ARTICLE node.** `seriesID: "worldview-index"`, `claimRefs`: the OBJECTION node IDs, `crossRefs`: the books/papers where each engagement lives, `readingLevel`: lay.
4. **The shareable handout = one REACH node.** `format: "web page"`, `sourceArticle`: the ARTICLE node above, `impactStatement`: "gives every opponent's position in its strongest form, with our response and their own sources linked."

### Row template (what each rendered index entry shows)

| Field | Content |
|---|---|
| Worldview | e.g. Naturalist realism (Cornell school) |
| Named occupants | Boyd, Brink, Railton; Wolf; Metz |
| Steelman | Their position in its strongest form, in their vocabulary |
| Where we engage it | Book 6 §4 (P5 regress), Book 11 divergence map |
| Our response | Linked `targetClaim` nodes — the priced exit, not a dismissal |
| Status | `challenged-open` / `challenged-survived` — honest state of play |
| External sources | SEP "The Meaning of Life" (Metz), etc. — `citationStatus` shown |

**First seeded row** (already live in the corpus): naturalist realism — engaged and priced in Book 6 v1.1 §4; sources SEP (Metz taxonomy; Cornell realism). Second: Tegmark MUH (Book 2 / Engravings 4). Third: paraconsistent logic (Triple Preservation Ledger, Card 1 countermodel). Seed from what exists; do not write new engagements to fill the page.

### Honesty constraint

This page is public-facing, so it inherits the writing canon: no worldview is described as defeated — only *priced*. Statuses show the real state (`challenged-open` where it's open). The page's credibility IS the steelmanning; the moment a reader finds a strawman on it, the whole index is suspect.

---

## HANDOFF NOTES FOR THE CODING AI

1. Conform to the canonical node spec exactly. No new nodeTypes. If an INDEX nodeType is ever wanted, it enters as a *proposal* through the Adversarial Review Gate — human accepts, machine doesn't. (The repo already reserves acceptance to humans.)
2. Both artifacts regenerate from node JSON. Build the renderer, not the data.
3. `coherenceDirection: "neutral"` on both — they are system/infrastructure nodes per the spec's own definition.
4. Suggested nodeIDs: ledger view → `tp:META/00/LEDGER`; worldview index article → `tp:META/00/WV-INDEX`; individual worldview objections → `tp:META/00/WV-####` sequence.
5. The Ledger view's "by burden gap" sort should diff against the Master Outline's repair ledger — when they disagree, the atoms are stale or the outline is. That disagreement report is itself a deliverable; it's how the two canons police each other.

---

*Truth descends as far as people require. The ledger is how we know what's carrying the weight; the index is how outsiders verify we read their side first.*
