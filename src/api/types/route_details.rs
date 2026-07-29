pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RouteDetails {
    /// Free form text giving the name of the entity's destination
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// Estimated time of arrival at destination
    #[serde(rename = "estimatedArrivalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival_time: Option<DateTime<FixedOffset>>,
}

impl RouteDetails {
    pub fn builder() -> RouteDetailsBuilder {
        <RouteDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RouteDetailsBuilder {
    destination_name: Option<String>,
    estimated_arrival_time: Option<DateTime<FixedOffset>>,
}

impl RouteDetailsBuilder {
    pub fn destination_name(mut self, value: impl Into<String>) -> Self {
        self.destination_name = Some(value.into());
        self
    }

    pub fn estimated_arrival_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RouteDetails`].
    pub fn build(self) -> Result<RouteDetails, BuildError> {
        Ok(RouteDetails {
            destination_name: self.destination_name,
            estimated_arrival_time: self.estimated_arrival_time,
        })
    }
}
