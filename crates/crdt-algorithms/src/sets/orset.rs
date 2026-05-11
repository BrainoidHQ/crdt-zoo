//! Observed-remove set.

use std::collections::{BTreeMap, BTreeSet};

use crdt_core::{
    ActorId, BoundedJoinSemilattice, CausalContext, CvRDT, Dot, DotOverflow, DotSet,
    JoinSemilattice,
};

/// A state-based add-wins observed-remove set.
///
/// Adds allocate dots. Removes clear only the dots the replica has observed for
/// an element. During merge, dots missing from a replica are removed only when
/// that replica's causal context proves it had observed them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ORSet<T, A = ActorId> {
    entries: BTreeMap<T, DotSet<A>>,
    context: CausalContext<A>,
}

impl<T, A> Default for ORSet<T, A> {
    fn default() -> Self {
        Self {
            entries: BTreeMap::new(),
            context: CausalContext::new(),
        }
    }
}

impl<T, A> ORSet<T, A> {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns element dots.
    pub fn entries(&self) -> &BTreeMap<T, DotSet<A>> {
        &self.entries
    }

    /// Returns the causal context.
    pub fn context(&self) -> &CausalContext<A> {
        &self.context
    }
}

impl<T, A> ORSet<T, A>
where
    T: Ord,
{
    /// Returns the dots currently making an element visible.
    pub fn dots(&self, element: &T) -> Option<&DotSet<A>> {
        self.entries.get(element)
    }

    /// Returns whether an element is currently visible.
    pub fn contains(&self, element: &T) -> bool {
        self.entries.contains_key(element)
    }

    /// Returns whether no elements are visible.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of visible elements.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T, A> ORSet<T, A>
where
    A: Clone + Ord,
    T: Ord,
{
    /// Creates a set from element dots and a causal context.
    ///
    /// Entry dots are always observed into the context, preserving the OR-Set
    /// invariant required by merge.
    pub fn from_entries(
        mut entries: BTreeMap<T, DotSet<A>>,
        mut context: CausalContext<A>,
    ) -> Self {
        entries.retain(|_, dots| !dots.is_empty());
        let observed = entries
            .values()
            .flat_map(|dots| dots.iter().cloned())
            .collect::<Vec<_>>();
        context.observe_all(observed);

        Self { entries, context }
    }

    /// Adds an element and returns the allocated dot.
    pub fn add(&mut self, actor: A, element: T) -> Result<Dot<A>, DotOverflow> {
        let dot = self.context.advance(actor)?;
        self.entries.entry(element).or_default().insert(dot.clone());
        Ok(dot)
    }

    /// Removes the dots currently observed for an element.
    ///
    /// Concurrent adds allocate dots that are not in this replica's context, so
    /// those dots survive merge.
    pub fn remove(&mut self, element: &T) -> bool {
        self.entries.remove(element).is_some()
    }
}

impl<T, A> ORSet<T, A>
where
    T: Clone + Ord,
{
    /// Returns the visible elements.
    pub fn elements(&self) -> BTreeSet<T> {
        self.entries.keys().cloned().collect()
    }
}

impl<T, A> JoinSemilattice for ORSet<T, A>
where
    A: Clone + Ord,
    T: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        let mut entries = BTreeMap::new();

        for (element, left_dots) in &self.entries {
            let dots = merge_element_dots(
                Some(left_dots),
                rhs.entries.get(element),
                &self.context,
                &rhs.context,
            );
            if !dots.is_empty() {
                entries.insert(element.clone(), dots);
            }
        }

        for (element, right_dots) in &rhs.entries {
            if self.entries.contains_key(element) {
                continue;
            }

            let dots = merge_element_dots(None, Some(right_dots), &self.context, &rhs.context);
            if !dots.is_empty() {
                entries.insert(element.clone(), dots);
            }
        }

        Self {
            entries,
            context: self.context.join(&rhs.context),
        }
    }
}

impl<T, A> BoundedJoinSemilattice for ORSet<T, A>
where
    A: Clone + Ord,
    T: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<T, A> CvRDT for ORSet<T, A>
where
    A: Clone + Ord,
    T: Clone + Ord,
{
    type Query = BTreeSet<T>;

    fn query(&self) -> Self::Query {
        self.elements()
    }
}

