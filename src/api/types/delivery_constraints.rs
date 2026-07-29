pub use crate::prelude::*;

/// DeliveryConstraints defines when Lattice should deliver the task to the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveryConstraints {
    /// Optional earliest time the task can attempt to be delivered.
    #[serde(rename = "deliverAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deliver_after: Option<DateTime<FixedOffset>>,
    /// The latest time by which the task should be delivered.
    /// If this deadline passes without successful delivery of the task, then the task will time
    /// out with DELIVERY_ERROR_CODE_TIMEOUT.
    /// This field is only required for tasks with retry strategies.
    #[serde(rename = "deliverBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deliver_before: Option<DateTime<FixedOffset>>,
}

impl DeliveryConstraints {
    pub fn builder() -> DeliveryConstraintsBuilder {
        <DeliveryConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveryConstraintsBuilder {
    deliver_after: Option<DateTime<FixedOffset>>,
    deliver_before: Option<DateTime<FixedOffset>>,
}

impl DeliveryConstraintsBuilder {
    pub fn deliver_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deliver_after = Some(value);
        self
    }

    pub fn deliver_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deliver_before = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveryConstraints`].
    pub fn build(self) -> Result<DeliveryConstraints, BuildError> {
        Ok(DeliveryConstraints {
            deliver_after: self.deliver_after,
            deliver_before: self.deliver_before,
        })
    }
}
