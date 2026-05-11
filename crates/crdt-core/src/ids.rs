//! Stable identifiers shared by CRDT implementations.

use std::fmt::{self, Display};

/// Identifier for an actor that performs updates.
///
/// Actor identifiers are expected to be stable and unique for a single logical
/// writer. Algorithms that allocate per-actor dots rely on this single-writer
/// property to avoid duplicate event identifiers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActorId(String);

impl ActorId {
    /// Creates an actor identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the inner string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ActorId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for ActorId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ActorId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Identifier for a replica that stores and exchanges state.
///
/// Replica ids are separate from actor ids because a single replica may host
/// multiple actors, and an actor may move between replicas if the application
/// preserves the actor's local counter state.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReplicaId(String);

impl ReplicaId {
    /// Creates a replica identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the inner string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ReplicaId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ReplicaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for ReplicaId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ReplicaId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
