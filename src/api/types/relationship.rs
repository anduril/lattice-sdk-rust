pub use crate::prelude::*;

/// The relationship component indicates a relationship to another entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Relationship {
    /// The entity ID to which this entity is related.
    #[serde(rename = "relatedEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_entity_id: Option<String>,
    /// A unique identifier for this relationship. Allows removing or updating relationships.
    #[serde(rename = "relationshipId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    /// The relationship type
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<RelationshipType>,
}

impl Relationship {
    pub fn builder() -> RelationshipBuilder {
        <RelationshipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RelationshipBuilder {
    related_entity_id: Option<String>,
    relationship_id: Option<String>,
    relationship_type: Option<RelationshipType>,
}

impl RelationshipBuilder {
    pub fn related_entity_id(mut self, value: impl Into<String>) -> Self {
        self.related_entity_id = Some(value.into());
        self
    }

    pub fn relationship_id(mut self, value: impl Into<String>) -> Self {
        self.relationship_id = Some(value.into());
        self
    }

    pub fn relationship_type(mut self, value: RelationshipType) -> Self {
        self.relationship_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Relationship`].
    pub fn build(self) -> Result<Relationship, BuildError> {
        Ok(Relationship {
            related_entity_id: self.related_entity_id,
            relationship_id: self.relationship_id,
            relationship_type: self.relationship_type,
        })
    }
}
