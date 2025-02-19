// @generated
impl serde::Serialize for LineFormation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.line_start.is_some() {
            len += 1;
        }
        if self.line_end.is_some() {
            len += 1;
        }
        if self.surface_speed_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.ads.thirdparty.v1.LineFormation", len)?;
        if let Some(v) = self.line_start.as_ref() {
            struct_ser.serialize_field("lineStart", v)?;
        }
        if let Some(v) = self.line_end.as_ref() {
            struct_ser.serialize_field("lineEnd", v)?;
        }
        if let Some(v) = self.surface_speed_ms.as_ref() {
            struct_ser.serialize_field("surfaceSpeedMs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineFormation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "line_start",
            "lineStart",
            "line_end",
            "lineEnd",
            "surface_speed_ms",
            "surfaceSpeedMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LineStart,
            LineEnd,
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
                            "lineStart" | "line_start" => Ok(GeneratedField::LineStart),
                            "lineEnd" | "line_end" => Ok(GeneratedField::LineEnd),
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
            type Value = LineFormation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.ads.thirdparty.v1.LineFormation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LineFormation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut line_start__ = None;
                let mut line_end__ = None;
                let mut surface_speed_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LineStart => {
                            if line_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineStart"));
                            }
                            line_start__ = map_.next_value()?;
                        }
                        GeneratedField::LineEnd => {
                            if line_end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineEnd"));
                            }
                            line_end__ = map_.next_value()?;
                        }
                        GeneratedField::SurfaceSpeedMs => {
                            if surface_speed_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surfaceSpeedMs"));
                            }
                            surface_speed_ms__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LineFormation {
                    line_start: line_start__,
                    line_end: line_end__,
                    surface_speed_ms: surface_speed_ms__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.ads.thirdparty.v1.LineFormation", FIELDS, GeneratedVisitor)
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
        if self.surface_speed_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.ads.thirdparty.v1.Marshal", len)?;
        if let Some(v) = self.objective.as_ref() {
            struct_ser.serialize_field("objective", v)?;
        }
        if let Some(v) = self.surface_speed_ms.as_ref() {
            struct_ser.serialize_field("surfaceSpeedMs", v)?;
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
            "surface_speed_ms",
            "surfaceSpeedMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objective,
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
                            "objective" => Ok(GeneratedField::Objective),
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
            type Value = Marshal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.ads.thirdparty.v1.Marshal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Marshal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objective__ = None;
                let mut surface_speed_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objective => {
                            if objective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objective"));
                            }
                            objective__ = map_.next_value()?;
                        }
                        GeneratedField::SurfaceSpeedMs => {
                            if surface_speed_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surfaceSpeedMs"));
                            }
                            surface_speed_ms__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Marshal {
                    objective: objective__,
                    surface_speed_ms: surface_speed_ms__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.ads.thirdparty.v1.Marshal", FIELDS, GeneratedVisitor)
    }
}
