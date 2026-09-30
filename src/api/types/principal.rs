pub use crate::prelude::*;

/// A Principal is an entity that has authority over this task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Principal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<System>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
    /// The Principal _this_ Principal is acting on behalf of.
    ///
    /// Likely only populated once in the nesting (i.e. the "on_behalf_of" Principal would not have another "on_behalf_of" in most cases).
    #[serde(rename = "onBehalfOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<Principal>>,
}

impl Principal {
    pub fn builder() -> PrincipalBuilder {
        <PrincipalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrincipalBuilder {
    system: Option<System>,
    user: Option<User>,
    team: Option<Team>,
    on_behalf_of: Option<Box<Principal>>,
}

impl PrincipalBuilder {
    pub fn system(mut self, value: System) -> Self {
        self.system = Some(value);
        self
    }

    pub fn user(mut self, value: User) -> Self {
        self.user = Some(value);
        self
    }

    pub fn team(mut self, value: Team) -> Self {
        self.team = Some(value);
        self
    }

    pub fn on_behalf_of(mut self, value: Box<Principal>) -> Self {
        self.on_behalf_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Principal`].
    pub fn build(self) -> Result<Principal, BuildError> {
        Ok(Principal {
            system: self.system,
            user: self.user,
            team: self.team,
            on_behalf_of: self.on_behalf_of,
        })
    }
}
