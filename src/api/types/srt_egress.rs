pub use crate::prelude::*;

/// SRT egress connection details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SrtEgress {
    /// The URL on which Lattice listens. The downstream consumer pulls from this URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Unique session identifier the consumer must supply on the SRT connection.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl SrtEgress {
    pub fn builder() -> SrtEgressBuilder {
        <SrtEgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SrtEgressBuilder {
    url: Option<String>,
    session_id: Option<String>,
}

impl SrtEgressBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SrtEgress`].
    pub fn build(self) -> Result<SrtEgress, BuildError> {
        Ok(SrtEgress {
            url: self.url,
            session_id: self.session_id,
        })
    }
}
