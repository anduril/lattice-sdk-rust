pub use crate::prelude::*;

/// Defines the current state of a task's delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveryState {
    /// The current status of the delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeliveryStateStatus>,
    /// Errors associated with the delivery, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DeliveryError>,
    /// Optional scheduling constraints for Lattice delivery of the task.
    #[serde(rename = "deliveryConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_constraints: Option<DeliveryConstraints>,
}

impl DeliveryState {
    pub fn builder() -> DeliveryStateBuilder {
        <DeliveryStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveryStateBuilder {
    status: Option<DeliveryStateStatus>,
    error: Option<DeliveryError>,
    delivery_constraints: Option<DeliveryConstraints>,
}

impl DeliveryStateBuilder {
    pub fn status(mut self, value: DeliveryStateStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error(mut self, value: DeliveryError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn delivery_constraints(mut self, value: DeliveryConstraints) -> Self {
        self.delivery_constraints = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveryState`].
    pub fn build(self) -> Result<DeliveryState, BuildError> {
        Ok(DeliveryState {
            status: self.status,
            error: self.error,
            delivery_constraints: self.delivery_constraints,
        })
    }
}
