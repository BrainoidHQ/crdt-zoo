# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/sets/gset.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Property-based checks |
| Lean model | `proofs/lean/Crdt/Algorithms/GSet.lean` | Proved |
| TLA+ model | Not added yet | Planned |
| Catalog metadata | `catalog/gset/algorithm.toml` | Present |

## Rust

Property tests generate bounded actor-like string sets and check the
join-semilattice laws. Unit tests check that `add` is inflationary and merge
returns the union.

## Lean

Checked theorem names:

- `Crdt.GSet.add_inflationary`
- `Crdt.GSet.contains_added`
- `Crdt.GSet.applyUpdate_inflationary`
- `Crdt.GSet.merge_monotone`
- `Crdt.GSet.merge_converges_for_same_states`

The Lean model represents sets as boolean membership predicates and proves
union join laws, monotonic add behavior, monotone merge, and a `CvRDT`
instance for add updates.

## Gaps

- Connect the predicate-set Lean model to finite-set extraction/query details.
- Add a bounded TLA+ model when common network-model wiring is available.
