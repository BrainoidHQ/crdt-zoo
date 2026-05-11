# CRDT Zoo

CRDT Zoo is a catalog of Conflict-free Replicated Data Types. Each exhibit links
the human-facing explanation in `catalog/`, the Rust implementation, the Lean
proof status, and the TLA+ model-checking status.

The current catalog covers algebraically small state-based CRDTs and the first
delete-aware set CRDTs. The delete-aware entries make the conflict policy
explicit: permanent remove-wins tombstones for 2P-Set, timestamp-based
resolution for LWW-Element-Set, and causally tracked add-wins removes for
OR-Set.

## Catalog Browser

<div class="catalog-browser" data-catalog-browser>
  <p>Run <code>cargo run -p catalog-gen</code> to generate the local catalog browser data.</p>
</div>

## Indexes

- [By family](indexes/by-family.md)
- [By kind](indexes/by-kind.md)
- [By proof status](indexes/by-proof-status.md)
- [By difficulty](indexes/by-difficulty.md)

## Reading Paths

Start with the small semilattices and counters:

1. [Bool OR](algorithms/bool-or.md)
2. [G-Counter](algorithms/gcounter.md)
3. [G-Set](algorithms/gset.md)
4. [Max Register](algorithms/max-register.md)
5. [PN-Counter](algorithms/pncounter.md)

Then compare the delete-aware set designs:

1. [2P-Set](algorithms/two-phase-set.md)
2. [LWW-Element-Set](algorithms/lww-element-set.md)
3. [OR-Set](algorithms/orset.md)

## Source Of Truth

The generated book pages come from `catalog/<algorithm>/`. Edit the catalog
source files first, then regenerate the book source.
