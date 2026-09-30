pub use crate::prelude::*;

/// Describes whether an entity is a threat or not.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Threat {
    /// Indicates that the entity has been determined to be a threat.
    #[serde(rename = "isThreat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_threat: Option<bool>,
}

impl Threat {
    pub fn builder() -> ThreatBuilder {
        <ThreatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreatBuilder {
    is_threat: Option<bool>,
}

impl ThreatBuilder {
    pub fn is_threat(mut self, value: bool) -> Self {
        self.is_threat = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Threat`].
    pub fn build(self) -> Result<Threat, BuildError> {
        Ok(Threat {
            is_threat: self.is_threat,
        })
    }
}
