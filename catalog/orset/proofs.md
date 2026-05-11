# Proof Status

## Rust

Rust unit tests cover dot allocation, observed remove, concurrent add-wins
merge, context invariants, and replica convergence after state exchange.
Property tests check the join-semilattice laws over generated valid OR-Set
states with entry dots and causal context dots.

## Lean

No Lean model is provided yet.

## TLA+

No TLA+ model is provided yet.

## Gaps

- The causal merge rule is law-tested but not mechanically proved.
- There is no bounded TLA+ model yet for concurrent add/remove histories.
- Garbage collection is documented but not implemented.
