//! Positive-negative counter.

use std::collections::BTreeMap;

use crdt_core::{BoundedJoinSemilattice, CvRDT, JoinSemilattice};

use super::{CounterOverflow, GCounter};

/// Observable PN-Counter value split into positive and negative totals.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNCounterValue {
    increments: u128,
    decrements: u128,
}

impl PNCounterValue {
    /// Creates a split counter value.
    pub fn new(increments: u128, decrements: u128) -> Self {
        Self {
            increments,
            decrements,
        }
    }

    /// Returns the grow-only increment total.
    pub fn increments(self) -> u128 {
        self.increments
    }

    /// Returns the grow-only decrement total.
    pub fn decrements(self) -> u128 {
        self.decrements
    }

    /// Returns the signed net value when it fits in `i128`.
    pub fn checked_net(self) -> Option<i128> {
        if self.increments >= self.decrements {
            i128::try_from(self.increments - self.decrements).ok()
        } else {
            let difference = self.decrements - self.increments;
            if difference == (1_u128 << 127) {
                Some(i128::MIN)
            } else {
                i128::try_from(difference).ok().map(|value| -value)
            }
        }
    }
}

/// A state-based counter supporting increments and decrements.
///
/// Internally this is a pair of G-Counters: one for increments and one for
/// decrements. Merge is component-wise maximum for both sides.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PNCounter<A = String> {
    increments: GCounter<A>,
    decrements: GCounter<A>,
}

impl<A> Default for PNCounter<A> {
    fn default() -> Self {
        Self {
            increments: GCounter::new(),
            decrements: GCounter::new(),
        }
    }
}

impl<A> PNCounter<A> {
    /// Creates an empty counter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the increment components.
    pub fn increments(&self) -> &GCounter<A> {
        &self.increments
    }

    /// Returns the decrement components.
    pub fn decrements(&self) -> &GCounter<A> {
        &self.decrements
    }

    /// Returns the split observable value.
    pub fn value(&self) -> PNCounterValue {
        PNCounterValue::new(self.increments.value(), self.decrements.value())
    }

    /// Returns the signed value when it fits in `i128`.
    pub fn checked_value(&self) -> Option<i128> {
        self.value().checked_net()
    }
}

impl<A> PNCounter<A>
where
    A: Ord,
{
    /// Creates a counter from increment and decrement component maps.
    pub fn from_components(increments: BTreeMap<A, u64>, decrements: BTreeMap<A, u64>) -> Self {
        Self {
            increments: GCounter::from_counts(increments),
            decrements: GCounter::from_counts(decrements),
        }
    }

    /// Returns one actor's increment component.
    pub fn increment_component(&self, actor: &A) -> u64 {
        self.increments.component(actor)
    }

    /// Returns one actor's decrement component.
    pub fn decrement_component(&self, actor: &A) -> u64 {
        self.decrements.component(actor)
    }

    /// Increments one actor's positive component by `amount`.
    pub fn increment_by(&mut self, actor: A, amount: u64) -> Result<(), CounterOverflow> {
        self.increments.increment_by(actor, amount)
    }

    /// Decrements one actor's negative component by `amount`.
    pub fn decrement_by(&mut self, actor: A, amount: u64) -> Result<(), CounterOverflow> {
        self.decrements.increment_by(actor, amount)
    }

    /// Increments one actor's positive component by one.
    pub fn increment(&mut self, actor: A) -> Result<(), CounterOverflow> {
        self.increment_by(actor, 1)
    }

    /// Decrements one actor's negative component by one.
    pub fn decrement(&mut self, actor: A) -> Result<(), CounterOverflow> {
        self.decrement_by(actor, 1)
    }
}

impl<A> JoinSemilattice for PNCounter<A>
where
    A: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self {
            increments: self.increments.join(&rhs.increments),
            decrements: self.decrements.join(&rhs.decrements),
        }
    }
}

impl<A> BoundedJoinSemilattice for PNCounter<A>
where
    A: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

