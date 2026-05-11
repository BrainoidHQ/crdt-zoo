# Specification

## State

The state is a map:

```text
ActorId -> u64
```

An absent actor component is interpreted as zero. The Rust implementation drops
zero-valued components when constructing a counter from a map so equivalent
states have a stable representation.

## Operations

The Rust API currently exposes:

- `GCounter::new()`
- `GCounter::from_counts(counts)`
- `increment(actor)`
- `increment_by(actor, amount)`
- `component(actor)`
- `counts()`
- `value()`
- `query()`
- `merge(other)`

Incrementing by zero is a no-op. Incrementing past `u64::MAX` returns
`CounterOverflow`.

## Merge

Merge is the component-wise maximum:

```text
merge(left, right)[actor] = max(left[actor], right[actor])
```

This join is associative, commutative, and idempotent. Those laws make duplicate
and reordered state delivery harmless.

## Query

The query result is the sum of all components:

```text
query(state) = sum(state[actor] for actor in actors)
```

Rust returns the sum as `u128` so summing many `u64` actor components does not
wrap at `u64::MAX`.

## Correctness Intuition

Each local increment only increases one actor component. Merge only increases
components by taking maxima. Therefore states move upward in the semilattice
order:

```text
x <= y iff merge(x, y) = y
```

If two replicas eventually receive the same maximum component values, their
states become equal and their queries return the same total.

## Complexity

Let `n` be the number of local actor components and `m` be the number of remote
actor components.

| Operation | Complexity |
| --- | --- |
| Increment | `O(log n)` |
| Component lookup | `O(log n)` |
| Merge | `O(m log n)` |
| Query | `O(n)` |
