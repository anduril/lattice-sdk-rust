pub use crate::prelude::*;

/// A message describing the component's health status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComponentMessage {
    /// The status associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComponentMessageStatus>,
    /// The human-readable content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ComponentMessage {
    pub fn builder() -> ComponentMessageBuilder {
        <ComponentMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComponentMessageBuilder {
    status: Option<ComponentMessageStatus>,
    message: Option<String>,
}

impl ComponentMessageBuilder {
    pub fn status(mut self, value: ComponentMessageStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ComponentMessage`].
    pub fn build(self) -> Result<ComponentMessage, BuildError> {
        Ok(ComponentMessage {
            status: self.status,
            message: self.message,
        })
    }
}
