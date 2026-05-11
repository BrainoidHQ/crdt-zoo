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
| Connection | Check that the implementation behaves like the specification. | Law tests, model-based tests, trace replay |

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
- future actor and replica identifiers
- future dots, version vectors, dot sets, and causal contexts
- future traits for operation-based and delta-state CRDTs

Trait laws belong in documentation even when Rust cannot enforce them. For
example, `JoinSemilattice` requires associativity, commutativity, and
idempotence. Implementors must make those laws true; `crdt-testkit` provides the
runtime checks.

### `crates/crdt-algorithms`

`crdt-algorithms` owns concrete CRDT implementations. Algorithms should use the
traits from `crdt-core` and should avoid inventing private equivalents of shared
concepts. The current implementation is `GCounter`.

Algorithm modules should be small enough to read locally, but complete enough to
show the intended API. Public APIs should make delivery assumptions and failure
modes visible. For example, `GCounter::increment_by` returns an overflow error
instead of silently wrapping a component.

### `crates/crdt-testkit`

`crdt-testkit` owns reusable checks. Its role is to prevent every algorithm from
rewriting the same law tests, history generators, and model comparison logic.
Today it contains deterministic law checks. Over time it should grow into:

- property-based law tests
- operation history generators
- network simulators
- reference model comparison helpers
- TLA+ trace replay utilities

### `proofs/lean`

Lean models algebraic structure. It is the right place for definitions such as
join-semilattice laws, convergence theorems for state-based CRDTs, and
algorithm-specific proofs that a model satisfies those laws.

Lean code should not mirror Rust implementation details such as ownership,
`BTreeMap` internals, serde formats, or error handling. It should model the math
that the Rust API commits to preserving.

### `proofs/tla`

TLA+ models distributed executions. It is the right place to explore message
ordering, delivery, duplication, drops, and bounded networks. TLA+ model checks
are powerful, but they are bounded checks unless a specification is separately
proved with TLAPS. Catalog pages must state checked bounds clearly.

The repository should use TLA+ terminology precisely:

- "model-checked by TLC" for finite-state exploration
- "model-checked by Apalache" for bounded symbolic checking when added
- "proved by TLAPS" only for mechanically checked TLA+ proofs
- "proved in Lean" only for checked Lean theorems

### `catalog`

`catalog/<algorithm>/` is the exhibit page for an algorithm. It connects the
Rust module, the Lean file, the TLA+ file, checked properties, examples,
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
pages, emits index pages, and writes `catalog.json` for the catalog browser.

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

pub struct Dot {
    pub actor: ActorId,
    pub counter: u64,
}

pub type VersionVector = BTreeMap<ActorId, u64>;
pub type DotSet = BTreeSet<Dot>;
```

Stable ordering makes examples reproducible and test failures readable. It also
keeps catalog pages honest because printed states do not change randomly between
runs.

## Design Constraints

Avoid these patterns unless there is a strong reason:

- making every CRDT a completely independent crate
- writing proof-like comments without executable tests or checked models
- calling bounded TLA+ model checking a proof
- leaving physical-clock assumptions vague for LWW algorithms
- omitting tombstone, causal context, or garbage-collection details for sets
- starting with sequence CRDTs before the core algebra is mature
- hiding network assumptions outside the algorithm page

## Current Skeleton

The current skeleton intentionally starts small:

```text
crates/
  crdt-core/
    src/lattice.rs
    src/state_based.rs
  crdt-algorithms/
    src/counters/gcounter.rs
  crdt-testkit/
    src/law_tests.rs

proofs/
  lean/Crdt/Algebra/JoinSemilattice.lean
  lean/Crdt/StateBased/CvRDT.lean
  lean/Crdt/Algorithms/GCounter.lean
  tla/modules/StateBasedCommon.tla
  tla/algorithms/gcounter/GCounter.tla
  tla/algorithms/gcounter/GCounter_MC.cfg

catalog/
  gcounter/
    algorithm.toml
    README.md
    spec.md
    examples.md
    proofs.md

docs/book/
  book.toml
  src/README.md
  theme/

tools/
  catalog-gen/
```

The next architectural step is not to add many CRDTs quickly. It is to make this
one path deeper: richer tests, a clearer reference model, stronger Lean
theorems, and more useful TLA+ invariants.
