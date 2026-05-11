# CRDT Zoo

CRDT Zoo is a catalog of Conflict-free Replicated Data Types. Each exhibit links
the human-facing explanation in `catalog/`, the Rust implementation, the Lean
proof status, and the TLA+ model-checking status.

## Catalog Browser

<div class="catalog-browser" data-catalog-browser>
  <p>Run <code>cargo run -p catalog-gen</code> to generate the local catalog browser data.</p>
</div>

## Indexes

- [By family](indexes/by-family.md)
- [By kind](indexes/by-kind.md)
- [By proof status](indexes/by-proof-status.md)
- [By difficulty](indexes/by-difficulty.md)

## Beginner Path

1. [G-Counter](algorithms/gcounter.md)

## Source Of Truth

The generated book pages come from `catalog/<algorithm>/`. Edit the catalog
source files first, then regenerate the book source.
