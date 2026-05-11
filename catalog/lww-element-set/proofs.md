# Proof Status

## Rust

Rust unit tests cover timestamp visibility, remove-wins ties, older updates,
component-wise maximum merge, and inflationary updates. Property tests check the
join-semilattice laws over generated timestamp maps.

## Lean

Checked theorem names:

- `Crdt.LWWElementSet.add_inflationary`
- `Crdt.LWWElementSet.remove_inflationary`
- `Crdt.LWWElementSet.added_visible_without_remove`
- `Crdt.LWWElementSet.equal_timestamps_remove_win`
- `Crdt.LWWElementSet.older_add_timestamp_ignored`
- `Crdt.LWWElementSet.older_remove_timestamp_ignored`
- `Crdt.LWWElementSet.merge_monotone`
- `Crdt.LWWElementSet.merge_converges_for_same_states`

The Lean model represents add and remove timestamp maps as total functions from
elements to optional natural-number timestamps. Missing map entries are modeled
as `none`; observed timestamps are modeled as `some timestamp`. Merge is
component-wise maximum over both add and remove components.

The checked model proves the join-semilattice laws through the shared
`JoinSemilattice` abstraction, proves add and remove timestamp updates are
inflationary, proves merge is monotone, proves equal timestamps are remove-wins,
and proves older component updates are ignored.

## TLA+

No TLA+ model is provided yet.

## Gaps

- There is no model check for clock skew histories yet.
- The Lean model abstracts from finite `BTreeMap` extraction and uses
  natural-number timestamps rather than a generic ordered timestamp type.
