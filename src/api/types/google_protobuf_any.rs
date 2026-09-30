pub use crate::prelude::*;

/// Contains an arbitrary serialized message along with a @type that describes the type of the serialized message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GoogleProtobufAny {
    /// The type of the serialized message.
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl GoogleProtobufAny {
    pub fn builder() -> GoogleProtobufAnyBuilder {
        <GoogleProtobufAnyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoogleProtobufAnyBuilder {
    r#type: Option<String>,
}

impl GoogleProtobufAnyBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GoogleProtobufAny`].
    pub fn build(self) -> Result<GoogleProtobufAny, BuildError> {
        Ok(GoogleProtobufAny {
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
