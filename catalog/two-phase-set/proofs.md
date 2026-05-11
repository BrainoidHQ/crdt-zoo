# Proof Status

## Rust

Rust unit tests cover permanent tombstones, component-wise merge, inflationary
adds and removes, and convergence after merge. Property tests check the
join-semilattice laws over generated add/remove components.

## Lean

No Lean model is provided yet.

## TLA+

No TLA+ model is provided yet.

## Gaps

- Garbage collection is documented but not implemented.
- There is no bounded distributed model yet for stale state returning after
  tombstone collection.
