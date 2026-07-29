pub use crate::prelude::*;

/// Indicators to describe entity to consumers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Indicators {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exercise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c2: Option<bool>,
    /// Indicates the Entity should be egressed to external sources.
    /// Integrations choose how the egressing happens (e.g. if an Entity needs fuzzing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egressable: Option<bool>,
    /// A signal of arbitrary importance such that the entity should be globally marked for all users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
}

impl Indicators {
    pub fn builder() -> IndicatorsBuilder {
        <IndicatorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IndicatorsBuilder {
    simulated: Option<bool>,
    exercise: Option<bool>,
    emergency: Option<bool>,
    c2: Option<bool>,
    egressable: Option<bool>,
    starred: Option<bool>,
}

impl IndicatorsBuilder {
    pub fn simulated(mut self, value: bool) -> Self {
        self.simulated = Some(value);
        self
    }

    pub fn exercise(mut self, value: bool) -> Self {
        self.exercise = Some(value);
        self
    }

    pub fn emergency(mut self, value: bool) -> Self {
        self.emergency = Some(value);
        self
    }

    pub fn c2(mut self, value: bool) -> Self {
        self.c2 = Some(value);
        self
    }

    pub fn egressable(mut self, value: bool) -> Self {
        self.egressable = Some(value);
        self
    }

    pub fn starred(mut self, value: bool) -> Self {
        self.starred = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Indicators`].
    pub fn build(self) -> Result<Indicators, BuildError> {
        Ok(Indicators {
            simulated: self.simulated,
            exercise: self.exercise,
            emergency: self.emergency,
            c2: self.c2,
            egressable: self.egressable,
            starred: self.starred,
        })
    }
}
