// @generated
impl serde::Serialize for PathSegment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.jadc2.thirdparty.v1.PathSegment", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
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
            "endpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
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
                formatter.write_str("struct anduril.tasks.jadc2.thirdparty.v1.PathSegment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PathSegment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PathSegment {
                    endpoint: endpoint__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.jadc2.thirdparty.v1.PathSegment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TeamTransit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transit_zone_entity_id.is_empty() {
            len += 1;
        }
        if self.surface_speed_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.jadc2.thirdparty.v1.TeamTransit", len)?;
        if !self.transit_zone_entity_id.is_empty() {
            struct_ser.serialize_field("transitZoneEntityId", &self.transit_zone_entity_id)?;
        }
        if let Some(v) = self.surface_speed_ms.as_ref() {
            struct_ser.serialize_field("surfaceSpeedMs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TeamTransit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transit_zone_entity_id",
            "transitZoneEntityId",
            "surface_speed_ms",
            "surfaceSpeedMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransitZoneEntityId,
            SurfaceSpeedMs,
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
                            "transitZoneEntityId" | "transit_zone_entity_id" => Ok(GeneratedField::TransitZoneEntityId),
                            "surfaceSpeedMs" | "surface_speed_ms" => Ok(GeneratedField::SurfaceSpeedMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TeamTransit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.jadc2.thirdparty.v1.TeamTransit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TeamTransit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transit_zone_entity_id__ = None;
                let mut surface_speed_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransitZoneEntityId => {
                            if transit_zone_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transitZoneEntityId"));
                            }
                            transit_zone_entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SurfaceSpeedMs => {
                            if surface_speed_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surfaceSpeedMs"));
                            }
                            surface_speed_ms__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TeamTransit {
                    transit_zone_entity_id: transit_zone_entity_id__.unwrap_or_default(),
                    surface_speed_ms: surface_speed_ms__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.jadc2.thirdparty.v1.TeamTransit", FIELDS, GeneratedVisitor)
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
        if !self.path.is_empty() {
            len += 1;
        }
        if self.surface_speed_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.jadc2.thirdparty.v1.Transit", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.surface_speed_ms.as_ref() {
            struct_ser.serialize_field("surfaceSpeedMs", v)?;
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
            "path",
            "surface_speed_ms",
            "surfaceSpeedMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            SurfaceSpeedMs,
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
                            "surfaceSpeedMs" | "surface_speed_ms" => Ok(GeneratedField::SurfaceSpeedMs),
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
                formatter.write_str("struct anduril.tasks.jadc2.thirdparty.v1.Transit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut surface_speed_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SurfaceSpeedMs => {
                            if surface_speed_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surfaceSpeedMs"));
                            }
                            surface_speed_ms__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Transit {
                    path: path__.unwrap_or_default(),
                    surface_speed_ms: surface_speed_ms__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.jadc2.thirdparty.v1.Transit", FIELDS, GeneratedVisitor)
    }
}
