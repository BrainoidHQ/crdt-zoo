//! Causal metadata for CRDTs that track observed updates.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display};

use crate::{ActorId, BoundedJoinSemilattice, JoinSemilattice};

/// Error returned when allocating a dot would overflow its actor counter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DotOverflow;

impl Display for DotOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dot counter overflow")
    }
}

impl Error for DotOverflow {}

/// A unique event identifier owned by one actor.
///
/// Dot counters are one-based. Counter `0` is treated as already included by
/// every version vector and should not be allocated for an update.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dot<A = ActorId> {
    actor: A,
    counter: u64,
}

impl<A> Dot<A> {
    /// Creates a dot from an actor and a counter.
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }

    /// Returns the actor that owns this dot.
    pub fn actor(&self) -> &A {
        &self.actor
    }

    /// Returns the actor-local counter.
    pub fn counter(&self) -> u64 {
        self.counter
    }

    /// Consumes the dot and returns its actor and counter.
    pub fn into_parts(self) -> (A, u64) {
        (self.actor, self.counter)
    }
}

/// A deterministic set of dots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DotSet<A = ActorId> {
    dots: BTreeSet<Dot<A>>,
}

impl<A> Default for DotSet<A> {
    fn default() -> Self {
        Self {
            dots: BTreeSet::new(),
        }
    }
}

impl<A> DotSet<A> {
    /// Creates an empty dot set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of dots.
    pub fn len(&self) -> usize {
        self.dots.len()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    /// Returns the stored dots.
    pub fn dots(&self) -> &BTreeSet<Dot<A>> {
        &self.dots
    }

    /// Iterates over the stored dots.
    pub fn iter(&self) -> impl Iterator<Item = &Dot<A>> {
        self.dots.iter()
    }

    /// Consumes the set and returns the stored dots.
    pub fn into_dots(self) -> BTreeSet<Dot<A>> {
        self.dots
    }
}

impl<A> DotSet<A>
where
    A: Ord,
{
    /// Creates a dot set from raw dots.
    pub fn from_dots(dots: BTreeSet<Dot<A>>) -> Self {
        Self { dots }
    }

    /// Inserts a dot.
    pub fn insert(&mut self, dot: Dot<A>) -> bool {
        self.dots.insert(dot)
    }

    /// Removes a dot.
    pub fn remove(&mut self, dot: &Dot<A>) -> bool {
        self.dots.remove(dot)
    }

    /// Returns whether the set contains a dot.
    pub fn contains(&self, dot: &Dot<A>) -> bool {
        self.dots.contains(dot)
    }
}

impl<A> JoinSemilattice for DotSet<A>
where
    A: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self {
            dots: self.dots.union(&rhs.dots).cloned().collect(),
        }
    }
}

impl<A> BoundedJoinSemilattice for DotSet<A>
where
    A: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

/// A version vector indexed by actor id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionVector<A = ActorId> {
    components: BTreeMap<A, u64>,
}

impl<A> Default for VersionVector<A> {
    fn default() -> Self {
        Self {
            components: BTreeMap::new(),
        }
    }
}

impl<A> VersionVector<A> {
    /// Creates an empty version vector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the actor components.
    pub fn components(&self) -> &BTreeMap<A, u64> {
        &self.components
    }

    /// Returns the number of non-zero actor components.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether the vector has no non-zero actor components.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl<A> VersionVector<A>
where
    A: Ord,
{
    /// Creates a version vector from component values.
    ///
    /// Zero-valued components are dropped so equivalent vectors have a single
    /// representation.
    pub fn from_components(mut components: BTreeMap<A, u64>) -> Self {
        components.retain(|_, counter| *counter != 0);
        Self { components }
    }

    /// Returns one actor's component, or zero when absent.
    pub fn component(&self, actor: &A) -> u64 {
        self.components.get(actor).copied().unwrap_or(0)
    }

    /// Returns whether this vector includes a dot.
    pub fn contains(&self, dot: &Dot<A>) -> bool {
        self.component(dot.actor()) >= dot.counter()
    }
}

impl<A> VersionVector<A>
where
    A: Clone + Ord,
{
    /// Returns the next dot for an actor without mutating the vector.
    pub fn next_dot(&self, actor: A) -> Result<Dot<A>, DotOverflow> {
        let next = self.component(&actor).checked_add(1).ok_or(DotOverflow)?;
        Ok(Dot::new(actor, next))
    }

    /// Records a dot by taking the component-wise maximum.
    pub fn observe(&mut self, dot: &Dot<A>) -> bool {
        if dot.counter() == 0 {
            return false;
        }

        let component = self.components.entry(dot.actor().clone()).or_insert(0);
        if *component < dot.counter() {
            *component = dot.counter();
            true
        } else {
            false
        }
    }

    /// Increments one actor's component and returns the allocated dot.
    pub fn increment(&mut self, actor: A) -> Result<Dot<A>, DotOverflow> {
        let dot = self.next_dot(actor)?;
        self.observe(&dot);
        Ok(dot)
    }
}

impl<A> JoinSemilattice for VersionVector<A>
where
    A: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        let mut components = self.components.clone();

        for (actor, rhs_counter) in &rhs.components {
            let counter = components.entry(actor.clone()).or_insert(0);
            if *counter < *rhs_counter {
                *counter = *rhs_counter;
            }
        }

