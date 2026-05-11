//! Grow-only set.

use std::collections::BTreeSet;

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// A state-based grow-only set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GSet<T> {
    elements: BTreeSet<T>,
}

impl<T> Default for GSet<T> {
    fn default() -> Self {
        Self {
            elements: BTreeSet::new(),
        }
    }
}

impl<T> GSet<T> {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the elements.
    pub fn elements(&self) -> &BTreeSet<T> {
        &self.elements
    }
}

impl<T> GSet<T>
where
    T: Ord,
{
    /// Creates a set from elements.
    pub fn from_elements(elements: BTreeSet<T>) -> Self {
        Self { elements }
    }

    /// Adds an element, returning whether it was newly inserted.
    pub fn add(&mut self, element: T) -> bool {
        self.elements.insert(element)
    }

    /// Returns whether the set contains an element.
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

impl<T> JoinSemilattice for GSet<T>
where
    T: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self {
            elements: self.elements.union(&rhs.elements).cloned().collect(),
        }
    }
}

impl<T> BoundedJoinSemilattice for GSet<T>
where
    T: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<T> CvRDT for GSet<T>
where
    T: Clone + Ord,
{
    type Query = BTreeSet<T>;

    fn query(&self) -> Self::Query {
        self.elements.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crdt_core::{CvRDT, JoinSemilattice};
    use proptest::prelude::*;

    use super::GSet;

    #[test]
    fn add_is_inflationary() {
        let before = GSet::from_elements(BTreeSet::from(["a".to_owned()]));
        let mut after = before.clone();

        after.add("b".to_owned());

        crdt_testkit::assert_inflationary(&before, &after);
    }

    #[test]
    fn merge_uses_union() {
        let left = GSet::from_elements(BTreeSet::from(["a".to_owned(), "b".to_owned()]));
        let right = GSet::from_elements(BTreeSet::from(["b".to_owned(), "c".to_owned()]));

        assert_eq!(
            left.join(&right).query(),
            BTreeSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()])
        );
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a in crdt_testkit::small_actor_set(4),
            b in crdt_testkit::small_actor_set(4),
            c in crdt_testkit::small_actor_set(4),
        ) {
            crdt_testkit::prop_join_semilattice_laws(
                GSet::from_elements(a),
                GSet::from_elements(b),
                GSet::from_elements(c),
            )?;
        }
    }
}
