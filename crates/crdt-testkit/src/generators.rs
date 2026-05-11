//! Small reusable generators for property-based CRDT tests.

use std::collections::{BTreeMap, BTreeSet};

use crdt_core::Dot;
use proptest::prelude::*;

/// Generates a small deterministic actor id.
pub fn small_actor_id() -> impl Strategy<Value = String> {
    prop::sample::select(vec!["a", "b", "c", "d"]).prop_map(str::to_owned)
}

/// Generates a small actor-to-counter-component map.
///
/// Zero values are intentionally included because constructors should normalize
/// them when a CRDT treats absent and zero-valued components as equivalent.
pub fn small_actor_counts(
    max_count: u64,
    max_actors: usize,
) -> impl Strategy<Value = BTreeMap<String, u64>> {
    prop::collection::btree_map(small_actor_id(), 0..=max_count, 0..=max_actors)
}

/// Generates a small set of actor-like strings.
pub fn small_actor_set(max_len: usize) -> impl Strategy<Value = BTreeSet<String>> {
    prop::collection::btree_set(small_actor_id(), 0..=max_len)
}

/// Generates a small dot using the deterministic actor id generator.
pub fn small_dot(max_counter: u64) -> impl Strategy<Value = Dot<String>> {
    (small_actor_id(), 1..=max_counter.max(1)).prop_map(|(actor, counter)| Dot::new(actor, counter))
}

/// Generates a small deterministic dot set.
pub fn small_dot_set(
    max_counter: u64,
    max_len: usize,
) -> impl Strategy<Value = BTreeSet<Dot<String>>> {
    prop::collection::btree_set(small_dot(max_counter), 0..=max_len)
}