        Self { components }
    }
}

impl<A> BoundedJoinSemilattice for VersionVector<A>
where
    A: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

/// A compact causal context.
///
/// Contiguous dots are represented by a version vector. Non-contiguous dots are
/// kept in a dot set until later observations make them compactable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalContext<A = ActorId> {
    clock: VersionVector<A>,
    dots: DotSet<A>,
}

impl<A> Default for CausalContext<A> {
    fn default() -> Self {
        Self {
            clock: VersionVector::new(),
            dots: DotSet::new(),
        }
    }
}

impl<A> CausalContext<A> {
    /// Creates an empty causal context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the compact version vector.
    pub fn clock(&self) -> &VersionVector<A> {
        &self.clock
    }

    /// Returns the non-contiguous dot set.
    pub fn dots(&self) -> &DotSet<A> {
        &self.dots
    }

    /// Returns whether the context has no observed dots.
    pub fn is_empty(&self) -> bool {
        self.clock.is_empty() && self.dots.is_empty()
    }
}

impl<A> CausalContext<A>
where
    A: Ord,
{
    /// Returns whether this context includes a dot.
    pub fn contains(&self, dot: &Dot<A>) -> bool {
        self.clock.contains(dot) || self.dots.contains(dot)
    }
}

impl<A> CausalContext<A>
where
    A: Clone + Ord,
{
    /// Creates a causal context from a vector and dot set.
    pub fn from_parts(clock: VersionVector<A>, dots: DotSet<A>) -> Self {
        let mut context = Self { clock, dots };
        context.compact();
        context
    }

    /// Returns the next dot for an actor without mutating the context.
    pub fn next_dot(&self, actor: A) -> Result<Dot<A>, DotOverflow> {
        let mut counter = self.clock.component(&actor);
        for dot in self.dots.iter() {
            if dot.actor() == &actor && counter < dot.counter() {
                counter = dot.counter();
            }
        }

        let next = counter.checked_add(1).ok_or(DotOverflow)?;
        Ok(Dot::new(actor, next))
    }

    /// Records a dot and compacts the context when possible.
    pub fn observe(&mut self, dot: Dot<A>) -> bool {
        if dot.counter() == 0 || self.contains(&dot) {
            return false;
        }

        let inserted = self.dots.insert(dot);
        self.compact();
        inserted
    }

    /// Records many dots.
    pub fn observe_all<I>(&mut self, dots: I)
    where
        I: IntoIterator<Item = Dot<A>>,
    {
        for dot in dots {
            self.observe(dot);
        }
    }

    /// Allocates and records the next dot for an actor.
    pub fn advance(&mut self, actor: A) -> Result<Dot<A>, DotOverflow> {
        let dot = self.next_dot(actor)?;
        self.observe(dot.clone());
        Ok(dot)
    }

    fn compact(&mut self) {
        loop {
            let next = self.dots.iter().find_map(|dot| {
                let expected = self.clock.component(dot.actor()).checked_add(1)?;
                (dot.counter() == expected).then(|| dot.clone())
            });

            let Some(dot) = next else {
                break;
            };

            self.clock.observe(&dot);
            self.dots.remove(&dot);
        }

        let clock = &self.clock;
        self.dots.dots.retain(|dot| !clock.contains(dot));
    }
}

impl<A> JoinSemilattice for CausalContext<A>
where
    A: Clone + Ord,
{
    fn join(&self, rhs: &Self) -> Self {
        Self::from_parts(self.clock.join(&rhs.clock), self.dots.join(&rhs.dots))
    }
}

impl<A> BoundedJoinSemilattice for CausalContext<A>
where
    A: Clone + Ord,
{
    fn bottom() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::{CausalContext, Dot, DotSet, JoinSemilattice, VersionVector};

    fn dot(actor: &str, counter: u64) -> Dot<String> {
        Dot::new(actor.to_owned(), counter)
    }

    #[test]
    fn version_vector_join_takes_componentwise_maximum() {
        let left = VersionVector::from_components(BTreeMap::from([
            ("a".to_owned(), 2),
            ("b".to_owned(), 1),
        ]));
        let right = VersionVector::from_components(BTreeMap::from([
            ("a".to_owned(), 1),
            ("c".to_owned(), 5),
        ]));

        let joined = left.join(&right);

        assert_eq!(joined.component(&"a".to_owned()), 2);
        assert_eq!(joined.component(&"b".to_owned()), 1);
        assert_eq!(joined.component(&"c".to_owned()), 5);
    }

    #[test]
    fn causal_context_compacts_contiguous_dots() {
        let dots = DotSet::from_dots(BTreeSet::from([dot("a", 1), dot("a", 3)]));
        let mut context = CausalContext::from_parts(VersionVector::new(), dots);

        assert_eq!(context.clock().component(&"a".to_owned()), 1);
        assert!(context.dots().contains(&dot("a", 3)));

        context.observe(dot("a", 2));

        assert_eq!(context.clock().component(&"a".to_owned()), 3);
        assert!(context.dots().is_empty());
    }

    #[test]
    fn causal_context_join_is_idempotent() {
        let mut context = CausalContext::new();
        context.observe(dot("a", 1));
        context.observe(dot("b", 3));

        assert_eq!(context.join(&context), context);
    }
}
