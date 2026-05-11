# Specification

## State

The state is an optional value:

```text
state in None | Some(value)
```

`None` is the bottom element.

## Operations

The Rust API exposes:

- `MaxRegister::new()`
- `MaxRegister::with_value(value)`
- `assign(value)`
- `value()`
- `into_value()`
- `query()`
- `merge(other)`

## Merge

Merge keeps the greater value:

```text
merge(None, x) = x
merge(x, None) = x
merge(Some(a), Some(b)) = Some(max(a, b))
```

## Query

The query result is a cloned `Option<T>`.

## Correctness Intuition

Each assignment joins the current register with a singleton value. Because
maximum is associative, commutative, and idempotent, replicas that observe the
same values converge on the same maximum.

## Complexity

| Operation | Complexity |
| --- | --- |
| Assign | `O(1)` comparison cost plus value move |
| Merge | `O(1)` comparison cost plus clone of winning value |
| Query | Clone cost of the stored value |
