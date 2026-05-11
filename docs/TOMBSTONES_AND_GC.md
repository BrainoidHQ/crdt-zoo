# Tombstones And Garbage Collection

Deletes in state-based CRDTs are metadata, not inverse adds. A replica must be
able to distinguish "I have not seen this add yet" from "I saw this add and
removed it." Tombstones, remove timestamps, dots, version vectors, and causal
contexts are the common ways this repository records that distinction.

## Delete Metadata By Set Type

| Algorithm | Delete metadata | Conflict policy |
| --- | --- | --- |
| 2P-Set | Removed elements in a grow-only remove set | Remove-wins permanently |
| LWW-Element-Set | Greatest remove timestamp per element | Timestamp-based, remove-wins ties |
| OR-Set | Observed dots retained in causal context | Add-wins for concurrent add/remove |

## Why Adds Cannot Simply Be Removed

Removing a value from local storage is not inflationary. A state-based CRDT
merge must be a semilattice join, so local updates must move the state upward in
the partial order. Delete metadata makes a removal an addition of information:

```text
"I observed and removed element e"
"I observed and removed dot d"
"I observed a remove timestamp t for element e"
```

That information can then be merged by union or maximum without losing
convergence.

## Garbage Collection Rule

Delete metadata can be collected only after the system has a stable lower bound
showing that no live replica can later reintroduce an older add state that needs
that metadata to suppress it.

A safe collection policy needs all of the following:

- a membership view for live replicas
- a durable lower bound for what each live replica has observed
- a plan for retired replicas or restored snapshots
- a rule that prevents stale state messages from being delivered after metadata
  is collected

Without those conditions, collection can make an old add visible again.

## Algorithm Notes

2P-Set tombstones are per element. Collection is safe only when every live
replica has observed both the add and the remove tombstone, and stale add-only
states cannot return.

LWW-Element-Set remove timestamps are compact but depend on the timestamp
policy. Collection must preserve enough timestamp information to reject older
adds that may still arrive.

OR-Set can compact contiguous dots into a version vector, but removed dots still
matter until all live replicas have advanced beyond them. Future delta-state
implementations will need stricter lower-bound tracking because partial deltas
can arrive out of order.
