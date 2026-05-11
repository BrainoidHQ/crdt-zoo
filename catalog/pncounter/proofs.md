# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/counters/pncounter.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Reference model | `crates/crdt-algorithms/src/counters/pncounter.rs` tests | Query agreement check |
| Lean model | `proofs/lean/Crdt/Algorithms/PNCounter.lean` | Proved |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/pncounter/algorithm.toml` | Present |

## Rust

Property tests generate bounded positive and negative component maps. They check
join-semilattice laws and inflationary increment/decrement updates.

## Lean

Checked theorem names:

- `Crdt.PNCounter.incrementBy_inflationary`
- `Crdt.PNCounter.decrementBy_inflationary`
- `Crdt.PNCounter.increment_inflationary`
- `Crdt.PNCounter.decrement_inflationary`
- `Crdt.PNCounter.applyUpdate_inflationary`
- `Crdt.PNCounter.merge_monotone`
- `Crdt.PNCounter.merge_converges_for_same_states`

The Lean model reuses the G-Counter function model for both positive and
negative components and proves component-wise merge laws plus inflationary
increment/decrement updates. It also packages increment and decrement updates as
a `CvRDT` instance.

## Gaps

- Mechanize query summation and signed-value overflow boundaries.
- Add a TLA+ model once the reusable state-message network module is wired into
  the model runner.
