//! Helpers for comparing implementations with simple reference models.

use std::fmt::Debug;

use crdt_core::CvRDT;

/// A simple reference model that can expose the same query result as an implementation.
pub trait ReferenceModel {
    /// The externally visible query result.
    type Query;

    /// Reads the reference model's observable value.
    fn query(&self) -> Self::Query;
}

/// Asserts that an implementation and reference model agree on their query result.
pub fn assert_query_matches<S, R>(implementation: &S, reference: &R)
where
    S: CvRDT<Query = R::Query>,
    R: ReferenceModel,
    R::Query: Debug + PartialEq,
{
    assert_eq!(implementation.query(), reference.query());
}
