# Max Register

Max Register is a state-based register whose value can only move upward
according to the value's `Ord` ordering. Merge keeps the maximum value observed
by either replica.

## Summary

| Field | Value |
| --- | --- |
| Family | Register |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Maximum value wins |
| Query result | Optional current value |
| Deletes | Not supported |
| Tombstones | None |
| Rust status | Implemented |
| Lean status | Partial proof |
| TLA+ status | Not provided yet |

## What It Solves

Max Register is useful for monotonic values such as high-water marks, maximum
observed sequence numbers, or highest known score. It is not a general-purpose
last-writer-wins register.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Merge is maximum. |
| Duplicate delivery tolerated | Yes | Re-merging a value is idempotent. |
| Reordered delivery tolerated | Yes | Maximum is commutative. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Lower assignments are intentionally ignored.
- Values must implement `Clone + Ord`.
- No timestamp or actor tie-break is stored.
