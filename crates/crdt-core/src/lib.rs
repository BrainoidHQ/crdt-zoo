#![forbid(unsafe_code)]

//! Core traits shared by CRDT implementations.

pub mod causal;
pub mod ids;
pub mod lattice;
pub mod state_based;

pub use causal::{CausalContext, Dot, DotOverflow, DotSet, VersionVector};
pub use ids::{ActorId, ReplicaId};
pub use lattice::{BoundedJoinSemilattice, JoinSemilattice};
pub use state_based::CvRDT;
