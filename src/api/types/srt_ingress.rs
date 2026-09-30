pub use crate::prelude::*;

/// SRT ingress connection details. Returned to the producer so it knows where to
/// push the stream.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SrtIngress {
    /// The URL the producer should push the SRT stream to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Unique session identifier the producer must include on the SRT connection. See
    /// SrtSettings for context.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl SrtIngress {
    pub fn builder() -> SrtIngressBuilder {
        <SrtIngressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SrtIngressBuilder {
    url: Option<String>,
    session_id: Option<String>,
}

impl SrtIngressBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SrtIngress`].
    pub fn build(self) -> Result<SrtIngress, BuildError> {
        Ok(SrtIngress {
            url: self.url,
            session_id: self.session_id,
        })
    }
}
