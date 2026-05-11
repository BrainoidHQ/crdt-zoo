# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/counters/gcounter.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Example-based checks |
| Lean model | `proofs/lean/Crdt/Algorithms/GCounter.lean` | Partial proof |
| TLA+ model | `proofs/tla/algorithms/gcounter/GCounter.tla` | TLC model check with bounded configuration |
| Catalog metadata | `catalog/gcounter/algorithm.toml` | Present |

## Lean

Current checked theorem names:

- `Crdt.GCounter.increment_inflationary`
- `Crdt.GCounter.merge_converges_for_same_states`

The Lean file currently covers core inflationary-update and convergence facts
for the simplified mathematical model. It is not yet a full mechanized proof of
every Rust API edge case.

## TLA+

Current TLC bounds:

```text
replicas = 2
counter components <= 2
```

These bounds are intentionally small. They are useful for checking that the
model is executable and that the initial network transitions preserve `TypeOK`.
They are not an unbounded proof of G-Counter correctness.

## Gaps

- Add property-based law tests.
- Add a reference model and implementation-vs-model comparison tests.
- Strengthen the Lean convergence theorem around observed update sets.
- Add richer TLA+ invariants beyond `TypeOK`.
