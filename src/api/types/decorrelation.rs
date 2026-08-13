pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Decorrelation {
    /// This will be specified if this entity was decorrelated against all other entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<DecorrelatedAll>,
    /// A list of decorrelated entities that have been explicitly decorrelated against this entity
    /// which prevents lower precedence correlations from overriding it in the future.
    /// For example, if an operator in the UI decorrelated tracks A and B, any automated
    /// correlators would be unable to correlate them since manual decorrelations have
    /// higher precedence than automatic ones. Precedence is determined by both correlation
    /// type and replication mode.
    #[serde(rename = "decorrelatedEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorrelated_entities: Option<Vec<DecorrelatedSingle>>,
}

impl Decorrelation {
    pub fn builder() -> DecorrelationBuilder {
        <DecorrelationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecorrelationBuilder {
    all: Option<DecorrelatedAll>,
    decorrelated_entities: Option<Vec<DecorrelatedSingle>>,
}

impl DecorrelationBuilder {
    pub fn all(mut self, value: DecorrelatedAll) -> Self {
        self.all = Some(value);
        self
    }

    pub fn decorrelated_entities(mut self, value: Vec<DecorrelatedSingle>) -> Self {
        self.decorrelated_entities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Decorrelation`].
    pub fn build(self) -> Result<Decorrelation, BuildError> {
        Ok(Decorrelation {
            all: self.all,
            decorrelated_entities: self.decorrelated_entities,
        })
    }
}
