# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/counters/pncounter.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Reference model | `crates/crdt-algorithms/src/counters/pncounter.rs` tests | Query agreement check |
| Lean model | Not added yet | Planned |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/pncounter/algorithm.toml` | Present |

## Rust

Property tests generate bounded positive and negative component maps. They check
join-semilattice laws and inflationary increment/decrement updates.

## Gaps

- Add a Lean model that reuses the G-Counter function model twice.
- Add a TLA+ model once the reusable state-message network module is wired into
  the model runner.
