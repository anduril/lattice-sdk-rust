pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityIdsSelector {
    /// Receive tasks as an assignee for one or more of the supplied entity ids.
    #[serde(rename = "entityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<String>>,
}

impl EntityIdsSelector {
    pub fn builder() -> EntityIdsSelectorBuilder {
        <EntityIdsSelectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityIdsSelectorBuilder {
    entity_ids: Option<Vec<String>>,
}

impl EntityIdsSelectorBuilder {
    pub fn entity_ids(mut self, value: Vec<String>) -> Self {
        self.entity_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityIdsSelector`].
    pub fn build(self) -> Result<EntityIdsSelector, BuildError> {
        Ok(EntityIdsSelector {
            entity_ids: self.entity_ids,
        })
    }
}
