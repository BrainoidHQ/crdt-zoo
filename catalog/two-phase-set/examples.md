# Examples

## Rust API

```rust
use crdt_algorithms::sets::TwoPhaseSet;
use crdt_core::CvRDT;

let mut left = TwoPhaseSet::new();
let mut right = TwoPhaseSet::new();

left.add("a".to_owned());
right.add("a".to_owned());
right.remove(&"a".to_owned());

left.merge(&right);

assert!(!left.contains(&"a".to_owned()));
assert!(left.query().is_empty());
```

## Remove-Wins Tombstone

```text
left  = adds {"a"}, removes {}
right = adds {},    removes {"a"}

merge(left, right) = adds {"a"}, removes {"a"}
query              = {}
```

The tombstone remains after merge, so a later `add("a")` on that merged state
does not make `"a"` visible.
