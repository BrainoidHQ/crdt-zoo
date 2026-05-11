# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/lattices/bool_or.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Lean model | Not added yet | Planned |
| TLA+ model | Not added yet | Planned only if useful |
| Catalog metadata | `catalog/bool-or/algorithm.toml` | Present |

## Rust

Property tests generate all boolean combinations and check associativity,
commutativity, and idempotence. Unit tests check that `enable` is inflationary.

## Gaps

- Add a small Lean model or derive this from a generic Bool OR lattice instance.
- A dedicated TLA+ model is low priority because the state space is trivial.
