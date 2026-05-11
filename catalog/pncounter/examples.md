# Examples

## Rust API

```rust
use crdt_algorithms::counters::PNCounter;
use crdt_core::CvRDT;

let mut left = PNCounter::new();
let mut right = PNCounter::new();

left.increment_by("left".to_owned(), 5)?;
right.decrement_by("right".to_owned(), 2)?;

left.merge(&right);
assert_eq!(left.checked_value(), Some(3));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Two-Replica Exchange

```text
left.increments  = {"a": 5}
left.decrements  = {}
right.increments = {}
right.decrements = {"b": 2}

merge(left, right).increments = {"a": 5}
merge(left, right).decrements = {"b": 2}

net = 3
```
