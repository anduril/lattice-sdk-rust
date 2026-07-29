pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HighValueTargetMatch {
    /// The ID of the high value target list that matches the target description.
    #[serde(rename = "highValueTargetListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target_list_id: Option<String>,
    /// The ID of the specific high value target description within a high value target list that was matched against.
    /// The ID is considered to be a globally unique identifier across all high value target IDs.
    #[serde(rename = "highValueTargetDescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_value_target_description_id: Option<String>,
}

impl HighValueTargetMatch {
    pub fn builder() -> HighValueTargetMatchBuilder {
        <HighValueTargetMatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HighValueTargetMatchBuilder {
    high_value_target_list_id: Option<String>,
    high_value_target_description_id: Option<String>,
}

impl HighValueTargetMatchBuilder {
    pub fn high_value_target_list_id(mut self, value: impl Into<String>) -> Self {
        self.high_value_target_list_id = Some(value.into());
        self
    }

    pub fn high_value_target_description_id(mut self, value: impl Into<String>) -> Self {
        self.high_value_target_description_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HighValueTargetMatch`].
    pub fn build(self) -> Result<HighValueTargetMatch, BuildError> {
        Ok(HighValueTargetMatch {
            high_value_target_list_id: self.high_value_target_list_id,
            high_value_target_description_id: self.high_value_target_description_id,
        })
    }
}
