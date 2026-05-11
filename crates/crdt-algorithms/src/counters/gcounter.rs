//! Grow-only counter.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display};

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

/// Error returned when incrementing a component would overflow `u64`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CounterOverflow;

impl Display for CounterOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("counter component overflow")
    }
}

impl Error for CounterOverflow {}

/// A state-based grow-only counter.
///
/// Each actor owns one monotonically increasing component. Merging takes the
/// component-wise maximum, so duplicate and out-of-order state delivery is
/// harmless.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GCounter<A = String> {
    counts: BTreeMap<A, u64>,
}

impl<A> Default for GCounter<A> {
    fn default() -> Self {
        Self {
            counts: BTreeMap::new(),
        }
    }
}

impl<A> GCounter<A> {
    /// Creates an empty counter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the per-actor components.
    pub fn counts(&self) -> &BTreeMap<A, u64> {
        &self.counts
    }

    /// Returns the sum of all components.
    pub fn value(&self) -> u128 {
        self.counts.values().map(|count| u128::from(*count)).sum()
    }
}

impl<A> GCounter<A>
where
    A: Ord,
{
    /// Creates a counter from component values.
    ///
    /// Zero-valued components are dropped so logically equivalent states have a
    /// single representation.
    pub fn from_counts(mut counts: BTreeMap<A, u64>) -> Self {
        counts.retain(|_, count| *count != 0);
        Self { counts }
    }

    /// Returns one actor's component value, or zero when absent.
    pub fn component(&self, actor: &A) -> u64 {
        self.counts.get(actor).copied().unwrap_or(0)
    }

    /// Increments one actor's component by `amount`.
    pub fn increment_by(&mut self, actor: A, amount: u64) -> Result<(), CounterOverflow> {
        if amount == 0 {
            return Ok(());
        }

        let component = self.counts.entry(actor).or_insert(0);
        let next = component.checked_add(amount).ok_or(CounterOverflow)?;
        *component = next;
        Ok(())
    }

    /// Increments one actor's component by one.
    pub fn increment(&mut self, actor: A) -> Result<(), CounterOverflow> {
        self.increment_by(actor, 1)
    }
}

impl<A> JoinSemilattice for GCounter<A>
where
    A: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        let mut counts = self.counts.clone();

        for (actor, rhs_count) in &rhs.counts {
            let lhs_count = counts.entry(actor.clone()).or_insert(0);
            if *lhs_count < *rhs_count {
                *lhs_count = *rhs_count;
            }
        }

        Self { counts }
    }
}

impl<A> BoundedJoinSemilattice for GCounter<A>
where
    A: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<A> CvRDT for GCounter<A>
where
    A: Clone + Ord,
{
    type Query = u128;

    fn query(&self) -> Self::Query {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crdt_core::{CvRDT, JoinSemilattice};
    use crdt_testkit::assert_inflationary;

    use super::GCounter;

    fn counter(pairs: &[(&str, u64)]) -> GCounter<String> {
        let counts = pairs
            .iter()
            .map(|(actor, count)| ((*actor).to_owned(), *count))
            .collect::<BTreeMap<_, _>>();
        GCounter::from_counts(counts)
    }

    #[test]
    fn empty_counter_queries_zero() {
        let counter = GCounter::<String>::new();

        assert_eq!(counter.query(), 0);
        assert!(counter.counts().is_empty());
    }

    #[test]
    fn increment_updates_one_actor_component() {
        let actor = "a".to_owned();
        let mut counter = GCounter::new();

        counter.increment(actor.clone()).unwrap();
        counter.increment_by(actor.clone(), 2).unwrap();

        assert_eq!(counter.component(&actor), 3);
        assert_eq!(counter.query(), 3);
    }

    #[test]
    fn increment_is_inflationary() {
        let actor = "a".to_owned();
        let before = counter(&[("a", 1), ("b", 4)]);
        let mut after = before.clone();

        after.increment(actor).unwrap();

        assert_inflationary(&before, &after);
    }

    #[test]
    fn join_takes_componentwise_maximum() {
        let left = counter(&[("a", 2), ("b", 1)]);
        let right = counter(&[("a", 1), ("c", 5)]);

        let joined = left.join(&right);

        assert_eq!(joined.component(&"a".to_owned()), 2);
        assert_eq!(joined.component(&"b".to_owned()), 1);
        assert_eq!(joined.component(&"c".to_owned()), 5);
        assert_eq!(joined.query(), 8);
    }

    #[test]
    fn join_semilattice_laws_hold_for_examples() {
        let a = counter(&[("a", 1), ("b", 2)]);
        let b = counter(&[("a", 3)]);
        let c = counter(&[("b", 4), ("c", 1)]);

        crdt_testkit::assert_join_semilattice_laws!(a, b, c);
    }

    #[test]
    fn replicas_converge_after_state_exchange() {
        let mut left = GCounter::new();
        let mut right = GCounter::new();

        left.increment("left".to_owned()).unwrap();
        right.increment_by("right".to_owned(), 2).unwrap();

        let left_snapshot = left.clone();
        let right_snapshot = right.clone();

        left.merge(&right_snapshot);
        right.merge(&left_snapshot);

        assert_eq!(left, right);
        assert_eq!(left.query(), 3);
    }
}
