pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error2 {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl Error2 {
    pub fn builder() -> Error2Builder {
        <Error2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Error2Builder {
    code: Option<String>,
    message: Option<String>,
}

impl Error2Builder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](Error2Builder::code)
    /// - [`message`](Error2Builder::message)
    pub fn build(self) -> Result<Error2, BuildError> {
        Ok(Error2 {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
