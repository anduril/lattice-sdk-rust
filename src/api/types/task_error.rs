pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error3 {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl Error3 {
    pub fn builder() -> Error3Builder {
        <Error3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Error3Builder {
    code: Option<String>,
    message: Option<String>,
}

impl Error3Builder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error3`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](Error3Builder::code)
    /// - [`message`](Error3Builder::message)
    pub fn build(self) -> Result<Error3, BuildError> {
        Ok(Error3 {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
