// @generated
/// Maps to a Line formation of assets with a speed. This is a simple line with two LLAs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineFormation {
    /// Line start
    #[prost(message, optional, tag="1")]
    pub line_start: ::core::option::Option<super::super::super::v2::Objective>,
    /// Line end
    #[prost(message, optional, tag="2")]
    pub line_end: ::core::option::Option<super::super::super::v2::Objective>,
    /// Speed in Meters/Second to get in Line Formation
    #[prost(message, optional, tag="3")]
    pub surface_speed_ms: ::core::option::Option<::pbjson_types::DoubleValue>,
}
/// Maps to BREVITY code Marshal.
/// Establish(ed) at a specific point, typically used to posture forces in preparation for an offensive operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Marshal {
    /// Objective to Marshal to.
    #[prost(message, optional, tag="1")]
    pub objective: ::core::option::Option<super::super::super::v2::Objective>,
    /// Speed in Meters/Second
    #[prost(message, optional, tag="2")]
    pub surface_speed_ms: ::core::option::Option<::pbjson_types::DoubleValue>,
}
/// Encoded file descriptor set for the `anduril.tasks.ads.thirdparty.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe0, 0x08, 0x0a, 0x33, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73,
    0x6b, 0x73, 0x2f, 0x61, 0x64, 0x73, 0x2f, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72, 0x74,
    0x79, 0x2f, 0x76, 0x31, 0x2f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70,
    0x75, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69,
    0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x73, 0x2e, 0x74, 0x68, 0x69, 0x72,
    0x64, 0x70, 0x61, 0x72, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x1a, 0x24, 0x61, 0x6e, 0x64, 0x75, 0x72,
    0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2f, 0x76, 0x32, 0x2f, 0x6f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x69, 0x76, 0x65, 0x2e, 0x70, 0x75, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xcb, 0x01, 0x0a, 0x0d, 0x4c, 0x69, 0x6e, 0x65, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x3a, 0x0a, 0x0a, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x52, 0x09, 0x6c, 0x69, 0x6e, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x12, 0x36, 0x0a,
    0x08, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e,
    0x76, 0x32, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x52, 0x07, 0x6c, 0x69,
    0x6e, 0x65, 0x45, 0x6e, 0x64, 0x12, 0x46, 0x0a, 0x10, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0e, 0x73,
    0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x53, 0x70, 0x65, 0x65, 0x64, 0x4d, 0x73, 0x42, 0xa9, 0x02,
    0x0a, 0x23, 0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61,
    0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x73, 0x2e, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72,
    0x74, 0x79, 0x2e, 0x76, 0x31, 0x42, 0x11, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x50, 0x75, 0x62, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4e, 0x67, 0x68, 0x65, 0x2e,
    0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x64, 0x65, 0x76, 0x2f, 0x61, 0x6e, 0x64, 0x75,
    0x72, 0x69, 0x6c, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x61, 0x70, 0x69, 0x73, 0x2d,
    0x67, 0x6f, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73, 0x6b, 0x73,
    0x2f, 0x61, 0x64, 0x73, 0x2f, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72, 0x74, 0x79, 0x2f,
    0x76, 0x31, 0x3b, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x76, 0x31, 0xa2, 0x02, 0x04, 0x41, 0x54, 0x41,
    0x54, 0xaa, 0x02, 0x1f, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x54, 0x61, 0x73, 0x6b,
    0x73, 0x2e, 0x41, 0x64, 0x73, 0x2e, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72, 0x74, 0x79,
    0x2e, 0x56, 0x31, 0xca, 0x02, 0x1f, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61,
    0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x73, 0x5c, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72,
    0x74, 0x79, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x2b, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c,
    0x54, 0x61, 0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x73, 0x5c, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0xea, 0x02, 0x23, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x3a, 0x3a, 0x54,
    0x61, 0x73, 0x6b, 0x73, 0x3a, 0x3a, 0x41, 0x64, 0x73, 0x3a, 0x3a, 0x54, 0x68, 0x69, 0x72, 0x64,
    0x70, 0x61, 0x72, 0x74, 0x79, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xbf, 0x03, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x28, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x65, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x08, 0x00, 0x65, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x63, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x10, 0x00, 0x19, 0x01, 0x1a, 0x57, 0x20, 0x4d, 0x61, 0x70, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x20, 0x4c, 0x69, 0x6e, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61,
    0x20, 0x73, 0x70, 0x65, 0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x4c, 0x4c, 0x41, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x15, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x12, 0x02, 0x2c, 0x1a, 0x0c, 0x20, 0x4c, 0x69, 0x6e, 0x65, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x1d, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x2a, 0x2b, 0x0a, 0x17,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02, 0x2a, 0x1a, 0x0a, 0x20, 0x4c, 0x69,
    0x6e, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x15, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x15, 0x1d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x28,
    0x29, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x18, 0x02, 0x33, 0x1a, 0x31,
    0x20, 0x53, 0x70, 0x65, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73,
    0x2f, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x69,
    0x6e, 0x20, 0x4c, 0x69, 0x6e, 0x65, 0x20, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x18, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x1e, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x31, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33, 0x0a, 0x89, 0x08, 0x0a, 0x31, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x2f, 0x61, 0x64, 0x73, 0x2f, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x2e,
    0x70, 0x75, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x61, 0x6e, 0x64, 0x75, 0x72,
    0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x73, 0x2e, 0x74, 0x68, 0x69,
    0x72, 0x64, 0x70, 0x61, 0x72, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x1a, 0x24, 0x61, 0x6e, 0x64, 0x75,
    0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2f, 0x76, 0x32, 0x2f, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x2e, 0x70, 0x75, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x8c, 0x01, 0x0a, 0x07, 0x4d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x12, 0x39, 0x0a, 0x09,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e,
    0x76, 0x32, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x52, 0x09, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x12, 0x46, 0x0a, 0x10, 0x73, 0x75, 0x72, 0x66, 0x61,
    0x63, 0x65, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52,
    0x0e, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x53, 0x70, 0x65, 0x65, 0x64, 0x4d, 0x73, 0x42,
    0xa7, 0x02, 0x0a, 0x23, 0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x61, 0x64, 0x73, 0x2e, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x42, 0x0f, 0x4d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c,
    0x50, 0x75, 0x62, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4e, 0x67, 0x68, 0x65, 0x2e,
    0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x64, 0x65, 0x76, 0x2f, 0x61, 0x6e, 0x64, 0x75,
    0x72, 0x69, 0x6c, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x61, 0x70, 0x69, 0x73, 0x2d,
    0x67, 0x6f, 0x2f, 0x61, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2f, 0x74, 0x61, 0x73, 0x6b, 0x73,
    0x2f, 0x61, 0x64, 0x73, 0x2f, 0x74, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72, 0x74, 0x79, 0x2f,
    0x76, 0x31, 0x3b, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x76, 0x31, 0xa2, 0x02, 0x04, 0x41, 0x54, 0x41,
    0x54, 0xaa, 0x02, 0x1f, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x2e, 0x54, 0x61, 0x73, 0x6b,
    0x73, 0x2e, 0x41, 0x64, 0x73, 0x2e, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72, 0x74, 0x79,
    0x2e, 0x56, 0x31, 0xca, 0x02, 0x1f, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c, 0x54, 0x61,
    0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x73, 0x5c, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70, 0x61, 0x72,
    0x74, 0x79, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x2b, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x5c,
    0x54, 0x61, 0x73, 0x6b, 0x73, 0x5c, 0x41, 0x64, 0x73, 0x5c, 0x54, 0x68, 0x69, 0x72, 0x64, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0xea, 0x02, 0x23, 0x41, 0x6e, 0x64, 0x75, 0x72, 0x69, 0x6c, 0x3a, 0x3a, 0x54,
    0x61, 0x73, 0x6b, 0x73, 0x3a, 0x3a, 0x41, 0x64, 0x73, 0x3a, 0x3a, 0x54, 0x68, 0x69, 0x72, 0x64,
    0x70, 0x61, 0x72, 0x74, 0x79, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xab, 0x03, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x28, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x65, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x08, 0x00, 0x65, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x9c, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x11, 0x00, 0x17, 0x01, 0x1a, 0x8f, 0x01, 0x20, 0x4d, 0x61, 0x70, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x42, 0x52, 0x45, 0x56, 0x49, 0x54, 0x59, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x4d, 0x61,
    0x72, 0x73, 0x68, 0x61, 0x6c, 0x2e, 0x0a, 0x20, 0x45, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69, 0x73,
    0x68, 0x28, 0x65, 0x64, 0x29, 0x20, 0x61, 0x74, 0x20, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x63, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2c, 0x20, 0x74, 0x79, 0x70, 0x69, 0x63,
    0x61, 0x6c, 0x6c, 0x79, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x6f, 0x73,
    0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x70,
    0x72, 0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61,
    0x6e, 0x20, 0x6f, 0x66, 0x66, 0x65, 0x6e, 0x73, 0x69, 0x76, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x11, 0x08, 0x0f, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x2b,
    0x1a, 0x1a, 0x20, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x4d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x20, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x1d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x13, 0x29, 0x2a, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x16, 0x02, 0x33, 0x1a, 0x18, 0x20, 0x53, 0x70, 0x65, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x4d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x16, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x1e, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x16, 0x31, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)