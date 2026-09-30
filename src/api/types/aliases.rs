pub use crate::prelude::*;

/// Available for any Entities with alternate ids in other systems.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Aliases {
    #[serde(rename = "alternateIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_ids: Option<Vec<AlternateId>>,
    /// The best available version of the entity's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Aliases {
    pub fn builder() -> AliasesBuilder {
        <AliasesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AliasesBuilder {
    alternate_ids: Option<Vec<AlternateId>>,
    name: Option<String>,
}

impl AliasesBuilder {
    pub fn alternate_ids(mut self, value: Vec<AlternateId>) -> Self {
        self.alternate_ids = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Aliases`].
    pub fn build(self) -> Result<Aliases, BuildError> {
        Ok(Aliases {
            alternate_ids: self.alternate_ids,
            name: self.name,
        })
    }
}
