# Examples

## Rust API

```rust
use crdt_algorithms::lattices::BoolOr;
use crdt_core::CvRDT;

let mut left = BoolOr::new(false);
let right = BoolOr::new(true);

left.merge(&right);
assert!(left.query());
```

## Two-Replica Exchange

```text
left  = false
right = true

merge(left, right) = true
merge(right, left) = true
```
