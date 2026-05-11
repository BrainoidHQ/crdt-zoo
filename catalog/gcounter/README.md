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
| Rust status | Implemented |
| Lean status | Partial proof |
| TLA+ status | Model-checked by TLC with bounded configuration |

## What It Solves

G-Counter represents a counter that can be incremented independently by multiple
replicas. Each actor owns one component. Replicas exchange state and merge by
taking the maximum value for each actor.

This is useful when the application only needs monotonic increments, such as
counting observed events where decrements are not part of the data type.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | State merge is commutative and idempotent. |
| Duplicate delivery tolerated | Yes | Merging the same state twice has no effect after the first merge. |
| Reordered delivery tolerated | Yes | Merge order does not change the final state. |
| Message drops tolerated | Eventually | Convergence requires eventual delivery through an anti-entropy process. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Decrement is not supported. Use PN-Counter when decrement support is needed.
- Components are `u64`; overflow is reported by the Rust API.
- Serialization support is not enabled yet.
- Property-based generators currently cover small actor maps.
- The TLA+ model currently checks a very small bound.