impl<A> CvRDT for PNCounter<A>
where
    A: Clone + Ord,
{
    type Query = PNCounterValue;

    fn query(&self) -> Self::Query {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crdt_core::JoinSemilattice;
    use crdt_testkit::ReferenceModel;
    use proptest::prelude::*;

    use super::{PNCounter, PNCounterValue};

    fn counter(increments: &[(&str, u64)], decrements: &[(&str, u64)]) -> PNCounter<String> {
        PNCounter::from_components(
            increments
                .iter()
                .map(|(actor, count)| ((*actor).to_owned(), *count))
                .collect(),
            decrements
                .iter()
                .map(|(actor, count)| ((*actor).to_owned(), *count))
                .collect(),
        )
    }

    struct ReferencePNCounter {
        increments: BTreeMap<String, u64>,
        decrements: BTreeMap<String, u64>,
    }

    impl ReferencePNCounter {
        fn from_counter(counter: &PNCounter<String>) -> Self {
            Self {
                increments: counter.increments().counts().clone(),
                decrements: counter.decrements().counts().clone(),
            }
        }
    }

    impl ReferenceModel for ReferencePNCounter {
        type Query = PNCounterValue;

        fn query(&self) -> Self::Query {
            PNCounterValue::new(
                self.increments
                    .values()
                    .map(|count| u128::from(*count))
                    .sum(),
                self.decrements
                    .values()
                    .map(|count| u128::from(*count))
                    .sum(),
            )
        }
    }

    #[test]
    fn increments_and_decrements_update_separate_components() {
        let mut counter = PNCounter::new();

        counter.increment_by("a".to_owned(), 5).unwrap();
        counter.decrement_by("a".to_owned(), 2).unwrap();
        counter.decrement("b".to_owned()).unwrap();

        assert_eq!(counter.increment_component(&"a".to_owned()), 5);
        assert_eq!(counter.decrement_component(&"a".to_owned()), 2);
        assert_eq!(counter.decrement_component(&"b".to_owned()), 1);
        assert_eq!(counter.checked_value(), Some(2));
    }

    #[test]
    fn split_value_reports_signed_extremes_when_representable() {
        assert_eq!(
            PNCounterValue::new(0, 1_u128 << 127).checked_net(),
            Some(i128::MIN)
        );
        assert_eq!(PNCounterValue::new(1_u128 << 127, 0).checked_net(), None);
    }

    #[test]
    fn merge_takes_componentwise_maximum_on_both_sides() {
        let left = counter(&[("a", 3)], &[("b", 1)]);
        let right = counter(&[("a", 1), ("c", 4)], &[("b", 5)]);

        let joined = left.join(&right);

        assert_eq!(joined.increment_component(&"a".to_owned()), 3);
        assert_eq!(joined.increment_component(&"c".to_owned()), 4);
        assert_eq!(joined.decrement_component(&"b".to_owned()), 5);
        assert_eq!(joined.checked_value(), Some(2));
    }

    #[test]
    fn reference_model_agrees_on_query() {
        let counter = counter(&[("a", 3), ("b", 4)], &[("a", 2)]);
        let reference = ReferencePNCounter::from_counter(&counter);

        crdt_testkit::assert_query_matches(&counter, &reference);
    }

    proptest! {
        #[test]
        fn join_semilattice_laws_hold(
            a_inc in crdt_testkit::small_actor_counts(6, 4),
            a_dec in crdt_testkit::small_actor_counts(6, 4),
            b_inc in crdt_testkit::small_actor_counts(6, 4),
            b_dec in crdt_testkit::small_actor_counts(6, 4),
            c_inc in crdt_testkit::small_actor_counts(6, 4),
            c_dec in crdt_testkit::small_actor_counts(6, 4),
        ) {
            crdt_testkit::prop_join_semilattice_laws(
                PNCounter::from_components(a_inc, a_dec),
                PNCounter::from_components(b_inc, b_dec),
                PNCounter::from_components(c_inc, c_dec),
            )?;
        }

        #[test]
        fn increment_and_decrement_are_inflationary(
            increments in crdt_testkit::small_actor_counts(6, 4),
            decrements in crdt_testkit::small_actor_counts(6, 4),
            actor in crdt_testkit::small_actor_id(),
            amount in 0u64..=6,
        ) {
            let before = PNCounter::from_components(increments, decrements);

            let mut incremented = before.clone();
            incremented.increment_by(actor.clone(), amount).unwrap();
            crdt_testkit::prop_inflationary(&before, &incremented)?;

            let mut decremented = before.clone();
            decremented.decrement_by(actor, amount).unwrap();
            crdt_testkit::prop_inflationary(&before, &decremented)?;
        }
    }
}
