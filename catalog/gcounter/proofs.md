# Proofs And Checks

## Verification Status

| Artifact | Location | Status |
| --- | --- | --- |
| Rust implementation | `crates/crdt-algorithms/src/counters/gcounter.rs` | Implemented |
| Law tests | `crates/crdt-testkit/src/law_tests.rs` | Example-based and property-based checks |
| Reference model | `crates/crdt-algorithms/src/counters/gcounter.rs` tests | Query agreement check |
| Lean model | `proofs/lean/Crdt/Algorithms/GCounter.lean` | Proved |
| TLA+ model | `proofs/tla/algorithms/gcounter/GCounter.tla` | TLAPS join-law and distributed-invariant proofs; TLC model check |
| Catalog metadata | `catalog/gcounter/algorithm.toml` | Present |

## Lean

Current checked theorem names:

- `Crdt.GCounter.increment_inflationary`
- `Crdt.GCounter.incrementBy_inflationary`
- `Crdt.GCounter.applyUpdate_inflationary`
- `Crdt.GCounter.merge_monotone`
- `Crdt.GCounter.merge_converges_for_same_states`

The Lean file covers core inflationary-update, monotone merge, and convergence
facts for the mathematical model. The update model matches the `increment_by`
shape by increasing the actor component by an arbitrary natural amount and is
packaged as a `CvRDT` instance.

The shared Lean `CvRDT` model also includes a generic theorem for replicas that
are mutually below the same observed join.

## TLA+

The TLA+ module contains `GCounterJoinLaws`, a TLAPS-checked theorem for the
component-wise maximum join laws and merge order facts:

- idempotence
- commutativity
- associativity
- inflationary merge
- monotone merge

The TLA+ module also contains TLAPS-checked distributed-invariant obligations:

- `PointwiseMaxType`: merge of two bounded states remains a bounded state.
- `PointwiseMaxUpperBound`: merging two states below the same upper bound
  remains below that bound.
- `GCounterDistributedInvariantInductive`: initialization establishes the
  distributed invariant, and every `Inc`, `SendState`, `Deliver`, or stuttering
  transition preserves it.

The distributed invariant covers:

- type correctness of replica states and network messages
- no phantom increments, meaning every observed actor component is bounded by
  that actor's owner component
- message payloads that do not exceed the owning replica's component

TLC is still run as a bounded executable model check.

Current TLC bounds:

```text
replicas = 2
counter components <= 2
```

These bounds are intentionally small. They are useful for checking that the
model is executable and that the distributed transitions preserve the checked
G-Counter safety properties.

The TLC configuration checks:

- `GCounterCorrectness`: type correctness, component-wise max idempotence,
  commutativity, associativity, inflationary merge, monotone merge, no phantom
  increments, and message payloads that do not exceed the owning replica's
  component.
- `StateMonotonic`: every replica component is non-decreasing across every
  local increment, send, delivery, or stuttering step.

## Gaps

- Broaden property generators beyond small actor maps.
- Connect the total-function Lean model to finite-map extraction/query details.
