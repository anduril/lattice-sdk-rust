pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 255].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub red: Option<f64>,
    /// The amount of green in the color as a value in the interval [0, 255].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub green: Option<f64>,
    /// The amount of blue in the color as a value in the interval [0, 255].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub blue: Option<f64>,
    /// The fraction of this color that should be applied to the pixel. That is,
    /// the final pixel color is defined by the equation:
    ///
    /// `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)`
    ///
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color. This
    /// uses a wrapper message rather than a simple float scalar so that it is
    /// possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is rendered as a solid color
    /// (as if the alpha value had been explicitly given a value of 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alpha: Option<f64>,
}

impl Color {
    pub fn builder() -> ColorBuilder {
        <ColorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ColorBuilder {
    red: Option<f64>,
    green: Option<f64>,
    blue: Option<f64>,
    alpha: Option<f64>,
}

impl ColorBuilder {
    pub fn red(mut self, value: f64) -> Self {
        self.red = Some(value);
        self
    }

    pub fn green(mut self, value: f64) -> Self {
        self.green = Some(value);
        self
    }

    pub fn blue(mut self, value: f64) -> Self {
        self.blue = Some(value);
        self
    }

    pub fn alpha(mut self, value: f64) -> Self {
        self.alpha = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Color`].
    pub fn build(self) -> Result<Color, BuildError> {
        Ok(Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        })
    }
}
