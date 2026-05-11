# Examples

## Rust API

```rust
use crdt_algorithms::sets::ORSet;
use crdt_core::{CvRDT, JoinSemilattice};

let mut left = ORSet::new();
let mut right = ORSet::new();

left.add("left".to_owned(), "task".to_owned()).unwrap();
right.merge(&left);

left.add("left".to_owned(), "task".to_owned()).unwrap();
right.remove(&"task".to_owned());

let joined = left.join(&right);

assert!(joined.contains(&"task".to_owned()));
assert_eq!(joined.query().len(), 1);
```

## Concurrent Add Wins

```text
left observes:  add task @ (left, 1)
right observes: add task @ (left, 1)

left adds:      task @ (left, 2)
right removes:  task, clearing only (left, 1)

merge result:   task @ (left, 2)
query:          {"task"}
```

The remove could only clear the dot it had observed. The concurrent second add
survives because it was not in the remover's causal context.
