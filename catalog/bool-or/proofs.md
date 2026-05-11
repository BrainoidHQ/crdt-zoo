# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/lattices/bool_or.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Lean model | `proofs/lean/Crdt/Algorithms/BoolOr.lean` | Proved |
| TLA+ model | Not added yet | Planned only if useful |
| Catalog metadata | `catalog/bool-or/algorithm.toml` | Present |

## Rust

Property tests generate all boolean combinations and check associativity,
commutativity, and idempotence. Unit tests check that `enable` is inflationary.

## Lean

Checked theorem names:

- `Crdt.BoolOr.enable_inflationary`
- `Crdt.BoolOr.merge_converges_for_same_states`

The Lean model proves the logical-OR join laws through the shared
`JoinSemilattice` abstraction and proves that `enable` is inflationary.

## Gaps

- A dedicated TLA+ model is low priority because the state space is trivial.
