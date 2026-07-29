pub use crate::prelude::*;

/// A message describing any transponder codes associated with Mode 1, 2, 3, 4, 5, S, C interrogations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransponderCodes {
    /// The mode 1 code assigned to military assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode1: Option<i64>,
    /// The Mode 2 code assigned to military assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode2: Option<i64>,
    /// The Mode 3 code assigned by ATC to the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode3: Option<i64>,
    /// The validity of the response from the Mode 4 interrogation.
    #[serde(rename = "mode4InterrogationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode4interrogation_response: Option<TransponderCodesMode4InterrogationResponse>,
    /// The Mode 5 transponder codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode5: Option<Mode5>,
    /// The Mode S transponder codes.
    #[serde(rename = "modeS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_s: Option<ModeS>,
    /// The Mode C altitude reported by the transponder in feet. Mode C provides pressure altitude in 100-foot increments up
    /// to 10,000 feet MSL. Valid altitudes include 0 ft (sea level). An unset field indicates no Mode C response was received.
    #[serde(rename = "modeCAltitudeFt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_c_altitude_ft: Option<i64>,
}

impl TransponderCodes {
    pub fn builder() -> TransponderCodesBuilder {
        <TransponderCodesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransponderCodesBuilder {
    mode1: Option<i64>,
    mode2: Option<i64>,
    mode3: Option<i64>,
    mode4interrogation_response: Option<TransponderCodesMode4InterrogationResponse>,
    mode5: Option<Mode5>,
    mode_s: Option<ModeS>,
    mode_c_altitude_ft: Option<i64>,
}

impl TransponderCodesBuilder {
    pub fn mode1(mut self, value: i64) -> Self {
        self.mode1 = Some(value);
        self
    }

    pub fn mode2(mut self, value: i64) -> Self {
        self.mode2 = Some(value);
        self
    }

    pub fn mode3(mut self, value: i64) -> Self {
        self.mode3 = Some(value);
        self
    }

    pub fn mode4interrogation_response(
        mut self,
        value: TransponderCodesMode4InterrogationResponse,
    ) -> Self {
        self.mode4interrogation_response = Some(value);
        self
    }

    pub fn mode5(mut self, value: Mode5) -> Self {
        self.mode5 = Some(value);
        self
    }

    pub fn mode_s(mut self, value: ModeS) -> Self {
        self.mode_s = Some(value);
        self
    }

    pub fn mode_c_altitude_ft(mut self, value: i64) -> Self {
        self.mode_c_altitude_ft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransponderCodes`].
    pub fn build(self) -> Result<TransponderCodes, BuildError> {
        Ok(TransponderCodes {
            mode1: self.mode1,
            mode2: self.mode2,
            mode3: self.mode3,
            mode4interrogation_response: self.mode4interrogation_response,
            mode5: self.mode5,
            mode_s: self.mode_s,
            mode_c_altitude_ft: self.mode_c_altitude_ft,
        })
    }
}
