pub use crate::prelude::*;

/// A single frame of manual control input forwarded by Lattice to an agent.
///
/// When an operator sends manual control input, for example, joystick movements using
/// `SendManualControlFrames`, Lattice packages each input into a `ManualControlFrame`
/// and forwards it to the executing agent via the `ListenForManualControlFrames`
/// streaming RPC.
///
/// Each frame carries sequencing metadata to support concurrent control sessions,
/// detect stale frames, and ensure proper ordering.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManualControlFrame {
    /// The ID of the manual control task this frame belongs to.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Unix timestamp in microseconds identifying the control session.
    /// Increments each time a client opens a new stream for this task.
    /// Agents should ignore frames with a lower epoch to handle stale streams
    /// or operator handoffs.
    #[serde(rename = "epochMicros")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_micros: Option<String>,
    /// The sequence number for a stream, incremented for each frame.
    /// Agents can use this to detect out-of-order delivery within the same epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    /// The time at which this frame was created.
    /// Agents can use this to detect stale frame data.
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub creation_time: Option<DateTime<FixedOffset>>,
    /// The control instructions for this frame, passed through from the client.
    /// The format of each task is specific to the task, and not visible to Lattice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
}

impl ManualControlFrame {
    pub fn builder() -> ManualControlFrameBuilder {
        <ManualControlFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualControlFrameBuilder {
    task_id: Option<String>,
    epoch_micros: Option<String>,
    sequence: Option<String>,
    creation_time: Option<DateTime<FixedOffset>>,
    specification: Option<GoogleProtobufAny>,
}

impl ManualControlFrameBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn epoch_micros(mut self, value: impl Into<String>) -> Self {
        self.epoch_micros = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: impl Into<String>) -> Self {
        self.sequence = Some(value.into());
        self
    }

    pub fn creation_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.creation_time = Some(value);
        self
    }

    pub fn specification(mut self, value: GoogleProtobufAny) -> Self {
        self.specification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualControlFrame`].
    pub fn build(self) -> Result<ManualControlFrame, BuildError> {
        Ok(ManualControlFrame {
            task_id: self.task_id,
            epoch_micros: self.epoch_micros,
            sequence: self.sequence,
            creation_time: self.creation_time,
            specification: self.specification,
        })
    }
}
