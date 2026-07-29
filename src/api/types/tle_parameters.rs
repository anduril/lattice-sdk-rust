pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TleParameters {
    /// Integer specifying TLE ephemeris type
    #[serde(rename = "ephemerisType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeris_type: Option<i64>,
    /// User-defined free-text message classification/caveats of this TLE
    #[serde(rename = "classificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_type: Option<String>,
    /// Norad catalog number: integer up to nine digits.
    #[serde(rename = "noradCatId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norad_cat_id: Option<i64>,
    #[serde(rename = "elementSetNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_set_no: Option<i64>,
    /// Optional: revolution number
    #[serde(rename = "revAtEpoch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev_at_epoch: Option<i64>,
    /// Drag parameter for SGP-4 in units 1 / Earth radii
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bstar: Option<f64>,
    /// Drag parameter for SGP4-XP in units m^2 / kg
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bterm: Option<f64>,
    /// First time derivative of mean motion in rev / day^2
    #[serde(rename = "meanMotionDot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mean_motion_dot: Option<f64>,
    /// Second time derivative of mean motion in rev / day^3. For use with SGP or PPT3.
    #[serde(rename = "meanMotionDdot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mean_motion_ddot: Option<f64>,
    /// Solar radiation pressure coefficient A_gamma / m in m^2 / kg. For use with SGP4-XP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub agom: Option<f64>,
}

impl TleParameters {
    pub fn builder() -> TleParametersBuilder {
        <TleParametersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TleParametersBuilder {
    ephemeris_type: Option<i64>,
    classification_type: Option<String>,
    norad_cat_id: Option<i64>,
    element_set_no: Option<i64>,
    rev_at_epoch: Option<i64>,
    bstar: Option<f64>,
    bterm: Option<f64>,
    mean_motion_dot: Option<f64>,
    mean_motion_ddot: Option<f64>,
    agom: Option<f64>,
}

impl TleParametersBuilder {
    pub fn ephemeris_type(mut self, value: i64) -> Self {
        self.ephemeris_type = Some(value);
        self
    }

    pub fn classification_type(mut self, value: impl Into<String>) -> Self {
        self.classification_type = Some(value.into());
        self
    }

    pub fn norad_cat_id(mut self, value: i64) -> Self {
        self.norad_cat_id = Some(value);
        self
    }

    pub fn element_set_no(mut self, value: i64) -> Self {
        self.element_set_no = Some(value);
        self
    }

    pub fn rev_at_epoch(mut self, value: i64) -> Self {
        self.rev_at_epoch = Some(value);
        self
    }

    pub fn bstar(mut self, value: f64) -> Self {
        self.bstar = Some(value);
        self
    }

    pub fn bterm(mut self, value: f64) -> Self {
        self.bterm = Some(value);
        self
    }

    pub fn mean_motion_dot(mut self, value: f64) -> Self {
        self.mean_motion_dot = Some(value);
        self
    }

    pub fn mean_motion_ddot(mut self, value: f64) -> Self {
        self.mean_motion_ddot = Some(value);
        self
    }

    pub fn agom(mut self, value: f64) -> Self {
        self.agom = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TleParameters`].
    pub fn build(self) -> Result<TleParameters, BuildError> {
        Ok(TleParameters {
            ephemeris_type: self.ephemeris_type,
            classification_type: self.classification_type,
            norad_cat_id: self.norad_cat_id,
            element_set_no: self.element_set_no,
            rev_at_epoch: self.rev_at_epoch,
            bstar: self.bstar,
            bterm: self.bterm,
            mean_motion_dot: self.mean_motion_dot,
            mean_motion_ddot: self.mean_motion_ddot,
            agom: self.agom,
        })
    }
}
