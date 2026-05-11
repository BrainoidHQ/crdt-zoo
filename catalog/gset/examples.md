# Examples

## Rust API

```rust
use crdt_algorithms::sets::GSet;
use crdt_core::CvRDT;

let mut left = GSet::new();
let mut right = GSet::new();

left.add("a".to_owned());
right.add("b".to_owned());

left.merge(&right);
assert!(left.contains(&"a".to_owned()));
assert!(left.contains(&"b".to_owned()));
assert_eq!(left.query().len(), 2);
```

## Two-Replica Exchange

```text
left  = {"a"}
right = {"b"}

merge(left, right) = {"a", "b"}
merge(right, left) = {"a", "b"}
```
