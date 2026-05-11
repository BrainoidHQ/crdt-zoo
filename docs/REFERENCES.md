# References

These references inform the repository design. They are not required reading for
every contribution, but they explain why the project separates Rust
implementation, algebraic proof, and distributed model checking.

## CRDTs

- Conflict-free Replicated Data Types (CRDTs):
  <https://arxiv.org/abs/1805.06358>
- Delta State Replicated Data Types:
  <https://arxiv.org/abs/1603.01529>
- Verifying Strong Eventual Consistency in Distributed Systems:
  <https://martin.kleppmann.com/2017/10/25/verifying-crdt-isabelle.html>

## Rust

- Cargo workspaces:
  <https://doc.rust-lang.org/cargo/reference/workspaces.html>
- Rust Fuzz Book:
  <https://rust-fuzz.github.io/book/cargo-fuzz.html>
- Miri:
  <https://github.com/rust-lang/miri/>
- Criterion.rs:
  <https://bheisler.github.io/criterion.rs/book/>

## Lean

- Lean language reference:
  <https://lean-lang.org/doc/reference/latest/>
- Lake build tool:
  <https://lean-lang.org/doc/reference/latest/Build-Tools-and-Distribution/Lake/>

## TLA+

- TLC overview:
  <https://docs.tlapl.us/using%3Atlc%3Astart>
- TLA+ Proof System:
  <https://proofs.tlapl.us/doc/web/content/Home.html>
- Apalache documentation:
  <https://apalache-mc.org/docs/apalache/index.html>

## Property Testing

- Proptest book:
  <https://altsysrq.github.io/proptest-book/>
