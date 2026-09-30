pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteIngressStreamResponse {}

impl DeleteIngressStreamResponse {
    pub fn builder() -> DeleteIngressStreamResponseBuilder {
        <DeleteIngressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteIngressStreamResponseBuilder {}

impl DeleteIngressStreamResponseBuilder {
    /// Consumes the builder and constructs a [`DeleteIngressStreamResponse`].
    pub fn build(self) -> Result<DeleteIngressStreamResponse, BuildError> {
        Ok(DeleteIngressStreamResponse {})
    }
}
