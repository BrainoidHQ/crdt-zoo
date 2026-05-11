# Proof Status

## Rust

Rust unit tests cover timestamp visibility, remove-wins ties, older updates,
component-wise maximum merge, and inflationary updates. Property tests check the
join-semilattice laws over generated timestamp maps.

## Lean

No Lean model is provided yet.

## TLA+

No TLA+ model is provided yet.

## Gaps

- Timestamp policy is documented but not modeled.
- There is no model check for clock skew histories yet.
