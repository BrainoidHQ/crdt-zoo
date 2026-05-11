# Examples

## Rust API

```rust
use crdt_algorithms::registers::MaxRegister;
use crdt_core::CvRDT;

let mut left = MaxRegister::new();
let mut right = MaxRegister::with_value(7);

left.assign(3);
left.merge(&right);

assert_eq!(left.query(), Some(7));
```

## Lower Assignment

```text
state = Some(10)
assign(4)
state = Some(10)
```
