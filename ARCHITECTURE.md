# Architecture

CRDT Zoo is organized around one rule: each CRDT should be understandable as a
single exhibit, not as an isolated implementation file. An exhibit contains the
Rust type, the algebraic specification, the distributed model, the tests, the
proof status, and the explanatory page that ties those pieces together.

## Layers

The repository has three conceptual layers.

| Layer | Purpose | Primary tools |
| --- | --- | --- |
| Specification | Define the abstract meaning of CRDTs and their laws. | Lean, TLA+ |
| Implementation | Provide usable Rust APIs and data structures. | Rust |
| Connection | Check that the implementation behaves like the specification. | Law tests, property tests, reference comparisons; trace replay planned |

The project does not try to prove compiled Rust code directly in Lean or TLA+.
That would make the first useful version too heavy. Instead, the intended flow
is:

1. Prove general algebraic facts in Lean.
2. Show that each CRDT model satisfies the assumptions of those facts.
3. Test the Rust implementation against those laws, reference models, and
   traces generated from bounded distributed models.

This separation keeps the formal models readable while still putting pressure on
the Rust code to match them.

## Workspace Boundaries

### `crates/crdt-core`

`crdt-core` owns shared vocabulary. It should contain traits and reusable data
types that many algorithms need:

- `JoinSemilattice`
- `BoundedJoinSemilattice`
- `CvRDT`
- actor and replica identifiers
- dots, version vectors, dot sets, and causal contexts
- future traits for operation-based and delta-state CRDTs

Trait laws belong in documentation even when Rust cannot enforce them. For
example, `JoinSemilattice` requires associativity, commutativity, and
idempotence. Implementors must make those laws true; `crdt-testkit` provides the
runtime checks.

### `crates/crdt-algorithms`

`crdt-algorithms` owns concrete CRDT implementations. Algorithms should use the
traits from `crdt-core` and should avoid inventing private equivalents of shared
concepts. The crate currently contains small lattice CRDTs, counters, registers,
and set CRDTs, including delete-aware 2P-Set, LWW-Element-Set, and OR-Set.

Algorithm modules should be small enough to read locally, but complete enough to
show the intended API. Public APIs should make delivery assumptions and failure
modes visible. For example, `GCounter::increment_by` returns an overflow error
instead of silently wrapping a component, and `ORSet::add` returns the allocated
dot or a dot-counter overflow error.

### `crates/crdt-testkit`

`crdt-testkit` owns reusable checks. Its role is to prevent every algorithm from
rewriting the same law tests, generators, and model comparison logic. It
currently contains:

- example-style join and inflationary-update assertions
- proptest-compatible join and inflationary-update checks
- small generators for actor ids, component maps, sets, dots, and dot sets
- reference-model query comparison helpers

The next additions should be operation history generators, network simulators,
and TLA+ trace replay utilities.

### `proofs/lean`

Lean models algebraic structure. It is the right place for definitions such as
join-semilattice laws, convergence theorems for state-based CRDTs, and
algorithm-specific proofs that a model satisfies those laws.

Lean code should not mirror Rust implementation details such as ownership,
`BTreeMap` internals, serde formats, or error handling. It should model the math
that the Rust API commits to preserving.

### `proofs/tla`

TLA+ models distributed executions and can also carry TLAPS proofs for facts
that are worth checking in TLA+ itself. TLC model checks are powerful, but they
are bounded checks unless a specification is separately proved with TLAPS.
Catalog pages must state checked bounds clearly.

The repository should use TLA+ terminology precisely:

- "model-checked by TLC" for finite-state exploration
- "model-checked by Apalache" for bounded symbolic checking when added
- "proved by TLAPS" only for mechanically checked TLA+ proofs
- "proved in Lean" only for checked Lean theorems

### `catalog`

`catalog/<algorithm>/` is the exhibit page for an algorithm. It connects the
Rust module, the Lean file, any TLA+ file, checked properties, examples,
complexity, and known limitations.

Every algorithm should have at least:

- `algorithm.toml`
- `README.md`
- `spec.md`
- `examples.md`
- `proofs.md`
- a Rust implementation
- law tests
- a proof or an explicit proof gap
- a model or an explicit model-checking gap

`catalog/` is the source of truth for the public catalog site. Generated mdBook
files under `docs/book/src/algorithms/`, `docs/book/src/indexes/`,
`docs/book/src/SUMMARY.md`, and `docs/book/src/assets/catalog.json` are build
outputs derived from `catalog/` by `tools/catalog-gen`.

### `docs/book`

`docs/book` owns the static site wrapper for GitHub Pages:

- `book.toml` configures mdBook.
- `src/README.md` is the hand-written landing page.
- `theme/` contains hand-written CSS and JavaScript.
- generated `src/` children are ignored by Git and rebuilt in CI.

### `tools/catalog-gen`

