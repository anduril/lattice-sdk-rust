pub use crate::prelude::*;

/// Determines the type of relationship between this entity and another.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RelationshipType {
    #[serde(rename = "trackedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_by: Option<TrackedBy>,
    #[serde(rename = "groupChild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_child: Option<GroupChild>,
    #[serde(rename = "groupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_parent: Option<GroupParent>,
    #[serde(rename = "mergedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_from: Option<MergedFrom>,
    #[serde(rename = "activeTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_target: Option<ActiveTarget>,
}

impl RelationshipType {
    pub fn builder() -> RelationshipTypeBuilder {
        <RelationshipTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RelationshipTypeBuilder {
    tracked_by: Option<TrackedBy>,
    group_child: Option<GroupChild>,
    group_parent: Option<GroupParent>,
    merged_from: Option<MergedFrom>,
    active_target: Option<ActiveTarget>,
}

impl RelationshipTypeBuilder {
    pub fn tracked_by(mut self, value: TrackedBy) -> Self {
        self.tracked_by = Some(value);
        self
    }

    pub fn group_child(mut self, value: GroupChild) -> Self {
        self.group_child = Some(value);
        self
    }

    pub fn group_parent(mut self, value: GroupParent) -> Self {
        self.group_parent = Some(value);
        self
    }

    pub fn merged_from(mut self, value: MergedFrom) -> Self {
        self.merged_from = Some(value);
        self
    }

    pub fn active_target(mut self, value: ActiveTarget) -> Self {
        self.active_target = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RelationshipType`].
    pub fn build(self) -> Result<RelationshipType, BuildError> {
        Ok(RelationshipType {
            tracked_by: self.tracked_by,
            group_child: self.group_child,
            group_parent: self.group_parent,
            merged_from: self.merged_from,
            active_target: self.active_target,
        })
    }
}
