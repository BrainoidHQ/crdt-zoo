# Specification

## State

The state is a boolean:

```text
state in {false, true}
```

## Operations

The Rust API exposes:

- `BoolOr::new(value)`
- `enable()`
- `value()`
- `query()`
- `merge(other)`

## Merge

Merge is logical OR:

```text
merge(left, right) = left OR right
```

This join is associative, commutative, and idempotent.

## Query

The query result is the current boolean value.

## Correctness Intuition

The induced order is `false <= true`. `enable` moves upward in that order, and
merge returns the least upper bound of the two flags.

## Complexity

| Operation | Complexity |
| --- | --- |
| Enable | `O(1)` |
| Merge | `O(1)` |
| Query | `O(1)` |
