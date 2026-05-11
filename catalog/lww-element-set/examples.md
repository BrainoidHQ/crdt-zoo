# Examples

## Rust API

```rust
use crdt_algorithms::sets::LWWElementSet;
use crdt_core::CvRDT;

let mut set = LWWElementSet::new();

set.add("a".to_owned(), 10);
set.remove("a".to_owned(), 9);
set.add("b".to_owned(), 3);
set.remove("b".to_owned(), 4);

assert!(set.contains(&"a".to_owned()));
assert!(!set.contains(&"b".to_owned()));
assert_eq!(set.query().len(), 1);
```

## Timestamp Tie

```text
add("a", 7)
remove("a", 7)

query = {}
```

This repository's LWW-Element-Set chooses remove-wins ties so equal timestamps
do not leave an element visible.
