pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManualControlFrameRequest {
    /// Unix timestamp in microseconds identifying the control session. The client generates this
    /// value when it begins a control session, and includes the same value in every request for
    /// that session. Higher values replace previous sessions. Lower values are rejected as stale.
    #[serde(rename = "sessionEpochMicros")]
    #[serde(default)]
    pub session_epoch_micros: String,
    /// The sequence number of this frame within the current session. The client must increase it
    /// with each frame so that agents can detect out-of-order delivery. Reset it to 1 when starting a
    /// new session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    /// The control instructions for this frame. The format is specific to the task, and is passed
    /// through to the agent without being inspected by Lattice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
}

impl ManualControlFrameRequest {
    pub fn builder() -> ManualControlFrameRequestBuilder {
        <ManualControlFrameRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualControlFrameRequestBuilder {
    session_epoch_micros: Option<String>,
    sequence: Option<String>,
    specification: Option<GoogleProtobufAny>,
}

impl ManualControlFrameRequestBuilder {
    pub fn session_epoch_micros(mut self, value: impl Into<String>) -> Self {
        self.session_epoch_micros = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: impl Into<String>) -> Self {
        self.sequence = Some(value.into());
        self
    }

    pub fn specification(mut self, value: GoogleProtobufAny) -> Self {
        self.specification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualControlFrameRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_epoch_micros`](ManualControlFrameRequestBuilder::session_epoch_micros)
    pub fn build(self) -> Result<ManualControlFrameRequest, BuildError> {
        Ok(ManualControlFrameRequest {
            session_epoch_micros: self
                .session_epoch_micros
                .ok_or_else(|| BuildError::missing_field("session_epoch_micros"))?,
            sequence: self.sequence,
            specification: self.specification,
        })
    }
}
