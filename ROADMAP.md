# Roadmap

The repository should grow from algebraically simple CRDTs toward algorithms
with causal metadata, garbage collection, and eventually sequence structures.
Each phase should leave behind better shared infrastructure, not only more
algorithm files.

## Phase 0: Initial Skeleton

Status: ✓ complete.

Goals:

- establish the Cargo workspace
- define `JoinSemilattice` and `CvRDT`
- implement G-Counter
- add reusable law checks
- add a minimal Lean model
- add a minimal TLA+ model and TLC config
- create the first catalog page
- provide a Nix dev shell

Exit criteria:

- `cargo test --workspace` passes
- `lake build` passes for Lean proofs
- TLC runs for the G-Counter config
- the G-Counter catalog page states proof and model-checking status accurately

## Phase 1: Semilattice Basics

Focus on CRDTs where the algebra is small and visible.

Planned items:

- Bool OR lattice
- Max register
- G-Counter hardening
- PN-Counter
- G-Set

Infrastructure to build:

- property-based law tests
- reusable arbitrary state generators
- a reference-model pattern
- more complete Lean convergence theorem for state-based CRDTs
- a reusable TLA+ network module
- CI jobs for Rust, Lean, and TLC

Expected outcome:

The repository should make it clear how to prove and test a simple CvRDT before
introducing deletes or causal metadata.

## Phase 2: Sets With Deletes

Focus on sets where removal needs explicit semantics.

Planned items:

- 2P-Set
- LWW-Element-Set
- OR-Set

Infrastructure to build:

- actor ids and replica ids
- dots and dot sets
- version vectors
- causal contexts
- delivery-assumption sections in every catalog page
- tombstone and garbage-collection documentation

Expected outcome:

The repository should explain why deletes are not just inverse adds. Each set
must state whether it is add-wins, remove-wins, timestamp-based, or causally
tracked.

## Phase 3: Registers And Maps

Focus on compositional CRDTs.

Planned items:

- LWW-Register
- MV-Register
- OR-Map
- Add-wins Map
- Remove-wins Map

Infrastructure to build:

- composition rules for nested CRDTs
- map-level model tests
- Lean abstractions for finite maps where practical
- clear timestamp tie-break and clock-skew policies for LWW types

Expected outcome:

The repository should show how smaller CRDTs combine into larger data types
without hiding conflict semantics.

## Phase 4: Delta-State CRDTs

Focus on efficient synchronization while preserving state-based convergence.

Planned items:

- Delta G-Counter
- Delta PN-Counter
- Delta OR-Set
- Delta OR-Map

Infrastructure to build:

- `DeltaCvRDT` trait
- delta law tests
- delta interval and anti-entropy examples
- benchmarks for full-state merge versus delta merge
- model checks that include lost, duplicated, or delayed deltas

Expected outcome:

The repository should show how delta-state CRDTs reduce payload size without
switching to the stronger delivery assumptions often required by operation-based
CRDTs.

## Phase 5: Sequence CRDTs

Focus on ordered collections and collaborative editing structures.

Planned items:

- RGA
- Logoot or LSEQ
- Treedoc-style structures

Infrastructure to build:

- position identifiers
- insertion and deletion histories
- richer trace replay
- benchmarks with realistic editing patterns
- careful documentation of metadata growth and compaction options

Expected outcome:

Sequence CRDTs should be added only after the repository can already explain,
test, and model causality-heavy set and map CRDTs.

## Cross-Cutting Milestones

These milestones apply across phases:

- CI for Rust formatting, clippy, tests, docs, Lean, and selected TLC configs
- generated catalog index from `algorithm.toml`
- consistent proof-status badges
- reproducible trace replay from TLA+ counterexamples
- optional fuzzing for serialization and delivery order
- scheduled benchmarks for selected algorithms
- `no_std` and serde policies once the public API stabilizes

## Priority Rule

Prefer one unusually complete exhibit over many shallow implementations. A new
CRDT should improve the shared test, proof, or documentation system whenever it
reveals a missing abstraction.
