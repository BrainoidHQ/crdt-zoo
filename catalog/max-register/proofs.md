# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/registers/max.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks over bounded integers |
| Lean model | `proofs/lean/Crdt/Algorithms/MaxRegister.lean` | Proved |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/max-register/algorithm.toml` | Present |

## Rust

Property tests generate optional `i32` states and check join-semilattice laws.
Additional property tests check that assignment is inflationary.

## Lean

Checked theorem names:

- `Crdt.MaxRegister.assign_inflationary`
- `Crdt.MaxRegister.applyUpdate_inflationary`
- `Crdt.MaxRegister.merge_monotone`
- `Crdt.MaxRegister.merge_converges_for_same_states`

The Lean model proves the `Option Nat` maximum register semilattice and
inflationary assignment behavior, monotone merge, and a `CvRDT` instance for
assignment updates.

## Gaps

- Generalize the Lean model beyond `Nat` if ordered payload proofs are needed.
- Add a small TLA+ model if register-specific distributed traces become useful.
