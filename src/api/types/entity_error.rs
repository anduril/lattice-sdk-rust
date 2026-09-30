pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    pub error_code: String,
    #[serde(default)]
    pub message: String,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    error_code: Option<String>,
    message: Option<String>,
}

impl ErrorBuilder {
    pub fn error_code(mut self, value: impl Into<String>) -> Self {
        self.error_code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_code`](ErrorBuilder::error_code)
    /// - [`message`](ErrorBuilder::message)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            error_code: self
                .error_code
                .ok_or_else(|| BuildError::missing_field("error_code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
