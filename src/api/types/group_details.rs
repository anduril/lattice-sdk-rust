pub use crate::prelude::*;

/// Details related to grouping for this entity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echelon: Option<Echelon>,
}

impl GroupDetails {
    pub fn builder() -> GroupDetailsBuilder {
        <GroupDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupDetailsBuilder {
    team: Option<Team>,
    echelon: Option<Echelon>,
}

impl GroupDetailsBuilder {
    pub fn team(mut self, value: Team) -> Self {
        self.team = Some(value);
        self
    }

    pub fn echelon(mut self, value: Echelon) -> Self {
        self.echelon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupDetails`].
    pub fn build(self) -> Result<GroupDetails, BuildError> {
        Ok(GroupDetails {
            team: self.team,
            echelon: self.echelon,
        })
    }
}
