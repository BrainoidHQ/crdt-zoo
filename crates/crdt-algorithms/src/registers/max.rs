//! Max register.

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// A state-based register that keeps the maximum assigned value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaxRegister<T> {
    value: Option<T>,
}

impl<T> Default for MaxRegister<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> MaxRegister<T> {
    /// Creates an empty register.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a register containing one value.
    pub fn with_value(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Returns the current value.
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Consumes the register and returns the current value.
    pub fn into_value(self) -> Option<T> {
        self.value
    }
}

impl<T> MaxRegister<T>
where
    T: Ord,
{
    /// Assigns a value, keeping it only when it is at least the current value.
    pub fn assign(&mut self, value: T) {
        match &self.value {
            Some(current) if current >= &value => {}
            _ => self.value = Some(value),
        }
    }
}

impl<T> JoinSemilattice for MaxRegister<T>
where
    T: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        match (&self.value, &rhs.value) {
            (Some(left), Some(right)) if left >= right => Self::with_value(left.clone()),
            (Some(_), Some(right)) => Self::with_value(right.clone()),
            (Some(left), None) => Self::with_value(left.clone()),
            (None, Some(right)) => Self::with_value(right.clone()),
            (None, None) => Self::new(),
        }
    }
}

impl<T> BoundedJoinSemilattice for MaxRegister<T>
where
    T: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<T> CvRDT for MaxRegister<T>
where
    T: Clone + Ord,
{
    type Query = Option<T>;

    fn query(&self) -> Self::Query {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use crdt_core::{CvRDT, JoinSemilattice};
    use proptest::prelude::*;

    use super::MaxRegister;

    #[test]
    fn assign_keeps_the_maximum_value() {
        let mut register = MaxRegister::new();

        register.assign(7);
        register.assign(3);
        register.assign(9);

        assert_eq!(register.query(), Some(9));
    }

    #[test]
    fn merge_keeps_the_maximum_value() {
        let left = MaxRegister::with_value(2);
        let right = MaxRegister::with_value(5);

        assert_eq!(left.join(&right).query(), Some(5));
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a in prop::option::of(-20i32..=20),
            b in prop::option::of(-20i32..=20),
            c in prop::option::of(-20i32..=20),
        ) {
            crdt_testkit::prop_join_semilattice_laws(
                MaxRegister { value: a },
                MaxRegister { value: b },
                MaxRegister { value: c },
            )?;
        }

        #[test]
        fn assign_is_inflationary(before in prop::option::of(-20i32..=20), update in -20i32..=20) {
            let before = MaxRegister { value: before };
            let mut after = before.clone();

            after.assign(update);

            crdt_testkit::prop_inflationary(&before, &after)?;
        }
    }
}
