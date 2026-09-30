pub use crate::prelude::*;

/// The entity object represents a single known object within the Lattice operational environment. It contains
/// all data associated with the entity, such as its name, ID, and other relevant components.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Entity {
    /// A Globally Unique Identifier (GUID) for your entity. This is a required
    /// field.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// A human-readable entity description that's helpful for debugging purposes and human
    /// traceability. If this field is empty, the Entity Manager API generates one for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates the entity is active and should have a lifecycle state of CREATE or UPDATE.
    /// Set this field to true when publishing an entity.
    #[serde(rename = "isLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    /// The time when the entity was first known to the entity producer. If this field is empty, the Entity Manager API uses the
    /// current timestamp of when the entity is first received.
    /// For example, when a drone is first powered on, it might report its startup time as the created time.
    /// The timestamp doesn't change for the lifetime of an entity.
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_time: Option<DateTime<FixedOffset>>,
    /// Future time that expires an entity and updates the is_live flag.
    /// For entities that are constantly updating, the expiry time also updates.
    /// In some cases, this may differ from is_live.
    /// Example: Entities with tasks exported to an external system must remain
    /// active even after they expire.
    /// This field is required when publishing a prepopulated entity.
    /// The expiry time must be in the future, but less than 30 days from the current time.
    #[serde(rename = "expiryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiry_time: Option<DateTime<FixedOffset>>,
    /// Use noExpiry only when the entity contains information that should be available to other
    /// tasks or integrations beyond its immediate operational context. For example, use noExpiry
    /// for long-living geographical entities that maintain persistent relevance across multiple
    /// operations or tasks.
    #[serde(rename = "noExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_expiry: Option<bool>,
    /// Human-readable descriptions of what the entity is currently doing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Geospatial data related to the entity, including its position, kinematics, and orientation. Populate either
    /// this field (and `location_uncertainty`) or `kinematics`, not both. Populating both can lead to conflicting or
    /// inconsistent kinematics data for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Indicates uncertainty of the entity's position and kinematics.
    #[serde(rename = "locationUncertainty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uncertainty: Option<LocationUncertainty>,
    /// Kinematics data related to the entity to a higher degree of granularity than Location. This is preferred for Track Entities.
    /// Populate either `location`/`location_uncertainty` or this field, not both.
    /// Populating both can lead to conflicting or inconsistent kinematics data for the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinematics: Option<Kinematics>,
    /// Geospatial representation of the entity, including entities that cover an area rather than a fixed point.
    #[serde(rename = "geoShape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_shape: Option<GeoShape>,
    /// Additional details on what the geospatial area or point represents, along with visual display details.
    #[serde(rename = "geoDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_details: Option<GeoDetails>,
    /// Entity name displayed in the Lattice UI side panel. Also includes identifiers that other systems can use to reference the same entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,
    /// If this entity is tracked by another entity, this component contains data related to how it's being tracked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked: Option<Tracked>,
    /// If this entity has been correlated or decorrelated to another one, this component contains information on the correlation or decorrelation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation: Option<Correlation>,
    /// View of the entity.
    #[serde(rename = "milView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mil_view: Option<MilView>,
    /// Ontology defines an entity's categorization in Lattice, and improves data retrieval and integration. Builds a standardized representation of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology: Option<Ontology>,
    /// Details an entity's available sensors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Sensors>,
    /// Details an entity's available payloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Payloads>,
    /// Details the entity's power source.
    #[serde(rename = "powerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_state: Option<PowerState>,
    /// The primary data source provenance for this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
    /// Provenance of override data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Box<Overrides>>,
    /// Describes an entity's specific characteristics and the operations that can be performed on the entity.
    /// For example, "simulated" informs the operator that the entity is from a simulation, and "deletable"
    /// informs the operator (and system) that the delete operation is valid against the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicators: Option<Indicators>,
    /// The prioritization associated with an entity, such as if it's a threat or a high-value target.
    #[serde(rename = "targetPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_priority: Option<TargetPriority>,
    /// Describes an entity's signal characteristics, primarily used when an entity is a signal of interest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<Signal>,
    /// A message describing any transponder codes associated with Mode 1, 2, 3, 4, 5, S, C interrogations. These are related to ADS-B modes.
    #[serde(rename = "transponderCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transponder_codes: Option<TransponderCodes>,
    /// Describes an entity's security classification levels at an overall classification level and on a per
    /// field level.
    #[serde(rename = "dataClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification: Option<Classification>,
    /// A catalog of tasks that can be performed by an entity.
    #[serde(rename = "taskCatalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_catalog: Option<TaskCatalog>,
    /// Media associated with an entity, such as videos, images, or thumbnails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// The relationships between this entity and other entities in the common operational picture (COP).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
    /// Visual details associated with the display of an entity in the client.
    #[serde(rename = "visualDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_details: Option<VisualDetails>,
    /// Physical dimensions of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    /// Additional information about an entity's route.
    #[serde(rename = "routeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_details: Option<RouteDetails>,
    /// Schedules associated with this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Schedules>,
    /// Health metrics or connection status reported by the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,
    /// Details for the group associated with this entity.
    #[serde(rename = "groupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_details: Option<GroupDetails>,
    /// Contains relevant supply information for the entity, such as fuel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplies: Option<Supplies>,
    /// Orbit information for space objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<Orbit>,
    /// Symbology/iconography for the entity respecting an existing standard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbology: Option<Symbology>,
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        <EntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityBuilder {
    entity_id: Option<String>,
    description: Option<String>,
    is_live: Option<bool>,
    created_time: Option<DateTime<FixedOffset>>,
    expiry_time: Option<DateTime<FixedOffset>>,
    no_expiry: Option<bool>,
    status: Option<Status>,
    location: Option<Location>,
    location_uncertainty: Option<LocationUncertainty>,
    kinematics: Option<Kinematics>,
    geo_shape: Option<GeoShape>,
    geo_details: Option<GeoDetails>,
    aliases: Option<Aliases>,
    tracked: Option<Tracked>,
    correlation: Option<Correlation>,
    mil_view: Option<MilView>,
    ontology: Option<Ontology>,
    sensors: Option<Sensors>,
    payloads: Option<Payloads>,
    power_state: Option<PowerState>,
    provenance: Option<Provenance>,
    overrides: Option<Box<Overrides>>,
    indicators: Option<Indicators>,
    target_priority: Option<TargetPriority>,
    signal: Option<Signal>,
    transponder_codes: Option<TransponderCodes>,
    data_classification: Option<Classification>,
    task_catalog: Option<TaskCatalog>,
    media: Option<Media>,
    relationships: Option<Relationships>,
    visual_details: Option<VisualDetails>,
    dimensions: Option<Dimensions>,
    route_details: Option<RouteDetails>,
    schedules: Option<Schedules>,
    health: Option<Health>,
    group_details: Option<GroupDetails>,
    supplies: Option<Supplies>,
    orbit: Option<Orbit>,
    symbology: Option<Symbology>,
}

