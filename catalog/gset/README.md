# G-Set

G-Set is a grow-only, state-based set. Elements can be added but never removed.
Merge is set union.

## Summary

| Field | Value |
| --- | --- |
| Family | Set |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Set union |
| Query result | Set of elements |
| Deletes | Not supported |
| Tombstones | None |
| Rust status | Implemented |
| Lean status | Partial proof |
| TLA+ status | Not provided yet |

## What It Solves

G-Set represents collections where membership only grows, such as observed ids,
feature flags that are never retracted, or append-only tags.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Union is commutative and idempotent. |
| Duplicate delivery tolerated | Yes | Re-adding an existing element has no effect. |
| Reordered delivery tolerated | Yes | Merge order does not change the union. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Removal is not supported. Use later set CRDTs when deletion semantics matter.
- Elements must implement `Clone + Ord`.
- Metadata size grows with the number of elements.
