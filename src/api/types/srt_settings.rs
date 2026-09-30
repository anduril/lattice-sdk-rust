pub use crate::prelude::*;

/// The Lattice video service supports SRT protocol for push operations (ingress)
/// and pull operations (egress).
///
/// When configuring SRT for ingress, CreateIngressStreamResponse will
/// return to the user a url to push to which contains a unique `sessionId` to use
/// on the connection. If supplied, passphrase will be applied on incoming
/// connections.
///
/// When configuring SRT for egress, CreateEgressStreamResponse will
/// return to the user a url from which to pull a stream. Use the supplied
/// sessionId and passphrase in your StreamId if applicable.
/// See the SRT documentation on Access Control for more information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SrtSettings {
    /// Optional passphrase for the stream, set by the user, that applies AES encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
}

impl SrtSettings {
    pub fn builder() -> SrtSettingsBuilder {
        <SrtSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SrtSettingsBuilder {
    passphrase: Option<String>,
}

impl SrtSettingsBuilder {
    pub fn passphrase(mut self, value: impl Into<String>) -> Self {
        self.passphrase = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SrtSettings`].
    pub fn build(self) -> Result<SrtSettings, BuildError> {
        Ok(SrtSettings {
            passphrase: self.passphrase,
        })
    }
}
