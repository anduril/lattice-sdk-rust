pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityOverride {
    /// The entity containing the overridden fields. The service will extract the overridable fields from
    /// the object and ignore all other fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    /// Additional information about the source of the override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
}

impl EntityOverride {
    pub fn builder() -> EntityOverrideBuilder {
        <EntityOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityOverrideBuilder {
    entity: Option<Entity>,
    provenance: Option<Provenance>,
}

impl EntityOverrideBuilder {
    pub fn entity(mut self, value: Entity) -> Self {
        self.entity = Some(value);
        self
    }

    pub fn provenance(mut self, value: Provenance) -> Self {
        self.provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityOverride`].
    pub fn build(self) -> Result<EntityOverride, BuildError> {
        Ok(EntityOverride {
            entity: self.entity,
            provenance: self.provenance,
        })
    }
}
