# G-Counter

## What It Solves

G-Counter is a state-based counter for replicas that only need increments. Each
replica owns one component and the observable value is the sum of all
components.

## State

The state is a map from actor id to an unsigned component counter.

## Operations

- `increment(actor)` increases one actor component by one.
- `increment_by(actor, amount)` increases one actor component by `amount`.

## Merge

Merge is the component-wise maximum. This makes duplicate, reordered, and
repeated state delivery harmless.

## Query

`query()` returns the sum of all actor components as `u128`.

## Correctness Intuition

Local increments only move one component upward. Merge also only moves
components upward and is associative, commutative, and idempotent. Replicas that
eventually receive the same component maxima therefore converge to the same
state.

## Lean Proofs

- `Crdt.GCounter.increment_inflationary`
- `Crdt.GCounter.merge_converges_for_same_states`

## TLA+ Model

The initial TLC config checks `TypeOK` for two replicas with component values
bounded by `0..2`.

## Rust API

```rust
use crdt_algorithms::counters::GCounter;
use crdt_core::CvRDT;

let mut left = GCounter::new();
let mut right = GCounter::new();

left.increment("left".to_owned())?;
right.increment_by("right".to_owned(), 2)?;

left.merge(&right);
assert_eq!(left.query(), 3);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Complexity

- Increment: `O(log actors)`
- Merge: `O(remote actors * log local actors)`
- Query: `O(actors)`

## Known Constraints

- Decrement is not supported.
- Actor components are `u64`; overflow is reported by the Rust API.
