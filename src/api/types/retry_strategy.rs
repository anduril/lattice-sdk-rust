pub use crate::prelude::*;

/// Sets an optional try strategy for tasks. Use this option to control how Lattice attempts to retry delivery of tasks to assets with intermittent access or network connectivity to your environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetryStrategy {
    #[serde(rename = "fixedRetryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_retry_strategy: Option<FixedRetry>,
}

impl RetryStrategy {
    pub fn builder() -> RetryStrategyBuilder {
        <RetryStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetryStrategyBuilder {
    fixed_retry_strategy: Option<FixedRetry>,
}

impl RetryStrategyBuilder {
    pub fn fixed_retry_strategy(mut self, value: FixedRetry) -> Self {
        self.fixed_retry_strategy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetryStrategy`].
    pub fn build(self) -> Result<RetryStrategy, BuildError> {
        Ok(RetryStrategy {
            fixed_retry_strategy: self.fixed_retry_strategy,
        })
    }
}
