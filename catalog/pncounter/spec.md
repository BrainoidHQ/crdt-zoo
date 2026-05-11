# Specification

## State

The state is a pair of grow-only maps:

```text
increments: ActorId -> u64
decrements: ActorId -> u64
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
- `increments()`
- `decrements()`
- `value()`
- `checked_value()`
- `query()`
- `merge(other)`

Incrementing or decrementing by zero is a no-op. If either internal `u64`
component would overflow, the update returns `CounterOverflow`.

## Merge

Merge takes component-wise maximum independently on both maps:

```text
merge(left, right).increments[a] = max(left.increments[a], right.increments[a])
merge(left, right).decrements[a] = max(left.decrements[a], right.decrements[a])
```

## Query

`query()` returns `PNCounterValue`, which stores:

```text
increments = sum(increment components)
decrements = sum(decrement components)
```

`PNCounterValue` exposes `increments()`, `decrements()`, and `checked_net()`.
The signed net value for a counter is available through `checked_value()`, which
returns `None` when the net value does not fit in `i128`.

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
