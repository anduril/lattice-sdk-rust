// @generated
/// Set the power state of a Platform. It is up to the Platform to interpret the power state and act accordingly.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerState {
    #[prost(enumeration="PowerState", tag="1")]
    pub power_state: i32,
}
/// Delete an entity from the internal tracker of a Platform.
/// Does not silence or suppress the track from re-forming if the tracking conditions are satisfied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTrack {
    #[prost(string, tag="1")]
    pub entity_id: ::prost::alloc::string::String,
}
/// Set this entity as a "High Priority Track".
/// The tasked Platform is responsible for maintaining a list of current High-Priority tracks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHighPriorityTrack {
    #[prost(string, tag="1")]
    pub entity_id: ::prost::alloc::string::String,
}
/// Unset this entity as a "High Priority Track".
/// The tasked Platform is responsible for maintaining a list of current High-Priority tracks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveHighPriorityTrack {
    #[prost(string, tag="1")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PowerState {
    Invalid = 0,
    On = 1,
    Off = 2,
}
impl PowerState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PowerState::Invalid => "POWER_STATE_INVALID",
            PowerState::On => "POWER_STATE_ON",
            PowerState::Off => "POWER_STATE_OFF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POWER_STATE_INVALID" => Some(Self::Invalid),
            "POWER_STATE_ON" => Some(Self::On),
            "POWER_STATE_OFF" => Some(Self::Off),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `anduril.tasks.ad.desertguardian.common.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9f, 0x0e, 0x0a, 0x40, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73,
    0x6b, 0x73, 0x2f, 0x61, 0x64, 0x2f, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72,
    0x64, 0x69, 0x61, 0x6e, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x70, 0x75, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x29, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74,
    0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75,
    0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31,
    0x22, 0x67, 0x0a, 0x0d, 0x53, 0x65, 0x74, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x56, 0x0a, 0x0b, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x35, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c,
    0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65, 0x72, 0x74,
    0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x76, 0x31, 0x2e, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0a, 0x70,
    0x6f, 0x77, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0x2a, 0x0a, 0x0b, 0x44, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x49, 0x64, 0x22, 0x33, 0x0a, 0x14, 0x53, 0x65, 0x74, 0x48, 0x69, 0x67, 0x68,
    0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x12, 0x1b, 0x0a,
    0x09, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x08, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x64, 0x22, 0x36, 0x0a, 0x17, 0x52, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x48, 0x69, 0x67, 0x68, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x54, 0x72, 0x61, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x49, 0x64, 0x2a, 0x4e, 0x0a, 0x0a, 0x50, 0x6f, 0x77, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x17, 0x0a, 0x13, 0x50, 0x4f, 0x57, 0x45, 0x52, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f,
    0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x50, 0x4f, 0x57,
    0x45, 0x52, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x13, 0x0a,
    0x0f, 0x50, 0x4f, 0x57, 0x45, 0x52, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x46, 0x46,
    0x10, 0x02, 0x42, 0x8f, 0x02, 0x0a, 0x2d, 0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72,
    0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x2e, 0x64, 0x65, 0x73, 0x65,
    0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f,
    0x6e, 0x2e, 0x76, 0x31, 0x42, 0x13, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x54, 0x61, 0x73, 0x6b,
    0x73, 0x50, 0x75, 0x62, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x05, 0x41, 0x54,
    0x41, 0x44, 0x43, 0xaa, 0x02, 0x29, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x54, 0x61,
    0x73, 0x6b, 0x73, 0x2e, 0x41, 0x64, 0x2e, 0x44, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61,
    0x72, 0x64, 0x69, 0x61, 0x6e, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x56, 0x31, 0xca,
    0x02, 0x29, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x5c,
    0x41, 0x64, 0x5c, 0x44, 0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61,
    0x6e, 0x5c, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x35, 0x41, 0x6e,
    0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x5c, 0x44,
    0x65, 0x73, 0x65, 0x72, 0x74, 0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x5c, 0x43, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0xea, 0x02, 0x2e, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x3a, 0x3a, 0x54,
    0x61, 0x73, 0x6b, 0x73, 0x3a, 0x3a, 0x41, 0x64, 0x3a, 0x3a, 0x44, 0x65, 0x73, 0x65, 0x72, 0x74,
    0x67, 0x75, 0x61, 0x72, 0x64, 0x69, 0x61, 0x6e, 0x3a, 0x3a, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xc3, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x05, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07,
    0x00, 0x46, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x46, 0x0a, 0x7b, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0d, 0x01, 0x1a, 0x6f, 0x20, 0x53, 0x65, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x20, 0x49,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x75, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50,
    0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x70, 0x72, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x63, 0x74, 0x20, 0x61, 0x63, 0x63,
    0x6f, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0b, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x02,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0d, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x0f, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x0f, 0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x02, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x10, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x11, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x12, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x14, 0x15,
    0x0a, 0xaa, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x17, 0x00, 0x19, 0x01, 0x1a, 0x9d, 0x01,
    0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x0a, 0x20, 0x44, 0x6f, 0x65, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x69, 0x6c, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x72, 0x20,
    0x73, 0x75, 0x70, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61,
    0x63, 0x6b, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x72, 0x65, 0x2d, 0x66, 0x6f, 0x72, 0x6d, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x69,
    0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x73, 0x61, 0x74, 0x69, 0x73, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x18, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x15, 0x16,
    0x0a, 0x96, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00, 0x1f, 0x01, 0x1a, 0x89, 0x01,
    0x20, 0x53, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x22, 0x48, 0x69, 0x67, 0x68, 0x20, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x20, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x2e, 0x0a, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x65, 0x64, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72,
    0x6d, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x48, 0x69, 0x67, 0x68, 0x2d, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e,
    0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x09, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x15, 0x16, 0x0a, 0x98, 0x01, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x1a, 0x8b, 0x01, 0x20, 0x55, 0x6e, 0x73,
    0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x61,
    0x73, 0x20, 0x61, 0x20, 0x22, 0x48, 0x69, 0x67, 0x68, 0x20, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x20, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x65, 0x64, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x20,
    0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x20, 0x48, 0x69, 0x67, 0x68, 0x2d, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x74,
    0x72, 0x61, 0x63, 0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x23, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];
// @@protoc_insertion_point(module)