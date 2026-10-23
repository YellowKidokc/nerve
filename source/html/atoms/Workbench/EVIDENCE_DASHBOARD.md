---
title: Evidence dashboard
type: dashboard
tags: [theophysics, dashboard]
---

# Evidence dashboard

Every note with `type: evidence-sheet` rolls up here. Sorted so the weakest load-bearing claim with the most riding on it is on top. Requires the Dataview plugin.

## Gated — resolve first

```dataview
TABLE WITHOUT ID
  file.link AS Sheet,
  evd_weakest_claim AS "Gated / weakest",
  evd_support AS Support,
  evd_counter AS Counter,
  evd_families AS Families
FROM #evidence-sheet OR ""
WHERE type = "evidence-sheet" AND evd_gated = true
SORT evd_counter DESC
```

## Unscored — coverage below floor

```dataview
TABLE WITHOUT ID
  file.link AS Sheet,
  round(evd_coverage * 100) + "%" AS Coverage,
  evd_unassessed AS "Unassessed / stale",
  evd_families AS Families
WHERE type = "evidence-sheet" AND evd_state = "UNSCORED"
SORT evd_coverage DESC
```

## Scored — by weakest load-bearing claim

```dataview
TABLE WITHOUT ID
  file.link AS Sheet,
  evd_balance AS Balance,
  evd_weakest_claim AS "Weakest claim",
  choice(evd_stable, "stable", "FLIPS") AS Robustness,
  evd_families AS Families,
  round(evd_coverage * 100) + "%" AS Coverage,
  coherence AS Coherence
WHERE type = "evidence-sheet" AND evd_state = "SCORED"
SORT evd_balance ASC
```

## Scoring-dependent verdicts

Sheets whose sign flips under ±25% weights or cap ±1. These are the results that depend on how we chose to score, not on the evidence.

```dataview
LIST evd_balance + " · " + evd_weakest_claim
WHERE type = "evidence-sheet" AND evd_state = "SCORED" AND evd_stable = false
SORT evd_balance ASC
```

## Machine-only assessments

Sheets where no human appears in the assessor list. Nothing here is admissible until a human ruling is recorded.

```dataview
LIST evd_assessors
WHERE type = "evidence-sheet" AND !contains(evd_assessors, "David")
```

## Coherence vs evidence

The two axes kept apart. High coherence with low balance is a well-built argument with no evidence; the reverse is evidence without a structure to carry it.

```dataview
TABLE WITHOUT ID
  file.link AS Sheet,
  coherence AS Coherence,
  evd_balance AS Balance,
  evd_state AS State
WHERE type = "evidence-sheet" AND coherence != null
SORT coherence DESC, evd_balance ASC
```

## Status roll-up

```dataview
TABLE WITHOUT ID
  status AS Status,
  length(rows) AS Sheets,
  round(sum(rows.evd_families)) AS "Total families"
WHERE type = "evidence-sheet"
GROUP BY status
```

---
*The dashboard reads frontmatter only. It cannot tell you a claim is true; it tells you which sheet to open next.*
