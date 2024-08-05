// @generated
impl serde::Serialize for Agent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Agent", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Agent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "entity_id",
            "entityId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            EntityId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Agent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Agent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Agent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut entity_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Agent {
                    asset_id: asset_id__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Agent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AltitudeConstraint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0. {
            len += 1;
        }
        if self.max != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.AltitudeConstraint", len)?;
        if self.min != 0. {
            struct_ser.serialize_field("min", &self.min)?;
        }
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AltitudeConstraint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AltitudeConstraint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.AltitudeConstraint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AltitudeConstraint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AltitudeConstraint {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.AltitudeConstraint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnglePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0. {
            len += 1;
        }
        if self.max != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.AnglePair", len)?;
        if self.min != 0. {
            struct_ser.serialize_field("min", &self.min)?;
        }
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnglePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnglePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.AnglePair")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnglePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AnglePair {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.AnglePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaConstraints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.altitude_constraint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.AreaConstraints", len)?;
        if let Some(v) = self.altitude_constraint.as_ref() {
            struct_ser.serialize_field("altitudeConstraint", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaConstraints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "altitude_constraint",
            "altitudeConstraint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AltitudeConstraint,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "altitudeConstraint" | "altitude_constraint" => Ok(GeneratedField::AltitudeConstraint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaConstraints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.AreaConstraints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AreaConstraints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut altitude_constraint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AltitudeConstraint => {
                            if altitude_constraint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitudeConstraint"));
                            }
                            altitude_constraint__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AreaConstraints {
                    altitude_constraint: altitude_constraint__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.AreaConstraints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaSearch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if !self.priors.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if !self.control_areas.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.AreaSearch", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if !self.priors.is_empty() {
            struct_ser.serialize_field("priors", &self.priors)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if !self.control_areas.is_empty() {
            struct_ser.serialize_field("controlAreas", &self.control_areas)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaSearch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "priors",
            "participants",
            "control_areas",
            "controlAreas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Priors,
            Participants,
            ControlAreas,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "priors" => Ok(GeneratedField::Priors),
                            "participants" => Ok(GeneratedField::Participants),
                            "controlAreas" | "control_areas" => Ok(GeneratedField::ControlAreas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaSearch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.AreaSearch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AreaSearch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut priors__ = None;
                let mut participants__ = None;
                let mut control_areas__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Priors => {
                            if priors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priors"));
                            }
                            priors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControlAreas => {
                            if control_areas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlAreas"));
                            }
                            control_areas__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AreaSearch {
                    objective: objective__,
                    priors: priors__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    control_areas: control_areas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.AreaSearch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzimuthElevationPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.azimuth != 0. {
            len += 1;
        }
        if self.elevation != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.AzimuthElevationPoint", len)?;
        if self.azimuth != 0. {
            struct_ser.serialize_field("azimuth", &self.azimuth)?;
        }
        if self.elevation != 0. {
            struct_ser.serialize_field("elevation", &self.elevation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzimuthElevationPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "azimuth",
            "elevation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Azimuth,
            Elevation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "azimuth" => Ok(GeneratedField::Azimuth),
                            "elevation" => Ok(GeneratedField::Elevation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzimuthElevationPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.AzimuthElevationPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzimuthElevationPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut azimuth__ = None;
                let mut elevation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Azimuth => {
                            if azimuth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azimuth"));
                            }
                            azimuth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Elevation => {
                            if elevation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elevation"));
                            }
                            elevation__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AzimuthElevationPoint {
                    azimuth: azimuth__.unwrap_or_default(),
                    elevation: elevation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.AzimuthElevationPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BattleDamageAssessment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.BattleDamageAssessment", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BattleDamageAssessment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BattleDamageAssessment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.BattleDamageAssessment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BattleDamageAssessment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BattleDamageAssessment {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.BattleDamageAssessment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlArea {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.control_area_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.ControlArea", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if self.control_area_type != 0 {
            let v = ControlAreaType::try_from(self.control_area_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.control_area_type)))?;
            struct_ser.serialize_field("controlAreaType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlArea {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "control_area_type",
            "controlAreaType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            ControlAreaType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "controlAreaType" | "control_area_type" => Ok(GeneratedField::ControlAreaType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlArea;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.ControlArea")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlArea, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut control_area_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControlAreaType => {
                            if control_area_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlAreaType"));
                            }
                            control_area_type__ = Some(map_.next_value::<ControlAreaType>()? as i32);
                        }
                    }
                }
                Ok(ControlArea {
                    entity_id: entity_id__.unwrap_or_default(),
                    control_area_type: control_area_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.ControlArea", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlAreaType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CONTROL_AREA_TYPE_INVALID",
            Self::KeepInZone => "CONTROL_AREA_TYPE_KEEP_IN_ZONE",
            Self::KeepOutZone => "CONTROL_AREA_TYPE_KEEP_OUT_ZONE",
            Self::DitchZone => "CONTROL_AREA_TYPE_DITCH_ZONE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ControlAreaType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTROL_AREA_TYPE_INVALID",
            "CONTROL_AREA_TYPE_KEEP_IN_ZONE",
            "CONTROL_AREA_TYPE_KEEP_OUT_ZONE",
            "CONTROL_AREA_TYPE_DITCH_ZONE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlAreaType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTROL_AREA_TYPE_INVALID" => Ok(ControlAreaType::Invalid),
                    "CONTROL_AREA_TYPE_KEEP_IN_ZONE" => Ok(ControlAreaType::KeepInZone),
                    "CONTROL_AREA_TYPE_KEEP_OUT_ZONE" => Ok(ControlAreaType::KeepOutZone),
                    "CONTROL_AREA_TYPE_DITCH_ZONE" => Ok(ControlAreaType::DitchZone),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DurationRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min.is_some() {
            len += 1;
        }
        if self.max.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.DurationRange", len)?;
        if let Some(v) = self.min.as_ref() {
            struct_ser.serialize_field("min", v)?;
        }
        if let Some(v) = self.max.as_ref() {
            struct_ser.serialize_field("max", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DurationRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DurationRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.DurationRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DurationRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = map_.next_value()?;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DurationRange {
                    min: min__,
                    max: max__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.DurationRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FramePoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.x != 0. {
            len += 1;
        }
        if self.y != 0. {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.FramePoint", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FramePoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(GeneratedField::X),
                            "y" => Ok(GeneratedField::Y),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FramePoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.FramePoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FramePoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::X => {
                            if x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FramePoint {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.FramePoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GimbalPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parameters.is_some() {
            len += 1;
        }
        if self.point_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.GimbalPoint", len)?;
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        if let Some(v) = self.point_type.as_ref() {
            match v {
                gimbal_point::PointType::LookAt(v) => {
                    struct_ser.serialize_field("lookAt", v)?;
                }
                gimbal_point::PointType::CelestialLocation(v) => {
                    struct_ser.serialize_field("celestialLocation", v)?;
                }
                gimbal_point::PointType::FrameLocation(v) => {
                    struct_ser.serialize_field("frameLocation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GimbalPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameters",
            "look_at",
            "lookAt",
            "celestial_location",
            "celestialLocation",
            "frame_location",
            "frameLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameters,
            LookAt,
            CelestialLocation,
            FrameLocation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "parameters" => Ok(GeneratedField::Parameters),
                            "lookAt" | "look_at" => Ok(GeneratedField::LookAt),
                            "celestialLocation" | "celestial_location" => Ok(GeneratedField::CelestialLocation),
                            "frameLocation" | "frame_location" => Ok(GeneratedField::FrameLocation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GimbalPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.GimbalPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GimbalPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameters__ = None;
                let mut point_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                        GeneratedField::LookAt => {
                            if point_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookAt"));
                            }
                            point_type__ = map_.next_value::<::std::option::Option<_>>()?.map(gimbal_point::PointType::LookAt)
;
                        }
                        GeneratedField::CelestialLocation => {
                            if point_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("celestialLocation"));
                            }
                            point_type__ = map_.next_value::<::std::option::Option<_>>()?.map(gimbal_point::PointType::CelestialLocation)
;
                        }
                        GeneratedField::FrameLocation => {
                            if point_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frameLocation"));
                            }
                            point_type__ = map_.next_value::<::std::option::Option<_>>()?.map(gimbal_point::PointType::FrameLocation)
;
                        }
                    }
                }
                Ok(GimbalPoint {
                    parameters: parameters__,
                    point_type: point_type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.GimbalPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GimbalZoom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.GimbalZoom", len)?;
        if let Some(v) = self.mode.as_ref() {
            match v {
                gimbal_zoom::Mode::SetHorizontalFov(v) => {
                    struct_ser.serialize_field("setHorizontalFov", v)?;
                }
                gimbal_zoom::Mode::SetMagnification(v) => {
                    struct_ser.serialize_field("setMagnification", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GimbalZoom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "set_horizontal_fov",
            "setHorizontalFov",
            "set_magnification",
            "setMagnification",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SetHorizontalFov,
            SetMagnification,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "setHorizontalFov" | "set_horizontal_fov" => Ok(GeneratedField::SetHorizontalFov),
                            "setMagnification" | "set_magnification" => Ok(GeneratedField::SetMagnification),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GimbalZoom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.GimbalZoom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GimbalZoom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SetHorizontalFov => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setHorizontalFov"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(gimbal_zoom::Mode::SetHorizontalFov)
;
                        }
                        GeneratedField::SetMagnification => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setMagnification"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(gimbal_zoom::Mode::SetMagnification)
;
                        }
                    }
                }
                Ok(GimbalZoom {
                    mode: mode__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.GimbalZoom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsrParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.speed.is_some() {
            len += 1;
        }
        if self.speed_m_s.is_some() {
            len += 1;
        }
        if self.standoff_distance_m.is_some() {
            len += 1;
        }
        if self.standoff_distance.is_some() {
            len += 1;
        }
        if self.standoff_angle.is_some() {
            len += 1;
        }
        if self.expiration_time_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.ISRParameters", len)?;
        if let Some(v) = self.speed.as_ref() {
            struct_ser.serialize_field("speed", v)?;
        }
        if let Some(v) = self.speed_m_s.as_ref() {
            struct_ser.serialize_field("speedMS", v)?;
        }
        if let Some(v) = self.standoff_distance_m.as_ref() {
            struct_ser.serialize_field("standoffDistanceM", v)?;
        }
        if let Some(v) = self.standoff_distance.as_ref() {
            struct_ser.serialize_field("standoffDistance", v)?;
        }
        if let Some(v) = self.standoff_angle.as_ref() {
            struct_ser.serialize_field("standoffAngle", v)?;
        }
        if let Some(v) = self.expiration_time_ms.as_ref() {
            struct_ser.serialize_field("expirationTimeMs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsrParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "speed",
            "speed_m_s",
            "speedMS",
            "standoff_distance_m",
            "standoffDistanceM",
            "standoff_distance",
            "standoffDistance",
            "standoff_angle",
            "standoffAngle",
            "expiration_time_ms",
            "expirationTimeMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Speed,
            SpeedMS,
            StandoffDistanceM,
            StandoffDistance,
            StandoffAngle,
            ExpirationTimeMs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "speed" => Ok(GeneratedField::Speed),
                            "speedMS" | "speed_m_s" => Ok(GeneratedField::SpeedMS),
                            "standoffDistanceM" | "standoff_distance_m" => Ok(GeneratedField::StandoffDistanceM),
                            "standoffDistance" | "standoff_distance" => Ok(GeneratedField::StandoffDistance),
                            "standoffAngle" | "standoff_angle" => Ok(GeneratedField::StandoffAngle),
                            "expirationTimeMs" | "expiration_time_ms" => Ok(GeneratedField::ExpirationTimeMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsrParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.ISRParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsrParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut speed__ = None;
                let mut speed_m_s__ = None;
                let mut standoff_distance_m__ = None;
                let mut standoff_distance__ = None;
                let mut standoff_angle__ = None;
                let mut expiration_time_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Speed => {
                            if speed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speed"));
                            }
                            speed__ = map_.next_value()?;
                        }
                        GeneratedField::SpeedMS => {
                            if speed_m_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speedMS"));
                            }
                            speed_m_s__ = map_.next_value()?;
                        }
                        GeneratedField::StandoffDistanceM => {
                            if standoff_distance_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standoffDistanceM"));
                            }
                            standoff_distance_m__ = map_.next_value()?;
                        }
                        GeneratedField::StandoffDistance => {
                            if standoff_distance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standoffDistance"));
                            }
                            standoff_distance__ = map_.next_value()?;
                        }
                        GeneratedField::StandoffAngle => {
                            if standoff_angle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standoffAngle"));
                            }
                            standoff_angle__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationTimeMs => {
                            if expiration_time_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTimeMs"));
                            }
                            expiration_time_ms__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IsrParameters {
                    speed: speed__,
                    speed_m_s: speed_m_s__,
                    standoff_distance_m: standoff_distance_m__,
                    standoff_distance: standoff_distance__,
                    standoff_angle: standoff_angle__,
                    expiration_time_ms: expiration_time_ms__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.ISRParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImproveTrackQuality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.termination_track_quality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.ImproveTrackQuality", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if self.termination_track_quality != 0 {
            struct_ser.serialize_field("terminationTrackQuality", &self.termination_track_quality)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImproveTrackQuality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "termination_track_quality",
            "terminationTrackQuality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            TerminationTrackQuality,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "terminationTrackQuality" | "termination_track_quality" => Ok(GeneratedField::TerminationTrackQuality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImproveTrackQuality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.ImproveTrackQuality")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImproveTrackQuality, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut termination_track_quality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::TerminationTrackQuality => {
                            if termination_track_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminationTrackQuality"));
                            }
                            termination_track_quality__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ImproveTrackQuality {
                    objective: objective__,
                    termination_track_quality: termination_track_quality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.ImproveTrackQuality", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Investigate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Investigate", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Investigate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Investigate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Investigate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Investigate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Investigate {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Investigate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LaunchTrackingMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "LAUNCH_TRACKING_MODE_INVALID",
            Self::GoToWaypoint => "LAUNCH_TRACKING_MODE_GO_TO_WAYPOINT",
            Self::TrackToWaypoint => "LAUNCH_TRACKING_MODE_TRACK_TO_WAYPOINT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LaunchTrackingMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LAUNCH_TRACKING_MODE_INVALID",
            "LAUNCH_TRACKING_MODE_GO_TO_WAYPOINT",
            "LAUNCH_TRACKING_MODE_TRACK_TO_WAYPOINT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LaunchTrackingMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LAUNCH_TRACKING_MODE_INVALID" => Ok(LaunchTrackingMode::Invalid),
                    "LAUNCH_TRACKING_MODE_GO_TO_WAYPOINT" => Ok(LaunchTrackingMode::GoToWaypoint),
                    "LAUNCH_TRACKING_MODE_TRACK_TO_WAYPOINT" => Ok(LaunchTrackingMode::TrackToWaypoint),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Loiter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.loiter_type.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Loiter", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.loiter_type.as_ref() {
            struct_ser.serialize_field("loiterType", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Loiter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "loiter_type",
            "loiterType",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            LoiterType,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "loiterType" | "loiter_type" => Ok(GeneratedField::LoiterType),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Loiter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Loiter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Loiter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut loiter_type__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::LoiterType => {
                            if loiter_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loiterType"));
                            }
                            loiter_type__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Loiter {
                    objective: objective__,
                    loiter_type: loiter_type__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Loiter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoiterType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.loiter_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.LoiterType", len)?;
        if let Some(v) = self.loiter_type.as_ref() {
            match v {
                loiter_type::LoiterType::OrbitType(v) => {
                    struct_ser.serialize_field("orbitType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoiterType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orbit_type",
            "orbitType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrbitType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orbitType" | "orbit_type" => Ok(GeneratedField::OrbitType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoiterType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.LoiterType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoiterType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut loiter_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrbitType => {
                            if loiter_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orbitType"));
                            }
                            loiter_type__ = map_.next_value::<::std::option::Option<_>>()?.map(loiter_type::LoiterType::OrbitType)
;
                        }
                    }
                }
                Ok(LoiterType {
                    loiter_type: loiter_type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.LoiterType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Map {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        if self.min_niirs.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Map", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        if let Some(v) = self.min_niirs.as_ref() {
            struct_ser.serialize_field("minNiirs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Map {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
            "min_niirs",
            "minNiirs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
            MinNiirs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "minNiirs" | "min_niirs" => Ok(GeneratedField::MinNiirs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Map;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Map")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Map, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                let mut min_niirs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                        GeneratedField::MinNiirs => {
                            if min_niirs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minNiirs"));
                            }
                            min_niirs__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Map {
                    objective: objective__,
                    parameters: parameters__,
                    min_niirs: min_niirs__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Map", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Marshal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Marshal", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Marshal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Marshal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Marshal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Marshal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Marshal {
                    objective: objective__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Marshal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Monitor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if !self.track_id.is_empty() {
            len += 1;
        }
        if !self.track_producer.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Monitor", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if !self.track_id.is_empty() {
            struct_ser.serialize_field("trackId", &self.track_id)?;
        }
        if !self.track_producer.is_empty() {
            struct_ser.serialize_field("trackProducer", &self.track_producer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Monitor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "track_id",
            "trackId",
            "track_producer",
            "trackProducer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            TrackId,
            TrackProducer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "trackId" | "track_id" => Ok(GeneratedField::TrackId),
                            "trackProducer" | "track_producer" => Ok(GeneratedField::TrackProducer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Monitor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Monitor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Monitor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut track_id__ = None;
                let mut track_producer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::TrackId => {
                            if track_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackId"));
                            }
                            track_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrackProducer => {
                            if track_producer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackProducer"));
                            }
                            track_producer__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Monitor {
                    objective: objective__,
                    track_id: track_id__.unwrap_or_default(),
                    track_producer: track_producer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Monitor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Objective {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.produced_by_asset_id.is_empty() {
            len += 1;
        }
        if self.objective.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Objective", len)?;
        if !self.produced_by_asset_id.is_empty() {
            struct_ser.serialize_field("producedByAssetId", &self.produced_by_asset_id)?;
        }
        if let Some(v) = self.objective.as_ref() {
            match v {
                objective::Objective::EntityId(v) => {
                    struct_ser.serialize_field("entityId", v)?;
                }
                objective::Objective::Point(v) => {
                    struct_ser.serialize_field("point", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Objective {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "produced_by_asset_id",
            "producedByAssetId",
            "entity_id",
            "entityId",
            "point",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProducedByAssetId,
            EntityId,
            Point,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "producedByAssetId" | "produced_by_asset_id" => Ok(GeneratedField::ProducedByAssetId),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "point" => Ok(GeneratedField::Point),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Objective;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Objective")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Objective, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut produced_by_asset_id__ = None;
                let mut objective__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProducedByAssetId => {
                            if produced_by_asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producedByAssetId"));
                            }
                            produced_by_asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityId => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            objective__ = map_.next_value::<::std::option::Option<_>>()?.map(objective::Objective::EntityId);
                        }
                        GeneratedField::Point => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("point"));
                            }
                            objective__ = map_.next_value::<::std::option::Option<_>>()?.map(objective::Objective::Point)
;
                        }
                    }
                }
                Ok(Objective {
                    produced_by_asset_id: produced_by_asset_id__.unwrap_or_default(),
                    objective: objective__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Objective", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DirectionInvalid => "ORBIT_DIRECTION_DIRECTION_INVALID",
            Self::Right => "ORBIT_DIRECTION_RIGHT",
            Self::Left => "ORBIT_DIRECTION_LEFT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrbitDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORBIT_DIRECTION_DIRECTION_INVALID",
            "ORBIT_DIRECTION_RIGHT",
            "ORBIT_DIRECTION_LEFT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitDirection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ORBIT_DIRECTION_DIRECTION_INVALID" => Ok(OrbitDirection::DirectionInvalid),
                    "ORBIT_DIRECTION_RIGHT" => Ok(OrbitDirection::Right),
                    "ORBIT_DIRECTION_LEFT" => Ok(OrbitDirection::Left),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitDuration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.OrbitDuration", len)?;
        if let Some(v) = self.duration.as_ref() {
            match v {
                orbit_duration::Duration::DurationRange(v) => {
                    struct_ser.serialize_field("durationRange", v)?;
                }
                orbit_duration::Duration::NumOfOrbits(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("numOfOrbits", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrbitDuration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "duration_range",
            "durationRange",
            "num_of_orbits",
            "numOfOrbits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DurationRange,
            NumOfOrbits,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "durationRange" | "duration_range" => Ok(GeneratedField::DurationRange),
                            "numOfOrbits" | "num_of_orbits" => Ok(GeneratedField::NumOfOrbits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitDuration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.OrbitDuration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrbitDuration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DurationRange => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationRange"));
                            }
                            duration__ = map_.next_value::<::std::option::Option<_>>()?.map(orbit_duration::Duration::DurationRange)
;
                        }
                        GeneratedField::NumOfOrbits => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOfOrbits"));
                            }
                            duration__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| orbit_duration::Duration::NumOfOrbits(x.0));
                        }
                    }
                }
                Ok(OrbitDuration {
                    duration: duration__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.OrbitDuration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ORBIT_PATTERN_INVALID",
            Self::Circle => "ORBIT_PATTERN_CIRCLE",
            Self::Racetrack => "ORBIT_PATTERN_RACETRACK",
            Self::FigureEight => "ORBIT_PATTERN_FIGURE_EIGHT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OrbitPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORBIT_PATTERN_INVALID",
            "ORBIT_PATTERN_CIRCLE",
            "ORBIT_PATTERN_RACETRACK",
            "ORBIT_PATTERN_FIGURE_EIGHT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ORBIT_PATTERN_INVALID" => Ok(OrbitPattern::Invalid),
                    "ORBIT_PATTERN_CIRCLE" => Ok(OrbitPattern::Circle),
                    "ORBIT_PATTERN_RACETRACK" => Ok(OrbitPattern::Racetrack),
                    "ORBIT_PATTERN_FIGURE_EIGHT" => Ok(OrbitPattern::FigureEight),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.direction != 0 {
            len += 1;
        }
        if self.pattern != 0 {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.OrbitType", len)?;
        if self.direction != 0 {
            let v = OrbitDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        if self.pattern != 0 {
            let v = OrbitPattern::try_from(self.pattern)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.pattern)))?;
            struct_ser.serialize_field("pattern", &v)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrbitType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "direction",
            "pattern",
            "duration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Direction,
            Pattern,
            Duration,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "direction" => Ok(GeneratedField::Direction),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "duration" => Ok(GeneratedField::Duration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.OrbitType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrbitType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut direction__ = None;
                let mut pattern__ = None;
                let mut duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<OrbitDirection>()? as i32);
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = Some(map_.next_value::<OrbitPattern>()? as i32);
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrbitType {
                    direction: direction__.unwrap_or_default(),
                    pattern: pattern__.unwrap_or_default(),
                    duration: duration__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.OrbitType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PathSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.end_point.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.PathSegment", len)?;
        if let Some(v) = self.end_point.as_ref() {
            match v {
                path_segment::EndPoint::Waypoint(v) => {
                    struct_ser.serialize_field("waypoint", v)?;
                }
                path_segment::EndPoint::Loiter(v) => {
                    struct_ser.serialize_field("loiter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PathSegment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "waypoint",
            "loiter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Waypoint,
            Loiter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "waypoint" => Ok(GeneratedField::Waypoint),
                            "loiter" => Ok(GeneratedField::Loiter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PathSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.PathSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PathSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut end_point__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Waypoint => {
                            if end_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("waypoint"));
                            }
                            end_point__ = map_.next_value::<::std::option::Option<_>>()?.map(path_segment::EndPoint::Waypoint)
;
                        }
                        GeneratedField::Loiter => {
                            if end_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loiter"));
                            }
                            end_point__ = map_.next_value::<::std::option::Option<_>>()?.map(path_segment::EndPoint::Loiter)
;
                        }
                    }
                }
                Ok(PathSegment {
                    end_point: end_point__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.PathSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayloadConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.capability_id.is_empty() {
            len += 1;
        }
        if self.quantity != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.PayloadConfiguration", len)?;
        if !self.capability_id.is_empty() {
            struct_ser.serialize_field("capabilityId", &self.capability_id)?;
        }
        if self.quantity != 0 {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayloadConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "capability_id",
            "capabilityId",
            "quantity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CapabilityId,
            Quantity,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "capabilityId" | "capability_id" => Ok(GeneratedField::CapabilityId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayloadConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.PayloadConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayloadConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut capability_id__ = None;
                let mut quantity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CapabilityId => {
                            if capability_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilityId"));
                            }
                            capability_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PayloadConfiguration {
                    capability_id: capability_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.PayloadConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Point {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference_name.is_empty() {
            len += 1;
        }
        if self.lla.is_some() {
            len += 1;
        }
        if !self.backing_entity_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Point", len)?;
        if !self.reference_name.is_empty() {
            struct_ser.serialize_field("referenceName", &self.reference_name)?;
        }
        if let Some(v) = self.lla.as_ref() {
            struct_ser.serialize_field("lla", v)?;
        }
        if !self.backing_entity_id.is_empty() {
            struct_ser.serialize_field("backingEntityId", &self.backing_entity_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Point {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference_name",
            "referenceName",
            "lla",
            "backing_entity_id",
            "backingEntityId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReferenceName,
            Lla,
            BackingEntityId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "referenceName" | "reference_name" => Ok(GeneratedField::ReferenceName),
                            "lla" => Ok(GeneratedField::Lla),
                            "backingEntityId" | "backing_entity_id" => Ok(GeneratedField::BackingEntityId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Point;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Point")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Point, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference_name__ = None;
                let mut lla__ = None;
                let mut backing_entity_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReferenceName => {
                            if reference_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceName"));
                            }
                            reference_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Lla => {
                            if lla__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lla"));
                            }
                            lla__ = map_.next_value()?;
                        }
                        GeneratedField::BackingEntityId => {
                            if backing_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backingEntityId"));
                            }
                            backing_entity_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Point {
                    reference_name: reference_name__.unwrap_or_default(),
                    lla: lla__,
                    backing_entity_id: backing_entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Point", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Prior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.prior.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Prior", len)?;
        if let Some(v) = self.prior.as_ref() {
            match v {
                prior::Prior::EntityId(v) => {
                    struct_ser.serialize_field("entityId", v)?;
                }
                prior::Prior::Point(v) => {
                    struct_ser.serialize_field("point", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Prior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "point",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            Point,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "point" => Ok(GeneratedField::Point),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Prior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Prior")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Prior, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prior__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if prior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            prior__ = map_.next_value::<::std::option::Option<_>>()?.map(prior::Prior::EntityId);
                        }
                        GeneratedField::Point => {
                            if prior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("point"));
                            }
                            prior__ = map_.next_value::<::std::option::Option<_>>()?.map(prior::Prior::Point)
;
                        }
                    }
                }
                Ok(Prior {
                    prior: prior__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Prior", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReleasePayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payloads.is_empty() {
            len += 1;
        }
        if self.objective.is_some() {
            len += 1;
        }
        if self.release_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.ReleasePayload", len)?;
        if !self.payloads.is_empty() {
            struct_ser.serialize_field("payloads", &self.payloads)?;
        }
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.release_method.as_ref() {
            match v {
                release_payload::ReleaseMethod::PrecisionRelease(v) => {
                    struct_ser.serialize_field("precisionRelease", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReleasePayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payloads",
            "objective",
            "precision_release",
            "precisionRelease",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Payloads,
            Objective,
            PrecisionRelease,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "payloads" => Ok(GeneratedField::Payloads),
                            "objective" => Ok(GeneratedField::Objective),
                            "precisionRelease" | "precision_release" => Ok(GeneratedField::PrecisionRelease),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReleasePayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.ReleasePayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReleasePayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payloads__ = None;
                let mut objective__ = None;
                let mut release_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Payloads => {
                            if payloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloads"));
                            }
                            payloads__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::PrecisionRelease => {
                            if release_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("precisionRelease"));
                            }
                            release_method__ = map_.next_value::<::std::option::Option<_>>()?.map(release_payload::ReleaseMethod::PrecisionRelease)
;
                        }
                    }
                }
                Ok(ReleasePayload {
                    payloads: payloads__.unwrap_or_default(),
                    objective: objective__,
                    release_method: release_method__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.ReleasePayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Route {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Route", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Route")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Route {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutePlan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.route.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.RoutePlan", len)?;
        if let Some(v) = self.route.as_ref() {
            struct_ser.serialize_field("route", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutePlan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Route,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "route" => Ok(GeneratedField::Route),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutePlan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.RoutePlan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoutePlan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Route => {
                            if route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            route__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RoutePlan {
                    route: route__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.RoutePlan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Scan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Scan", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Scan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Scan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Scan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Scan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Scan {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Scan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetLaunchRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plan.is_some() {
            len += 1;
        }
        if self.tracking_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.SetLaunchRoute", len)?;
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        if self.tracking_mode != 0 {
            let v = LaunchTrackingMode::try_from(self.tracking_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tracking_mode)))?;
            struct_ser.serialize_field("trackingMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetLaunchRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plan",
            "tracking_mode",
            "trackingMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plan,
            TrackingMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plan" => Ok(GeneratedField::Plan),
                            "trackingMode" | "tracking_mode" => Ok(GeneratedField::TrackingMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetLaunchRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.SetLaunchRoute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetLaunchRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plan__ = None;
                let mut tracking_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                        GeneratedField::TrackingMode => {
                            if tracking_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackingMode"));
                            }
                            tracking_mode__ = Some(map_.next_value::<LaunchTrackingMode>()? as i32);
                        }
                    }
                }
                Ok(SetLaunchRoute {
                    plan: plan__,
                    tracking_mode: tracking_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.SetLaunchRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Shadow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Shadow", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Shadow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Shadow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Shadow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Shadow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Shadow {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Shadow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Smack {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Smack", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Smack {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Smack;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Smack")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Smack, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Smack {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Smack", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Strike {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.ingress_angle.is_some() {
            len += 1;
        }
        if self.strike_release_constraint.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Strike", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.ingress_angle.as_ref() {
            struct_ser.serialize_field("ingressAngle", v)?;
        }
        if let Some(v) = self.strike_release_constraint.as_ref() {
            struct_ser.serialize_field("strikeReleaseConstraint", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Strike {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "ingress_angle",
            "ingressAngle",
            "strike_release_constraint",
            "strikeReleaseConstraint",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            IngressAngle,
            StrikeReleaseConstraint,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "ingressAngle" | "ingress_angle" => Ok(GeneratedField::IngressAngle),
                            "strikeReleaseConstraint" | "strike_release_constraint" => Ok(GeneratedField::StrikeReleaseConstraint),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Strike;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Strike")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Strike, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut ingress_angle__ = None;
                let mut strike_release_constraint__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::IngressAngle => {
                            if ingress_angle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingressAngle"));
                            }
                            ingress_angle__ = map_.next_value()?;
                        }
                        GeneratedField::StrikeReleaseConstraint => {
                            if strike_release_constraint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strikeReleaseConstraint"));
                            }
                            strike_release_constraint__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Strike {
                    objective: objective__,
                    ingress_angle: ingress_angle__,
                    strike_release_constraint: strike_release_constraint__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Strike", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StrikeParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payloads_to_employ.is_empty() {
            len += 1;
        }
        if self.desired_impact_time.is_some() {
            len += 1;
        }
        if self.run_in_bearing != 0. {
            len += 1;
        }
        if self.glide_slope_angle != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.StrikeParameters", len)?;
        if !self.payloads_to_employ.is_empty() {
            struct_ser.serialize_field("payloadsToEmploy", &self.payloads_to_employ)?;
        }
        if let Some(v) = self.desired_impact_time.as_ref() {
            struct_ser.serialize_field("desiredImpactTime", v)?;
        }
        if self.run_in_bearing != 0. {
            struct_ser.serialize_field("runInBearing", &self.run_in_bearing)?;
        }
        if self.glide_slope_angle != 0. {
            struct_ser.serialize_field("glideSlopeAngle", &self.glide_slope_angle)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StrikeParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payloads_to_employ",
            "payloadsToEmploy",
            "desired_impact_time",
            "desiredImpactTime",
            "run_in_bearing",
            "runInBearing",
            "glide_slope_angle",
            "glideSlopeAngle",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PayloadsToEmploy,
            DesiredImpactTime,
            RunInBearing,
            GlideSlopeAngle,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "payloadsToEmploy" | "payloads_to_employ" => Ok(GeneratedField::PayloadsToEmploy),
                            "desiredImpactTime" | "desired_impact_time" => Ok(GeneratedField::DesiredImpactTime),
                            "runInBearing" | "run_in_bearing" => Ok(GeneratedField::RunInBearing),
                            "glideSlopeAngle" | "glide_slope_angle" => Ok(GeneratedField::GlideSlopeAngle),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StrikeParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.StrikeParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StrikeParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payloads_to_employ__ = None;
                let mut desired_impact_time__ = None;
                let mut run_in_bearing__ = None;
                let mut glide_slope_angle__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PayloadsToEmploy => {
                            if payloads_to_employ__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadsToEmploy"));
                            }
                            payloads_to_employ__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DesiredImpactTime => {
                            if desired_impact_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("desiredImpactTime"));
                            }
                            desired_impact_time__ = map_.next_value()?;
                        }
                        GeneratedField::RunInBearing => {
                            if run_in_bearing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runInBearing"));
                            }
                            run_in_bearing__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GlideSlopeAngle => {
                            if glide_slope_angle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("glideSlopeAngle"));
                            }
                            glide_slope_angle__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StrikeParameters {
                    payloads_to_employ: payloads_to_employ__.unwrap_or_default(),
                    desired_impact_time: desired_impact_time__,
                    run_in_bearing: run_in_bearing__.unwrap_or_default(),
                    glide_slope_angle: glide_slope_angle__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.StrikeParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StrikeReleaseConstraint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.strike_release_constraint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.StrikeReleaseConstraint", len)?;
        if let Some(v) = self.strike_release_constraint.as_ref() {
            match v {
                strike_release_constraint::StrikeReleaseConstraint::ReleaseArea(v) => {
                    struct_ser.serialize_field("releaseArea", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StrikeReleaseConstraint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "release_area",
            "releaseArea",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReleaseArea,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "releaseArea" | "release_area" => Ok(GeneratedField::ReleaseArea),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StrikeReleaseConstraint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.StrikeReleaseConstraint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StrikeReleaseConstraint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut strike_release_constraint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReleaseArea => {
                            if strike_release_constraint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("releaseArea"));
                            }
                            strike_release_constraint__ = map_.next_value::<::std::option::Option<_>>()?.map(strike_release_constraint::StrikeReleaseConstraint::ReleaseArea)
;
                        }
                    }
                }
                Ok(StrikeReleaseConstraint {
                    strike_release_constraint: strike_release_constraint__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.StrikeReleaseConstraint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskCatalog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_definitions.is_empty() {
            len += 1;
        }
        if self.is_asset_inhibited {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.TaskCatalog", len)?;
        if !self.task_definitions.is_empty() {
            struct_ser.serialize_field("taskDefinitions", &self.task_definitions)?;
        }
        if self.is_asset_inhibited {
            struct_ser.serialize_field("isAssetInhibited", &self.is_asset_inhibited)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskCatalog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_definitions",
            "taskDefinitions",
            "is_asset_inhibited",
            "isAssetInhibited",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskDefinitions,
            IsAssetInhibited,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "taskDefinitions" | "task_definitions" => Ok(GeneratedField::TaskDefinitions),
                            "isAssetInhibited" | "is_asset_inhibited" => Ok(GeneratedField::IsAssetInhibited),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskCatalog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.TaskCatalog")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskCatalog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_definitions__ = None;
                let mut is_asset_inhibited__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskDefinitions => {
                            if task_definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskDefinitions"));
                            }
                            task_definitions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAssetInhibited => {
                            if is_asset_inhibited__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAssetInhibited"));
                            }
                            is_asset_inhibited__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TaskCatalog {
                    task_definitions: task_definitions__.unwrap_or_default(),
                    is_asset_inhibited: is_asset_inhibited__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.TaskCatalog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskDefinition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_specification_url.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.TaskDefinition", len)?;
        if !self.task_specification_url.is_empty() {
            struct_ser.serialize_field("taskSpecificationUrl", &self.task_specification_url)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskDefinition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_specification_url",
            "taskSpecificationUrl",
            "display_name",
            "displayName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskSpecificationUrl,
            DisplayName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "taskSpecificationUrl" | "task_specification_url" => Ok(GeneratedField::TaskSpecificationUrl),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskDefinition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.TaskDefinition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskDefinition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_specification_url__ = None;
                let mut display_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskSpecificationUrl => {
                            if task_specification_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskSpecificationUrl"));
                            }
                            task_specification_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TaskDefinition {
                    task_specification_url: task_specification_url__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.TaskDefinition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Transit", len)?;
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plan,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plan" => Ok(GeneratedField::Plan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Transit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Transit {
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Transit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VisualId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.VisualId", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VisualId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VisualId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.VisualId")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VisualId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VisualId {
                    objective: objective__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.VisualId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VolumeSearch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.objective.is_some() {
            len += 1;
        }
        if !self.priors.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if !self.control_areas.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.VolumeSearch", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if !self.priors.is_empty() {
            struct_ser.serialize_field("priors", &self.priors)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if !self.control_areas.is_empty() {
            struct_ser.serialize_field("controlAreas", &self.control_areas)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VolumeSearch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objective",
            "priors",
            "participants",
            "control_areas",
            "controlAreas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
            Priors,
            Participants,
            ControlAreas,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objective" => Ok(GeneratedField::Objective),
                            "priors" => Ok(GeneratedField::Priors),
                            "participants" => Ok(GeneratedField::Participants),
                            "controlAreas" | "control_areas" => Ok(GeneratedField::ControlAreas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VolumeSearch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.VolumeSearch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VolumeSearch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut priors__ = None;
                let mut participants__ = None;
                let mut control_areas__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::Priors => {
                            if priors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priors"));
                            }
                            priors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ControlAreas => {
                            if control_areas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlAreas"));
                            }
                            control_areas__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VolumeSearch {
                    objective: objective__,
                    priors: priors__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    control_areas: control_areas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.VolumeSearch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Waypoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.point.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.v2.Waypoint", len)?;
        if let Some(v) = self.point.as_ref() {
            match v {
                waypoint::Point::LlaPoint(v) => {
                    struct_ser.serialize_field("llaPoint", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Waypoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lla_point",
            "llaPoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LlaPoint,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "llaPoint" | "lla_point" => Ok(GeneratedField::LlaPoint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Waypoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.v2.Waypoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Waypoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut point__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LlaPoint => {
                            if point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("llaPoint"));
                            }
                            point__ = map_.next_value::<::std::option::Option<_>>()?.map(waypoint::Point::LlaPoint)
;
                        }
                    }
                }
                Ok(Waypoint {
                    point: point__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.v2.Waypoint", FIELDS, GeneratedVisitor)
    }
}
