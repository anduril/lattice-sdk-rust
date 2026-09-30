pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrbitMeanElementsMetadata {
    /// Creation date/time in UTC
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub creation_date: Option<DateTime<FixedOffset>>,
    /// Creating agency or operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,
    /// ID that uniquely identifies a message from a given originator.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Reference frame, assumed to be Earth-centered
    #[serde(rename = "refFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_frame: Option<OrbitMeanElementsMetadataRefFrame>,
    /// Reference frame epoch in UTC - mandatory only if not intrinsic to frame definition
    #[serde(rename = "refFrameEpoch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub ref_frame_epoch: Option<DateTime<FixedOffset>>,
    #[serde(rename = "meanElementTheory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_element_theory: Option<OrbitMeanElementsMetadataMeanElementTheory>,
}

impl OrbitMeanElementsMetadata {
    pub fn builder() -> OrbitMeanElementsMetadataBuilder {
        <OrbitMeanElementsMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrbitMeanElementsMetadataBuilder {
    creation_date: Option<DateTime<FixedOffset>>,
    originator: Option<String>,
    message_id: Option<String>,
    ref_frame: Option<OrbitMeanElementsMetadataRefFrame>,
    ref_frame_epoch: Option<DateTime<FixedOffset>>,
    mean_element_theory: Option<OrbitMeanElementsMetadataMeanElementTheory>,
}

impl OrbitMeanElementsMetadataBuilder {
    pub fn creation_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn originator(mut self, value: impl Into<String>) -> Self {
        self.originator = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn ref_frame(mut self, value: OrbitMeanElementsMetadataRefFrame) -> Self {
        self.ref_frame = Some(value);
        self
    }

    pub fn ref_frame_epoch(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ref_frame_epoch = Some(value);
        self
    }

    pub fn mean_element_theory(
        mut self,
        value: OrbitMeanElementsMetadataMeanElementTheory,
    ) -> Self {
        self.mean_element_theory = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrbitMeanElementsMetadata`].
    pub fn build(self) -> Result<OrbitMeanElementsMetadata, BuildError> {
        Ok(OrbitMeanElementsMetadata {
            creation_date: self.creation_date,
            originator: self.originator,
            message_id: self.message_id,
            ref_frame: self.ref_frame,
            ref_frame_epoch: self.ref_frame_epoch,
            mean_element_theory: self.mean_element_theory,
        })
    }
}
