# Specification

## State

The state is:

```text
entries : Element -> DotSet
context : CausalContext
```

Each dot is `(actor, counter)`. The causal context contains every dot this state
has observed, including dots removed from `entries`.

## Operations

The Rust API exposes:

- `ORSet::new()`
- `ORSet::from_entries(entries, context)`
- `add(actor, element)`
- `remove(element)`
- `contains(element)`
- `elements()`
- `dots(element)`
- `entries()`
- `context()`
- `len()`
- `is_empty()`
- `query()`
- `merge(other)`

## Merge

For each element, merge keeps a dot when either:

- both replicas still contain the dot, or
- the other replica's causal context does not contain the dot.

It drops a dot when the other replica has observed the dot but no longer stores
it for that element.

The merged causal context is the join of both causal contexts.

## Query

The query result contains every element whose dot set is non-empty.

## Correctness Intuition

Remove is not a global anti-add. It only removes the add dots that were visible
to the remover. A concurrent add creates a dot absent from the remover's causal
context, so merge preserves that dot and the element remains visible.

## Complexity

Let `n` be the number of local elements, `m` the number of remote elements, and
`d` the number of dots being compared for an element.

| Operation | Complexity |
| --- | --- |
| Add | `O(log n + log d)` plus causal context compaction |
| Remove | `O(log n)` |
| Contains | `O(log n)` |
| Merge | `O(n + m + total dots)` clone and dot filtering cost |
| Query | `O(n)` clone cost |
