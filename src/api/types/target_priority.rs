pub use crate::prelude::*;

/// The target prioritization associated with an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TargetPriority {
    /// Describes the target priority in relation to high value target lists.
    #[serde(rename = "highValueTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target: Option<HighValueTarget>,
    /// Describes whether the entity should be treated as a threat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat: Option<Threat>,
}

impl TargetPriority {
    pub fn builder() -> TargetPriorityBuilder {
        <TargetPriorityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TargetPriorityBuilder {
    high_value_target: Option<HighValueTarget>,
    threat: Option<Threat>,
}

impl TargetPriorityBuilder {
    pub fn high_value_target(mut self, value: HighValueTarget) -> Self {
        self.high_value_target = Some(value);
        self
    }

    pub fn threat(mut self, value: Threat) -> Self {
        self.threat = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TargetPriority`].
    pub fn build(self) -> Result<TargetPriority, BuildError> {
        Ok(TargetPriority {
            high_value_target: self.high_value_target,
            threat: self.threat,
        })
    }
}
