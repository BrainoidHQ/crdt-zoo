# 2P-Set

2P-Set is a state-based set with permanent removals. It keeps a grow-only add
set and a grow-only remove set. An element is visible only when it is in the add
set and not in the remove set.

## Summary

| Field | Value |
| --- | --- |
| Family | Set |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Remove-wins after a tombstone exists |
| Query result | Set of visible elements |
| Deletes | Permanent tombstone |
| Tombstones | One element tombstone per removed value |
| Rust status | Implemented |
| Lean status | Not provided yet |
| TLA+ status | Not provided yet |

## What It Solves

2P-Set is useful when an element should never be reintroduced after removal,
such as one-time revocations or closed identifiers.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Merge is component-wise union. |
| Duplicate delivery tolerated | Yes | Repeated states do not change the union. |
| Reordered delivery tolerated | Yes | Merge order does not affect the result. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- A removed element cannot be added again.
- The remove set is a tombstone set and grows with distinct removed elements.
- Garbage collection is safe only when every live replica has observed the
  tombstone and no stale add-only state for that element can reappear.
- Elements must implement `Clone + Ord`.
