# Contributing

CRDT Zoo values changes that keep implementation, specification, verification,
and documentation aligned. A change is not considered complete only because the
Rust code compiles. The catalog page should also explain what the code means and
which correctness checks currently support it.

## Development Environment

Use the Nix dev shell:

```sh
nix develop
```

Useful commands:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace

cd proofs/lean
lake build

cd ../tla
tlc -config algorithms/gcounter/GCounter_MC.cfg algorithms/gcounter/GCounter.tla
```

## Expected Change Shape

For implementation changes:

- keep public APIs small and documented
- document trait laws and delivery assumptions
- add or update law tests
- update the catalog page when behavior changes
- avoid unrelated refactors

For proof or model changes:

- state exactly what is proved or checked
- keep checked bounds visible in catalog metadata
- do not describe bounded TLC or Apalache checks as full proofs
- prefer small reusable modules over large duplicated specifications

For documentation changes:

- write in English
- keep claims tied to files, tests, or proof artifacts
- distinguish current behavior from planned behavior
- link to the relevant catalog entry or source file

## Algorithm Quality Bar

Before a new algorithm is considered ready for the catalog, it should have:

- Rust implementation
- Rust examples or tests
- law tests
- reference model or an explicit reason it is deferred
- Lean model or an explicit proof gap
- TLA+ model or an explicit model-checking gap
- catalog metadata
- catalog README
- documented complexity
- documented network assumptions
- documented limitations

Small incremental pull requests are fine, but each one should leave the
repository in a coherent state.

## Style Guidelines

- Prefer `BTreeMap` and `BTreeSet` for deterministic output unless performance
  requires another structure.
- Do not use `unsafe` in CRDT implementations unless there is a documented,
  reviewed reason and Miri coverage.
- Treat clock assumptions as part of the algorithm. LWW types must document
  tie-breaks, monotonicity requirements, and clock-skew behavior.
- Treat garbage collection as part of the algorithm. OR-Set-like structures must
  document tombstones, causal context, and safe reclamation conditions.
- Keep Lean models mathematical rather than Rust-shaped.
- Keep TLA+ models focused on distributed behavior rather than pure algebra.

## Commit Checklist

Before committing, run the checks that match your change:

- Rust-only change: `cargo fmt --all --check`, `cargo clippy ...`, and
  `cargo test --workspace`
- Lean change: `lake build` in `proofs/lean`
- TLA+ change: TLC for the affected configs
- Catalog change: check links and make sure proof status is honest

If a check cannot be run locally, mention the reason in the change description.
