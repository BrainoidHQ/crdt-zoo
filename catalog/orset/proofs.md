# Proof Status

## Rust

Rust unit tests cover dot allocation, observed remove, concurrent add-wins
merge, context invariants, and replica convergence after state exchange.
Property tests check the join-semilattice laws over generated valid OR-Set
states with entry dots and causal context dots. The generated states are
normalized through `ORSet::from_entries`, so retained entry dots are observed
into the causal context before law checks run.

## Lean

Checked theorem names:

- `Crdt.ORSet.add_contains_dot`
- `Crdt.ORSet.add_observes_dot`
- `Crdt.ORSet.add_inflationary`
- `Crdt.ORSet.remove_inflationary`
- `Crdt.ORSet.observed_remove_clears_dot`
- `Crdt.ORSet.concurrent_add_wins`
- `Crdt.ORSet.merge_monotone`
- `Crdt.ORSet.merge_converges_for_same_states`

The Lean model represents entry dot sets as boolean predicates over
element-dot pairs, plus a causal context predicate over dots. State carries the
OR-Set invariant that every retained entry dot has been observed by the causal
context. The merge rule keeps a dot when the other side still has it or the
other side has not observed it, and joins causal contexts by boolean union.

The checked model proves the OR-Set join-semilattice laws through the shared
`JoinSemilattice` abstraction, proves fresh-dot add and observed remove are
inflationary, proves local add records and observes its allocated dot, proves
observed remove clears the element's currently retained dots, and proves a
concurrent add survives merge when the remover's context has not observed that
dot.

## TLA+

No TLA+ model is provided yet.

## Gaps

- The Lean model abstracts from finite `BTreeMap`/`BTreeSet` extraction and
  dot allocation overflow details.
- No separate reference model is implemented yet.
- There is no bounded TLA+ model yet for concurrent add/remove histories.
- Garbage collection is documented but not implemented.
