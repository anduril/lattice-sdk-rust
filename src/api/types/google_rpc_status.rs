pub use crate::prelude::*;

/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of [google.rpc.Code][google.rpc.Code].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the [google.rpc.Status.details][google.rpc.Status.details] field, or localized by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A list of messages that carry the error details.  There is a common set of message types for APIs to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GoogleProtobufAny>>,
}

impl GoogleRpcStatus {
    pub fn builder() -> GoogleRpcStatusBuilder {
        <GoogleRpcStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoogleRpcStatusBuilder {
    code: Option<i64>,
    message: Option<String>,
    details: Option<Vec<GoogleProtobufAny>>,
}

impl GoogleRpcStatusBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn details(mut self, value: Vec<GoogleProtobufAny>) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoogleRpcStatus`].
    pub fn build(self) -> Result<GoogleRpcStatus, BuildError> {
        Ok(GoogleRpcStatus {
            code: self.code,
            message: self.message,
            details: self.details,
        })
    }
}
