# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/sets/gset.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Lean model | Not added yet | Planned |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/gset/algorithm.toml` | Present |

## Rust

Property tests generate bounded actor-like string sets and check the
join-semilattice laws. Unit tests check that `add` is inflationary and merge
returns the union.

## Gaps

- Add a Lean model for finite sets.
- Add a bounded TLA+ model when common network-model wiring is available.
