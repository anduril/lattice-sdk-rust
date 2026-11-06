pub use crate::prelude::*;

/// Chooses which streams QueryIngressStreams returns.
///
/// A stream must match every field you set. For a field that takes a list, the
/// stream only needs to match one value in it. Fields you leave out don't limit
/// the results. The server rejects unrecognized values, so a typo never quietly
/// returns more streams than you asked for. The filter only works within the
/// streams you already have permission to see.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngressStreamFilter {
    /// Returns only streams that match by who created them. Leave it out to include
    /// every creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<CreatorFilter>,
    /// Returns only streams whose ingress ID matches. Compares against the part of
    /// `ingress_id` before any `#`. For a stream you created, that's the ID you chose.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<StringFilter>,
    /// Returns only streams whose title matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<StringFilter>,
    /// Returns only streams in one of these statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<IngressStreamFilterStatusesItem>>,
}

impl IngressStreamFilter {
    pub fn builder() -> IngressStreamFilterBuilder {
        <IngressStreamFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngressStreamFilterBuilder {
    creator: Option<CreatorFilter>,
    ingress_id: Option<StringFilter>,
    title: Option<StringFilter>,
    statuses: Option<Vec<IngressStreamFilterStatusesItem>>,
}

impl IngressStreamFilterBuilder {
    pub fn creator(mut self, value: CreatorFilter) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn ingress_id(mut self, value: StringFilter) -> Self {
        self.ingress_id = Some(value);
        self
    }

    pub fn title(mut self, value: StringFilter) -> Self {
        self.title = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<IngressStreamFilterStatusesItem>) -> Self {
        self.statuses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngressStreamFilter`].
    pub fn build(self) -> Result<IngressStreamFilter, BuildError> {
        Ok(IngressStreamFilter {
            creator: self.creator,
            ingress_id: self.ingress_id,
            title: self.title,
            statuses: self.statuses,
        })
    }
}
