# Specification

## State

The state is a set:

```text
state subset Element
```

## Operations

The Rust API exposes:

- `GSet::new()`
- `GSet::from_elements(elements)`
- `add(element)`
- `contains(element)`
- `elements()`
- `len()`
- `is_empty()`
- `query()`
- `merge(other)`

## Merge

Merge is set union:

```text
merge(left, right) = left union right
```

## Query

The query result is a cloned `BTreeSet<T>`.

## Correctness Intuition

Each add inserts an element and never removes one. Union is associative,
commutative, and idempotent, so replicas that eventually observe the same added
elements converge to the same set.

## Complexity

Let `n` be the number of local elements and `m` be the number of remote elements.

| Operation | Complexity |
| --- | --- |
| Add | `O(log n)` |
| Contains | `O(log n)` |
| Merge | `O(n + m)` clone and union cost |
| Query | `O(n)` clone cost |
