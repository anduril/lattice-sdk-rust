pub use crate::prelude::*;

/// The Value represents the information against which an entity field is evaluated. It is one of
/// a fixed set of types, each of which correspond to specific comparators. See "ComparatorType"
/// for the full list of Value <-> Comparator mappings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Value {
    #[serde(rename = "booleanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_type: Option<BooleanType>,
    #[serde(rename = "numericType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_type: Option<NumericType>,
    #[serde(rename = "stringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_type: Option<StringType>,
    #[serde(rename = "enumType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<EnumType>,
    #[serde(rename = "timestampType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_type: Option<TimestampType>,
    #[serde(rename = "boundedShapeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_shape_type: Option<BoundedShapeType>,
    #[serde(rename = "positionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_type: Option<PositionType>,
    #[serde(rename = "headingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_type: Option<HeadingType>,
    #[serde(rename = "listType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_type: Option<Box<ListType>>,
    #[serde(rename = "rangeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_type: Option<RangeType>,
}

impl Value {
    pub fn builder() -> ValueBuilder {
        <ValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValueBuilder {
    boolean_type: Option<BooleanType>,
    numeric_type: Option<NumericType>,
    string_type: Option<StringType>,
    enum_type: Option<EnumType>,
    timestamp_type: Option<TimestampType>,
    bounded_shape_type: Option<BoundedShapeType>,
    position_type: Option<PositionType>,
    heading_type: Option<HeadingType>,
    list_type: Option<Box<ListType>>,
    range_type: Option<RangeType>,
}

impl ValueBuilder {
    pub fn boolean_type(mut self, value: BooleanType) -> Self {
        self.boolean_type = Some(value);
        self
    }

    pub fn numeric_type(mut self, value: NumericType) -> Self {
        self.numeric_type = Some(value);
        self
    }

    pub fn string_type(mut self, value: StringType) -> Self {
        self.string_type = Some(value);
        self
    }

    pub fn enum_type(mut self, value: EnumType) -> Self {
        self.enum_type = Some(value);
        self
    }

    pub fn timestamp_type(mut self, value: TimestampType) -> Self {
        self.timestamp_type = Some(value);
        self
    }

    pub fn bounded_shape_type(mut self, value: BoundedShapeType) -> Self {
        self.bounded_shape_type = Some(value);
        self
    }

    pub fn position_type(mut self, value: PositionType) -> Self {
        self.position_type = Some(value);
        self
    }

    pub fn heading_type(mut self, value: HeadingType) -> Self {
        self.heading_type = Some(value);
        self
    }

    pub fn list_type(mut self, value: Box<ListType>) -> Self {
        self.list_type = Some(value);
        self
    }

    pub fn range_type(mut self, value: RangeType) -> Self {
        self.range_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Value`].
    pub fn build(self) -> Result<Value, BuildError> {
        Ok(Value {
            boolean_type: self.boolean_type,
            numeric_type: self.numeric_type,
            string_type: self.string_type,
            enum_type: self.enum_type,
            timestamp_type: self.timestamp_type,
            bounded_shape_type: self.bounded_shape_type,
            position_type: self.position_type,
            heading_type: self.heading_type,
            list_type: self.list_type,
            range_type: self.range_type,
        })
    }
}
