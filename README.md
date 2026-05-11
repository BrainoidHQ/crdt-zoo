# CRDT Zoo

CRDT Zoo is a catalog-style Rust workspace for learning, implementing, and
checking Conflict-free Replicated Data Types (CRDTs). The repository is not
intended to be only a collection of Rust modules. Each algorithm is treated as a
complete exhibit that connects Rust APIs, mathematical specifications, Lean
proofs, TLA+ models, executable tests, and user-facing documentation.

The central correctness idea is convergence: replicas may update independently,
but replicas that eventually observe the same set of updates must reach the same
state deterministically. This repository uses that idea as the shared vocabulary
for code, proofs, tests, and catalog pages.

## Repository Status

This is an early skeleton. The first complete exhibit is the Grow-only Counter
(G-Counter), with:

- a Rust implementation in `crates/crdt-algorithms`
- shared algebraic traits in `crates/crdt-core`
- reusable law checks in `crates/crdt-testkit`
- a Lean model in `proofs/lean`
- a TLA+ model and TLC config in `proofs/tla`
- catalog metadata and documentation in `catalog/gcounter`

The next milestone is to make G-Counter more exhaustive before adding many new
algorithms. A strong first exhibit gives later CRDTs a template for API shape,
proof status, model-checking bounds, examples, and documentation quality.

## Workspace Layout

```text
crdt-zoo/
  crates/
    crdt-core/        shared traits and algebraic vocabulary
    crdt-algorithms/  concrete CRDT implementations
    crdt-testkit/     reusable law checks and future trace/model testing tools

  proofs/
    lean/             algebraic models and mechanically checked Lean proofs
    tla/              distributed execution models and TLC configs

  catalog/
    gcounter/         per-algorithm metadata and documentation

  docs/               contributor-facing design and process documentation
```

## Documentation

- [Architecture](ARCHITECTURE.md): repository structure, design boundaries, and
  Rust/Lean/TLA+ responsibilities
- [Roadmap](ROADMAP.md): planned algorithm phases and repository milestones
- [Contributing](CONTRIBUTING.md): development workflow and expectations for
  new changes
- [Verification Strategy](docs/VERIFICATION.md): how tests, Lean, TLA+, models,
  traces, and fuzzing fit together
- [Adding Algorithms](docs/ADDING_ALGORITHMS.md): the expected checklist for a
  new catalog entry
- [References](docs/REFERENCES.md): background material that informs the design

## Quick Start

Use the Nix Flake dev shell to get Rust, Lean, and TLA+ tooling:

```sh
nix develop
```

Run the Rust test suite:

```sh
cargo test --workspace
```

Run the Lean proofs:

```sh
cd proofs/lean
lake build
```

Run the current TLA+ model:

```sh
cd proofs/tla
tlc -config algorithms/gcounter/GCounter_MC.cfg algorithms/gcounter/GCounter.tla
```

## Design Principles

- Keep specifications, implementations, tests, proofs, and documentation close
  enough that they can be reviewed as one unit.
- Be explicit about what is proved, what is model-checked, and what is only
  tested.
- Prefer deterministic data structures such as `BTreeMap` and `BTreeSet` when
  stable examples, serialization, or test output matter.
- Treat delivery assumptions as part of an algorithm's public contract.
- Start with algebraically simple CRDTs before introducing dots, causal
  contexts, garbage collection, or sequence CRDTs.

## Current Catalog

| Algorithm | Family | Rust | Lean | TLA+ |
| --- | --- | --- | --- | --- |
| [G-Counter](catalog/gcounter/README.md) | Counter, CvRDT | Initial implementation | Initial model | TLC config |

## License

CRDT Zoo is distributed under the MIT License. See [LICENSE](LICENSE).
