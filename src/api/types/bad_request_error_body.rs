pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadRequestErrorBody {
    #[serde(default)]
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl BadRequestErrorBody {
    pub fn builder() -> BadRequestErrorBodyBuilder {
        <BadRequestErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyBuilder {
    error: Option<String>,
    error_description: Option<String>,
}

impl BadRequestErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn error_description(mut self, value: impl Into<String>) -> Self {
        self.error_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BadRequestErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](BadRequestErrorBodyBuilder::error)
    pub fn build(self) -> Result<BadRequestErrorBody, BuildError> {
        Ok(BadRequestErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            error_description: self.error_description,
        })
    }
}
