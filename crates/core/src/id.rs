//! Document and object identifiers.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A unique identifier for a document or object within Nexivora.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    /// Generate a new random identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an identifier from a UUID string.
    pub fn parse(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }

    /// The underlying UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Id {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_roundtrip() {
        let id = Id::new();
        let s = id.to_string();
        assert_eq!(Id::parse(&s), Some(id));
    }

    #[test]
    fn id_unique() {
        let a = Id::new();
        let b = Id::new();
        assert_ne!(a, b);
    }
}
