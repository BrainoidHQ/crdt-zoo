# OR-Set

OR-Set is an observed-remove set. Adds allocate unique dots, and removes clear
only the dots a replica has already observed for an element. A concurrent add
allocates a dot that the remover has not observed, so it survives merge.

## Summary

| Field | Value |
| --- | --- |
| Family | Set |
| Replication style | State-based CRDT, CvRDT |
| Conflict semantics | Add-wins for concurrent add/remove |
| Query result | Set of visible elements |
| Deletes | Observed-remove by dot |
| Tombstones | Causal context records observed dots |
| Rust status | Implemented |
| Lean status | Proved core causal-dot model laws |
| TLA+ status | Not provided yet |

## What It Solves

OR-Set supports deletion without making remove an inverse add. It distinguishes
which concrete add events are being removed, which lets concurrent adds remain
visible.

## Delivery Assumptions

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | No | Causal metadata is embedded in the state. |
| Duplicate delivery tolerated | Yes | Merge is idempotent. |
| Reordered delivery tolerated | Yes | Merge order does not affect the joined state. |
| Message drops tolerated | Eventually | Convergence requires eventual anti-entropy. |
| Exactly-once delivery | No | Duplicate state messages are safe. |

## Known Constraints

- Actor ids must be stable and unique per logical writer.
- Metadata includes visible add dots plus a causal context. Contiguous observed
  dots compact into a version vector; non-contiguous observations remain as dot
  entries.
- `add` can return `DotOverflow` if an actor-local dot counter is exhausted.
- Garbage collection needs a stable lower bound proving all live replicas have
  observed removed dots.
- Elements and actor ids must implement `Clone + Ord`.
