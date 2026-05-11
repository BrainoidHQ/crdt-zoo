#![forbid(unsafe_code)]

//! Shared tests and assertions for CRDT implementations.

pub mod generators;
pub mod law_tests;
pub mod reference;

pub use generators::{small_actor_counts, small_actor_id, small_actor_set};
pub use law_tests::{
    assert_inflationary, check_join_semilattice_laws, prop_inflationary, prop_join_semilattice_laws,
};
pub use reference::{assert_query_matches, ReferenceModel};
