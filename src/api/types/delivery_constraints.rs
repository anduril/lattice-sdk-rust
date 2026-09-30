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
    /// Requires the agent to acknowledge the request before Lattice considers it delivered.
    /// Without this, a request sent over a streaming agent connection is marked delivered as soon
    /// as the send returns, which only proves it reached a local buffer and not that the agent
    /// received it. With this set, the task is not marked delivered until the agent reports a
    /// status confirming receipt; Lattice re-sends until it does, and eventually fails delivery
    /// with DELIVERY_ERROR_CODE_NOT_ACKNOWLEDGED. Requires deliver_before, which bounds that
    /// retrying.
    #[serde(rename = "requireAcknowledgement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_acknowledgement: Option<bool>,
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
    require_acknowledgement: Option<bool>,
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

    pub fn require_acknowledgement(mut self, value: bool) -> Self {
        self.require_acknowledgement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveryConstraints`].
    pub fn build(self) -> Result<DeliveryConstraints, BuildError> {
        Ok(DeliveryConstraints {
            deliver_after: self.deliver_after,
            deliver_before: self.deliver_before,
            require_acknowledgement: self.require_acknowledgement,
        })
    }
}
