// @generated
/// Set the transmit state of an RF Platform such as a Radar, Beacon, or Radio.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTransmitState {
    #[prost(enumeration="TransmitState", tag="1")]
    pub transmit_state: i32,
}
/// Set the surveillance state of a passive (listen-only) RF Platform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSurveillanceState {
    #[prost(enumeration="SurveillanceState", tag="1")]
    pub surveillance_state: i32,
}
/// Set whether or not an RF Platform has Emmission Control (EmCon).
/// If supported, RF platforms should only expose the SetTransmitState task when EmissionControlState is EMISSION_CONTROL_STATE_ALLOWED.
/// When in EMISSION_CONTROL_STATE_NOT_ALLOWED, the Platform should be in TRANSMIT_STATE_NOT_TRANSMITTING, and should remove SetTransmitState from the task Catalog.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetEmissionControlState {
    #[prost(enumeration="EmissionControlState", tag="1")]
    pub emcon_state: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransmitState {
    Invalid = 0,
    Transmitting = 1,
    NotTransmitting = 2,
}
impl TransmitState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransmitState::Invalid => "TRANSMIT_STATE_INVALID",
            TransmitState::Transmitting => "TRANSMIT_STATE_TRANSMITTING",
            TransmitState::NotTransmitting => "TRANSMIT_STATE_NOT_TRANSMITTING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSMIT_STATE_INVALID" => Some(Self::Invalid),
            "TRANSMIT_STATE_TRANSMITTING" => Some(Self::Transmitting),
            "TRANSMIT_STATE_NOT_TRANSMITTING" => Some(Self::NotTransmitting),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SurveillanceState {
    Invalid = 0,
    Surveilling = 1,
    NotSurveilling = 2,
}
impl SurveillanceState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SurveillanceState::Invalid => "SURVEILLANCE_STATE_INVALID",
            SurveillanceState::Surveilling => "SURVEILLANCE_STATE_SURVEILLING",
            SurveillanceState::NotSurveilling => "SURVEILLANCE_STATE_NOT_SURVEILLING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SURVEILLANCE_STATE_INVALID" => Some(Self::Invalid),
            "SURVEILLANCE_STATE_SURVEILLING" => Some(Self::Surveilling),
            "SURVEILLANCE_STATE_NOT_SURVEILLING" => Some(Self::NotSurveilling),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmissionControlState {
    Invalid = 0,
    Allowed = 1,
    NotAllowed = 2,
}
impl EmissionControlState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EmissionControlState::Invalid => "EMISSION_CONTROL_STATE_INVALID",
            EmissionControlState::Allowed => "EMISSION_CONTROL_STATE_ALLOWED",
            EmissionControlState::NotAllowed => "EMISSION_CONTROL_STATE_NOT_ALLOWED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMISSION_CONTROL_STATE_INVALID" => Some(Self::Invalid),
            "EMISSION_CONTROL_STATE_ALLOWED" => Some(Self::Allowed),
            "EMISSION_CONTROL_STATE_NOT_ALLOWED" => Some(Self::NotAllowed),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `anduril.tasks.ad.desertguardian.rf.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa8, 0x13, 0x0a, 0x38, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73,
    0x6b, 0x73, 0x2f, 0x61, 0x64, 0x2f, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72,
    0x64, 0x69, 0x61, 0x6e, 0x2f, 0x72, 0x66, 0x2f, 0x76, 0x31, 0x2f, 0x72, 0x66, 0x5f, 0x74, 0x61,
    0x73, 0x6b, 0x73, 0x2e, 0x70, 0x75, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x25, 0x61,
    0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e,
    0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x72,
    0x66, 0x2e, 0x76, 0x31, 0x22, 0x6f, 0x0a, 0x10, 0x53, 0x65, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x6d, 0x69, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x5b, 0x0a, 0x0e, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x6d, 0x69, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x34, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73,
    0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69,
    0x61, 0x6e, 0x2e, 0x72, 0x66, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69,
    0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69, 0x74,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0x7f, 0x0a, 0x14, 0x53, 0x65, 0x74, 0x53, 0x75, 0x72, 0x76,
    0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x67, 0x0a,
    0x12, 0x73, 0x75, 0x72, 0x76, 0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x38, 0x2e, 0x61, 0x6e, 0x64, 0x75,
    0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73,
    0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x72, 0x66, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x75, 0x72, 0x76, 0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x52, 0x11, 0x73, 0x75, 0x72, 0x76, 0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e, 0x63,
    0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0x77, 0x0a, 0x17, 0x53, 0x65, 0x74, 0x45, 0x6d, 0x69,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x5c, 0x0a, 0x0b, 0x65, 0x6d, 0x63, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3b, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c,
    0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74,
    0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x72, 0x66, 0x2e, 0x76, 0x31, 0x2e, 0x45,
    0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x52, 0x0a, 0x65, 0x6d, 0x63, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x2a,
    0x71, 0x0a, 0x0d, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x1a, 0x0a, 0x16, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x41,
    0x54, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x1f, 0x0a, 0x1b,
    0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x54,
    0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x23, 0x0a,
    0x1f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f,
    0x4e, 0x4f, 0x54, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x54, 0x49, 0x4e, 0x47,
    0x10, 0x02, 0x2a, 0x7f, 0x0a, 0x11, 0x53, 0x75, 0x72, 0x76, 0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e,
    0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x1e, 0x0a, 0x1a, 0x53, 0x55, 0x52, 0x56, 0x45,
    0x49, 0x4c, 0x4c, 0x41, 0x4e, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x49, 0x4e,
    0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x22, 0x0a, 0x1e, 0x53, 0x55, 0x52, 0x56, 0x45,
    0x49, 0x4c, 0x4c, 0x41, 0x4e, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x53, 0x55,
    0x52, 0x56, 0x45, 0x49, 0x4c, 0x4c, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x26, 0x0a, 0x22, 0x53,
    0x55, 0x52, 0x56, 0x45, 0x49, 0x4c, 0x4c, 0x41, 0x4e, 0x43, 0x45, 0x5f, 0x53, 0x54, 0x41, 0x54,
    0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x53, 0x55, 0x52, 0x56, 0x45, 0x49, 0x4c, 0x4c, 0x49, 0x4e,
    0x47, 0x10, 0x02, 0x2a, 0x86, 0x01, 0x0a, 0x14, 0x45, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x22, 0x0a, 0x1e,
    0x45, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x4f, 0x4c,
    0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00,
    0x12, 0x22, 0x0a, 0x1e, 0x45, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4e,
    0x54, 0x52, 0x4f, 0x4c, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57,
    0x45, 0x44, 0x10, 0x01, 0x12, 0x26, 0x0a, 0x22, 0x45, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e,
    0x5f, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x4f, 0x4c, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4e,
    0x4f, 0x54, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x10, 0x02, 0x42, 0xcd, 0x02, 0x0a,
    0x29, 0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73,
    0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72,
    0x64, 0x69, 0x61, 0x6e, 0x2e, 0x72, 0x66, 0x2e, 0x76, 0x31, 0x42, 0x0f, 0x52, 0x66, 0x54, 0x61,
    0x73, 0x6b, 0x73, 0x50, 0x75, 0x62, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x54, 0x67,
    0x68, 0x65, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x64, 0x65, 0x76, 0x2f, 0x61,
    0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x61, 0x70,
    0x69, 0x73, 0x2d, 0x67, 0x6f, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61,
    0x73, 0x6b, 0x73, 0x2f, 0x61, 0x64, 0x2f, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61,
    0x72, 0x64, 0x69, 0x61, 0x6e, 0x2f, 0x72, 0x66, 0x2f, 0x76, 0x31, 0x3b, 0x74, 0x61, 0x73, 0x6b,
    0x73, 0x76, 0x31, 0xa2, 0x02, 0x05, 0x41, 0x54, 0x41, 0x44, 0x52, 0xaa, 0x02, 0x25, 0x41, 0x6e,
    0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x41, 0x64, 0x2e, 0x44,
    0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x52, 0x66,
    0x2e, 0x56, 0x31, 0xca, 0x02, 0x25, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61,
    0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x5c, 0x44, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61,
    0x72, 0x64, 0x69, 0x61, 0x6e, 0x5c, 0x52, 0x66, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x31, 0x41, 0x6e,
    0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x5c, 0x44,
    0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x5c, 0x52, 0x66,
    0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x2a, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x3a, 0x3a, 0x54, 0x61, 0x73, 0x6b, 0x73,
    0x3a, 0x3a, 0x41, 0x64, 0x3a, 0x3a, 0x44, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72,
    0x64, 0x69, 0x61, 0x6e, 0x3a, 0x3a, 0x52, 0x66, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x84, 0x0a, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x2c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2e, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x05, 0x00, 0x6b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x6b,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x06, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x42, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x08, 0x00, 0x42, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01, 0x1a, 0x4d, 0x20, 0x53, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x52, 0x46, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72,
    0x6d, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x52, 0x61, 0x64, 0x61,
    0x72, 0x2c, 0x20, 0x42, 0x65, 0x61, 0x63, 0x6f, 0x6e, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x52, 0x61,
    0x64, 0x69, 0x6f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x10, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x10, 0x05, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x12, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x12, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x24, 0x25, 0x0a, 0x50, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x17, 0x00, 0x19, 0x01, 0x1a, 0x44, 0x20, 0x53, 0x65, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x75, 0x72, 0x76, 0x65, 0x69, 0x6c, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x70, 0x61, 0x73, 0x73, 0x69, 0x76,
    0x65, 0x20, 0x28, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x2d, 0x6f, 0x6e, 0x6c, 0x79, 0x29, 0x20,
    0x52, 0x46, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x18, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x18, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x14, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x29, 0x2a,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x1b, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x1e, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1e, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x1e, 0x27, 0x28, 0x0a, 0xf7, 0x02, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x24, 0x00, 0x26, 0x01,
    0x1a, 0xea, 0x02, 0x20, 0x53, 0x65, 0x74, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x52, 0x46, 0x20, 0x50, 0x6c, 0x61,
    0x74, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x68, 0x61, 0x73, 0x20, 0x45, 0x6d, 0x6d, 0x69, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x28, 0x45, 0x6d, 0x43,
    0x6f, 0x6e, 0x29, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74,
    0x65, 0x64, 0x2c, 0x20, 0x52, 0x46, 0x20, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x73,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x65, 0x78, 0x70,
    0x6f, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x65, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x6d, 0x69, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x45, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x69, 0x73, 0x20, 0x45, 0x4d, 0x49, 0x53, 0x53,
    0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x4f, 0x4c, 0x5f, 0x53, 0x54, 0x41, 0x54,
    0x45, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x2e, 0x0a, 0x20, 0x57, 0x68, 0x65, 0x6e,
    0x20, 0x69, 0x6e, 0x20, 0x45, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4e,
    0x54, 0x52, 0x4f, 0x4c, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41,
    0x4c, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x6c, 0x61, 0x74,
    0x66, 0x6f, 0x72, 0x6d, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69,
    0x6e, 0x20, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45,
    0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x4d, 0x49, 0x54, 0x54, 0x49, 0x4e,
    0x47, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x72, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x20, 0x53, 0x65, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69, 0x74,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x61, 0x73, 0x6b, 0x20, 0x43, 0x61, 0x74, 0x61, 0x6c, 0x6f, 0x67, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x24, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x25, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x25, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25,
    0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x25, 0x26,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x28, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x28, 0x05, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x29, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x29, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x29, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x2a, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x2b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x2b, 0x27, 0x28, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)