# Adding Algorithms

Every algorithm should enter the repository as a catalog exhibit. The goal is to
make the implementation useful, the semantics inspectable, and the proof status
honest.

## Checklist

1. Create `catalog/<id>/algorithm.toml`.
2. Create `catalog/<id>/README.md`.
3. Add the Rust implementation.
4. Add a reference model or document why it is deferred.
5. Add law tests.
6. Add a TLA+ model or document why it is deferred.
7. Add TLC or Apalache configs with explicit bounds.
8. Add a Lean model or document why it is deferred.
9. Register the algorithm in docs or generated catalog indexes.
10. Run the relevant checks.

## Metadata Template

```toml
id = "g-counter"
name = "Grow-only Counter"
family = "counter"
kind = ["state-based", "CvRDT"]
difficulty = "beginner"
status = "initial"

[rust]
crate = "crdt-algorithms"
module = "counters::gcounter"
type = "GCounter"

[proofs.lean]
file = "../../proofs/lean/Crdt/Algorithms/GCounter.lean"
theorems = [
  "Crdt.GCounter.increment_inflationary",
]

[proofs.tla]
file = "../../proofs/tla/algorithms/gcounter/GCounter.tla"
model = "../../proofs/tla/algorithms/gcounter/GCounter_MC.cfg"
checked_bounds = "replicas = 2, counter components <= 2"

[properties]
laws = [
  "join is associative",
  "join is commutative",
  "join is idempotent",
  "update is inflationary",
]
```

## Catalog README Template

Every catalog README should answer these questions:

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

## Proof Status Language

Use precise language:

- "Lean proved" means checked by Lean.
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
