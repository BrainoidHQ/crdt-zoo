# Verification Strategy

CRDT Zoo uses several kinds of evidence because no single tool covers the whole
repository boundary. Lean proves algebraic facts about mathematical models. TLA+
explores distributed executions. Rust tests check the implementation. Reference
models, trace replay, and fuzzing connect those worlds.

## What Counts As Evidence

| Evidence | What it supports | What it does not prove |
| --- | --- | --- |
| Rust unit tests | Concrete API behavior and regressions | General CRDT correctness |
| Rust law tests | Algebraic laws over tested inputs | Laws for all possible values |
| Property tests | Laws over many generated inputs | Exhaustive correctness |
| Reference models | Agreement with a simpler model | That the reference model is complete |
| TLA+ with TLAPS | Machine-checked TLA+ theorems | That unchecked temporal properties also hold |
| TLA+ with TLC | Bounded distributed executions | Unbounded correctness |
| TLA+ with Apalache | Bounded symbolic checks | Unbounded correctness |
| Lean | Machine-checked mathematical theorems | That Rust code exactly implements the model |
| Trace replay | Rust behavior on model-generated histories | Coverage beyond generated traces |
| Fuzzing | Robustness against unusual inputs | Semantic correctness by itself |

Catalog pages should describe these categories precisely.

## State-Based CRDT Checks

For a state-based CRDT, the core laws are:

```text
join is associative
join is commutative
join is idempotent
update is inflationary
merge is monotone
```

In Rust, these become reusable law checks in `crdt-testkit`. The long-term goal
is to keep those checks property-based for every algorithm that has an input
generator. The reusable generators now cover actor ids, component maps,
actor-like sets, dots, and dot sets.

In Lean, the target is a general theorem: if replica states are joins of the
updates they have observed, then replicas with the same observed updates
converge. The current Lean model includes the supporting semilattice order facts
and a generic convergence theorem for states that are mutually below the same
observed join.

In TLA+, TLAPS should prove small algebraic or inductive facts when the proof is
maintainable. TLC remains the target for bounded exploration of local updates,
message send, message delivery, reordering, duplication, and eventually selected
drop behaviors.

## Delete-Aware Set Checks

Sets with deletes need checks beyond ordinary set membership examples:

- the state merge must still be associative, commutative, and idempotent
- each add and remove operation must be inflationary in the semilattice order
- remove must not be modeled as subtracting local state without metadata
- 2P-Set histories should show that tombstones prevent re-add
- LWW-Element-Set histories should cover older timestamps and equal-timestamp
  tie-breaks
- OR-Set histories should cover observed remove and concurrent add-wins merge

For OR-Set, property tests should generate valid states where every entry dot is
represented in the causal context. This keeps generated states close to the
invariant that production updates maintain.

## Operation-Based CRDT Checks

For an operation-based CRDT, the key concern is commuting concurrent operations
under the required delivery assumptions.

Catalog pages must state:

- causal delivery required: yes or no
- duplicate tolerant: yes or no
- drop tolerant: yes or no
- exactly-once required: yes or no

Tests should include histories where concurrent operations are applied in
different valid orders. TLA+ should model the delivery layer assumptions instead
of leaving them implicit.

## Reference Models

Each non-trivial CRDT should eventually have a reference model. The reference
model can be slower or less memory efficient than the production implementation.
Its job is to be simple enough to inspect.

The test pattern is:

```text
generate a history
apply it to the implementation
apply it to the reference model
compare query results and important invariants
```

Reference models are especially useful for OR-Set, maps, and sequence CRDTs
where the optimized representation can obscure the intended semantics.

## Trace Replay

TLA+ counterexamples should become Rust regression tests whenever practical.
The intended workflow is:

```text
TLA+ trace:
  r1 inc
  r2 inc
  deliver r1 -> r2
  deliver r2 -> r1

Rust replay:
  apply the same actions
  assert final states and queries
```

Trace replay is a bridge between specification exploration and implementation
regression testing.

## Fuzzing

Fuzzing is useful for boundaries that are hard to cover with handwritten tests:

- serialization and deserialization
- corrupted inputs
- unusual actor ids
- delta delivery order
- duplicate messages
- very large histories

Fuzzing should complement law tests and model checks. It should not replace a
clear specification.

## CI Targets

The intended CI matrix is:

```text
Rust:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo doc --workspace --no-deps

Lean:
  cd proofs/lean
  lake build

TLA+:
  run TLAPS for selected mechanized TLA+ proofs
  run TLC for selected small configs
  run Apalache for selected bounded checks once added

Optional:
  cargo +nightly miri test
  cargo fuzz smoke test
  scheduled benchmarks
```

Benchmarks should not gate every change, but they are valuable for detecting
regressions in metadata-heavy algorithms.
