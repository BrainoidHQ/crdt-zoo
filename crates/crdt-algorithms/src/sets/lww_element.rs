//! Last-writer-wins element set.

use std::collections::{BTreeMap, BTreeSet};

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// A timestamp-based state set with remove-wins ties.
///
/// Each element stores the greatest add timestamp and the greatest remove
/// timestamp observed for that element. An element is visible when its add
/// timestamp is strictly greater than its remove timestamp.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LWWElementSet<T, TS = u64> {
    adds: BTreeMap<T, TS>,
    removes: BTreeMap<T, TS>,
}

impl<T, TS> Default for LWWElementSet<T, TS> {
    fn default() -> Self {
        Self {
            adds: BTreeMap::new(),
            removes: BTreeMap::new(),
        }
    }
}

impl<T, TS> LWWElementSet<T, TS> {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns add timestamps by element.
    pub fn adds(&self) -> &BTreeMap<T, TS> {
        &self.adds
    }

    /// Returns remove timestamps by element.
    pub fn removes(&self) -> &BTreeMap<T, TS> {
        &self.removes
    }
}

impl<T, TS> LWWElementSet<T, TS>
where
    T: Ord,
{
    /// Creates a set from add and remove timestamp maps.
    pub fn from_timestamps(adds: BTreeMap<T, TS>, removes: BTreeMap<T, TS>) -> Self {
        Self { adds, removes }
    }

    /// Returns the greatest observed add timestamp for an element.
    pub fn add_timestamp(&self, element: &T) -> Option<&TS> {
        self.adds.get(element)
    }

    /// Returns the greatest observed remove timestamp for an element.
    pub fn remove_timestamp(&self, element: &T) -> Option<&TS> {
        self.removes.get(element)
    }
}

impl<T, TS> LWWElementSet<T, TS>
where
    T: Ord,
    TS: Ord,
{
    /// Adds an element at a timestamp.
    ///
    /// Older timestamps are ignored so the state never moves downward.
    pub fn add(&mut self, element: T, timestamp: TS) -> bool {
        insert_max(&mut self.adds, element, timestamp)
    }

    /// Removes an element at a timestamp.
    ///
    /// Remove timestamps may be recorded before an add for the same element is
    /// observed. Equal add and remove timestamps resolve to removed.
    pub fn remove(&mut self, element: T, timestamp: TS) -> bool {
        insert_max(&mut self.removes, element, timestamp)
    }

    /// Returns whether an element is currently visible.
    pub fn contains(&self, element: &T) -> bool {
        let Some(add_timestamp) = self.adds.get(element) else {
            return false;
        };

        match self.removes.get(element) {
            Some(remove_timestamp) => add_timestamp > remove_timestamp,
            None => true,
        }
    }

    /// Returns whether no elements are visible.
    pub fn is_empty(&self) -> bool {
        self.adds.keys().all(|element| !self.contains(element))
    }
}

impl<T, TS> LWWElementSet<T, TS>
where
    T: Clone + Ord,
    TS: Ord,
{
    /// Returns the visible elements.
    pub fn elements(&self) -> BTreeSet<T> {
        self.adds
            .keys()
            .filter(|element| self.contains(element))
            .cloned()
            .collect()
    }

    /// Returns the number of visible elements.
    pub fn len(&self) -> usize {
        self.elements().len()
    }
}

impl<T, TS> JoinSemilattice for LWWElementSet<T, TS>
where
    T: Clone + Ord,
    TS: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self {
            adds: join_max_maps(&self.adds, &rhs.adds),
            removes: join_max_maps(&self.removes, &rhs.removes),
        }
    }
}

impl<T, TS> BoundedJoinSemilattice for LWWElementSet<T, TS>
where
    T: Clone + Ord,
    TS: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<T, TS> CvRDT for LWWElementSet<T, TS>
where
    T: Clone + Ord,
    TS: Clone + Ord,
{
    type Query = BTreeSet<T>;

    fn query(&self) -> Self::Query {
        self.elements()
    }
}

