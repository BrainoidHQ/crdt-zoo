# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/registers/max.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks over bounded integers |
| Lean model | Not added yet | Planned |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/max-register/algorithm.toml` | Present |

## Rust

Property tests generate optional `i32` states and check join-semilattice laws.
Additional property tests check that assignment is inflationary.

## Gaps

- Add a Lean model for `Option Nat` with maximum.
- Add a small TLA+ model if register-specific distributed traces become useful.
