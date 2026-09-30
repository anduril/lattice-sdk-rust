pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcmDetails {
    #[serde(rename = "acmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_type: Option<AcmDetailsAcmType>,
    /// Used for loosely typed associations, such as assignment to a specific fires unit.
    /// Limit to 150 characters.
    #[serde(rename = "acmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_description: Option<String>,
}

impl AcmDetails {
    pub fn builder() -> AcmDetailsBuilder {
        <AcmDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcmDetailsBuilder {
    acm_type: Option<AcmDetailsAcmType>,
    acm_description: Option<String>,
}

impl AcmDetailsBuilder {
    pub fn acm_type(mut self, value: AcmDetailsAcmType) -> Self {
        self.acm_type = Some(value);
        self
    }

    pub fn acm_description(mut self, value: impl Into<String>) -> Self {
        self.acm_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AcmDetails`].
    pub fn build(self) -> Result<AcmDetails, BuildError> {
        Ok(AcmDetails {
            acm_type: self.acm_type,
            acm_description: self.acm_description,
        })
    }
}
