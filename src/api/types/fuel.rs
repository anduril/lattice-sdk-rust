pub use crate::prelude::*;

/// Fuel describes an entity's repository of fuels stores including current amount, operational requirements, and maximum authorized capacity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Fuel {
    /// Unique fuel identifier
    #[serde(rename = "fuelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_id: Option<String>,
    /// Long form name of the fuel source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Timestamp the information was reported
    #[serde(rename = "reportedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub reported_date: Option<DateTime<FixedOffset>>,
    /// Amount of gallons on hand
    #[serde(rename = "amountGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gallons: Option<i64>,
    /// How much the asset is allowed to have available (in gallons)
    #[serde(rename = "maxAuthorizedCapacityGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_authorized_capacity_gallons: Option<i64>,
    /// Minimum required for operations (in gallons)
    #[serde(rename = "operationalRequirementGallons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_requirement_gallons: Option<i64>,
    /// Fuel in a single asset may have different levels of classification
    /// Use case: fuel for a SECRET asset while diesel fuel may be UNCLASSIFIED
    #[serde(rename = "dataClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification: Option<Classification>,
    /// Source of information
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
}

impl Fuel {
    pub fn builder() -> FuelBuilder {
        <FuelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FuelBuilder {
    fuel_id: Option<String>,
    name: Option<String>,
    reported_date: Option<DateTime<FixedOffset>>,
    amount_gallons: Option<i64>,
    max_authorized_capacity_gallons: Option<i64>,
    operational_requirement_gallons: Option<i64>,
    data_classification: Option<Classification>,
    data_source: Option<String>,
}

impl FuelBuilder {
    pub fn fuel_id(mut self, value: impl Into<String>) -> Self {
        self.fuel_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn reported_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.reported_date = Some(value);
        self
    }

    pub fn amount_gallons(mut self, value: i64) -> Self {
        self.amount_gallons = Some(value);
        self
    }

    pub fn max_authorized_capacity_gallons(mut self, value: i64) -> Self {
        self.max_authorized_capacity_gallons = Some(value);
        self
    }

    pub fn operational_requirement_gallons(mut self, value: i64) -> Self {
        self.operational_requirement_gallons = Some(value);
        self
    }

    pub fn data_classification(mut self, value: Classification) -> Self {
        self.data_classification = Some(value);
        self
    }

    pub fn data_source(mut self, value: impl Into<String>) -> Self {
        self.data_source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Fuel`].
    pub fn build(self) -> Result<Fuel, BuildError> {
        Ok(Fuel {
            fuel_id: self.fuel_id,
            name: self.name,
            reported_date: self.reported_date,
            amount_gallons: self.amount_gallons,
            max_authorized_capacity_gallons: self.max_authorized_capacity_gallons,
            operational_requirement_gallons: self.operational_requirement_gallons,
            data_classification: self.data_classification,
            data_source: self.data_source,
        })
    }
}
