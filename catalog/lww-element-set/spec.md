# Specification

## State

The state is two maps:

```text
adds    : Element -> Timestamp
removes : Element -> Timestamp
```

Missing entries mean no timestamp has been observed for that component.

## Operations

The Rust API exposes:

- `LWWElementSet::new()`
- `LWWElementSet::from_timestamps(adds, removes)`
- `add(element, timestamp)`
- `remove(element, timestamp)`
- `contains(element)`
- `elements()`
- `add_timestamp(element)`
- `remove_timestamp(element)`
- `adds()`
- `removes()`
- `len()`
- `is_empty()`
- `query()`
- `merge(other)`

## Merge

Merge keeps the maximum timestamp for each element in each component:

```text
merge(left, right).adds[e]    = max(left.adds[e], right.adds[e])
merge(left, right).removes[e] = max(left.removes[e], right.removes[e])
```

## Query

An element is visible when:

```text
adds[e] exists and (removes[e] does not exist or adds[e] > removes[e])
```

Equal timestamps are remove-wins.

## Correctness Intuition

The state grows by replacing component timestamps only with greater timestamps.
Merge is a pointwise maximum, which is associative, commutative, and
idempotent. Delete semantics are not causal: the largest timestamp decides.

## Complexity

Let `n` be the local component size and `m` be the remote component size.

| Operation | Complexity |
| --- | --- |
| Add | `O(log n)` |
| Remove | `O(log n)` |
| Contains | `O(log n)` |
| Merge | `O(n + m)` clone and map update cost |
| Query | `O(n log n)` timestamp lookup and clone cost |
