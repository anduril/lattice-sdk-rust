pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Altitude {
    #[serde(rename = "haeWgs84")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hae_wgs84: Option<AltitudeAboveWgs84Ellipsoid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asf: Option<AltitudeAboveSeaFloor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bss: Option<AltitudeBelowSeaSurface>,
    #[serde(rename = "pressureSdp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_sdp: Option<AltitudeAboveStandardDatumPlanePressure>,
    #[serde(rename = "pressureAmsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_amsl: Option<AltitudeAboveMeanSeaLevelPressure>,
    #[serde(rename = "egm96Amsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egm96amsl: Option<AltitudeAboveMeanSeaLevelEgm96>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agl: Option<AltitudeAboveGroundLevel>,
}

impl Altitude {
    pub fn builder() -> AltitudeBuilder {
        <AltitudeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeBuilder {
    hae_wgs84: Option<AltitudeAboveWgs84Ellipsoid>,
    asf: Option<AltitudeAboveSeaFloor>,
    bss: Option<AltitudeBelowSeaSurface>,
    pressure_sdp: Option<AltitudeAboveStandardDatumPlanePressure>,
    pressure_amsl: Option<AltitudeAboveMeanSeaLevelPressure>,
    egm96amsl: Option<AltitudeAboveMeanSeaLevelEgm96>,
    agl: Option<AltitudeAboveGroundLevel>,
}

impl AltitudeBuilder {
    pub fn hae_wgs84(mut self, value: AltitudeAboveWgs84Ellipsoid) -> Self {
        self.hae_wgs84 = Some(value);
        self
    }

    pub fn asf(mut self, value: AltitudeAboveSeaFloor) -> Self {
        self.asf = Some(value);
        self
    }

    pub fn bss(mut self, value: AltitudeBelowSeaSurface) -> Self {
        self.bss = Some(value);
        self
    }

    pub fn pressure_sdp(mut self, value: AltitudeAboveStandardDatumPlanePressure) -> Self {
        self.pressure_sdp = Some(value);
        self
    }

    pub fn pressure_amsl(mut self, value: AltitudeAboveMeanSeaLevelPressure) -> Self {
        self.pressure_amsl = Some(value);
        self
    }

    pub fn egm96amsl(mut self, value: AltitudeAboveMeanSeaLevelEgm96) -> Self {
        self.egm96amsl = Some(value);
        self
    }

    pub fn agl(mut self, value: AltitudeAboveGroundLevel) -> Self {
        self.agl = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Altitude`].
    pub fn build(self) -> Result<Altitude, BuildError> {
        Ok(Altitude {
            hae_wgs84: self.hae_wgs84,
            asf: self.asf,
            bss: self.bss,
            pressure_sdp: self.pressure_sdp,
            pressure_amsl: self.pressure_amsl,
            egm96amsl: self.egm96amsl,
            agl: self.agl,
        })
    }
}
