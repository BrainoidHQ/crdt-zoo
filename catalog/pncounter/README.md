# PN-Counter

PN-Counter is a state-based counter that supports both increments and
decrements. It stores two grow-only counters: one for increment components and
one for decrement components.

## Summary

| Field | Value |
| --- | --- |
| Family | Counter |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Component-wise maximum on both sides |
| Query result | Split positive and negative totals |
| Deletes | Not supported |
| Tombstones | None |
| Rust status | Implemented |
| Lean status | Proved |
| TLA+ status | Not provided yet |

## What It Solves

PN-Counter supports monotonic records of increments and decrements without
requiring operation delivery order. The net value is the positive total minus
the negative total.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Both internal counters merge by maximum. |
| Duplicate delivery tolerated | Yes | Re-merging state is idempotent. |
| Reordered delivery tolerated | Yes | Merge order does not affect the joined state. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Components are `u64`; overflow is reported by the Rust API.
- Incrementing or decrementing by zero is a no-op.
- Query returns split totals. Use `checked_value()` for a signed `i128` when it
  fits.
- The Lean model covers component growth and merge laws; query summation and
  signed overflow boundaries are not mechanized yet.
- No algorithm-specific TLA+ model has been added yet.
