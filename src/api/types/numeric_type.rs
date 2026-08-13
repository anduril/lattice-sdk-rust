pub use crate::prelude::*;

/// The NumericType represents static numeric values. It supports all numeric primitives supported
/// by the proto3 language specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NumericType {
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub double_value: Option<f64>,
    #[serde(rename = "floatValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub float_value: Option<f64>,
    #[serde(rename = "int32Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int32value: Option<i64>,
    #[serde(rename = "int64Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int64value: Option<String>,
    #[serde(rename = "uint32Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uint32value: Option<i64>,
    #[serde(rename = "uint64Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uint64value: Option<String>,
}

impl NumericType {
    pub fn builder() -> NumericTypeBuilder {
        <NumericTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NumericTypeBuilder {
    double_value: Option<f64>,
    float_value: Option<f64>,
    int32value: Option<i64>,
    int64value: Option<String>,
    uint32value: Option<i64>,
    uint64value: Option<String>,
}

impl NumericTypeBuilder {
    pub fn double_value(mut self, value: f64) -> Self {
        self.double_value = Some(value);
        self
    }

    pub fn float_value(mut self, value: f64) -> Self {
        self.float_value = Some(value);
        self
    }

    pub fn int32value(mut self, value: i64) -> Self {
        self.int32value = Some(value);
        self
    }

    pub fn int64value(mut self, value: impl Into<String>) -> Self {
        self.int64value = Some(value.into());
        self
    }

    pub fn uint32value(mut self, value: i64) -> Self {
        self.uint32value = Some(value);
        self
    }

    pub fn uint64value(mut self, value: impl Into<String>) -> Self {
        self.uint64value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NumericType`].
    pub fn build(self) -> Result<NumericType, BuildError> {
        Ok(NumericType {
            double_value: self.double_value,
            float_value: self.float_value,
            int32value: self.int32value,
            int64value: self.int64value,
            uint32value: self.uint32value,
            uint64value: self.uint64value,
        })
    }
}
