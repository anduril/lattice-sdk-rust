pub use crate::prelude::*;

/// A representation of a single emitter notation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmitterNotation {
    #[serde(rename = "emitterNotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitter_notation: Option<String>,
    /// confidence as a percentage that the emitter notation in this component is accurate
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub confidence: Option<f64>,
}

impl EmitterNotation {
    pub fn builder() -> EmitterNotationBuilder {
        <EmitterNotationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmitterNotationBuilder {
    emitter_notation: Option<String>,
    confidence: Option<f64>,
}

impl EmitterNotationBuilder {
    pub fn emitter_notation(mut self, value: impl Into<String>) -> Self {
        self.emitter_notation = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmitterNotation`].
    pub fn build(self) -> Result<EmitterNotation, BuildError> {
        Ok(EmitterNotation {
            emitter_notation: self.emitter_notation,
            confidence: self.confidence,
        })
    }
}
