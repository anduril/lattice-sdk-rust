pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEgressStreamResponse {}

impl DeleteEgressStreamResponse {
    pub fn builder() -> DeleteEgressStreamResponseBuilder {
        <DeleteEgressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEgressStreamResponseBuilder {}

impl DeleteEgressStreamResponseBuilder {
    /// Consumes the builder and constructs a [`DeleteEgressStreamResponse`].
    pub fn build(self) -> Result<DeleteEgressStreamResponse, BuildError> {
        Ok(DeleteEgressStreamResponse {})
    }
}
