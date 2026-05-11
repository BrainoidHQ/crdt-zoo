//! Traits for state-based CRDTs.

use crate::JoinSemilattice;

/// A convergent, state-based CRDT.
///
/// `merge` is the semilattice join. Implementations must ensure local updates
/// are inflationary with respect to [`JoinSemilattice::leq`].
pub trait CvRDT: JoinSemilattice {
    /// The externally visible query result.
    type Query;

    /// Reads the observable value of this replica state.
    fn query(&self) -> Self::Query;

    /// Merges another replica state into this one.
    fn merge(&mut self, rhs: &Self) {
        *self = self.join(rhs);
    }
}
