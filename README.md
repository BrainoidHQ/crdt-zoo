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

Phase 0 is complete, and the catalog now includes the Phase 1 semilattice
basics plus the Phase 2 Rust implementations for sets with deletes. The first
complete exhibit is the Grow-only Counter (G-Counter), with:

- a Rust implementation in `crates/crdt-algorithms`
- shared algebraic traits in `crates/crdt-core`
- reusable law checks in `crates/crdt-testkit`
- a Lean model in `proofs/lean`
- a TLA+ model and TLC config in `proofs/tla`
- catalog metadata and documentation in `catalog/gcounter`

The Phase 1 implementations add Bool OR, Max Register, PN-Counter, and G-Set
alongside property-based law tests, small reusable generators, reference-model
comparison helpers, Lean models for the core algebraic laws, and CI jobs for
Rust, Lean, and TLC.

The Phase 2 implementations add 2P-Set, LWW-Element-Set, and OR-Set, plus shared
actor ids, replica ids, dots, dot sets, version vectors, causal contexts, and
tombstone/garbage-collection documentation. The 2P-Set has a Lean predicate-set
model for its core laws; Lean/TLA+ models for the more advanced delete-aware
sets are still deferred and called out in the catalog pages.

## Implemented Semantics

The catalog is organized by the conflict semantics each CRDT exposes:

| Semantics | Algorithms |
| --- | --- |
| Grow-only semilattice state | Bool OR, G-Counter, G-Set, Max Register |
| Split positive/negative growth | PN-Counter |
| Permanent remove-wins tombstones | 2P-Set |
| Timestamp-based delete policy | LWW-Element-Set |
| Causally tracked add-wins deletes | OR-Set |

Delete-aware sets are intentionally documented more heavily than G-Set. Their
catalog pages state whether deletion is permanent, timestamp-based, or tied to
observed dots, and [Tombstones And Garbage Collection](docs/TOMBSTONES_AND_GC.md)
describes when delete metadata can be reclaimed safely.

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
    */                per-algorithm metadata and source documentation

  docs/
    book/             mdBook configuration, theme, and generated site source
    *.md              contributor-facing design and process documentation

  tools/
    catalog-gen/      converts catalog/ into mdBook Markdown and JSON
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
- [Tombstones And Garbage Collection](docs/TOMBSTONES_AND_GC.md): delete
  metadata and collection constraints for set CRDTs
- [References](docs/REFERENCES.md): background material that informs the design

The GitHub Pages site is generated from `catalog/` through `tools/catalog-gen`
and mdBook. The generated `docs/book/src/SUMMARY.md`,
`docs/book/src/algorithms/`, `docs/book/src/indexes/`, and
`docs/book/src/assets/catalog.json` files are local build outputs, not the source
of truth.

## Quick Start

Use the Nix Flake dev shell to get Rust, Lean, and TLA+ tooling:

```sh
nix develop
```

Run the Rust test suite:

```sh
cargo test --workspace
```

Generate and build the catalog site:

```sh
cargo run -p catalog-gen
mdbook build docs/book
```

Serve the catalog site locally:

```sh
mdbook serve docs/book --open
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

Run the current TLAPS proof:

```sh
cd proofs/tla/algorithms/gcounter
tlapm --cleanfp --nofp --solver z3 --threads 1 GCounter.tla
```

## Design Principles

- Keep specifications, implementations, tests, proofs, and documentation close
  enough that they can be reviewed as one unit.
- Be explicit about what is proved, what is model-checked, and what is only
  tested.
- Prefer deterministic data structures such as `BTreeMap` and `BTreeSet` when
  stable examples, serialization, or test output matter.
- Treat delivery assumptions as part of an algorithm's public contract.
- Treat delete metadata as part of the algorithm, not an implementation detail.
- Add sequence CRDTs only after the causal set and map infrastructure is mature.

## Current Catalog

| Algorithm | Family | Rust | Lean | TLA+ |
| --- | --- | --- | --- | --- |
| [Bool OR](catalog/bool-or/README.md) | Lattice, CvRDT | Implemented | Proved | Not provided |
| [G-Counter](catalog/gcounter/README.md) | Counter, CvRDT | Implemented | Partial proof | TLC bounded model check |
| [G-Set](catalog/gset/README.md) | Set, CvRDT | Implemented | Partial proof | Not provided |
| [LWW-Element-Set](catalog/lww-element-set/README.md) | Set, CvRDT | Implemented | Not provided | Not provided |
| [Max Register](catalog/max-register/README.md) | Register, CvRDT | Implemented | Partial proof | Not provided |
| [OR-Set](catalog/orset/README.md) | Set, CvRDT | Implemented | Not provided | Not provided |
| [PN-Counter](catalog/pncounter/README.md) | Counter, CvRDT | Implemented | Partial proof | Not provided |
| [2P-Set](catalog/two-phase-set/README.md) | Set, CvRDT | Implemented | Partial proof | Not provided |

## License

CRDT Zoo is distributed under the MIT License. See [LICENSE](LICENSE).
