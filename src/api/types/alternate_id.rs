pub use crate::prelude::*;

/// An alternate id for an Entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlternateId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AlternateIdType>,
}

impl AlternateId {
    pub fn builder() -> AlternateIdBuilder {
        <AlternateIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlternateIdBuilder {
    id: Option<String>,
    r#type: Option<AlternateIdType>,
}

impl AlternateIdBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AlternateIdType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlternateId`].
    pub fn build(self) -> Result<AlternateId, BuildError> {
        Ok(AlternateId {
            id: self.id,
            r#type: self.r#type,
        })
    }
}
