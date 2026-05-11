//! Small reusable generators for property-based CRDT tests.

use std::collections::{BTreeMap, BTreeSet};

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
