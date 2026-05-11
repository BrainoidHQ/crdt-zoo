# Specification

## State

The state is a pair of grow-only sets:

```text
adds    subset Element
removes subset Element
```

## Operations

The Rust API exposes:

- `TwoPhaseSet::new()`
- `TwoPhaseSet::from_parts(adds, removes)`
- `add(element)`
- `remove(element)`
- `contains(element)`
- `elements()`
- `adds()`
- `removes()`
- `len()`
- `is_empty()`
- `query()`
- `merge(other)`

## Merge

Merge is component-wise union:

```text
merge(left, right).adds    = left.adds union right.adds
merge(left, right).removes = left.removes union right.removes
```

## Query

The query result is:

```text
adds \ removes
```

`add` returns whether the add component changed. Once an element is in
`removes`, local `add` calls for that element are no-ops. `remove` returns
whether it newly inserted a tombstone; removing a locally absent element still
records that durable remove metadata.

## Correctness Intuition

Both components grow monotonically, and merge is union on both components.
Removal is not the inverse of add: it records durable metadata that suppresses
matching adds in every future merge.

## Complexity

Let `n` be the local component size and `m` be the remote component size.

| Operation | Complexity |
| --- | --- |
| Add | `O(log n)` |
| Remove | `O(log n)` |
| Contains | `O(log n)` |
| Merge | `O(n + m)` clone and union cost |
| Query | `O(n)` clone and difference cost |
