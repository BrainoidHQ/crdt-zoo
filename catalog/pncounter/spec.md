# Specification

## State

The state is a pair of grow-only maps:

```text
positive: ActorId -> u64
negative: ActorId -> u64
```

Absent actor components are interpreted as zero.

## Operations

The Rust API exposes:

- `PNCounter::new()`
- `PNCounter::from_components(increments, decrements)`
- `increment(actor)`
- `increment_by(actor, amount)`
- `decrement(actor)`
- `decrement_by(actor, amount)`
- `increment_component(actor)`
- `decrement_component(actor)`
- `value()`
- `checked_value()`
- `query()`
- `merge(other)`

## Merge

Merge takes component-wise maximum independently on both maps:

```text
merge(left, right).positive[a] = max(left.positive[a], right.positive[a])
merge(left, right).negative[a] = max(left.negative[a], right.negative[a])
```

## Query

`query()` returns `PNCounterValue`, which stores:

```text
increments = sum(positive components)
decrements = sum(negative components)
```

The signed net value is available through `checked_value()`.

## Correctness Intuition

Each local increment or decrement only increases one component in one internal
G-Counter. Merge only increases components by taking maxima. Therefore every
local update is inflationary in the product semilattice.

## Complexity

Let `n` be the number of local actor components and `m` be the number of remote
actor components.

| Operation | Complexity |
| --- | --- |
| Increment/decrement | `O(log n)` |
| Component lookup | `O(log n)` |
| Merge | `O(m log n)` for each internal counter |
| Query | `O(n)` |
