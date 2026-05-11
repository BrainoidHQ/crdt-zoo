#![forbid(unsafe_code)]

//! Core traits shared by CRDT implementations.

pub mod lattice;
pub mod state_based;

pub use lattice::{BoundedJoinSemilattice, JoinSemilattice};
pub use state_based::CvRDT;
