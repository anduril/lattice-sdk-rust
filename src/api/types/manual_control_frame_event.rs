pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManualControlFrameEvent {
    #[serde(flatten)]
    pub manual_control_frame_fields: ManualControlFrame,
}

impl ManualControlFrameEvent {
    pub fn builder() -> ManualControlFrameEventBuilder {
        <ManualControlFrameEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualControlFrameEventBuilder {
    manual_control_frame_fields: Option<ManualControlFrame>,
}

impl ManualControlFrameEventBuilder {
    pub fn manual_control_frame_fields(mut self, value: ManualControlFrame) -> Self {
        self.manual_control_frame_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualControlFrameEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`manual_control_frame_fields`](ManualControlFrameEventBuilder::manual_control_frame_fields)
    pub fn build(self) -> Result<ManualControlFrameEvent, BuildError> {
        Ok(ManualControlFrameEvent {
            manual_control_frame_fields: self
                .manual_control_frame_fields
                .ok_or_else(|| BuildError::missing_field("manual_control_frame_fields"))?,
        })
    }
}
