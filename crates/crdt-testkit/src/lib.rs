#![forbid(unsafe_code)]

//! Shared tests and assertions for CRDT implementations.

pub mod law_tests;

pub use law_tests::{assert_inflationary, check_join_semilattice_laws};
