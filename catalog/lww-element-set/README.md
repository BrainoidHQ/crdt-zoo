# LWW-Element-Set

LWW-Element-Set is a timestamp-based set. Each element records the greatest add
timestamp and the greatest remove timestamp observed for that element. The
implementation uses remove-wins semantics when timestamps are equal.

## Summary

| Field | Value |
| --- | --- |
| Family | Set |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Timestamp-based, remove-wins on equal timestamps |
| Query result | Set of visible elements |
| Deletes | Timestamped remove marker |
| Tombstones | Remove timestamps by element |
| Rust status | Implemented |
| Lean status | Not provided yet |
| TLA+ status | Not provided yet |

## What It Solves

LWW-Element-Set is useful when an application already has a trusted timestamp
policy and wants compact delete metadata compared with per-add dots.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Merge keeps maximum timestamps per component. |
| Duplicate delivery tolerated | Yes | Repeated states keep the same maxima. |
| Reordered delivery tolerated | Yes | Merge order does not affect timestamp maxima. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Correctness depends on the application's timestamp policy.
- Clock skew can make an older real-world action win if it carries a larger
  timestamp.
- Equal add and remove timestamps resolve to removed.
- Remove timestamps are delete metadata and need the same care as tombstones for
  garbage collection.
- Elements must implement `Clone + Ord`; timestamps must implement `Clone + Ord`.
