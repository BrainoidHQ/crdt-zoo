//! Reusable law checks for CRDT implementations.

use std::fmt::Debug;

use crdt_core::JoinSemilattice;

/// Checks the associativity, commutativity, and idempotence laws for `join`.
pub fn check_join_semilattice_laws<T>(a: &T, b: &T, c: &T)
where
    T: JoinSemilattice + Debug,
{
    assert_eq!(
        a.join(b).join(c),
        a.join(&b.join(c)),
        "join must be associative",
    );
    assert_eq!(a.join(b), b.join(a), "join must be commutative");
    assert_eq!(a.join(a), a.clone(), "join must be idempotent");
}

/// Checks that an update moved a state upward in the semilattice order.
pub fn assert_inflationary<T>(before: &T, after: &T)
where
    T: JoinSemilattice + Debug,
{
    assert!(
        before.leq(after),
        "updates must be inflationary: before={before:?}, after={after:?}",
    );
}

/// Checks the associativity, commutativity, and idempotence laws for `join`.
#[macro_export]
macro_rules! assert_join_semilattice_laws {
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::law_tests::check_join_semilattice_laws(&$a, &$b, &$c)
    };
}

#[cfg(test)]
mod tests {
    use crdt_core::JoinSemilattice;

    use super::check_join_semilattice_laws;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct BoolOr(bool);

    impl JoinSemilattice for BoolOr {
        fn join(&self, rhs: &Self) -> Self {
            Self(self.0 || rhs.0)
        }
    }

    #[test]
    fn bool_or_is_a_join_semilattice() {
        check_join_semilattice_laws(&BoolOr(false), &BoolOr(true), &BoolOr(false));
    }
}
