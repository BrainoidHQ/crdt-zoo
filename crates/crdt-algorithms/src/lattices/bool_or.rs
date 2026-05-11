//! Boolean OR lattice.

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// A state-based boolean flag whose merge is logical OR.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BoolOr(bool);

impl BoolOr {
    /// Creates a flag with the given value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the current flag value.
    pub fn value(self) -> bool {
        self.0
    }

    /// Sets the flag to true.
    pub fn enable(&mut self) {
        self.0 = true;
    }
}

impl JoinSemilattice for BoolOr {
    fn join(&self, rhs: &Self) -> Self {
        Self(self.0 || rhs.0)
    }
}

impl BoundedJoinSemilattice for BoolOr {
    fn bottom() -> Self {
        Self(false)
    }
}

impl CvRDT for BoolOr {
    type Query = bool;

    fn query(&self) -> Self::Query {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crdt_core::{CvRDT, JoinSemilattice};
    use proptest::prelude::*;

    use super::BoolOr;

    #[test]
    fn enable_is_inflationary() {
        let before = BoolOr::new(false);
        let mut after = before;

        after.enable();

        crdt_testkit::assert_inflationary(&before, &after);
        assert!(after.query());
    }

    #[test]
    fn merge_uses_logical_or() {
        assert!(BoolOr::new(false).join(&BoolOr::new(true)).value());
        assert!(!BoolOr::new(false).join(&BoolOr::new(false)).value());
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(a in any::<bool>(), b in any::<bool>(), c in any::<bool>()) {
            crdt_testkit::prop_join_semilattice_laws(
                BoolOr::new(a),
                BoolOr::new(b),
                BoolOr::new(c),
            )?;
        }
    }
}
