---
std_title: "DLowe 2026-07-28 · SIMULATION RESULTS — Truth Is Ontological · Coherence, Fine-Tuning · Argument"
doc_title: "SIMULATION RESULTS — Truth Is Ontological"
author_code: "DLowe"
upload_date: "2026-07-28"
keywords: ["Truth", "Coherence", "Fine-Tuning"]
move: "Argument"
original_file: "SIMULATION_RESULTS.md"
previous_names: ["SIMULATION_RESULTS"]
---
# SIMULATION RESULTS — Truth Is Ontological
### `truth_series_simulation.py` · run 2026-07-28 · chart: `truth_series_simulation.png`

David's argument (2026-07-28): *"The universe couldn't run on a lie as its trusted standard — one small flaw in the four or five fundamental pieces, compounded over billions of parts, and it's unstable. It wouldn't work."*

Two models tested. Both confirm.

## Model A — One small lie in the foundation

One fundamental constant (of the 4–5 the universe is built from) carries a relative error ε. Every interaction inherits it multiplicatively. System is "functional" only while total deviation < 1%.

**Maximum survivable foundational error:**

| Compounding events | ε_max |
|---|---|
| 10³ | 9.95 × 10⁻⁶ (0.001%) |
| 10⁶ | 9.95 × 10⁻⁹ |
| 10⁹ | 9.95 × 10⁻¹² |
| 10¹² | 9.99 × 10⁻¹⁵ |

The functional window for ε shrinks toward zero as events grow. This is not a measurement of the actual universe; it is a numerical demonstration of a structural argument about compounding systems. A system with universe-scale interaction counts cannot tolerate much trusted foundational drift before the error dominates the system's history. The chart's left panel: even ε = 10⁻⁶ crosses the tolerance line inside 10⁸ events. Smaller lies only buy time — the trajectory is identical.

## Model B — Correction only works against truth

Per-event errors, 200 runs, 2000 events each:

| Regime | Result after 2000 events |
|---|---|
| No correction | RMS deviation 45.2 — grows as σ√T ≈ 44.7, unbounded |
| Correction vs TRUE standard | RMS deviation 3.1 — bounded at σ/√(2θ) ≈ 3.2, stable forever |
| Correction vs FALSE standard | converges to +25.0 — **stable, calm, and wrong** |

Error correction is real — the green line holds forever. But correction converges to *whatever standard it trusts*. Correction against a false standard produces a system that is stable, self-consistent, and detached from reality — the yellow line, perfectly calm around a lie. **Correction itself presupposes truth.** You cannot answer the "what about error correction?" objection with a mechanism that requires a true reference to function.

## Conclusion

```text
1. A compounding system built on a false trusted standard compounds toward collapse.
   The foundation must be faithful enough to bear the system's history.
2. Error correction does not soften this. Correction requires a true
   standard to correct against. Truth is prior to repair.
3. Therefore truth is not merely a feature OF a coherent world. It is a
   precondition FOR coherence, correction, measurement, and fruitfulness.
   Truth is ontological.
```

This is the simulation flank of Playbook Move 1/1B and PMM A11: the ground cannot be deceptive, because nothing deceptive compounds.

*Labels: numerical demonstration of a structural argument — not a measurement of the actual universe. The multiplicative model is the worst case for the critic and the standard case for compounding error in engineering (tolerance stack-up). Runs: `python truth_series_simulation.py`.*
