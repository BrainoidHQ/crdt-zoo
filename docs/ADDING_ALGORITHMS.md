# Adding Algorithms

Every algorithm should enter the repository as a catalog exhibit. The goal is to
make the implementation useful, the semantics inspectable, and the proof status
honest.

## Checklist

1. Create `catalog/<id>/algorithm.toml`.
2. Create `catalog/<id>/README.md`.
3. Create `catalog/<id>/spec.md`.
4. Create `catalog/<id>/examples.md`.
5. Create `catalog/<id>/proofs.md`.
6. Add the Rust implementation.
7. Add a reference model or document why it is deferred.
8. Add law tests.
9. Add a TLA+ model or document why it is deferred.
10. Add TLAPS proofs for tractable TLA+ theorems, or document why they are
    deferred.
11. Add TLC or Apalache configs with explicit bounds.
12. Add a Lean model or document why it is deferred.
13. Run `cargo run -p catalog-gen -- --check`.
14. Run `cargo test -p catalog-gen`.
15. Run `cargo run -p catalog-gen` and `mdbook build docs/book`.
16. Update the repository-level documentation when the new algorithm changes a
    family, shared abstraction, or phase status.

## Metadata Template

```toml
id = "gcounter"
slug = "gcounter"
name = "G-Counter"
long_name = "Grow-only Counter"
family = "counter"
difficulty = "beginner"
summary = "A state-based grow-only counter CRDT whose merge is component-wise maximum."
kind = ["state-based", "CvRDT"]
tags = ["counter", "join-semilattice", "beginner", "state-based"]

[docs]
readme = "README.md"
spec = "spec.md"
examples = "examples.md"
proofs = "proofs.md"

[rust]
crate = "crdt-algorithms"
module = "counters::gcounter"
type = "GCounter"
status = "implemented"
source = "../../crates/crdt-algorithms/src/counters/gcounter.rs"
api_docs_path = "rustdoc/crdt_algorithms/counters/gcounter/struct.GCounter.html"

[proofs.lean]
status = "proved"
file = "../../proofs/lean/Crdt/Algorithms/GCounter.lean"
theorems = [
  "Crdt.GCounter.incrementBy_inflationary",
  "Crdt.GCounter.increment_inflationary",
  "Crdt.GCounter.applyUpdate_inflationary",
  "Crdt.GCounter.merge_monotone",
  "Crdt.GCounter.merge_converges_for_same_states",
]

[proofs.tla]
status = "model-checked"
file = "../../proofs/tla/algorithms/gcounter/GCounter.tla"
model = "../../proofs/tla/algorithms/gcounter/GCounter_MC.cfg"
checked_bounds = "replicas = 2, counter components <= 2"

[delivery]
causal_required = false
duplicate_tolerant = true
drop_tolerant = true
exactly_once_required = false

[features]
serde = false
no_std = false
delta = false
tombstone_free = true

[properties]
laws = [
  "join is associative",
  "join is commutative",
  "join is idempotent",
  "increment is inflationary",
  "merge is monotone",
]
```

Use `status = "proved"` only after the listed Lean theorem names are checked by
`lake build`. If an algorithm has no Lean or TLA+ artifact yet, keep an empty
`[proofs]` table in `algorithm.toml` and describe the gap in `proofs.md`.

## Catalog Documentation Template

Every catalog exhibit is split into four Markdown files:

- `README.md`: overview, summary, delivery assumptions, and known constraints
- `spec.md`: state, operations, merge/effect, query, laws, and complexity
- `examples.md`: Rust API examples and small execution histories
- `proofs.md`: Rust, Lean, TLA+, law-test status, checked bounds, and gaps

Together they should answer these questions:

- What problem does this CRDT solve?
- Is it state-based, operation-based, delta-state, or hybrid?
- What is the state?
- What are the operations?
- What is `merge` or `effect`?
- What does `query` return?
- What are the delivery assumptions?
- What is the metadata size?
- Does it use tombstones?
- When is garbage collection safe?
- Which properties are proved?
- Which model-checking bounds were checked?
- What does the Rust API look like?
- What are the time and space costs?
- What are the known limitations?
- Which references are relevant?

The mdBook pages are generated from these files. Do not hand-edit generated
files under `docs/book/src/algorithms/`, `docs/book/src/indexes/`,
`docs/book/src/SUMMARY.md`, or `docs/book/src/assets/catalog.json`.

## Proof Status Language

Use precise language:

- "Lean proved" means checked by Lean.
- "TLAPS proved" means checked by TLAPS.
- "TLC checked for replicas <= 3" means bounded model checking.
- "Rust law-tested" means tested over the configured examples or generated
  inputs.
- "No proof yet" is acceptable and better than overstating confidence.

Avoid saying "proved" for a bounded model check.

## Delivery Assumptions

Every operation-based or delta-state algorithm must include a delivery table:

| Assumption | Required? | Notes |
| --- | --- | --- |
| Causal delivery | yes/no | Explain why. |
| Duplicate delivery tolerated | yes/no | Explain idempotence strategy. |
| Message drops tolerated | yes/no | Explain anti-entropy strategy. |
| Exactly-once delivery | yes/no | Avoid requiring this unless necessary. |

State-based CRDTs should still document whether repeated, reordered, or dropped
state messages affect convergence.

## Delete-Aware Algorithms

Algorithms that support deletion must document deletion as explicit metadata.
The catalog page should state:

- whether the visible conflict policy is add-wins, remove-wins,
  timestamp-based, or causally tracked
- what metadata records the remove, such as tombstones, remove timestamps,
  removed dots, version vectors, or causal contexts
- whether an element can be re-added after removal
- when delete metadata can be garbage-collected, or why collection is deferred
- what actor-id, replica-id, timestamp, or clock-skew assumptions the API makes

Prefer shared causal types from `crdt-core` over local equivalents. OR-Set-like
algorithms should use `ActorId`, `Dot`, `DotSet`, `VersionVector`, and
`CausalContext` unless there is a documented reason not to.

## Difficulty Levels

Use difficulty to guide readers:

- `beginner`: small state and obvious join, such as G-Counter or G-Set
- `intermediate`: deletes, timestamps, or causal metadata
- `advanced`: composition, garbage collection, or complex delivery assumptions
- `research`: sequence CRDTs, compaction-heavy designs, or incomplete proof
  strategy

## Review Questions

Before merging a new algorithm, ask:

- Can a reader find the Rust type from the catalog page?
- Can a reader tell which properties are actually checked?
- Are model-checking bounds visible?
- Are network assumptions visible?
- Are edge cases covered by tests?
- Are limitations stated before they surprise users?
