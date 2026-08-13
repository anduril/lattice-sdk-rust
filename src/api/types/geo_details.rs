pub use crate::prelude::*;

/// A component that describes a geo-entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GeoDetailsType>,
    #[serde(rename = "controlArea")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_area: Option<ControlAreaDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<AcmDetails>,
    #[serde(rename = "visualDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_details: Option<GeoVisualDetails>,
}

impl GeoDetails {
    pub fn builder() -> GeoDetailsBuilder {
        <GeoDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoDetailsBuilder {
    r#type: Option<GeoDetailsType>,
    control_area: Option<ControlAreaDetails>,
    acm: Option<AcmDetails>,
    visual_details: Option<GeoVisualDetails>,
}

impl GeoDetailsBuilder {
    pub fn r#type(mut self, value: GeoDetailsType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn control_area(mut self, value: ControlAreaDetails) -> Self {
        self.control_area = Some(value);
        self
    }

    pub fn acm(mut self, value: AcmDetails) -> Self {
        self.acm = Some(value);
        self
    }

    pub fn visual_details(mut self, value: GeoVisualDetails) -> Self {
        self.visual_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoDetails`].
    pub fn build(self) -> Result<GeoDetails, BuildError> {
        Ok(GeoDetails {
            r#type: self.r#type,
            control_area: self.control_area,
            acm: self.acm,
            visual_details: self.visual_details,
        })
    }
}
