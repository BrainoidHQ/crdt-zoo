# Bool OR

Bool OR is the smallest useful state-based CRDT in the catalog. The state is a
single boolean value. Local updates can only move the flag from `false` to
`true`, and merge is logical OR.

## Summary

| Field | Value |
| --- | --- |
| Family | Lattice |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Logical OR |
| Query result | Boolean flag |
| Deletes | Not supported |
| Tombstones | None |
| Rust status | Implemented |
| Lean status | Not provided yet |
| TLA+ status | Not provided yet |

## What It Solves

Bool OR represents a grow-only flag, such as "has this event happened at least
once?" Once any replica enables the flag, all replicas that receive that state
will observe `true`.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Merge is commutative and idempotent. |
| Duplicate delivery tolerated | Yes | Re-merging `true` has no additional effect. |
| Reordered delivery tolerated | Yes | OR is commutative. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- The flag cannot be reset to `false`.
- No algorithm-specific Lean or TLA+ model has been added yet.