fn insert_max<T, TS>(map: &mut BTreeMap<T, TS>, element: T, timestamp: TS) -> bool
where
    T: Ord,
    TS: Ord,
{
    match map.get_mut(&element) {
        Some(current) => {
            if timestamp.cmp(current).is_gt() {
                *current = timestamp;
                true
            } else {
                false
            }
        }
        None => {
            map.insert(element, timestamp);
            true
        }
    }
}

fn join_max_maps<T, TS>(left: &BTreeMap<T, TS>, right: &BTreeMap<T, TS>) -> BTreeMap<T, TS>
where
    T: Clone + Ord,
    TS: Clone + Ord,
{
    let mut joined = left.clone();

    for (element, timestamp) in right {
        insert_max(&mut joined, element.clone(), timestamp.clone());
    }

    joined
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crdt_core::{CvRDT, JoinSemilattice};
    use proptest::prelude::*;

    use super::LWWElementSet;

    fn set(adds: &[(&str, u64)], removes: &[(&str, u64)]) -> LWWElementSet<String> {
        LWWElementSet::from_timestamps(
            adds.iter()
                .map(|(element, timestamp)| ((*element).to_owned(), *timestamp))
                .collect(),
            removes
                .iter()
                .map(|(element, timestamp)| ((*element).to_owned(), *timestamp))
                .collect(),
        )
    }

    #[test]
    fn latest_timestamp_decides_visibility() {
        let mut set = LWWElementSet::new();

        set.add("a".to_owned(), 10);
        set.remove("a".to_owned(), 9);
        set.add("b".to_owned(), 3);
        set.remove("b".to_owned(), 4);

        assert!(set.contains(&"a".to_owned()));
        assert!(!set.contains(&"b".to_owned()));
        assert_eq!(set.query(), BTreeSet::from(["a".to_owned()]));
    }

    #[test]
    fn equal_timestamps_are_remove_wins() {
        let mut set = LWWElementSet::new();

        set.add("a".to_owned(), 7);
        set.remove("a".to_owned(), 7);

        assert!(!set.contains(&"a".to_owned()));
    }

    #[test]
    fn older_updates_are_ignored() {
        let mut set = set(&[("a", 10)], &[]);

        assert!(!set.add("a".to_owned(), 8));
        assert_eq!(set.add_timestamp(&"a".to_owned()), Some(&10));
    }

    #[test]
    fn merge_keeps_maximum_timestamps() {
        let left = set(&[("a", 5), ("b", 2)], &[("a", 1)]);
        let right = set(&[("a", 3)], &[("b", 4)]);

        let joined = left.join(&right);

        assert_eq!(joined.add_timestamp(&"a".to_owned()), Some(&5));
        assert_eq!(joined.remove_timestamp(&"b".to_owned()), Some(&4));
        assert_eq!(joined.query(), BTreeSet::from(["a".to_owned()]));
    }

    #[test]
    fn add_and_remove_are_inflationary() {
        let before = set(&[("a", 1)], &[]);

        let mut added = before.clone();
        added.add("a".to_owned(), 2);
        crdt_testkit::assert_inflationary(&before, &added);

        let mut removed = before.clone();
        removed.remove("a".to_owned(), 3);
        crdt_testkit::assert_inflationary(&before, &removed);
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a_adds in crdt_testkit::small_actor_counts(6, 4),
            a_removes in crdt_testkit::small_actor_counts(6, 4),
            b_adds in crdt_testkit::small_actor_counts(6, 4),
            b_removes in crdt_testkit::small_actor_counts(6, 4),
            c_adds in crdt_testkit::small_actor_counts(6, 4),
            c_removes in crdt_testkit::small_actor_counts(6, 4),
        ) {
            crdt_testkit::prop_join_semilattice_laws(
                LWWElementSet::from_timestamps(a_adds, a_removes),
                LWWElementSet::from_timestamps(b_adds, b_removes),
                LWWElementSet::from_timestamps(c_adds, c_removes),
            )?;
        }
    }
}
