pub use crate::prelude::*;

/// The relationships between this entity and other entities in the common operational picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Relationships {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
}

impl Relationships {
    pub fn builder() -> RelationshipsBuilder {
        <RelationshipsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RelationshipsBuilder {
    relationships: Option<Vec<Relationship>>,
}

impl RelationshipsBuilder {
    pub fn relationships(mut self, value: Vec<Relationship>) -> Self {
        self.relationships = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Relationships`].
    pub fn build(self) -> Result<Relationships, BuildError> {
        Ok(Relationships {
            relationships: self.relationships,
        })
    }
}
