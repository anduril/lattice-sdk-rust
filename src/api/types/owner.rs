pub use crate::prelude::*;

/// Owner designates the entity responsible for writes of task data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Owner {
    /// Entity ID of the owner.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl Owner {
    pub fn builder() -> OwnerBuilder {
        <OwnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerBuilder {
    entity_id: Option<String>,
}

impl OwnerBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Owner`].
    pub fn build(self) -> Result<Owner, BuildError> {
        Ok(Owner {
            entity_id: self.entity_id,
        })
    }
}
