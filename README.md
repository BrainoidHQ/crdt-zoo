# CRDT Zoo

CRDT Zoo is a catalog-style workspace for CRDT implementations, executable
tests, lightweight formal models, and per-algorithm documentation.

## Layout

- `crates/crdt-core`: shared CRDT traits and algebraic vocabulary
- `crates/crdt-algorithms`: concrete CRDT implementations
- `crates/crdt-testkit`: reusable law checks for implementations
- `proofs/lean`: Lean models and proofs
- `proofs/tla`: TLA+ models and TLC configs
- `catalog`: per-algorithm catalog pages and metadata

## Development

Use the Nix Flake dev shell to get the Rust, Lean, and TLA+ tooling:

```sh
nix develop
cargo test --workspace
```
