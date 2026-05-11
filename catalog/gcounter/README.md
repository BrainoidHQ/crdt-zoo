# G-Counter

G-Counter is a grow-only, state-based counter. It is the first CRDT exhibit in
this repository because its join operation is small, deterministic, and easy to
connect across Rust, Lean, TLA+, and tests.

## Summary

| Field | Value |
| --- | --- |
| Family | Counter |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Component-wise maximum |
| Query result | Sum of all actor components |
| Deletes | Not supported |
| Tombstones | None |
| Current status | Initial skeleton |

## What It Solves

G-Counter represents a counter that can be incremented independently by multiple
replicas. Each actor owns one component. Replicas exchange state and merge by
taking the maximum value for each actor.

This is useful when the application only needs monotonic increments, such as
counting observed events where decrements are not part of the data type.

## State

The state is a map:

```text
ActorId -> u64
```

An absent actor component is interpreted as zero. The Rust implementation drops
zero-valued components when constructing a counter from a map so equivalent
states have a stable representation.

## Operations

The Rust API currently exposes:

- `GCounter::new()`
- `GCounter::from_counts(counts)`
- `increment(actor)`
- `increment_by(actor, amount)`
- `component(actor)`
- `counts()`
- `query()`
- `merge(other)`

Incrementing by zero is a no-op. Incrementing past `u64::MAX` returns
`CounterOverflow`.

## Merge

Merge is the component-wise maximum:

```text
merge(left, right)[actor] = max(left[actor], right[actor])
```

This join is associative, commutative, and idempotent. Those laws make duplicate
and reordered state delivery harmless.

## Query

The query result is the sum of all components:

```text
query(state) = sum(state[actor] for actor in actors)
```

Rust returns the sum as `u128` so summing many `u64` actor components does not
wrap at `u64::MAX`.

## Correctness Intuition

Each local increment only increases one actor component. Merge only increases
components by taking maxima. Therefore states move upward in the semilattice
order:

```text
x <= y iff merge(x, y) = y
```

If two replicas eventually receive the same maximum component values, their
states become equal and their queries return the same total.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | State merge is commutative and idempotent. |
| Duplicate delivery tolerated | Yes | Merging the same state twice has no effect after the first merge. |
| Reordered delivery tolerated | Yes | Merge order does not change the final state. |
| Message drops tolerated | Eventually | Convergence requires eventual delivery through some anti-entropy process. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

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

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/counters/gcounter.rs` | Initial implementation |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Example-based checks |
| Lean model | `proofs/lean/Crdt/Algorithms/GCounter.lean` | Initial semilattice and inflationary-update proof |
| TLA+ model | `proofs/tla/algorithms/gcounter/GCounter.tla` | TLC config checks bounded distributed execution |
| Catalog metadata | `catalog/gcounter/algorithm.toml` | Present |

Current Lean theorems:

- `Crdt.GCounter.increment_inflationary`
- `Crdt.GCounter.merge_converges_for_same_states`

Current TLC bounds:

```text
replicas = 2
counter components <= 2
```

These bounds are intentionally small. They are useful for checking that the
model is executable and that the initial network transitions preserve `TypeOK`.
They are not an unbounded proof of G-Counter correctness.

## Complexity

Let `n` be the number of local actor components and `m` be the number of remote
actor components.

| Operation | Complexity |
| --- | --- |
| Increment | `O(log n)` |
| Component lookup | `O(log n)` |
| Merge | `O(m log n)` |
| Query | `O(n)` |

## Known Constraints

- Decrement is not supported. Use PN-Counter when decrement support is needed.
- Components are `u64`; overflow is reported by the Rust API.
- There is not yet a property-based generator for arbitrary counters.
- There is not yet a separate reference model.
- The TLA+ model currently checks a very small bound.

## Next Steps

- Add property-based law tests.
- Add a reference model and implementation-vs-model comparison tests.
- Strengthen the Lean convergence theorem around observed update sets.
- Add richer TLA+ invariants beyond `TypeOK`.
- Add examples that show multi-replica synchronization histories.