impl EntityBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_live(mut self, value: bool) -> Self {
        self.is_live = Some(value);
        self
    }

    pub fn created_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_time = Some(value);
        self
    }

    pub fn expiry_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiry_time = Some(value);
        self
    }

    pub fn no_expiry(mut self, value: bool) -> Self {
        self.no_expiry = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn location(mut self, value: Location) -> Self {
        self.location = Some(value);
        self
    }

    pub fn location_uncertainty(mut self, value: LocationUncertainty) -> Self {
        self.location_uncertainty = Some(value);
        self
    }

    pub fn kinematics(mut self, value: Kinematics) -> Self {
        self.kinematics = Some(value);
        self
    }

    pub fn geo_shape(mut self, value: GeoShape) -> Self {
        self.geo_shape = Some(value);
        self
    }

    pub fn geo_details(mut self, value: GeoDetails) -> Self {
        self.geo_details = Some(value);
        self
    }

    pub fn aliases(mut self, value: Aliases) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn tracked(mut self, value: Tracked) -> Self {
        self.tracked = Some(value);
        self
    }

    pub fn correlation(mut self, value: Correlation) -> Self {
        self.correlation = Some(value);
        self
    }

    pub fn mil_view(mut self, value: MilView) -> Self {
        self.mil_view = Some(value);
        self
    }

    pub fn ontology(mut self, value: Ontology) -> Self {
        self.ontology = Some(value);
        self
    }

    pub fn sensors(mut self, value: Sensors) -> Self {
        self.sensors = Some(value);
        self
    }

    pub fn payloads(mut self, value: Payloads) -> Self {
        self.payloads = Some(value);
        self
    }

    pub fn power_state(mut self, value: PowerState) -> Self {
        self.power_state = Some(value);
        self
    }

    pub fn provenance(mut self, value: Provenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn overrides(mut self, value: Box<Overrides>) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn indicators(mut self, value: Indicators) -> Self {
        self.indicators = Some(value);
        self
    }

    pub fn target_priority(mut self, value: TargetPriority) -> Self {
        self.target_priority = Some(value);
        self
    }

    pub fn signal(mut self, value: Signal) -> Self {
        self.signal = Some(value);
        self
    }

    pub fn transponder_codes(mut self, value: TransponderCodes) -> Self {
        self.transponder_codes = Some(value);
        self
    }

    pub fn data_classification(mut self, value: Classification) -> Self {
        self.data_classification = Some(value);
        self
    }

    pub fn task_catalog(mut self, value: TaskCatalog) -> Self {
        self.task_catalog = Some(value);
        self
    }

    pub fn media(mut self, value: Media) -> Self {
        self.media = Some(value);
        self
    }

    pub fn relationships(mut self, value: Relationships) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn visual_details(mut self, value: VisualDetails) -> Self {
        self.visual_details = Some(value);
        self
    }

    pub fn dimensions(mut self, value: Dimensions) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn route_details(mut self, value: RouteDetails) -> Self {
        self.route_details = Some(value);
        self
    }

    pub fn schedules(mut self, value: Schedules) -> Self {
        self.schedules = Some(value);
        self
    }

    pub fn health(mut self, value: Health) -> Self {
        self.health = Some(value);
        self
    }

    pub fn group_details(mut self, value: GroupDetails) -> Self {
        self.group_details = Some(value);
        self
    }

    pub fn supplies(mut self, value: Supplies) -> Self {
        self.supplies = Some(value);
        self
    }

    pub fn orbit(mut self, value: Orbit) -> Self {
        self.orbit = Some(value);
        self
    }

    pub fn symbology(mut self, value: Symbology) -> Self {
        self.symbology = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Entity`].
    pub fn build(self) -> Result<Entity, BuildError> {
        Ok(Entity {
            entity_id: self.entity_id,
            description: self.description,
            is_live: self.is_live,
            created_time: self.created_time,
            expiry_time: self.expiry_time,
            no_expiry: self.no_expiry,
            status: self.status,
            location: self.location,
            location_uncertainty: self.location_uncertainty,
            kinematics: self.kinematics,
            geo_shape: self.geo_shape,
            geo_details: self.geo_details,
            aliases: self.aliases,
            tracked: self.tracked,
            correlation: self.correlation,
            mil_view: self.mil_view,
            ontology: self.ontology,
            sensors: self.sensors,
            payloads: self.payloads,
            power_state: self.power_state,
            provenance: self.provenance,
            overrides: self.overrides,
            indicators: self.indicators,
            target_priority: self.target_priority,
            signal: self.signal,
            transponder_codes: self.transponder_codes,
            data_classification: self.data_classification,
            task_catalog: self.task_catalog,
            media: self.media,
            relationships: self.relationships,
            visual_details: self.visual_details,
            dimensions: self.dimensions,
            route_details: self.route_details,
            schedules: self.schedules,
            health: self.health,
            group_details: self.group_details,
            supplies: self.supplies,
            orbit: self.orbit,
            symbology: self.symbology,
        })
    }
}
