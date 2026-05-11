# Examples

## Rust API

```rust
use crdt_algorithms::counters::GCounter;
use crdt_core::CvRDT;

let mut left = GCounter::new();
let mut right = GCounter::new();

left.increment("left".to_owned())?;
right.increment_by("right".to_owned(), 2)?;

left.merge(&right);
assert_eq!(left.query(), 3);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Two-Replica Exchange

```text
left  = {"a": 1}
right = {"b": 2}

merge(left, right) = {"a": 1, "b": 2}
merge(right, left) = {"a": 1, "b": 2}

query = 3
```

## Overflow Handling

Each actor component is a `u64`. Incrementing a component past `u64::MAX`
returns `CounterOverflow` instead of wrapping.
