# Proof Status

## Rust

Rust unit tests cover permanent tombstones, component-wise merge, visible query
behavior, and inflationary adds and removes. Property tests check the
join-semilattice laws over generated add/remove components.

## Lean

Checked theorem names:

- `Crdt.TwoPhaseSet.add_inflationary`
- `Crdt.TwoPhaseSet.remove_inflationary`
- `Crdt.TwoPhaseSet.remove_wins`
- `Crdt.TwoPhaseSet.add_after_remove_noop`
- `Crdt.TwoPhaseSet.applyUpdate_inflationary`
- `Crdt.TwoPhaseSet.merge_monotone`
- `Crdt.TwoPhaseSet.merge_converges_for_same_states`

The Lean model represents add and remove components as boolean membership
predicates. It proves component-wise union laws, inflationary add/remove
updates, monotone merge, a `CvRDT` instance for add/remove updates, and the
permanent remove-wins behavior for an element after it is tombstoned.

## TLA+

No TLA+ model is provided yet.

## Gaps

- Garbage collection is documented but not implemented.
- Connect the predicate-set Lean model to finite-set extraction/query details.
- There is no bounded distributed model yet for stale state returning after
  tombstone collection.