fn merge_element_dots<A>(
    left: Option<&DotSet<A>>,
    right: Option<&DotSet<A>>,
    left_context: &CausalContext<A>,
    right_context: &CausalContext<A>,
) -> DotSet<A>
where
    A: Clone + Ord,
{
    let mut dots = BTreeSet::new();

    if let Some(left) = left {
        for dot in left.iter() {
            if right.is_some_and(|right| right.contains(dot)) || !right_context.contains(dot) {
                dots.insert(dot.clone());
            }
        }
    }

    if let Some(right) = right {
        for dot in right.iter() {
            if left.is_some_and(|left| left.contains(dot)) || !left_context.contains(dot) {
                dots.insert(dot.clone());
            }
        }
    }

    DotSet::from_dots(dots)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crdt_core::{CausalContext, CvRDT, Dot, DotSet, JoinSemilattice, VersionVector};
    use proptest::prelude::*;

    use super::ORSet;

    fn dot(actor: &str, counter: u64) -> Dot<String> {
        Dot::new(actor.to_owned(), counter)
    }

    fn dot_set(dots: &[Dot<String>]) -> DotSet<String> {
        DotSet::from_dots(dots.iter().cloned().collect())
    }

    fn set(
        entries: &[(&str, &[Dot<String>])],
        context_dots: &[Dot<String>],
    ) -> ORSet<String, String> {
        let entries = entries
            .iter()
            .map(|(element, dots)| ((*element).to_owned(), dot_set(dots)))
            .collect();
        let context = CausalContext::from_parts(
            VersionVector::new(),
            DotSet::from_dots(context_dots.iter().cloned().collect()),
        );
        ORSet::from_entries(entries, context)
    }

    fn dot_set_strategy(max_len: usize) -> impl Strategy<Value = BTreeSet<Dot<String>>> {
        crdt_testkit::small_dot_set(5, max_len)
    }

    fn orset_strategy() -> impl Strategy<Value = ORSet<String, String>> {
        (
            prop::collection::btree_map(crdt_testkit::small_actor_id(), dot_set_strategy(4), 0..=4),
            dot_set_strategy(8),
        )
            .prop_map(|(entries, context_dots)| {
                let entries = entries
                    .into_iter()
                    .map(|(element, dots)| (element, DotSet::from_dots(dots)))
                    .collect::<BTreeMap<_, _>>();
                let context = CausalContext::from_parts(
                    VersionVector::new(),
                    DotSet::from_dots(context_dots),
                );
                ORSet::from_entries(entries, context)
            })
    }

    #[test]
    fn add_allocates_dots_and_makes_element_visible() {
        let mut set = ORSet::new();

        let first = set.add("a".to_owned(), "x".to_owned()).unwrap();
        let second = set.add("a".to_owned(), "x".to_owned()).unwrap();

        assert_eq!(first, dot("a", 1));
        assert_eq!(second, dot("a", 2));
        assert!(set.contains(&"x".to_owned()));
        assert_eq!(set.dots(&"x".to_owned()).unwrap().len(), 2);
    }

    #[test]
    fn observed_remove_wins_over_observed_add() {
        let mut left = ORSet::new();
        let mut right = ORSet::new();

        left.add("a".to_owned(), "x".to_owned()).unwrap();
        right.merge(&left);
        right.remove(&"x".to_owned());
        left.merge(&right);

        assert!(!left.contains(&"x".to_owned()));
    }

    #[test]
    fn concurrent_add_wins_over_remove() {
        let mut left = ORSet::new();
        let mut right = ORSet::new();

        left.add("a".to_owned(), "x".to_owned()).unwrap();
        right.merge(&left);

        left.add("a".to_owned(), "x".to_owned()).unwrap();
        right.remove(&"x".to_owned());

        let joined = left.join(&right);

        assert!(joined.contains(&"x".to_owned()));
        assert_eq!(
            joined.dots(&"x".to_owned()).unwrap().dots(),
            &BTreeSet::from([dot("a", 2)])
        );
    }

    #[test]
    fn replicas_converge_after_state_exchange() {
        let mut left = ORSet::new();
        let mut right = ORSet::new();

        left.add("a".to_owned(), "x".to_owned()).unwrap();
        right.add("b".to_owned(), "y".to_owned()).unwrap();

        let left_snapshot = left.clone();
        let right_snapshot = right.clone();

        left.merge(&right_snapshot);
        right.merge(&left_snapshot);

        assert_eq!(left, right);
        assert_eq!(
            left.query(),
            BTreeSet::from(["x".to_owned(), "y".to_owned()])
        );
    }

    #[test]
    fn from_entries_observes_entry_dots_into_context() {
        let set = set(&[("x", &[dot("a", 1), dot("a", 2)])], &[]);

        assert!(set.context().contains(&dot("a", 1)));
        assert!(set.context().contains(&dot("a", 2)));
    }

    #[test]
    fn add_and_remove_are_inflationary() {
        let mut before = ORSet::new();
        before.add("a".to_owned(), "x".to_owned()).unwrap();

        let mut added = before.clone();
        added.add("a".to_owned(), "y".to_owned()).unwrap();
        crdt_testkit::assert_inflationary(&before, &added);

        let mut removed = before.clone();
        removed.remove(&"x".to_owned());
        crdt_testkit::assert_inflationary(&before, &removed);
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a in orset_strategy(),
            b in orset_strategy(),
            c in orset_strategy(),
        ) {
            crdt_testkit::prop_join_semilattice_laws(a, b, c)?;
        }
    }
}
