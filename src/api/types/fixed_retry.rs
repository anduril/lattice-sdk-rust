pub use crate::prelude::*;

/// Defaults to an interval of 5 seconds. If the DeliverBefore field in the task's DeliveryConstraints isn't populated, Lattice does not retry delivery and instead logs a warning.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FixedRetry {
    /// Specifies the interval between retries. A default interval of 5 seconds is used if this field is not set.
    #[serde(rename = "retryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<String>,
}

impl FixedRetry {
    pub fn builder() -> FixedRetryBuilder {
        <FixedRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedRetryBuilder {
    retry_interval: Option<String>,
}

impl FixedRetryBuilder {
    pub fn retry_interval(mut self, value: impl Into<String>) -> Self {
        self.retry_interval = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FixedRetry`].
    pub fn build(self) -> Result<FixedRetry, BuildError> {
        Ok(FixedRetry {
            retry_interval: self.retry_interval,
        })
    }
}
