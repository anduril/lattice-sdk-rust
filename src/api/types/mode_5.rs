pub use crate::prelude::*;

/// Describes the Mode 5 transponder interrogation status and codes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Mode5 {
    /// The validity of the response from the Mode 5 interrogation.
    #[serde(rename = "mode5InterrogationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode5interrogation_response: Option<Mode5Mode5InterrogationResponse>,
    /// The Mode 5 code assigned to military assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode5: Option<i64>,
    /// The Mode 5 platform identification code.
    #[serde(rename = "mode5PlatformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode5platform_id: Option<i64>,
}

impl Mode5 {
    pub fn builder() -> Mode5Builder {
        <Mode5Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Mode5Builder {
    mode5interrogation_response: Option<Mode5Mode5InterrogationResponse>,
    mode5: Option<i64>,
    mode5platform_id: Option<i64>,
}

impl Mode5Builder {
    pub fn mode5interrogation_response(mut self, value: Mode5Mode5InterrogationResponse) -> Self {
        self.mode5interrogation_response = Some(value);
        self
    }

    pub fn mode5(mut self, value: i64) -> Self {
        self.mode5 = Some(value);
        self
    }

    pub fn mode5platform_id(mut self, value: i64) -> Self {
        self.mode5platform_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Mode5`].
    pub fn build(self) -> Result<Mode5, BuildError> {
        Ok(Mode5 {
            mode5interrogation_response: self.mode5interrogation_response,
            mode5: self.mode5,
            mode5platform_id: self.mode5platform_id,
        })
    }
}
