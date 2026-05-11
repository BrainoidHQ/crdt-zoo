//! Two-phase set.

use std::collections::BTreeSet;

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// A state-based set where removal permanently tombstones an element.
///
/// A 2P-Set stores grow-only add and remove sets. An element is visible exactly
/// when it has been added and has not been removed. Once an element appears in
/// the remove set, later local `add` calls for the same element have no effect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwoPhaseSet<T> {
    adds: BTreeSet<T>,
    removes: BTreeSet<T>,
}

impl<T> Default for TwoPhaseSet<T> {
    fn default() -> Self {
        Self {
            adds: BTreeSet::new(),
            removes: BTreeSet::new(),
        }
    }
}

impl<T> TwoPhaseSet<T> {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the grow-only add set.
    pub fn adds(&self) -> &BTreeSet<T> {
        &self.adds
    }

    /// Returns the grow-only remove tombstone set.
    pub fn removes(&self) -> &BTreeSet<T> {
        &self.removes
    }
}

impl<T> TwoPhaseSet<T>
where
    T: Ord,
{
    /// Creates a set from add and remove components.
    pub fn from_parts(adds: BTreeSet<T>, removes: BTreeSet<T>) -> Self {
        Self { adds, removes }
    }

    /// Adds an element unless it has already been tombstoned.
    ///
    /// Returns whether the visible add component changed.
    pub fn add(&mut self, element: T) -> bool {
        if self.removes.contains(&element) {
            return false;
        }

        self.adds.insert(element)
    }

    /// Returns whether an element is currently visible.
    pub fn contains(&self, element: &T) -> bool {
        self.adds.contains(element) && !self.removes.contains(element)
    }
}

impl<T> TwoPhaseSet<T>
where
    T: Clone + Ord,
{
    /// Tombstones an element.
    ///
    /// The element does not need to be locally visible. Removing an absent
    /// element still records a tombstone that will suppress matching adds after
    /// merge.
    pub fn remove(&mut self, element: &T) -> bool {
        self.removes.insert(element.clone())
    }

    /// Returns the visible elements.
    pub fn elements(&self) -> BTreeSet<T> {
        self.adds.difference(&self.removes).cloned().collect()
    }

    /// Returns the number of visible elements.
    pub fn len(&self) -> usize {
        self.elements().len()
    }

    /// Returns whether no elements are visible.
    pub fn is_empty(&self) -> bool {
        self.adds.is_subset(&self.removes)
    }
}

impl<T> JoinSemilattice for TwoPhaseSet<T>
where
    T: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self {
            adds: self.adds.union(&rhs.adds).cloned().collect(),
            removes: self.removes.union(&rhs.removes).cloned().collect(),
        }
    }
}

impl<T> BoundedJoinSemilattice for TwoPhaseSet<T>
where
    T: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<T> CvRDT for TwoPhaseSet<T>
where
    T: Clone + Ord,
{
    type Query = BTreeSet<T>;

    fn query(&self) -> Self::Query {
        self.elements()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crdt_core::{CvRDT, JoinSemilattice};
    use proptest::prelude::*;

    use super::TwoPhaseSet;

    fn set(adds: &[&str], removes: &[&str]) -> TwoPhaseSet<String> {
        TwoPhaseSet::from_parts(
            adds.iter().map(|element| (*element).to_owned()).collect(),
            removes
                .iter()
                .map(|element| (*element).to_owned())
                .collect(),
        )
    }

    #[test]
    fn removed_elements_are_not_visible() {
        let mut set = TwoPhaseSet::new();

        set.add("a".to_owned());
        set.add("b".to_owned());
        set.remove(&"a".to_owned());

        assert!(!set.contains(&"a".to_owned()));
        assert!(set.contains(&"b".to_owned()));
        assert_eq!(set.query(), BTreeSet::from(["b".to_owned()]));
    }

    #[test]
    fn tombstone_prevents_readding() {
        let mut set = TwoPhaseSet::new();

        set.add("a".to_owned());
        set.remove(&"a".to_owned());

        assert!(!set.add("a".to_owned()));
        assert!(!set.contains(&"a".to_owned()));
    }

    #[test]
    fn merge_unions_both_components() {
        let left = set(&["a", "b"], &["a"]);
        let right = set(&["a", "c"], &["c"]);

        let joined = left.join(&right);

        assert_eq!(
            joined.adds(),
            &BTreeSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()])
        );
        assert_eq!(
            joined.removes(),
            &BTreeSet::from(["a".to_owned(), "c".to_owned()])
        );
        assert_eq!(joined.query(), BTreeSet::from(["b".to_owned()]));
    }

    #[test]
    fn add_and_remove_are_inflationary() {
        let before = set(&["a"], &[]);

        let mut added = before.clone();
        added.add("b".to_owned());
        crdt_testkit::assert_inflationary(&before, &added);

        let mut removed = before.clone();
        removed.remove(&"a".to_owned());
        crdt_testkit::assert_inflationary(&before, &removed);
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a_adds in crdt_testkit::small_actor_set(4),
            a_removes in crdt_testkit::small_actor_set(4),
            b_adds in crdt_testkit::small_actor_set(4),
            b_removes in crdt_testkit::small_actor_set(4),
            c_adds in crdt_testkit::small_actor_set(4),
            c_removes in crdt_testkit::small_actor_set(4),
        ) {
            crdt_testkit::prop_join_semilattice_laws(
                TwoPhaseSet::from_parts(a_adds, a_removes),
                TwoPhaseSet::from_parts(b_adds, b_removes),
                TwoPhaseSet::from_parts(c_adds, c_removes),
            )?;
        }
    }
}
