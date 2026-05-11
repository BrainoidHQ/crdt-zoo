//! Algebraic structures used to define state-based CRDTs.

/// A join-semilattice.
///
/// Implementors must satisfy these laws for all values `a`, `b`, and `c`:
///
/// - associativity: `a.join(&b).join(&c) == a.join(&b.join(&c))`
/// - commutativity: `a.join(&b) == b.join(&a)`
/// - idempotence: `a.join(&a) == a`
///
/// The induced partial order is `a <= b` exactly when `a.join(&b) == b`.
pub trait JoinSemilattice: Clone + PartialEq {
    /// Returns the least upper bound of `self` and `rhs`.
    fn join(&self, rhs: &Self) -> Self;

    /// Returns whether `self` is below or equal to `rhs` in the induced order.
    fn leq(&self, rhs: &Self) -> bool {
        self.join(rhs) == rhs.clone()
    }
}

/// A join-semilattice with a least element.
pub trait BoundedJoinSemilattice: JoinSemilattice {
    /// Returns the least element.
    fn bottom() -> Self;
}
