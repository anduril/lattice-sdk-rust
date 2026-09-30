pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MeanKeplerianElements {
    /// UTC time of validity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub epoch: Option<DateTime<FixedOffset>>,
    /// Preferred: semi major axis in kilometers
    #[serde(rename = "semiMajorAxisKm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub semi_major_axis_km: Option<f64>,
    /// If using SGP/SGP4, provide the Keplerian Mean Motion in revolutions per day
    #[serde(rename = "meanMotion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mean_motion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub eccentricity: Option<f64>,
    /// Angle of inclination in deg
    #[serde(rename = "inclinationDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub inclination_deg: Option<f64>,
    /// Right ascension of the ascending node in deg
    #[serde(rename = "raOfAscNodeDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ra_of_asc_node_deg: Option<f64>,
    /// Argument of pericenter in deg
    #[serde(rename = "argOfPericenterDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub arg_of_pericenter_deg: Option<f64>,
    /// Mean anomaly in deg
    #[serde(rename = "meanAnomalyDeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mean_anomaly_deg: Option<f64>,
    /// Optional: gravitational coefficient (Gravitational Constant x central mass) in kg^3 / s^2
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gm: Option<f64>,
}

impl MeanKeplerianElements {
    pub fn builder() -> MeanKeplerianElementsBuilder {
        <MeanKeplerianElementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeanKeplerianElementsBuilder {
    epoch: Option<DateTime<FixedOffset>>,
    semi_major_axis_km: Option<f64>,
    mean_motion: Option<f64>,
    eccentricity: Option<f64>,
    inclination_deg: Option<f64>,
    ra_of_asc_node_deg: Option<f64>,
    arg_of_pericenter_deg: Option<f64>,
    mean_anomaly_deg: Option<f64>,
    gm: Option<f64>,
}

impl MeanKeplerianElementsBuilder {
    pub fn epoch(mut self, value: DateTime<FixedOffset>) -> Self {
        self.epoch = Some(value);
        self
    }

    pub fn semi_major_axis_km(mut self, value: f64) -> Self {
        self.semi_major_axis_km = Some(value);
        self
    }

    pub fn mean_motion(mut self, value: f64) -> Self {
        self.mean_motion = Some(value);
        self
    }

    pub fn eccentricity(mut self, value: f64) -> Self {
        self.eccentricity = Some(value);
        self
    }

    pub fn inclination_deg(mut self, value: f64) -> Self {
        self.inclination_deg = Some(value);
        self
    }

    pub fn ra_of_asc_node_deg(mut self, value: f64) -> Self {
        self.ra_of_asc_node_deg = Some(value);
        self
    }

    pub fn arg_of_pericenter_deg(mut self, value: f64) -> Self {
        self.arg_of_pericenter_deg = Some(value);
        self
    }

    pub fn mean_anomaly_deg(mut self, value: f64) -> Self {
        self.mean_anomaly_deg = Some(value);
        self
    }

    pub fn gm(mut self, value: f64) -> Self {
        self.gm = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeanKeplerianElements`].
    pub fn build(self) -> Result<MeanKeplerianElements, BuildError> {
        Ok(MeanKeplerianElements {
            epoch: self.epoch,
            semi_major_axis_km: self.semi_major_axis_km,
            mean_motion: self.mean_motion,
            eccentricity: self.eccentricity,
            inclination_deg: self.inclination_deg,
            ra_of_asc_node_deg: self.ra_of_asc_node_deg,
            arg_of_pericenter_deg: self.arg_of_pericenter_deg,
            mean_anomaly_deg: self.mean_anomaly_deg,
            gm: self.gm,
        })
    }
}
