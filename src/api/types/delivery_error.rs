pub use crate::prelude::*;

/// DeliveryError contains an error code and message associated with task delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveryError {
    /// Error code for Delivery error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<DeliveryErrorCode>,
    /// Descriptive human-readable string regarding this delivery error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DeliveryError {
    pub fn builder() -> DeliveryErrorBuilder {
        <DeliveryErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveryErrorBuilder {
    code: Option<DeliveryErrorCode>,
    message: Option<String>,
}

impl DeliveryErrorBuilder {
    pub fn code(mut self, value: DeliveryErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeliveryError`].
    pub fn build(self) -> Result<DeliveryError, BuildError> {
        Ok(DeliveryError {
            code: self.code,
            message: self.message,
        })
    }
}