`tools/catalog-gen` validates `catalog/*/algorithm.toml`, checks referenced
Rust/Lean/TLA+ files, combines the per-algorithm Markdown files into mdBook
pages, emits grouped index pages, writes `SUMMARY.md`, and writes
`catalog.json` for the catalog browser.

## State-Based CRDT Vocabulary

State-based CRDTs use merge as a semilattice join:

```text
merge(x, y) = x join y

x join y = y join x
(x join y) join z = x join (y join z)
x join x = x

x <= y iff x join y = y
```

Local updates must be inflationary:

```text
x <= update(x)
```

If every replica state can be understood as the join of the updates it has
observed, then replicas that observe the same updates converge to the same
state.

## Operation-Based CRDT Vocabulary

Operation-based CRDTs use two conceptual functions:

```text
prepare : local state -> op
effect  : state -> op -> state
```

Concurrent operations must commute when delivered according to the algorithm's
requirements:

```text
effect(effect(s, op1), op2) = effect(effect(s, op2), op1)
```

Operation-based algorithms often require stronger delivery assumptions than
state-based ones. Catalog pages must state:

- whether causal delivery is required
- whether duplicate delivery is tolerated
- whether message loss is tolerated
- whether exactly-once delivery is required

## Shared Data Structures

The repository should prefer deterministic structures when they are visible in
documentation, serialized output, tests, or examples:

```rust
use std::collections::{BTreeMap, BTreeSet};

pub struct ActorId(String);

pub struct Dot<A = ActorId> { /* actor, counter */ }

pub struct VersionVector<A = ActorId> { /* actor -> counter */ }
pub struct DotSet<A = ActorId> { /* BTreeSet<Dot<A>> */ }
pub struct CausalContext<A = ActorId> { /* compact clock + dot set */ }
```

Stable ordering makes examples reproducible and test failures readable. It also
keeps catalog pages honest because printed states do not change randomly between
runs.

`ActorId` identifies a logical writer. `ReplicaId` identifies storage and
exchange locations. OR-Set currently allocates dots from actor ids; future
algorithms should keep that distinction so moving an actor between replicas does
not change its causal identity.

`CausalContext` stores contiguous observations in a version vector and keeps
non-contiguous dots in a dot set. It is a metadata structure, not a delivery
guarantee. State-based algorithms still converge through eventual anti-entropy,
while the context explains which dots a replica has observed.

## Design Constraints

Avoid these patterns unless there is a strong reason:

- making every CRDT a completely independent crate
- writing proof-like comments without executable tests or checked models
- calling bounded TLA+ model checking a proof
- leaving physical-clock assumptions vague for LWW algorithms
- omitting tombstone, causal context, or garbage-collection details for sets
- starting with sequence CRDTs before the core algebra is mature
- hiding network assumptions outside the algorithm page

## Current Repository Shape

The repository now has eight implemented state-based CRDTs: the Phase 1
semilattice basics and the Phase 2 delete-aware sets. Each implemented
algorithm has Rust law tests and a checked Lean model; G-Counter is currently
the only algorithm with a TLA+ model and TLAPS/TLC coverage.

```text
crates/
  crdt-core/
    src/causal.rs
    src/ids.rs
    src/lattice.rs
    src/state_based.rs
  crdt-algorithms/
    src/counters/gcounter.rs
    src/counters/pncounter.rs
    src/lattices/bool_or.rs
    src/registers/max.rs
    src/sets/gset.rs
    src/sets/lww_element.rs
    src/sets/orset.rs
    src/sets/two_phase.rs
  crdt-testkit/
    src/generators.rs
    src/law_tests.rs
    src/reference.rs

proofs/
  lean/Crdt/Algebra/JoinSemilattice.lean
  lean/Crdt/StateBased/CvRDT.lean
  lean/Crdt/Algorithms/BoolOr.lean
  lean/Crdt/Algorithms/GCounter.lean
  lean/Crdt/Algorithms/GSet.lean
  lean/Crdt/Algorithms/LWWElementSet.lean
  lean/Crdt/Algorithms/MaxRegister.lean
  lean/Crdt/Algorithms/ORSet.lean
  lean/Crdt/Algorithms/PNCounter.lean
  lean/Crdt/Algorithms/TwoPhaseSet.lean
  tla/modules/StateBasedCommon.tla
  tla/algorithms/gcounter/GCounter.tla
  tla/algorithms/gcounter/GCounter_MC.cfg

catalog/
  bool-or/
  gcounter/
  gset/
  lww-element-set/
  max-register/
  orset/
  pncounter/
  two-phase-set/

docs/book/
  book.toml
  src/README.md
  theme/

tools/
  catalog-gen/
```

The next architectural step is to make the remaining delete-aware path deeper:
reference models for OR-Set-like histories, tighter links between the Lean
causal-dot model and finite Rust maps, and TLA+ models that exercise stale,
duplicated, reordered, and concurrent state delivery.
