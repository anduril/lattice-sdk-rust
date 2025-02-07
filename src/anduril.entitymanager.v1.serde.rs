// @generated
impl serde::Serialize for ActiveTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ActiveTarget", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActiveTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActiveTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ActiveTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActiveTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ActiveTarget {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ActiveTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Alert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.alert_code.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.level != 0 {
            len += 1;
        }
        if self.activated_time.is_some() {
            len += 1;
        }
        if !self.active_conditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Alert", len)?;
        if !self.alert_code.is_empty() {
            struct_ser.serialize_field("alertCode", &self.alert_code)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.level != 0 {
            let v = AlertLevel::try_from(self.level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.level)))?;
            struct_ser.serialize_field("level", &v)?;
        }
        if let Some(v) = self.activated_time.as_ref() {
            struct_ser.serialize_field("activatedTime", v)?;
        }
        if !self.active_conditions.is_empty() {
            struct_ser.serialize_field("activeConditions", &self.active_conditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Alert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "alert_code",
            "alertCode",
            "description",
            "level",
            "activated_time",
            "activatedTime",
            "active_conditions",
            "activeConditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AlertCode,
            Description,
            Level,
            ActivatedTime,
            ActiveConditions,
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
                            "alertCode" | "alert_code" => Ok(GeneratedField::AlertCode),
                            "description" => Ok(GeneratedField::Description),
                            "level" => Ok(GeneratedField::Level),
                            "activatedTime" | "activated_time" => Ok(GeneratedField::ActivatedTime),
                            "activeConditions" | "active_conditions" => Ok(GeneratedField::ActiveConditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Alert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Alert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Alert, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut alert_code__ = None;
                let mut description__ = None;
                let mut level__ = None;
                let mut activated_time__ = None;
                let mut active_conditions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AlertCode => {
                            if alert_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alertCode"));
                            }
                            alert_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map_.next_value::<AlertLevel>()? as i32);
                        }
                        GeneratedField::ActivatedTime => {
                            if activated_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activatedTime"));
                            }
                            activated_time__ = map_.next_value()?;
                        }
                        GeneratedField::ActiveConditions => {
                            if active_conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeConditions"));
                            }
                            active_conditions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Alert {
                    alert_code: alert_code__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    level: level__.unwrap_or_default(),
                    activated_time: activated_time__,
                    active_conditions: active_conditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Alert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AlertCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.condition_code.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.AlertCondition", len)?;
        if !self.condition_code.is_empty() {
            struct_ser.serialize_field("conditionCode", &self.condition_code)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AlertCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "condition_code",
            "conditionCode",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConditionCode,
            Description,
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
                            "conditionCode" | "condition_code" => Ok(GeneratedField::ConditionCode),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AlertCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.AlertCondition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AlertCondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition_code__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConditionCode => {
                            if condition_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionCode"));
                            }
                            condition_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AlertCondition {
                    condition_code: condition_code__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.AlertCondition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AlertLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ALERT_LEVEL_INVALID",
            Self::Advisory => "ALERT_LEVEL_ADVISORY",
            Self::Caution => "ALERT_LEVEL_CAUTION",
            Self::Warning => "ALERT_LEVEL_WARNING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AlertLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALERT_LEVEL_INVALID",
            "ALERT_LEVEL_ADVISORY",
            "ALERT_LEVEL_CAUTION",
            "ALERT_LEVEL_WARNING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AlertLevel;

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
                    "ALERT_LEVEL_INVALID" => Ok(AlertLevel::Invalid),
                    "ALERT_LEVEL_ADVISORY" => Ok(AlertLevel::Advisory),
                    "ALERT_LEVEL_CAUTION" => Ok(AlertLevel::Caution),
                    "ALERT_LEVEL_WARNING" => Ok(AlertLevel::Warning),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Aliases {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.alternate_ids.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Aliases", len)?;
        if !self.alternate_ids.is_empty() {
            struct_ser.serialize_field("alternateIds", &self.alternate_ids)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Aliases {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "alternate_ids",
            "alternateIds",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AlternateIds,
            Name,
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
                            "alternateIds" | "alternate_ids" => Ok(GeneratedField::AlternateIds),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Aliases;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Aliases")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Aliases, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut alternate_ids__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AlternateIds => {
                            if alternate_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alternateIds"));
                            }
                            alternate_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Aliases {
                    alternate_ids: alternate_ids__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Aliases", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AltIdType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ALT_ID_TYPE_INVALID",
            Self::TrackId2 => "ALT_ID_TYPE_TRACK_ID_2",
            Self::TrackId1 => "ALT_ID_TYPE_TRACK_ID_1",
            Self::SpiId => "ALT_ID_TYPE_SPI_ID",
            Self::NitfFileTitle => "ALT_ID_TYPE_NITF_FILE_TITLE",
            Self::TrackRepoAlertId => "ALT_ID_TYPE_TRACK_REPO_ALERT_ID",
            Self::AssetId => "ALT_ID_TYPE_ASSET_ID",
            Self::Link16TrackNumber => "ALT_ID_TYPE_LINK16_TRACK_NUMBER",
            Self::Link16Ju => "ALT_ID_TYPE_LINK16_JU",
            Self::NcctMessageId => "ALT_ID_TYPE_NCCT_MESSAGE_ID",
            Self::Callsign => "ALT_ID_TYPE_CALLSIGN",
            Self::MmsiId => "ALT_ID_TYPE_MMSI_ID",
            Self::VmfUrn => "ALT_ID_TYPE_VMF_URN",
            Self::ImoId => "ALT_ID_TYPE_IMO_ID",
            Self::VmfTargetNumber => "ALT_ID_TYPE_VMF_TARGET_NUMBER",
            Self::SerialNumber => "ALT_ID_TYPE_SERIAL_NUMBER",
            Self::RegistrationId => "ALT_ID_TYPE_REGISTRATION_ID",
            Self::IbsGid => "ALT_ID_TYPE_IBS_GID",
            Self::Dodaac => "ALT_ID_TYPE_DODAAC",
            Self::Uic => "ALT_ID_TYPE_UIC",
            Self::NoradCatId => "ALT_ID_TYPE_NORAD_CAT_ID",
            Self::UnoosaName => "ALT_ID_TYPE_UNOOSA_NAME",
            Self::UnoosaId => "ALT_ID_TYPE_UNOOSA_ID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AltIdType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALT_ID_TYPE_INVALID",
            "ALT_ID_TYPE_TRACK_ID_2",
            "ALT_ID_TYPE_TRACK_ID_1",
            "ALT_ID_TYPE_SPI_ID",
            "ALT_ID_TYPE_NITF_FILE_TITLE",
            "ALT_ID_TYPE_TRACK_REPO_ALERT_ID",
            "ALT_ID_TYPE_ASSET_ID",
            "ALT_ID_TYPE_LINK16_TRACK_NUMBER",
            "ALT_ID_TYPE_LINK16_JU",
            "ALT_ID_TYPE_NCCT_MESSAGE_ID",
            "ALT_ID_TYPE_CALLSIGN",
            "ALT_ID_TYPE_MMSI_ID",
            "ALT_ID_TYPE_VMF_URN",
            "ALT_ID_TYPE_IMO_ID",
            "ALT_ID_TYPE_VMF_TARGET_NUMBER",
            "ALT_ID_TYPE_SERIAL_NUMBER",
            "ALT_ID_TYPE_REGISTRATION_ID",
            "ALT_ID_TYPE_IBS_GID",
            "ALT_ID_TYPE_DODAAC",
            "ALT_ID_TYPE_UIC",
            "ALT_ID_TYPE_NORAD_CAT_ID",
            "ALT_ID_TYPE_UNOOSA_NAME",
            "ALT_ID_TYPE_UNOOSA_ID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AltIdType;

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
                    "ALT_ID_TYPE_INVALID" => Ok(AltIdType::Invalid),
                    "ALT_ID_TYPE_TRACK_ID_2" => Ok(AltIdType::TrackId2),
                    "ALT_ID_TYPE_TRACK_ID_1" => Ok(AltIdType::TrackId1),
                    "ALT_ID_TYPE_SPI_ID" => Ok(AltIdType::SpiId),
                    "ALT_ID_TYPE_NITF_FILE_TITLE" => Ok(AltIdType::NitfFileTitle),
                    "ALT_ID_TYPE_TRACK_REPO_ALERT_ID" => Ok(AltIdType::TrackRepoAlertId),
                    "ALT_ID_TYPE_ASSET_ID" => Ok(AltIdType::AssetId),
                    "ALT_ID_TYPE_LINK16_TRACK_NUMBER" => Ok(AltIdType::Link16TrackNumber),
                    "ALT_ID_TYPE_LINK16_JU" => Ok(AltIdType::Link16Ju),
                    "ALT_ID_TYPE_NCCT_MESSAGE_ID" => Ok(AltIdType::NcctMessageId),
                    "ALT_ID_TYPE_CALLSIGN" => Ok(AltIdType::Callsign),
                    "ALT_ID_TYPE_MMSI_ID" => Ok(AltIdType::MmsiId),
                    "ALT_ID_TYPE_VMF_URN" => Ok(AltIdType::VmfUrn),
                    "ALT_ID_TYPE_IMO_ID" => Ok(AltIdType::ImoId),
                    "ALT_ID_TYPE_VMF_TARGET_NUMBER" => Ok(AltIdType::VmfTargetNumber),
                    "ALT_ID_TYPE_SERIAL_NUMBER" => Ok(AltIdType::SerialNumber),
                    "ALT_ID_TYPE_REGISTRATION_ID" => Ok(AltIdType::RegistrationId),
                    "ALT_ID_TYPE_IBS_GID" => Ok(AltIdType::IbsGid),
                    "ALT_ID_TYPE_DODAAC" => Ok(AltIdType::Dodaac),
                    "ALT_ID_TYPE_UIC" => Ok(AltIdType::Uic),
                    "ALT_ID_TYPE_NORAD_CAT_ID" => Ok(AltIdType::NoradCatId),
                    "ALT_ID_TYPE_UNOOSA_NAME" => Ok(AltIdType::UnoosaName),
                    "ALT_ID_TYPE_UNOOSA_ID" => Ok(AltIdType::UnoosaId),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AlternateId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.AlternateId", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.r#type != 0 {
            let v = AltIdType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AlternateId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Type,
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
                            "id" => Ok(GeneratedField::Id),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AlternateId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.AlternateId")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AlternateId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<AltIdType>()? as i32);
                        }
                    }
                }
                Ok(AlternateId {
                    id: id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.AlternateId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AndOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.children.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.AndOperation", len)?;
        if let Some(v) = self.children.as_ref() {
            match v {
                and_operation::Children::PredicateSet(v) => {
                    struct_ser.serialize_field("predicateSet", v)?;
                }
                and_operation::Children::StatementSet(v) => {
                    struct_ser.serialize_field("statementSet", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate_set",
            "predicateSet",
            "statement_set",
            "statementSet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PredicateSet,
            StatementSet,
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
                            "predicateSet" | "predicate_set" => Ok(GeneratedField::PredicateSet),
                            "statementSet" | "statement_set" => Ok(GeneratedField::StatementSet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AndOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.AndOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AndOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut children__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PredicateSet => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicateSet"));
                            }
                            children__ = map_.next_value::<::std::option::Option<_>>()?.map(and_operation::Children::PredicateSet)
;
                        }
                        GeneratedField::StatementSet => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statementSet"));
                            }
                            children__ = map_.next_value::<::std::option::Option<_>>()?.map(and_operation::Children::StatementSet)
;
                        }
                    }
                }
                Ok(AndOperation {
                    children: children__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.AndOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AngleOfArrival {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relative_pose.is_some() {
            len += 1;
        }
        if self.bearing_elevation_covariance_rad2.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.AngleOfArrival", len)?;
        if let Some(v) = self.relative_pose.as_ref() {
            struct_ser.serialize_field("relativePose", v)?;
        }
        if let Some(v) = self.bearing_elevation_covariance_rad2.as_ref() {
            struct_ser.serialize_field("bearingElevationCovarianceRad2", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AngleOfArrival {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relative_pose",
            "relativePose",
            "bearing_elevation_covariance_rad2",
            "bearingElevationCovarianceRad2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelativePose,
            BearingElevationCovarianceRad2,
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
                            "relativePose" | "relative_pose" => Ok(GeneratedField::RelativePose),
                            "bearingElevationCovarianceRad2" | "bearing_elevation_covariance_rad2" => Ok(GeneratedField::BearingElevationCovarianceRad2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AngleOfArrival;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.AngleOfArrival")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AngleOfArrival, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relative_pose__ = None;
                let mut bearing_elevation_covariance_rad2__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RelativePose => {
                            if relative_pose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativePose"));
                            }
                            relative_pose__ = map_.next_value()?;
                        }
                        GeneratedField::BearingElevationCovarianceRad2 => {
                            if bearing_elevation_covariance_rad2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bearingElevationCovarianceRad2"));
                            }
                            bearing_elevation_covariance_rad2__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AngleOfArrival {
                    relative_pose: relative_pose__,
                    bearing_elevation_covariance_rad2: bearing_elevation_covariance_rad2__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.AngleOfArrival", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArmyEchelon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ARMY_ECHELON_INVALID",
            Self::FireTeam => "ARMY_ECHELON_FIRE_TEAM",
            Self::Squad => "ARMY_ECHELON_SQUAD",
            Self::Platoon => "ARMY_ECHELON_PLATOON",
            Self::Company => "ARMY_ECHELON_COMPANY",
            Self::Battalion => "ARMY_ECHELON_BATTALION",
            Self::Regiment => "ARMY_ECHELON_REGIMENT",
            Self::Brigade => "ARMY_ECHELON_BRIGADE",
            Self::Division => "ARMY_ECHELON_DIVISION",
            Self::Corps => "ARMY_ECHELON_CORPS",
            Self::Army => "ARMY_ECHELON_ARMY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ArmyEchelon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ARMY_ECHELON_INVALID",
            "ARMY_ECHELON_FIRE_TEAM",
            "ARMY_ECHELON_SQUAD",
            "ARMY_ECHELON_PLATOON",
            "ARMY_ECHELON_COMPANY",
            "ARMY_ECHELON_BATTALION",
            "ARMY_ECHELON_REGIMENT",
            "ARMY_ECHELON_BRIGADE",
            "ARMY_ECHELON_DIVISION",
            "ARMY_ECHELON_CORPS",
            "ARMY_ECHELON_ARMY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArmyEchelon;

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
                    "ARMY_ECHELON_INVALID" => Ok(ArmyEchelon::Invalid),
                    "ARMY_ECHELON_FIRE_TEAM" => Ok(ArmyEchelon::FireTeam),
                    "ARMY_ECHELON_SQUAD" => Ok(ArmyEchelon::Squad),
                    "ARMY_ECHELON_PLATOON" => Ok(ArmyEchelon::Platoon),
                    "ARMY_ECHELON_COMPANY" => Ok(ArmyEchelon::Company),
                    "ARMY_ECHELON_BATTALION" => Ok(ArmyEchelon::Battalion),
                    "ARMY_ECHELON_REGIMENT" => Ok(ArmyEchelon::Regiment),
                    "ARMY_ECHELON_BRIGADE" => Ok(ArmyEchelon::Brigade),
                    "ARMY_ECHELON_DIVISION" => Ok(ArmyEchelon::Division),
                    "ARMY_ECHELON_CORPS" => Ok(ArmyEchelon::Corps),
                    "ARMY_ECHELON_ARMY" => Ok(ArmyEchelon::Army),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Bandwidth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bandwidth_hz.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Bandwidth", len)?;
        if let Some(v) = self.bandwidth_hz.as_ref() {
            struct_ser.serialize_field("bandwidthHz", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bandwidth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bandwidth_hz",
            "bandwidthHz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BandwidthHz,
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
                            "bandwidthHz" | "bandwidth_hz" => Ok(GeneratedField::BandwidthHz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bandwidth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Bandwidth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Bandwidth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bandwidth_hz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BandwidthHz => {
                            if bandwidth_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandwidthHz"));
                            }
                            bandwidth_hz__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Bandwidth {
                    bandwidth_hz: bandwidth_hz__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Bandwidth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BandwidthRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum_bandwidth.is_some() {
            len += 1;
        }
        if self.maximum_bandwidth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.BandwidthRange", len)?;
        if let Some(v) = self.minimum_bandwidth.as_ref() {
            struct_ser.serialize_field("minimumBandwidth", v)?;
        }
        if let Some(v) = self.maximum_bandwidth.as_ref() {
            struct_ser.serialize_field("maximumBandwidth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BandwidthRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_bandwidth",
            "minimumBandwidth",
            "maximum_bandwidth",
            "maximumBandwidth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumBandwidth,
            MaximumBandwidth,
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
                            "minimumBandwidth" | "minimum_bandwidth" => Ok(GeneratedField::MinimumBandwidth),
                            "maximumBandwidth" | "maximum_bandwidth" => Ok(GeneratedField::MaximumBandwidth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BandwidthRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.BandwidthRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BandwidthRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum_bandwidth__ = None;
                let mut maximum_bandwidth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinimumBandwidth => {
                            if minimum_bandwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumBandwidth"));
                            }
                            minimum_bandwidth__ = map_.next_value()?;
                        }
                        GeneratedField::MaximumBandwidth => {
                            if maximum_bandwidth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximumBandwidth"));
                            }
                            maximum_bandwidth__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BandwidthRange {
                    minimum_bandwidth: minimum_bandwidth__,
                    maximum_bandwidth: maximum_bandwidth__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.BandwidthRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BooleanType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.BooleanType", len)?;
        if self.value {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BooleanType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BooleanType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.BooleanType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BooleanType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BooleanType {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.BooleanType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BoundedShapeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.BoundedShapeType", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                bounded_shape_type::Value::PolygonValue(v) => {
                    struct_ser.serialize_field("polygonValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BoundedShapeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "polygon_value",
            "polygonValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolygonValue,
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
                            "polygonValue" | "polygon_value" => Ok(GeneratedField::PolygonValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BoundedShapeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.BoundedShapeType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BoundedShapeType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolygonValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("polygonValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(bounded_shape_type::Value::PolygonValue)
;
                        }
                    }
                }
                Ok(BoundedShapeType {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.BoundedShapeType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Classification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default.is_some() {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Classification", len)?;
        if let Some(v) = self.default.as_ref() {
            struct_ser.serialize_field("default", v)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Classification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default",
            "fields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Default,
            Fields,
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
                            "default" => Ok(GeneratedField::Default),
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Classification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Classification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Classification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default__ = None;
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Default => {
                            if default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            default__ = map_.next_value()?;
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Classification {
                    default: default__,
                    fields: fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Classification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassificationInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.level != 0 {
            len += 1;
        }
        if !self.caveats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ClassificationInformation", len)?;
        if self.level != 0 {
            let v = ClassificationLevels::try_from(self.level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.level)))?;
            struct_ser.serialize_field("level", &v)?;
        }
        if !self.caveats.is_empty() {
            struct_ser.serialize_field("caveats", &self.caveats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassificationInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "level",
            "caveats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
            Caveats,
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
                            "level" => Ok(GeneratedField::Level),
                            "caveats" => Ok(GeneratedField::Caveats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassificationInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ClassificationInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClassificationInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                let mut caveats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map_.next_value::<ClassificationLevels>()? as i32);
                        }
                        GeneratedField::Caveats => {
                            if caveats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caveats"));
                            }
                            caveats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClassificationInformation {
                    level: level__.unwrap_or_default(),
                    caveats: caveats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ClassificationInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassificationLevels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CLASSIFICATION_LEVELS_INVALID",
            Self::Unclassified => "CLASSIFICATION_LEVELS_UNCLASSIFIED",
            Self::ControlledUnclassified => "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED",
            Self::Confidential => "CLASSIFICATION_LEVELS_CONFIDENTIAL",
            Self::Secret => "CLASSIFICATION_LEVELS_SECRET",
            Self::TopSecret => "CLASSIFICATION_LEVELS_TOP_SECRET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClassificationLevels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLASSIFICATION_LEVELS_INVALID",
            "CLASSIFICATION_LEVELS_UNCLASSIFIED",
            "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED",
            "CLASSIFICATION_LEVELS_CONFIDENTIAL",
            "CLASSIFICATION_LEVELS_SECRET",
            "CLASSIFICATION_LEVELS_TOP_SECRET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassificationLevels;

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
                    "CLASSIFICATION_LEVELS_INVALID" => Ok(ClassificationLevels::Invalid),
                    "CLASSIFICATION_LEVELS_UNCLASSIFIED" => Ok(ClassificationLevels::Unclassified),
                    "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED" => Ok(ClassificationLevels::ControlledUnclassified),
                    "CLASSIFICATION_LEVELS_CONFIDENTIAL" => Ok(ClassificationLevels::Confidential),
                    "CLASSIFICATION_LEVELS_SECRET" => Ok(ClassificationLevels::Secret),
                    "CLASSIFICATION_LEVELS_TOP_SECRET" => Ok(ClassificationLevels::TopSecret),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Comparator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "COMPARATOR_INVALID",
            Self::MatchAll => "COMPARATOR_MATCH_ALL",
            Self::Equality => "COMPARATOR_EQUALITY",
            Self::In => "COMPARATOR_IN",
            Self::LessThan => "COMPARATOR_LESS_THAN",
            Self::GreaterThan => "COMPARATOR_GREATER_THAN",
            Self::LessThanEqualTo => "COMPARATOR_LESS_THAN_EQUAL_TO",
            Self::GreaterThanEqualTo => "COMPARATOR_GREATER_THAN_EQUAL_TO",
            Self::Within => "COMPARATOR_WITHIN",
            Self::Exists => "COMPARATOR_EXISTS",
            Self::CaseInsensitiveEquality => "COMPARATOR_CASE_INSENSITIVE_EQUALITY",
            Self::CaseInsensitiveEqualityIn => "COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN",
            Self::RangeClosed => "COMPARATOR_RANGE_CLOSED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Comparator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COMPARATOR_INVALID",
            "COMPARATOR_MATCH_ALL",
            "COMPARATOR_EQUALITY",
            "COMPARATOR_IN",
            "COMPARATOR_LESS_THAN",
            "COMPARATOR_GREATER_THAN",
            "COMPARATOR_LESS_THAN_EQUAL_TO",
            "COMPARATOR_GREATER_THAN_EQUAL_TO",
            "COMPARATOR_WITHIN",
            "COMPARATOR_EXISTS",
            "COMPARATOR_CASE_INSENSITIVE_EQUALITY",
            "COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN",
            "COMPARATOR_RANGE_CLOSED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Comparator;

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
                    "COMPARATOR_INVALID" => Ok(Comparator::Invalid),
                    "COMPARATOR_MATCH_ALL" => Ok(Comparator::MatchAll),
                    "COMPARATOR_EQUALITY" => Ok(Comparator::Equality),
                    "COMPARATOR_IN" => Ok(Comparator::In),
                    "COMPARATOR_LESS_THAN" => Ok(Comparator::LessThan),
                    "COMPARATOR_GREATER_THAN" => Ok(Comparator::GreaterThan),
                    "COMPARATOR_LESS_THAN_EQUAL_TO" => Ok(Comparator::LessThanEqualTo),
                    "COMPARATOR_GREATER_THAN_EQUAL_TO" => Ok(Comparator::GreaterThanEqualTo),
                    "COMPARATOR_WITHIN" => Ok(Comparator::Within),
                    "COMPARATOR_EXISTS" => Ok(Comparator::Exists),
                    "COMPARATOR_CASE_INSENSITIVE_EQUALITY" => Ok(Comparator::CaseInsensitiveEquality),
                    "COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN" => Ok(Comparator::CaseInsensitiveEqualityIn),
                    "COMPARATOR_RANGE_CLOSED" => Ok(Comparator::RangeClosed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ComponentHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.health != 0 {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ComponentHealth", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.health != 0 {
            let v = HealthStatus::try_from(self.health)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health)))?;
            struct_ser.serialize_field("health", &v)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ComponentHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "health",
            "messages",
            "update_time",
            "updateTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Health,
            Messages,
            UpdateTime,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "health" => Ok(GeneratedField::Health),
                            "messages" => Ok(GeneratedField::Messages),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ComponentHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ComponentHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ComponentHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut health__ = None;
                let mut messages__ = None;
                let mut update_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Health => {
                            if health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("health"));
                            }
                            health__ = Some(map_.next_value::<HealthStatus>()? as i32);
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ComponentHealth {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    health: health__.unwrap_or_default(),
                    messages: messages__.unwrap_or_default(),
                    update_time: update_time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ComponentHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ComponentMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ComponentMessage", len)?;
        if self.status != 0 {
            let v = HealthStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ComponentMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Message,
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
                            "status" => Ok(GeneratedField::Status),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ComponentMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ComponentMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ComponentMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<HealthStatus>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ComponentMessage {
                    status: status__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ComponentMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConnectionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CONNECTION_STATUS_INVALID",
            Self::Online => "CONNECTION_STATUS_ONLINE",
            Self::Offline => "CONNECTION_STATUS_OFFLINE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONNECTION_STATUS_INVALID",
            "CONNECTION_STATUS_ONLINE",
            "CONNECTION_STATUS_OFFLINE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionStatus;

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
                    "CONNECTION_STATUS_INVALID" => Ok(ConnectionStatus::Invalid),
                    "CONNECTION_STATUS_ONLINE" => Ok(ConnectionStatus::Online),
                    "CONNECTION_STATUS_OFFLINE" => Ok(ConnectionStatus::Offline),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContactDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone_number.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ContactDetails", len)?;
        if !self.phone_number.is_empty() {
            struct_ser.serialize_field("phoneNumber", &self.phone_number)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContactDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone_number",
            "phoneNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PhoneNumber,
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
                            "phoneNumber" | "phone_number" => Ok(GeneratedField::PhoneNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContactDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ContactDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContactDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContactDetails {
                    phone_number: phone_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ContactDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Correlated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.primary_entity_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.scores.is_empty() {
            len += 1;
        }
        if self.expires_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Correlated", len)?;
        if !self.primary_entity_id.is_empty() {
            struct_ser.serialize_field("primaryEntityId", &self.primary_entity_id)?;
        }
        if self.status != 0 {
            let v = CorrelationStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.scores.is_empty() {
            struct_ser.serialize_field("scores", &self.scores)?;
        }
        if let Some(v) = self.expires_time.as_ref() {
            struct_ser.serialize_field("expiresTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Correlated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "primary_entity_id",
            "primaryEntityId",
            "status",
            "scores",
            "expires_time",
            "expiresTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrimaryEntityId,
            Status,
            Scores,
            ExpiresTime,
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
                            "primaryEntityId" | "primary_entity_id" => Ok(GeneratedField::PrimaryEntityId),
                            "status" => Ok(GeneratedField::Status),
                            "scores" => Ok(GeneratedField::Scores),
                            "expiresTime" | "expires_time" => Ok(GeneratedField::ExpiresTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Correlated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Correlated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Correlated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut primary_entity_id__ = None;
                let mut status__ = None;
                let mut scores__ = None;
                let mut expires_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrimaryEntityId => {
                            if primary_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryEntityId"));
                            }
                            primary_entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<CorrelationStatus>()? as i32);
                        }
                        GeneratedField::Scores => {
                            if scores__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scores"));
                            }
                            scores__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiresTime => {
                            if expires_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresTime"));
                            }
                            expires_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Correlated {
                    primary_entity_id: primary_entity_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    scores: scores__.unwrap_or_default(),
                    expires_time: expires_time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Correlated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Correlation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.membership.is_some() {
            len += 1;
        }
        if self.decorrelation.is_some() {
            len += 1;
        }
        if self.correlation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Correlation", len)?;
        if let Some(v) = self.membership.as_ref() {
            struct_ser.serialize_field("membership", v)?;
        }
        if let Some(v) = self.decorrelation.as_ref() {
            struct_ser.serialize_field("decorrelation", v)?;
        }
        if let Some(v) = self.correlation.as_ref() {
            match v {
                correlation::Correlation::Primary(v) => {
                    struct_ser.serialize_field("primary", v)?;
                }
                correlation::Correlation::Secondary(v) => {
                    struct_ser.serialize_field("secondary", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Correlation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "membership",
            "decorrelation",
            "primary",
            "secondary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Membership,
            Decorrelation,
            Primary,
            Secondary,
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
                            "membership" => Ok(GeneratedField::Membership),
                            "decorrelation" => Ok(GeneratedField::Decorrelation),
                            "primary" => Ok(GeneratedField::Primary),
                            "secondary" => Ok(GeneratedField::Secondary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Correlation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Correlation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Correlation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut membership__ = None;
                let mut decorrelation__ = None;
                let mut correlation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Membership => {
                            if membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("membership"));
                            }
                            membership__ = map_.next_value()?;
                        }
                        GeneratedField::Decorrelation => {
                            if decorrelation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decorrelation"));
                            }
                            decorrelation__ = map_.next_value()?;
                        }
                        GeneratedField::Primary => {
                            if correlation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            correlation__ = map_.next_value::<::std::option::Option<_>>()?.map(correlation::Correlation::Primary)
;
                        }
                        GeneratedField::Secondary => {
                            if correlation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondary"));
                            }
                            correlation__ = map_.next_value::<::std::option::Option<_>>()?.map(correlation::Correlation::Secondary)
;
                        }
                    }
                }
                Ok(Correlation {
                    membership: membership__,
                    decorrelation: decorrelation__,
                    correlation: correlation__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Correlation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationMembership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.correlation_set_id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.membership.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.CorrelationMembership", len)?;
        if !self.correlation_set_id.is_empty() {
            struct_ser.serialize_field("correlationSetId", &self.correlation_set_id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.membership.as_ref() {
            match v {
                correlation_membership::Membership::Primary(v) => {
                    struct_ser.serialize_field("primary", v)?;
                }
                correlation_membership::Membership::NonPrimary(v) => {
                    struct_ser.serialize_field("nonPrimary", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationMembership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "correlation_set_id",
            "correlationSetId",
            "metadata",
            "primary",
            "non_primary",
            "nonPrimary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CorrelationSetId,
            Metadata,
            Primary,
            NonPrimary,
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
                            "correlationSetId" | "correlation_set_id" => Ok(GeneratedField::CorrelationSetId),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "primary" => Ok(GeneratedField::Primary),
                            "nonPrimary" | "non_primary" => Ok(GeneratedField::NonPrimary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationMembership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.CorrelationMembership")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CorrelationMembership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut correlation_set_id__ = None;
                let mut metadata__ = None;
                let mut membership__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CorrelationSetId => {
                            if correlation_set_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("correlationSetId"));
                            }
                            correlation_set_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Primary => {
                            if membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            membership__ = map_.next_value::<::std::option::Option<_>>()?.map(correlation_membership::Membership::Primary)
;
                        }
                        GeneratedField::NonPrimary => {
                            if membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonPrimary"));
                            }
                            membership__ = map_.next_value::<::std::option::Option<_>>()?.map(correlation_membership::Membership::NonPrimary)
;
                        }
                    }
                }
                Ok(CorrelationMembership {
                    correlation_set_id: correlation_set_id__.unwrap_or_default(),
                    metadata: metadata__,
                    membership: membership__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.CorrelationMembership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provenance.is_some() {
            len += 1;
        }
        if self.replication_mode != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.CorrelationMetadata", len)?;
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        if self.replication_mode != 0 {
            let v = CorrelationReplicationMode::try_from(self.replication_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.replication_mode)))?;
            struct_ser.serialize_field("replicationMode", &v)?;
        }
        if self.r#type != 0 {
            let v = CorrelationType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provenance",
            "replication_mode",
            "replicationMode",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provenance,
            ReplicationMode,
            Type,
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
                            "provenance" => Ok(GeneratedField::Provenance),
                            "replicationMode" | "replication_mode" => Ok(GeneratedField::ReplicationMode),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.CorrelationMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CorrelationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provenance__ = None;
                let mut replication_mode__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                        GeneratedField::ReplicationMode => {
                            if replication_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replicationMode"));
                            }
                            replication_mode__ = Some(map_.next_value::<CorrelationReplicationMode>()? as i32);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<CorrelationType>()? as i32);
                        }
                    }
                }
                Ok(CorrelationMetadata {
                    provenance: provenance__,
                    replication_mode: replication_mode__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.CorrelationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationReplicationMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CORRELATION_REPLICATION_MODE_INVALID",
            Self::Local => "CORRELATION_REPLICATION_MODE_LOCAL",
            Self::Global => "CORRELATION_REPLICATION_MODE_GLOBAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationReplicationMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CORRELATION_REPLICATION_MODE_INVALID",
            "CORRELATION_REPLICATION_MODE_LOCAL",
            "CORRELATION_REPLICATION_MODE_GLOBAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationReplicationMode;

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
                    "CORRELATION_REPLICATION_MODE_INVALID" => Ok(CorrelationReplicationMode::Invalid),
                    "CORRELATION_REPLICATION_MODE_LOCAL" => Ok(CorrelationReplicationMode::Local),
                    "CORRELATION_REPLICATION_MODE_GLOBAL" => Ok(CorrelationReplicationMode::Global),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationScore {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.other_entity_id.is_empty() {
            len += 1;
        }
        if self.score != 0. {
            len += 1;
        }
        if self.interpretation != 0 {
            len += 1;
        }
        if self.link16_compliant {
            len += 1;
        }
        if self.other_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.CorrelationScore", len)?;
        if !self.other_entity_id.is_empty() {
            struct_ser.serialize_field("otherEntityId", &self.other_entity_id)?;
        }
        if self.score != 0. {
            struct_ser.serialize_field("score", &self.score)?;
        }
        if self.interpretation != 0 {
            let v = ScoreInterpretation::try_from(self.interpretation)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.interpretation)))?;
            struct_ser.serialize_field("interpretation", &v)?;
        }
        if self.link16_compliant {
            struct_ser.serialize_field("link16Compliant", &self.link16_compliant)?;
        }
        if self.other_status != 0 {
            let v = CorrelationStatus::try_from(self.other_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.other_status)))?;
            struct_ser.serialize_field("otherStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationScore {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "other_entity_id",
            "otherEntityId",
            "score",
            "interpretation",
            "link16_compliant",
            "link16Compliant",
            "other_status",
            "otherStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OtherEntityId,
            Score,
            Interpretation,
            Link16Compliant,
            OtherStatus,
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
                            "otherEntityId" | "other_entity_id" => Ok(GeneratedField::OtherEntityId),
                            "score" => Ok(GeneratedField::Score),
                            "interpretation" => Ok(GeneratedField::Interpretation),
                            "link16Compliant" | "link16_compliant" => Ok(GeneratedField::Link16Compliant),
                            "otherStatus" | "other_status" => Ok(GeneratedField::OtherStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationScore;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.CorrelationScore")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CorrelationScore, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut other_entity_id__ = None;
                let mut score__ = None;
                let mut interpretation__ = None;
                let mut link16_compliant__ = None;
                let mut other_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OtherEntityId => {
                            if other_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherEntityId"));
                            }
                            other_entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Score => {
                            if score__.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            score__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Interpretation => {
                            if interpretation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interpretation"));
                            }
                            interpretation__ = Some(map_.next_value::<ScoreInterpretation>()? as i32);
                        }
                        GeneratedField::Link16Compliant => {
                            if link16_compliant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link16Compliant"));
                            }
                            link16_compliant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtherStatus => {
                            if other_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherStatus"));
                            }
                            other_status__ = Some(map_.next_value::<CorrelationStatus>()? as i32);
                        }
                    }
                }
                Ok(CorrelationScore {
                    other_entity_id: other_entity_id__.unwrap_or_default(),
                    score: score__.unwrap_or_default(),
                    interpretation: interpretation__.unwrap_or_default(),
                    link16_compliant: link16_compliant__.unwrap_or_default(),
                    other_status: other_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.CorrelationScore", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CORRELATION_STATUS_INVALID",
            Self::ManualInspection => "CORRELATION_STATUS_MANUAL_INSPECTION",
            Self::AutoSuggested => "CORRELATION_STATUS_AUTO_SUGGESTED",
            Self::StartCorrelate => "CORRELATION_STATUS_START_CORRELATE",
            Self::Confirmed => "CORRELATION_STATUS_CONFIRMED",
            Self::Denied => "CORRELATION_STATUS_DENIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CORRELATION_STATUS_INVALID",
            "CORRELATION_STATUS_MANUAL_INSPECTION",
            "CORRELATION_STATUS_AUTO_SUGGESTED",
            "CORRELATION_STATUS_START_CORRELATE",
            "CORRELATION_STATUS_CONFIRMED",
            "CORRELATION_STATUS_DENIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationStatus;

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
                    "CORRELATION_STATUS_INVALID" => Ok(CorrelationStatus::Invalid),
                    "CORRELATION_STATUS_MANUAL_INSPECTION" => Ok(CorrelationStatus::ManualInspection),
                    "CORRELATION_STATUS_AUTO_SUGGESTED" => Ok(CorrelationStatus::AutoSuggested),
                    "CORRELATION_STATUS_START_CORRELATE" => Ok(CorrelationStatus::StartCorrelate),
                    "CORRELATION_STATUS_CONFIRMED" => Ok(CorrelationStatus::Confirmed),
                    "CORRELATION_STATUS_DENIED" => Ok(CorrelationStatus::Denied),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CorrelationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "CORRELATION_TYPE_INVALID",
            Self::Manual => "CORRELATION_TYPE_MANUAL",
            Self::Automated => "CORRELATION_TYPE_AUTOMATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CorrelationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CORRELATION_TYPE_INVALID",
            "CORRELATION_TYPE_MANUAL",
            "CORRELATION_TYPE_AUTOMATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorrelationType;

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
                    "CORRELATION_TYPE_INVALID" => Ok(CorrelationType::Invalid),
                    "CORRELATION_TYPE_MANUAL" => Ok(CorrelationType::Manual),
                    "CORRELATION_TYPE_AUTOMATED" => Ok(CorrelationType::Automated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CronWindow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cron_expression.is_empty() {
            len += 1;
        }
        if self.duration_millis != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.CronWindow", len)?;
        if !self.cron_expression.is_empty() {
            struct_ser.serialize_field("cronExpression", &self.cron_expression)?;
        }
        if self.duration_millis != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("durationMillis", ToString::to_string(&self.duration_millis).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CronWindow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cron_expression",
            "cronExpression",
            "duration_millis",
            "durationMillis",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CronExpression,
            DurationMillis,
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
                            "cronExpression" | "cron_expression" => Ok(GeneratedField::CronExpression),
                            "durationMillis" | "duration_millis" => Ok(GeneratedField::DurationMillis),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CronWindow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.CronWindow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CronWindow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cron_expression__ = None;
                let mut duration_millis__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CronExpression => {
                            if cron_expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cronExpression"));
                            }
                            cron_expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationMillis => {
                            if duration_millis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationMillis"));
                            }
                            duration_millis__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CronWindow {
                    cron_expression: cron_expression__.unwrap_or_default(),
                    duration_millis: duration_millis__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.CronWindow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecorrelatedAll {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.DecorrelatedAll", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecorrelatedAll {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecorrelatedAll;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.DecorrelatedAll")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DecorrelatedAll, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DecorrelatedAll {
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.DecorrelatedAll", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecorrelatedSingle {
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
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.DecorrelatedSingle", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecorrelatedSingle {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            Metadata,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecorrelatedSingle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.DecorrelatedSingle")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DecorrelatedSingle, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DecorrelatedSingle {
                    entity_id: entity_id__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.DecorrelatedSingle", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Decorrelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.all.is_some() {
            len += 1;
        }
        if !self.decorrelated_entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Decorrelation", len)?;
        if let Some(v) = self.all.as_ref() {
            struct_ser.serialize_field("all", v)?;
        }
        if !self.decorrelated_entities.is_empty() {
            struct_ser.serialize_field("decorrelatedEntities", &self.decorrelated_entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decorrelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "all",
            "decorrelated_entities",
            "decorrelatedEntities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            All,
            DecorrelatedEntities,
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
                            "all" => Ok(GeneratedField::All),
                            "decorrelatedEntities" | "decorrelated_entities" => Ok(GeneratedField::DecorrelatedEntities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decorrelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Decorrelation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Decorrelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut all__ = None;
                let mut decorrelated_entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::All => {
                            if all__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            all__ = map_.next_value()?;
                        }
                        GeneratedField::DecorrelatedEntities => {
                            if decorrelated_entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decorrelatedEntities"));
                            }
                            decorrelated_entities__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Decorrelation {
                    all: all__,
                    decorrelated_entities: decorrelated_entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Decorrelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Dimensions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.length_m != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Dimensions", len)?;
        if self.length_m != 0. {
            struct_ser.serialize_field("lengthM", &self.length_m)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dimensions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "length_m",
            "lengthM",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LengthM,
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
                            "lengthM" | "length_m" => Ok(GeneratedField::LengthM),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dimensions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Dimensions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dimensions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut length_m__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LengthM => {
                            if length_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lengthM"));
                            }
                            length_m__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Dimensions {
                    length_m: length_m__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Dimensions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicStatement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        if self.selector.is_some() {
            len += 1;
        }
        if self.comparator.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.DynamicStatement", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.selector.as_ref() {
            struct_ser.serialize_field("selector", v)?;
        }
        if let Some(v) = self.comparator.as_ref() {
            struct_ser.serialize_field("comparator", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicStatement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "selector",
            "comparator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            Selector,
            Comparator,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "selector" => Ok(GeneratedField::Selector),
                            "comparator" => Ok(GeneratedField::Comparator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicStatement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.DynamicStatement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicStatement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                let mut selector__ = None;
                let mut comparator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Selector => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selector"));
                            }
                            selector__ = map_.next_value()?;
                        }
                        GeneratedField::Comparator => {
                            if comparator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparator"));
                            }
                            comparator__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DynamicStatement {
                    filter: filter__,
                    selector: selector__,
                    comparator: comparator__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.DynamicStatement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Echelon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.echelon_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Echelon", len)?;
        if let Some(v) = self.echelon_type.as_ref() {
            match v {
                echelon::EchelonType::ArmyEchelon(v) => {
                    let v = ArmyEchelon::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("armyEchelon", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Echelon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "army_echelon",
            "armyEchelon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArmyEchelon,
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
                            "armyEchelon" | "army_echelon" => Ok(GeneratedField::ArmyEchelon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Echelon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Echelon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Echelon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut echelon_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArmyEchelon => {
                            if echelon_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("armyEchelon"));
                            }
                            echelon_type__ = map_.next_value::<::std::option::Option<ArmyEchelon>>()?.map(|x| echelon::EchelonType::ArmyEchelon(x as i32));
                        }
                    }
                }
                Ok(Echelon {
                    echelon_type: echelon_type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Echelon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmitterNotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.emitter_notation.is_empty() {
            len += 1;
        }
        if self.confidence.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.EmitterNotation", len)?;
        if !self.emitter_notation.is_empty() {
            struct_ser.serialize_field("emitterNotation", &self.emitter_notation)?;
        }
        if let Some(v) = self.confidence.as_ref() {
            struct_ser.serialize_field("confidence", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmitterNotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "emitter_notation",
            "emitterNotation",
            "confidence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmitterNotation,
            Confidence,
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
                            "emitterNotation" | "emitter_notation" => Ok(GeneratedField::EmitterNotation),
                            "confidence" => Ok(GeneratedField::Confidence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmitterNotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.EmitterNotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmitterNotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut emitter_notation__ = None;
                let mut confidence__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmitterNotation => {
                            if emitter_notation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitterNotation"));
                            }
                            emitter_notation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Confidence => {
                            if confidence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confidence"));
                            }
                            confidence__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EmitterNotation {
                    emitter_notation: emitter_notation__.unwrap_or_default(),
                    confidence: confidence__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.EmitterNotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Entity {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_live {
            len += 1;
        }
        if self.created_time.is_some() {
            len += 1;
        }
        if self.expiry_time.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.location.is_some() {
            len += 1;
        }
        if self.location_uncertainty.is_some() {
            len += 1;
        }
        if self.geo_shape.is_some() {
            len += 1;
        }
        if self.geo_details.is_some() {
            len += 1;
        }
        if self.aliases.is_some() {
            len += 1;
        }
        if self.tracked.is_some() {
            len += 1;
        }
        if self.correlation.is_some() {
            len += 1;
        }
        if self.mil_view.is_some() {
            len += 1;
        }
        if self.ontology.is_some() {
            len += 1;
        }
        if self.sensors.is_some() {
            len += 1;
        }
        if self.payloads.is_some() {
            len += 1;
        }
        if self.power_state.is_some() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        if self.overrides.is_some() {
            len += 1;
        }
        if self.indicators.is_some() {
            len += 1;
        }
        if self.target_priority.is_some() {
            len += 1;
        }
        if self.signal.is_some() {
            len += 1;
        }
        if self.transponder_codes.is_some() {
            len += 1;
        }
        if self.data_classification.is_some() {
            len += 1;
        }
        if self.task_catalog.is_some() {
            len += 1;
        }
        if self.relationships.is_some() {
            len += 1;
        }
        if self.visual_details.is_some() {
            len += 1;
        }
        if self.dimensions.is_some() {
            len += 1;
        }
        if self.route_details.is_some() {
            len += 1;
        }
        if self.schedules.is_some() {
            len += 1;
        }
        if self.health.is_some() {
            len += 1;
        }
        if self.group_details.is_some() {
            len += 1;
        }
        if self.supplies.is_some() {
            len += 1;
        }
        if self.orbit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Entity", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_live {
            struct_ser.serialize_field("isLive", &self.is_live)?;
        }
        if let Some(v) = self.created_time.as_ref() {
            struct_ser.serialize_field("createdTime", v)?;
        }
        if let Some(v) = self.expiry_time.as_ref() {
            struct_ser.serialize_field("expiryTime", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.location.as_ref() {
            struct_ser.serialize_field("location", v)?;
        }
        if let Some(v) = self.location_uncertainty.as_ref() {
            struct_ser.serialize_field("locationUncertainty", v)?;
        }
        if let Some(v) = self.geo_shape.as_ref() {
            struct_ser.serialize_field("geoShape", v)?;
        }
        if let Some(v) = self.geo_details.as_ref() {
            struct_ser.serialize_field("geoDetails", v)?;
        }
        if let Some(v) = self.aliases.as_ref() {
            struct_ser.serialize_field("aliases", v)?;
        }
        if let Some(v) = self.tracked.as_ref() {
            struct_ser.serialize_field("tracked", v)?;
        }
        if let Some(v) = self.correlation.as_ref() {
            struct_ser.serialize_field("correlation", v)?;
        }
        if let Some(v) = self.mil_view.as_ref() {
            struct_ser.serialize_field("milView", v)?;
        }
        if let Some(v) = self.ontology.as_ref() {
            struct_ser.serialize_field("ontology", v)?;
        }
        if let Some(v) = self.sensors.as_ref() {
            struct_ser.serialize_field("sensors", v)?;
        }
        if let Some(v) = self.payloads.as_ref() {
            struct_ser.serialize_field("payloads", v)?;
        }
        if let Some(v) = self.power_state.as_ref() {
            struct_ser.serialize_field("powerState", v)?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        if let Some(v) = self.overrides.as_ref() {
            struct_ser.serialize_field("overrides", v)?;
        }
        if let Some(v) = self.indicators.as_ref() {
            struct_ser.serialize_field("indicators", v)?;
        }
        if let Some(v) = self.target_priority.as_ref() {
            struct_ser.serialize_field("targetPriority", v)?;
        }
        if let Some(v) = self.signal.as_ref() {
            struct_ser.serialize_field("signal", v)?;
        }
        if let Some(v) = self.transponder_codes.as_ref() {
            struct_ser.serialize_field("transponderCodes", v)?;
        }
        if let Some(v) = self.data_classification.as_ref() {
            struct_ser.serialize_field("dataClassification", v)?;
        }
        if let Some(v) = self.task_catalog.as_ref() {
            struct_ser.serialize_field("taskCatalog", v)?;
        }
        if let Some(v) = self.relationships.as_ref() {
            struct_ser.serialize_field("relationships", v)?;
        }
        if let Some(v) = self.visual_details.as_ref() {
            struct_ser.serialize_field("visualDetails", v)?;
        }
        if let Some(v) = self.dimensions.as_ref() {
            struct_ser.serialize_field("dimensions", v)?;
        }
        if let Some(v) = self.route_details.as_ref() {
            struct_ser.serialize_field("routeDetails", v)?;
        }
        if let Some(v) = self.schedules.as_ref() {
            struct_ser.serialize_field("schedules", v)?;
        }
        if let Some(v) = self.health.as_ref() {
            struct_ser.serialize_field("health", v)?;
        }
        if let Some(v) = self.group_details.as_ref() {
            struct_ser.serialize_field("groupDetails", v)?;
        }
        if let Some(v) = self.supplies.as_ref() {
            struct_ser.serialize_field("supplies", v)?;
        }
        if let Some(v) = self.orbit.as_ref() {
            struct_ser.serialize_field("orbit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Entity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "description",
            "is_live",
            "isLive",
            "created_time",
            "createdTime",
            "expiry_time",
            "expiryTime",
            "status",
            "location",
            "location_uncertainty",
            "locationUncertainty",
            "geo_shape",
            "geoShape",
            "geo_details",
            "geoDetails",
            "aliases",
            "tracked",
            "correlation",
            "mil_view",
            "milView",
            "ontology",
            "sensors",
            "payloads",
            "power_state",
            "powerState",
            "provenance",
            "overrides",
            "indicators",
            "target_priority",
            "targetPriority",
            "signal",
            "transponder_codes",
            "transponderCodes",
            "data_classification",
            "dataClassification",
            "task_catalog",
            "taskCatalog",
            "relationships",
            "visual_details",
            "visualDetails",
            "dimensions",
            "route_details",
            "routeDetails",
            "schedules",
            "health",
            "group_details",
            "groupDetails",
            "supplies",
            "orbit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            Description,
            IsLive,
            CreatedTime,
            ExpiryTime,
            Status,
            Location,
            LocationUncertainty,
            GeoShape,
            GeoDetails,
            Aliases,
            Tracked,
            Correlation,
            MilView,
            Ontology,
            Sensors,
            Payloads,
            PowerState,
            Provenance,
            Overrides,
            Indicators,
            TargetPriority,
            Signal,
            TransponderCodes,
            DataClassification,
            TaskCatalog,
            Relationships,
            VisualDetails,
            Dimensions,
            RouteDetails,
            Schedules,
            Health,
            GroupDetails,
            Supplies,
            Orbit,
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
                            "description" => Ok(GeneratedField::Description),
                            "isLive" | "is_live" => Ok(GeneratedField::IsLive),
                            "createdTime" | "created_time" => Ok(GeneratedField::CreatedTime),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            "status" => Ok(GeneratedField::Status),
                            "location" => Ok(GeneratedField::Location),
                            "locationUncertainty" | "location_uncertainty" => Ok(GeneratedField::LocationUncertainty),
                            "geoShape" | "geo_shape" => Ok(GeneratedField::GeoShape),
                            "geoDetails" | "geo_details" => Ok(GeneratedField::GeoDetails),
                            "aliases" => Ok(GeneratedField::Aliases),
                            "tracked" => Ok(GeneratedField::Tracked),
                            "correlation" => Ok(GeneratedField::Correlation),
                            "milView" | "mil_view" => Ok(GeneratedField::MilView),
                            "ontology" => Ok(GeneratedField::Ontology),
                            "sensors" => Ok(GeneratedField::Sensors),
                            "payloads" => Ok(GeneratedField::Payloads),
                            "powerState" | "power_state" => Ok(GeneratedField::PowerState),
                            "provenance" => Ok(GeneratedField::Provenance),
                            "overrides" => Ok(GeneratedField::Overrides),
                            "indicators" => Ok(GeneratedField::Indicators),
                            "targetPriority" | "target_priority" => Ok(GeneratedField::TargetPriority),
                            "signal" => Ok(GeneratedField::Signal),
                            "transponderCodes" | "transponder_codes" => Ok(GeneratedField::TransponderCodes),
                            "dataClassification" | "data_classification" => Ok(GeneratedField::DataClassification),
                            "taskCatalog" | "task_catalog" => Ok(GeneratedField::TaskCatalog),
                            "relationships" => Ok(GeneratedField::Relationships),
                            "visualDetails" | "visual_details" => Ok(GeneratedField::VisualDetails),
                            "dimensions" => Ok(GeneratedField::Dimensions),
                            "routeDetails" | "route_details" => Ok(GeneratedField::RouteDetails),
                            "schedules" => Ok(GeneratedField::Schedules),
                            "health" => Ok(GeneratedField::Health),
                            "groupDetails" | "group_details" => Ok(GeneratedField::GroupDetails),
                            "supplies" => Ok(GeneratedField::Supplies),
                            "orbit" => Ok(GeneratedField::Orbit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Entity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Entity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Entity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut description__ = None;
                let mut is_live__ = None;
                let mut created_time__ = None;
                let mut expiry_time__ = None;
                let mut status__ = None;
                let mut location__ = None;
                let mut location_uncertainty__ = None;
                let mut geo_shape__ = None;
                let mut geo_details__ = None;
                let mut aliases__ = None;
                let mut tracked__ = None;
                let mut correlation__ = None;
                let mut mil_view__ = None;
                let mut ontology__ = None;
                let mut sensors__ = None;
                let mut payloads__ = None;
                let mut power_state__ = None;
                let mut provenance__ = None;
                let mut overrides__ = None;
                let mut indicators__ = None;
                let mut target_priority__ = None;
                let mut signal__ = None;
                let mut transponder_codes__ = None;
                let mut data_classification__ = None;
                let mut task_catalog__ = None;
                let mut relationships__ = None;
                let mut visual_details__ = None;
                let mut dimensions__ = None;
                let mut route_details__ = None;
                let mut schedules__ = None;
                let mut health__ = None;
                let mut group_details__ = None;
                let mut supplies__ = None;
                let mut orbit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsLive => {
                            if is_live__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isLive"));
                            }
                            is_live__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedTime => {
                            if created_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdTime"));
                            }
                            created_time__ = map_.next_value()?;
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = map_.next_value()?;
                        }
                        GeneratedField::LocationUncertainty => {
                            if location_uncertainty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationUncertainty"));
                            }
                            location_uncertainty__ = map_.next_value()?;
                        }
                        GeneratedField::GeoShape => {
                            if geo_shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("geoShape"));
                            }
                            geo_shape__ = map_.next_value()?;
                        }
                        GeneratedField::GeoDetails => {
                            if geo_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("geoDetails"));
                            }
                            geo_details__ = map_.next_value()?;
                        }
                        GeneratedField::Aliases => {
                            if aliases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            aliases__ = map_.next_value()?;
                        }
                        GeneratedField::Tracked => {
                            if tracked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracked"));
                            }
                            tracked__ = map_.next_value()?;
                        }
                        GeneratedField::Correlation => {
                            if correlation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("correlation"));
                            }
                            correlation__ = map_.next_value()?;
                        }
                        GeneratedField::MilView => {
                            if mil_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("milView"));
                            }
                            mil_view__ = map_.next_value()?;
                        }
                        GeneratedField::Ontology => {
                            if ontology__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ontology"));
                            }
                            ontology__ = map_.next_value()?;
                        }
                        GeneratedField::Sensors => {
                            if sensors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensors"));
                            }
                            sensors__ = map_.next_value()?;
                        }
                        GeneratedField::Payloads => {
                            if payloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloads"));
                            }
                            payloads__ = map_.next_value()?;
                        }
                        GeneratedField::PowerState => {
                            if power_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("powerState"));
                            }
                            power_state__ = map_.next_value()?;
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = map_.next_value()?;
                        }
                        GeneratedField::Indicators => {
                            if indicators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indicators"));
                            }
                            indicators__ = map_.next_value()?;
                        }
                        GeneratedField::TargetPriority => {
                            if target_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPriority"));
                            }
                            target_priority__ = map_.next_value()?;
                        }
                        GeneratedField::Signal => {
                            if signal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signal"));
                            }
                            signal__ = map_.next_value()?;
                        }
                        GeneratedField::TransponderCodes => {
                            if transponder_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transponderCodes"));
                            }
                            transponder_codes__ = map_.next_value()?;
                        }
                        GeneratedField::DataClassification => {
                            if data_classification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataClassification"));
                            }
                            data_classification__ = map_.next_value()?;
                        }
                        GeneratedField::TaskCatalog => {
                            if task_catalog__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskCatalog"));
                            }
                            task_catalog__ = map_.next_value()?;
                        }
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = map_.next_value()?;
                        }
                        GeneratedField::VisualDetails => {
                            if visual_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("visualDetails"));
                            }
                            visual_details__ = map_.next_value()?;
                        }
                        GeneratedField::Dimensions => {
                            if dimensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            dimensions__ = map_.next_value()?;
                        }
                        GeneratedField::RouteDetails => {
                            if route_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeDetails"));
                            }
                            route_details__ = map_.next_value()?;
                        }
                        GeneratedField::Schedules => {
                            if schedules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedules"));
                            }
                            schedules__ = map_.next_value()?;
                        }
                        GeneratedField::Health => {
                            if health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("health"));
                            }
                            health__ = map_.next_value()?;
                        }
                        GeneratedField::GroupDetails => {
                            if group_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupDetails"));
                            }
                            group_details__ = map_.next_value()?;
                        }
                        GeneratedField::Supplies => {
                            if supplies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplies"));
                            }
                            supplies__ = map_.next_value()?;
                        }
                        GeneratedField::Orbit => {
                            if orbit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orbit"));
                            }
                            orbit__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Entity {
                    entity_id: entity_id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    is_live: is_live__.unwrap_or_default(),
                    created_time: created_time__,
                    expiry_time: expiry_time__,
                    status: status__,
                    location: location__,
                    location_uncertainty: location_uncertainty__,
                    geo_shape: geo_shape__,
                    geo_details: geo_details__,
                    aliases: aliases__,
                    tracked: tracked__,
                    correlation: correlation__,
                    mil_view: mil_view__,
                    ontology: ontology__,
                    sensors: sensors__,
                    payloads: payloads__,
                    power_state: power_state__,
                    provenance: provenance__,
                    overrides: overrides__,
                    indicators: indicators__,
                    target_priority: target_priority__,
                    signal: signal__,
                    transponder_codes: transponder_codes__,
                    data_classification: data_classification__,
                    task_catalog: task_catalog__,
                    relationships: relationships__,
                    visual_details: visual_details__,
                    dimensions: dimensions__,
                    route_details: route_details__,
                    schedules: schedules__,
                    health: health__,
                    group_details: group_details__,
                    supplies: supplies__,
                    orbit: orbit__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Entity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_type != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.EntityEvent", len)?;
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_type",
            "eventType",
            "time",
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventType,
            Time,
            Entity,
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
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "time" => Ok(GeneratedField::Time),
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.EntityEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_type__ = None;
                let mut time__ = None;
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EntityEvent {
                    event_type: event_type__.unwrap_or_default(),
                    time: time__,
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.EntityEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.EnumType", len)?;
        if self.value != 0 {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.EnumType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EnumType {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.EnumType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorEllipse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.probability.is_some() {
            len += 1;
        }
        if self.semi_major_axis_m.is_some() {
            len += 1;
        }
        if self.semi_minor_axis_m.is_some() {
            len += 1;
        }
        if self.orientation_d.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ErrorEllipse", len)?;
        if let Some(v) = self.probability.as_ref() {
            struct_ser.serialize_field("probability", v)?;
        }
        if let Some(v) = self.semi_major_axis_m.as_ref() {
            struct_ser.serialize_field("semiMajorAxisM", v)?;
        }
        if let Some(v) = self.semi_minor_axis_m.as_ref() {
            struct_ser.serialize_field("semiMinorAxisM", v)?;
        }
        if let Some(v) = self.orientation_d.as_ref() {
            struct_ser.serialize_field("orientationD", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorEllipse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "probability",
            "semi_major_axis_m",
            "semiMajorAxisM",
            "semi_minor_axis_m",
            "semiMinorAxisM",
            "orientation_d",
            "orientationD",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Probability,
            SemiMajorAxisM,
            SemiMinorAxisM,
            OrientationD,
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
                            "probability" => Ok(GeneratedField::Probability),
                            "semiMajorAxisM" | "semi_major_axis_m" => Ok(GeneratedField::SemiMajorAxisM),
                            "semiMinorAxisM" | "semi_minor_axis_m" => Ok(GeneratedField::SemiMinorAxisM),
                            "orientationD" | "orientation_d" => Ok(GeneratedField::OrientationD),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorEllipse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ErrorEllipse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorEllipse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut probability__ = None;
                let mut semi_major_axis_m__ = None;
                let mut semi_minor_axis_m__ = None;
                let mut orientation_d__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Probability => {
                            if probability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("probability"));
                            }
                            probability__ = map_.next_value()?;
                        }
                        GeneratedField::SemiMajorAxisM => {
                            if semi_major_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semiMajorAxisM"));
                            }
                            semi_major_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::SemiMinorAxisM => {
                            if semi_minor_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semiMinorAxisM"));
                            }
                            semi_minor_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::OrientationD => {
                            if orientation_d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orientationD"));
                            }
                            orientation_d__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ErrorEllipse {
                    probability: probability__,
                    semi_major_axis_m: semi_major_axis_m__,
                    semi_minor_axis_m: semi_minor_axis_m__,
                    orientation_d: orientation_d__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ErrorEllipse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "EVENT_TYPE_INVALID",
            Self::Created => "EVENT_TYPE_CREATED",
            Self::Update => "EVENT_TYPE_UPDATE",
            Self::Deleted => "EVENT_TYPE_DELETED",
            Self::Preexisting => "EVENT_TYPE_PREEXISTING",
            Self::PostExpiryOverride => "EVENT_TYPE_POST_EXPIRY_OVERRIDE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EVENT_TYPE_INVALID",
            "EVENT_TYPE_CREATED",
            "EVENT_TYPE_UPDATE",
            "EVENT_TYPE_DELETED",
            "EVENT_TYPE_PREEXISTING",
            "EVENT_TYPE_POST_EXPIRY_OVERRIDE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventType;

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
                    "EVENT_TYPE_INVALID" => Ok(EventType::Invalid),
                    "EVENT_TYPE_CREATED" => Ok(EventType::Created),
                    "EVENT_TYPE_UPDATE" => Ok(EventType::Update),
                    "EVENT_TYPE_DELETED" => Ok(EventType::Deleted),
                    "EVENT_TYPE_PREEXISTING" => Ok(EventType::Preexisting),
                    "EVENT_TYPE_POST_EXPIRY_OVERRIDE" => Ok(EventType::PostExpiryOverride),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldClassificationInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.classification_information.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.FieldClassificationInformation", len)?;
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if let Some(v) = self.classification_information.as_ref() {
            struct_ser.serialize_field("classificationInformation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldClassificationInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_path",
            "fieldPath",
            "classification_information",
            "classificationInformation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldPath,
            ClassificationInformation,
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
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "classificationInformation" | "classification_information" => Ok(GeneratedField::ClassificationInformation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldClassificationInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.FieldClassificationInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldClassificationInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_path__ = None;
                let mut classification_information__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassificationInformation => {
                            if classification_information__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classificationInformation"));
                            }
                            classification_information__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FieldClassificationInformation {
                    field_path: field_path__.unwrap_or_default(),
                    classification_information: classification_information__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.FieldClassificationInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldOfView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fov_id != 0 {
            len += 1;
        }
        if !self.mount_id.is_empty() {
            len += 1;
        }
        if self.projected_frustum.is_some() {
            len += 1;
        }
        if self.projected_center_ray.is_some() {
            len += 1;
        }
        if self.center_ray_pose.is_some() {
            len += 1;
        }
        if self.horizontal_fov != 0. {
            len += 1;
        }
        if self.vertical_fov != 0. {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.FieldOfView", len)?;
        if self.fov_id != 0 {
            struct_ser.serialize_field("fovId", &self.fov_id)?;
        }
        if !self.mount_id.is_empty() {
            struct_ser.serialize_field("mountId", &self.mount_id)?;
        }
        if let Some(v) = self.projected_frustum.as_ref() {
            struct_ser.serialize_field("projectedFrustum", v)?;
        }
        if let Some(v) = self.projected_center_ray.as_ref() {
            struct_ser.serialize_field("projectedCenterRay", v)?;
        }
        if let Some(v) = self.center_ray_pose.as_ref() {
            struct_ser.serialize_field("centerRayPose", v)?;
        }
        if self.horizontal_fov != 0. {
            struct_ser.serialize_field("horizontalFov", &self.horizontal_fov)?;
        }
        if self.vertical_fov != 0. {
            struct_ser.serialize_field("verticalFov", &self.vertical_fov)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if self.mode != 0 {
            let v = SensorMode::try_from(self.mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldOfView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fov_id",
            "fovId",
            "mount_id",
            "mountId",
            "projected_frustum",
            "projectedFrustum",
            "projected_center_ray",
            "projectedCenterRay",
            "center_ray_pose",
            "centerRayPose",
            "horizontal_fov",
            "horizontalFov",
            "vertical_fov",
            "verticalFov",
            "range",
            "mode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FovId,
            MountId,
            ProjectedFrustum,
            ProjectedCenterRay,
            CenterRayPose,
            HorizontalFov,
            VerticalFov,
            Range,
            Mode,
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
                            "fovId" | "fov_id" => Ok(GeneratedField::FovId),
                            "mountId" | "mount_id" => Ok(GeneratedField::MountId),
                            "projectedFrustum" | "projected_frustum" => Ok(GeneratedField::ProjectedFrustum),
                            "projectedCenterRay" | "projected_center_ray" => Ok(GeneratedField::ProjectedCenterRay),
                            "centerRayPose" | "center_ray_pose" => Ok(GeneratedField::CenterRayPose),
                            "horizontalFov" | "horizontal_fov" => Ok(GeneratedField::HorizontalFov),
                            "verticalFov" | "vertical_fov" => Ok(GeneratedField::VerticalFov),
                            "range" => Ok(GeneratedField::Range),
                            "mode" => Ok(GeneratedField::Mode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldOfView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.FieldOfView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldOfView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fov_id__ = None;
                let mut mount_id__ = None;
                let mut projected_frustum__ = None;
                let mut projected_center_ray__ = None;
                let mut center_ray_pose__ = None;
                let mut horizontal_fov__ = None;
                let mut vertical_fov__ = None;
                let mut range__ = None;
                let mut mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FovId => {
                            if fov_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fovId"));
                            }
                            fov_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MountId => {
                            if mount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mountId"));
                            }
                            mount_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectedFrustum => {
                            if projected_frustum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectedFrustum"));
                            }
                            projected_frustum__ = map_.next_value()?;
                        }
                        GeneratedField::ProjectedCenterRay => {
                            if projected_center_ray__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectedCenterRay"));
                            }
                            projected_center_ray__ = map_.next_value()?;
                        }
                        GeneratedField::CenterRayPose => {
                            if center_ray_pose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("centerRayPose"));
                            }
                            center_ray_pose__ = map_.next_value()?;
                        }
                        GeneratedField::HorizontalFov => {
                            if horizontal_fov__.is_some() {
                                return Err(serde::de::Error::duplicate_field("horizontalFov"));
                            }
                            horizontal_fov__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VerticalFov => {
                            if vertical_fov__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verticalFov"));
                            }
                            vertical_fov__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = map_.next_value()?;
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value::<SensorMode>()? as i32);
                        }
                    }
                }
                Ok(FieldOfView {
                    fov_id: fov_id__.unwrap_or_default(),
                    mount_id: mount_id__.unwrap_or_default(),
                    projected_frustum: projected_frustum__,
                    projected_center_ray: projected_center_ray__,
                    center_ray_pose: center_ray_pose__,
                    horizontal_fov: horizontal_fov__.unwrap_or_default(),
                    vertical_fov: vertical_fov__.unwrap_or_default(),
                    range: range__,
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.FieldOfView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fixed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Fixed", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fixed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fixed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Fixed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fixed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Fixed {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Fixed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lower_bound != 0. {
            len += 1;
        }
        if self.upper_bound != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.FloatRange", len)?;
        if self.lower_bound != 0. {
            struct_ser.serialize_field("lowerBound", &self.lower_bound)?;
        }
        if self.upper_bound != 0. {
            struct_ser.serialize_field("upperBound", &self.upper_bound)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lower_bound",
            "lowerBound",
            "upper_bound",
            "upperBound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LowerBound,
            UpperBound,
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
                            "lowerBound" | "lower_bound" => Ok(GeneratedField::LowerBound),
                            "upperBound" | "upper_bound" => Ok(GeneratedField::UpperBound),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.FloatRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FloatRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lower_bound__ = None;
                let mut upper_bound__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LowerBound => {
                            if lower_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowerBound"));
                            }
                            lower_bound__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpperBound => {
                            if upper_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperBound"));
                            }
                            upper_bound__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FloatRange {
                    lower_bound: lower_bound__.unwrap_or_default(),
                    upper_bound: upper_bound__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.FloatRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Frequency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.frequency_hz.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Frequency", len)?;
        if let Some(v) = self.frequency_hz.as_ref() {
            struct_ser.serialize_field("frequencyHz", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Frequency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "frequency_hz",
            "frequencyHz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FrequencyHz,
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
                            "frequencyHz" | "frequency_hz" => Ok(GeneratedField::FrequencyHz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Frequency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Frequency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Frequency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut frequency_hz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FrequencyHz => {
                            if frequency_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frequencyHz"));
                            }
                            frequency_hz__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Frequency {
                    frequency_hz: frequency_hz__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Frequency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FrequencyRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum_frequency_hz.is_some() {
            len += 1;
        }
        if self.maximum_frequency_hz.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.FrequencyRange", len)?;
        if let Some(v) = self.minimum_frequency_hz.as_ref() {
            struct_ser.serialize_field("minimumFrequencyHz", v)?;
        }
        if let Some(v) = self.maximum_frequency_hz.as_ref() {
            struct_ser.serialize_field("maximumFrequencyHz", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FrequencyRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_frequency_hz",
            "minimumFrequencyHz",
            "maximum_frequency_hz",
            "maximumFrequencyHz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumFrequencyHz,
            MaximumFrequencyHz,
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
                            "minimumFrequencyHz" | "minimum_frequency_hz" => Ok(GeneratedField::MinimumFrequencyHz),
                            "maximumFrequencyHz" | "maximum_frequency_hz" => Ok(GeneratedField::MaximumFrequencyHz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FrequencyRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.FrequencyRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FrequencyRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum_frequency_hz__ = None;
                let mut maximum_frequency_hz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinimumFrequencyHz => {
                            if minimum_frequency_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumFrequencyHz"));
                            }
                            minimum_frequency_hz__ = map_.next_value()?;
                        }
                        GeneratedField::MaximumFrequencyHz => {
                            if maximum_frequency_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximumFrequencyHz"));
                            }
                            maximum_frequency_hz__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FrequencyRange {
                    minimum_frequency_hz: minimum_frequency_hz__,
                    maximum_frequency_hz: maximum_frequency_hz__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.FrequencyRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fuel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fuel_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.reported_date.is_some() {
            len += 1;
        }
        if self.amount_gallons != 0 {
            len += 1;
        }
        if self.max_authorized_capacity_gallons != 0 {
            len += 1;
        }
        if self.operational_requirement_gallons != 0 {
            len += 1;
        }
        if self.data_classification.is_some() {
            len += 1;
        }
        if !self.data_source.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Fuel", len)?;
        if !self.fuel_id.is_empty() {
            struct_ser.serialize_field("fuelId", &self.fuel_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.reported_date.as_ref() {
            struct_ser.serialize_field("reportedDate", v)?;
        }
        if self.amount_gallons != 0 {
            struct_ser.serialize_field("amountGallons", &self.amount_gallons)?;
        }
        if self.max_authorized_capacity_gallons != 0 {
            struct_ser.serialize_field("maxAuthorizedCapacityGallons", &self.max_authorized_capacity_gallons)?;
        }
        if self.operational_requirement_gallons != 0 {
            struct_ser.serialize_field("operationalRequirementGallons", &self.operational_requirement_gallons)?;
        }
        if let Some(v) = self.data_classification.as_ref() {
            struct_ser.serialize_field("dataClassification", v)?;
        }
        if !self.data_source.is_empty() {
            struct_ser.serialize_field("dataSource", &self.data_source)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fuel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fuel_id",
            "fuelId",
            "name",
            "reported_date",
            "reportedDate",
            "amount_gallons",
            "amountGallons",
            "max_authorized_capacity_gallons",
            "maxAuthorizedCapacityGallons",
            "operational_requirement_gallons",
            "operationalRequirementGallons",
            "data_classification",
            "dataClassification",
            "data_source",
            "dataSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FuelId,
            Name,
            ReportedDate,
            AmountGallons,
            MaxAuthorizedCapacityGallons,
            OperationalRequirementGallons,
            DataClassification,
            DataSource,
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
                            "fuelId" | "fuel_id" => Ok(GeneratedField::FuelId),
                            "name" => Ok(GeneratedField::Name),
                            "reportedDate" | "reported_date" => Ok(GeneratedField::ReportedDate),
                            "amountGallons" | "amount_gallons" => Ok(GeneratedField::AmountGallons),
                            "maxAuthorizedCapacityGallons" | "max_authorized_capacity_gallons" => Ok(GeneratedField::MaxAuthorizedCapacityGallons),
                            "operationalRequirementGallons" | "operational_requirement_gallons" => Ok(GeneratedField::OperationalRequirementGallons),
                            "dataClassification" | "data_classification" => Ok(GeneratedField::DataClassification),
                            "dataSource" | "data_source" => Ok(GeneratedField::DataSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fuel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Fuel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fuel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fuel_id__ = None;
                let mut name__ = None;
                let mut reported_date__ = None;
                let mut amount_gallons__ = None;
                let mut max_authorized_capacity_gallons__ = None;
                let mut operational_requirement_gallons__ = None;
                let mut data_classification__ = None;
                let mut data_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FuelId => {
                            if fuel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fuelId"));
                            }
                            fuel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportedDate => {
                            if reported_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportedDate"));
                            }
                            reported_date__ = map_.next_value()?;
                        }
                        GeneratedField::AmountGallons => {
                            if amount_gallons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountGallons"));
                            }
                            amount_gallons__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxAuthorizedCapacityGallons => {
                            if max_authorized_capacity_gallons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAuthorizedCapacityGallons"));
                            }
                            max_authorized_capacity_gallons__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OperationalRequirementGallons => {
                            if operational_requirement_gallons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationalRequirementGallons"));
                            }
                            operational_requirement_gallons__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DataClassification => {
                            if data_classification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataClassification"));
                            }
                            data_classification__ = map_.next_value()?;
                        }
                        GeneratedField::DataSource => {
                            if data_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataSource"));
                            }
                            data_source__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Fuel {
                    fuel_id: fuel_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    reported_date: reported_date__,
                    amount_gallons: amount_gallons__.unwrap_or_default(),
                    max_authorized_capacity_gallons: max_authorized_capacity_gallons__.unwrap_or_default(),
                    operational_requirement_gallons: operational_requirement_gallons__.unwrap_or_default(),
                    data_classification: data_classification__,
                    data_source: data_source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Fuel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoDetails", len)?;
        if self.r#type != 0 {
            let v = GeoType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
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
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<GeoType>()? as i32);
                        }
                    }
                }
                Ok(GeoDetails {
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoEllipse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.semi_major_axis_m.is_some() {
            len += 1;
        }
        if self.semi_minor_axis_m.is_some() {
            len += 1;
        }
        if self.orientation_d.is_some() {
            len += 1;
        }
        if self.height_m.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoEllipse", len)?;
        if let Some(v) = self.semi_major_axis_m.as_ref() {
            struct_ser.serialize_field("semiMajorAxisM", v)?;
        }
        if let Some(v) = self.semi_minor_axis_m.as_ref() {
            struct_ser.serialize_field("semiMinorAxisM", v)?;
        }
        if let Some(v) = self.orientation_d.as_ref() {
            struct_ser.serialize_field("orientationD", v)?;
        }
        if let Some(v) = self.height_m.as_ref() {
            struct_ser.serialize_field("heightM", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoEllipse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "semi_major_axis_m",
            "semiMajorAxisM",
            "semi_minor_axis_m",
            "semiMinorAxisM",
            "orientation_d",
            "orientationD",
            "height_m",
            "heightM",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SemiMajorAxisM,
            SemiMinorAxisM,
            OrientationD,
            HeightM,
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
                            "semiMajorAxisM" | "semi_major_axis_m" => Ok(GeneratedField::SemiMajorAxisM),
                            "semiMinorAxisM" | "semi_minor_axis_m" => Ok(GeneratedField::SemiMinorAxisM),
                            "orientationD" | "orientation_d" => Ok(GeneratedField::OrientationD),
                            "heightM" | "height_m" => Ok(GeneratedField::HeightM),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoEllipse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoEllipse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoEllipse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut semi_major_axis_m__ = None;
                let mut semi_minor_axis_m__ = None;
                let mut orientation_d__ = None;
                let mut height_m__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SemiMajorAxisM => {
                            if semi_major_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semiMajorAxisM"));
                            }
                            semi_major_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::SemiMinorAxisM => {
                            if semi_minor_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semiMinorAxisM"));
                            }
                            semi_minor_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::OrientationD => {
                            if orientation_d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orientationD"));
                            }
                            orientation_d__ = map_.next_value()?;
                        }
                        GeneratedField::HeightM => {
                            if height_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heightM"));
                            }
                            height_m__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GeoEllipse {
                    semi_major_axis_m: semi_major_axis_m__,
                    semi_minor_axis_m: semi_minor_axis_m__,
                    orientation_d: orientation_d__,
                    height_m: height_m__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoEllipse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoEllipsoid {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.forward_axis_m.is_some() {
            len += 1;
        }
        if self.side_axis_m.is_some() {
            len += 1;
        }
        if self.up_axis_m.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoEllipsoid", len)?;
        if let Some(v) = self.forward_axis_m.as_ref() {
            struct_ser.serialize_field("forwardAxisM", v)?;
        }
        if let Some(v) = self.side_axis_m.as_ref() {
            struct_ser.serialize_field("sideAxisM", v)?;
        }
        if let Some(v) = self.up_axis_m.as_ref() {
            struct_ser.serialize_field("upAxisM", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoEllipsoid {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "forward_axis_m",
            "forwardAxisM",
            "side_axis_m",
            "sideAxisM",
            "up_axis_m",
            "upAxisM",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ForwardAxisM,
            SideAxisM,
            UpAxisM,
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
                            "forwardAxisM" | "forward_axis_m" => Ok(GeneratedField::ForwardAxisM),
                            "sideAxisM" | "side_axis_m" => Ok(GeneratedField::SideAxisM),
                            "upAxisM" | "up_axis_m" => Ok(GeneratedField::UpAxisM),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoEllipsoid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoEllipsoid")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoEllipsoid, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut forward_axis_m__ = None;
                let mut side_axis_m__ = None;
                let mut up_axis_m__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ForwardAxisM => {
                            if forward_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardAxisM"));
                            }
                            forward_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::SideAxisM => {
                            if side_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sideAxisM"));
                            }
                            side_axis_m__ = map_.next_value()?;
                        }
                        GeneratedField::UpAxisM => {
                            if up_axis_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upAxisM"));
                            }
                            up_axis_m__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GeoEllipsoid {
                    forward_axis_m: forward_axis_m__,
                    side_axis_m: side_axis_m__,
                    up_axis_m: up_axis_m__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoEllipsoid", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoLine {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.positions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoLine", len)?;
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoLine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "positions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Positions,
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
                            "positions" => Ok(GeneratedField::Positions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoLine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoLine")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoLine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut positions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GeoLine {
                    positions: positions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoLine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.position.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoPoint", len)?;
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "position",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Position,
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
                            "position" => Ok(GeneratedField::Position),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut position__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GeoPoint {
                    position: position__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoPolygon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rings.is_empty() {
            len += 1;
        }
        if self.is_rectangle {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoPolygon", len)?;
        if !self.rings.is_empty() {
            struct_ser.serialize_field("rings", &self.rings)?;
        }
        if self.is_rectangle {
            struct_ser.serialize_field("isRectangle", &self.is_rectangle)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoPolygon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rings",
            "is_rectangle",
            "isRectangle",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rings,
            IsRectangle,
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
                            "rings" => Ok(GeneratedField::Rings),
                            "isRectangle" | "is_rectangle" => Ok(GeneratedField::IsRectangle),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoPolygon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoPolygon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoPolygon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rings__ = None;
                let mut is_rectangle__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rings => {
                            if rings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rings"));
                            }
                            rings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsRectangle => {
                            if is_rectangle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isRectangle"));
                            }
                            is_rectangle__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GeoPolygon {
                    rings: rings__.unwrap_or_default(),
                    is_rectangle: is_rectangle__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoPolygon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoPolygonPosition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.position.is_some() {
            len += 1;
        }
        if self.height_m.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoPolygonPosition", len)?;
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if let Some(v) = self.height_m.as_ref() {
            struct_ser.serialize_field("heightM", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoPolygonPosition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "position",
            "height_m",
            "heightM",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Position,
            HeightM,
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
                            "position" => Ok(GeneratedField::Position),
                            "heightM" | "height_m" => Ok(GeneratedField::HeightM),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoPolygonPosition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoPolygonPosition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoPolygonPosition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut position__ = None;
                let mut height_m__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                        GeneratedField::HeightM => {
                            if height_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heightM"));
                            }
                            height_m__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GeoPolygonPosition {
                    position: position__,
                    height_m: height_m__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoPolygonPosition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoShape {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shape.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GeoShape", len)?;
        if let Some(v) = self.shape.as_ref() {
            match v {
                geo_shape::Shape::Point(v) => {
                    struct_ser.serialize_field("point", v)?;
                }
                geo_shape::Shape::Line(v) => {
                    struct_ser.serialize_field("line", v)?;
                }
                geo_shape::Shape::Polygon(v) => {
                    struct_ser.serialize_field("polygon", v)?;
                }
                geo_shape::Shape::Ellipse(v) => {
                    struct_ser.serialize_field("ellipse", v)?;
                }
                geo_shape::Shape::Ellipsoid(v) => {
                    struct_ser.serialize_field("ellipsoid", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeoShape {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "point",
            "line",
            "polygon",
            "ellipse",
            "ellipsoid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Point,
            Line,
            Polygon,
            Ellipse,
            Ellipsoid,
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
                            "point" => Ok(GeneratedField::Point),
                            "line" => Ok(GeneratedField::Line),
                            "polygon" => Ok(GeneratedField::Polygon),
                            "ellipse" => Ok(GeneratedField::Ellipse),
                            "ellipsoid" => Ok(GeneratedField::Ellipsoid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoShape;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GeoShape")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GeoShape, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shape__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Point => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("point"));
                            }
                            shape__ = map_.next_value::<::std::option::Option<_>>()?.map(geo_shape::Shape::Point)
;
                        }
                        GeneratedField::Line => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            shape__ = map_.next_value::<::std::option::Option<_>>()?.map(geo_shape::Shape::Line)
;
                        }
                        GeneratedField::Polygon => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("polygon"));
                            }
                            shape__ = map_.next_value::<::std::option::Option<_>>()?.map(geo_shape::Shape::Polygon)
;
                        }
                        GeneratedField::Ellipse => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ellipse"));
                            }
                            shape__ = map_.next_value::<::std::option::Option<_>>()?.map(geo_shape::Shape::Ellipse)
;
                        }
                        GeneratedField::Ellipsoid => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ellipsoid"));
                            }
                            shape__ = map_.next_value::<::std::option::Option<_>>()?.map(geo_shape::Shape::Ellipsoid)
;
                        }
                    }
                }
                Ok(GeoShape {
                    shape: shape__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GeoShape", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GeoType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "GEO_TYPE_INVALID",
            Self::General => "GEO_TYPE_GENERAL",
            Self::Hazard => "GEO_TYPE_HAZARD",
            Self::Emergency => "GEO_TYPE_EMERGENCY",
            Self::EngagementZone => "GEO_TYPE_ENGAGEMENT_ZONE",
            Self::ControlArea => "GEO_TYPE_CONTROL_AREA",
            Self::Bullseye => "GEO_TYPE_BULLSEYE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for GeoType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "GEO_TYPE_INVALID",
            "GEO_TYPE_GENERAL",
            "GEO_TYPE_HAZARD",
            "GEO_TYPE_EMERGENCY",
            "GEO_TYPE_ENGAGEMENT_ZONE",
            "GEO_TYPE_CONTROL_AREA",
            "GEO_TYPE_BULLSEYE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeoType;

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
                    "GEO_TYPE_INVALID" => Ok(GeoType::Invalid),
                    "GEO_TYPE_GENERAL" => Ok(GeoType::General),
                    "GEO_TYPE_HAZARD" => Ok(GeoType::Hazard),
                    "GEO_TYPE_EMERGENCY" => Ok(GeoType::Emergency),
                    "GEO_TYPE_ENGAGEMENT_ZONE" => Ok(GeoType::EngagementZone),
                    "GEO_TYPE_CONTROL_AREA" => Ok(GeoType::ControlArea),
                    "GEO_TYPE_BULLSEYE" => Ok(GeoType::Bullseye),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntityRequest {
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
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GetEntityRequest", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetEntityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GetEntityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetEntityRequest {
                    entity_id: entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GetEntityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GetEntityResponse", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
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
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEntityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GetEntityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetEntityResponse {
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GetEntityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupChild {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GroupChild", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupChild {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupChild;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GroupChild")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GroupChild, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GroupChild {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GroupChild", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GroupDetails", len)?;
        if let Some(v) = self.group_type.as_ref() {
            match v {
                group_details::GroupType::Echelon(v) => {
                    struct_ser.serialize_field("echelon", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "echelon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Echelon,
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
                            "echelon" => Ok(GeneratedField::Echelon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GroupDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GroupDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Echelon => {
                            if group_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("echelon"));
                            }
                            group_type__ = map_.next_value::<::std::option::Option<_>>()?.map(group_details::GroupType::Echelon)
;
                        }
                    }
                }
                Ok(GroupDetails {
                    group_type: group_type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GroupDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupParent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.GroupParent", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupParent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupParent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.GroupParent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GroupParent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GroupParent {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.GroupParent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeadingType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.HeadingType", len)?;
        if self.value != 0 {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeadingType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeadingType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.HeadingType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeadingType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HeadingType {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.HeadingType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Health {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.connection_status != 0 {
            len += 1;
        }
        if self.health_status != 0 {
            len += 1;
        }
        if !self.components.is_empty() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        if !self.active_alerts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Health", len)?;
        if self.connection_status != 0 {
            let v = ConnectionStatus::try_from(self.connection_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.connection_status)))?;
            struct_ser.serialize_field("connectionStatus", &v)?;
        }
        if self.health_status != 0 {
            let v = HealthStatus::try_from(self.health_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("healthStatus", &v)?;
        }
        if !self.components.is_empty() {
            struct_ser.serialize_field("components", &self.components)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        if !self.active_alerts.is_empty() {
            struct_ser.serialize_field("activeAlerts", &self.active_alerts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Health {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "connection_status",
            "connectionStatus",
            "health_status",
            "healthStatus",
            "components",
            "update_time",
            "updateTime",
            "active_alerts",
            "activeAlerts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConnectionStatus,
            HealthStatus,
            Components,
            UpdateTime,
            ActiveAlerts,
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
                            "connectionStatus" | "connection_status" => Ok(GeneratedField::ConnectionStatus),
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            "components" => Ok(GeneratedField::Components),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            "activeAlerts" | "active_alerts" => Ok(GeneratedField::ActiveAlerts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Health;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Health")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Health, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut connection_status__ = None;
                let mut health_status__ = None;
                let mut components__ = None;
                let mut update_time__ = None;
                let mut active_alerts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConnectionStatus => {
                            if connection_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionStatus"));
                            }
                            connection_status__ = Some(map_.next_value::<ConnectionStatus>()? as i32);
                        }
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map_.next_value::<HealthStatus>()? as i32);
                        }
                        GeneratedField::Components => {
                            if components__.is_some() {
                                return Err(serde::de::Error::duplicate_field("components"));
                            }
                            components__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                        GeneratedField::ActiveAlerts => {
                            if active_alerts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeAlerts"));
                            }
                            active_alerts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Health {
                    connection_status: connection_status__.unwrap_or_default(),
                    health_status: health_status__.unwrap_or_default(),
                    components: components__.unwrap_or_default(),
                    update_time: update_time__,
                    active_alerts: active_alerts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Health", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "HEALTH_STATUS_INVALID",
            Self::Healthy => "HEALTH_STATUS_HEALTHY",
            Self::Warn => "HEALTH_STATUS_WARN",
            Self::Fail => "HEALTH_STATUS_FAIL",
            Self::Offline => "HEALTH_STATUS_OFFLINE",
            Self::NotReady => "HEALTH_STATUS_NOT_READY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HEALTH_STATUS_INVALID",
            "HEALTH_STATUS_HEALTHY",
            "HEALTH_STATUS_WARN",
            "HEALTH_STATUS_FAIL",
            "HEALTH_STATUS_OFFLINE",
            "HEALTH_STATUS_NOT_READY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthStatus;

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
                    "HEALTH_STATUS_INVALID" => Ok(HealthStatus::Invalid),
                    "HEALTH_STATUS_HEALTHY" => Ok(HealthStatus::Healthy),
                    "HEALTH_STATUS_WARN" => Ok(HealthStatus::Warn),
                    "HEALTH_STATUS_FAIL" => Ok(HealthStatus::Fail),
                    "HEALTH_STATUS_OFFLINE" => Ok(HealthStatus::Offline),
                    "HEALTH_STATUS_NOT_READY" => Ok(HealthStatus::NotReady),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Heartbeat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Heartbeat", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Heartbeat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = Heartbeat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Heartbeat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Heartbeat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Heartbeat {
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Heartbeat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HighValueTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_high_value_target {
            len += 1;
        }
        if self.target_priority != 0 {
            len += 1;
        }
        if !self.target_matches.is_empty() {
            len += 1;
        }
        if self.is_high_payoff_target {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.HighValueTarget", len)?;
        if self.is_high_value_target {
            struct_ser.serialize_field("isHighValueTarget", &self.is_high_value_target)?;
        }
        if self.target_priority != 0 {
            struct_ser.serialize_field("targetPriority", &self.target_priority)?;
        }
        if !self.target_matches.is_empty() {
            struct_ser.serialize_field("targetMatches", &self.target_matches)?;
        }
        if self.is_high_payoff_target {
            struct_ser.serialize_field("isHighPayoffTarget", &self.is_high_payoff_target)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HighValueTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_high_value_target",
            "isHighValueTarget",
            "target_priority",
            "targetPriority",
            "target_matches",
            "targetMatches",
            "is_high_payoff_target",
            "isHighPayoffTarget",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsHighValueTarget,
            TargetPriority,
            TargetMatches,
            IsHighPayoffTarget,
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
                            "isHighValueTarget" | "is_high_value_target" => Ok(GeneratedField::IsHighValueTarget),
                            "targetPriority" | "target_priority" => Ok(GeneratedField::TargetPriority),
                            "targetMatches" | "target_matches" => Ok(GeneratedField::TargetMatches),
                            "isHighPayoffTarget" | "is_high_payoff_target" => Ok(GeneratedField::IsHighPayoffTarget),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HighValueTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.HighValueTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HighValueTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_high_value_target__ = None;
                let mut target_priority__ = None;
                let mut target_matches__ = None;
                let mut is_high_payoff_target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsHighValueTarget => {
                            if is_high_value_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isHighValueTarget"));
                            }
                            is_high_value_target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetPriority => {
                            if target_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPriority"));
                            }
                            target_priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TargetMatches => {
                            if target_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetMatches"));
                            }
                            target_matches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsHighPayoffTarget => {
                            if is_high_payoff_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isHighPayoffTarget"));
                            }
                            is_high_payoff_target__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HighValueTarget {
                    is_high_value_target: is_high_value_target__.unwrap_or_default(),
                    target_priority: target_priority__.unwrap_or_default(),
                    target_matches: target_matches__.unwrap_or_default(),
                    is_high_payoff_target: is_high_payoff_target__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.HighValueTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HighValueTargetMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.high_value_target_list_id.is_empty() {
            len += 1;
        }
        if !self.high_value_target_description_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.HighValueTargetMatch", len)?;
        if !self.high_value_target_list_id.is_empty() {
            struct_ser.serialize_field("highValueTargetListId", &self.high_value_target_list_id)?;
        }
        if !self.high_value_target_description_id.is_empty() {
            struct_ser.serialize_field("highValueTargetDescriptionId", &self.high_value_target_description_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HighValueTargetMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "high_value_target_list_id",
            "highValueTargetListId",
            "high_value_target_description_id",
            "highValueTargetDescriptionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HighValueTargetListId,
            HighValueTargetDescriptionId,
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
                            "highValueTargetListId" | "high_value_target_list_id" => Ok(GeneratedField::HighValueTargetListId),
                            "highValueTargetDescriptionId" | "high_value_target_description_id" => Ok(GeneratedField::HighValueTargetDescriptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HighValueTargetMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.HighValueTargetMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HighValueTargetMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut high_value_target_list_id__ = None;
                let mut high_value_target_description_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HighValueTargetListId => {
                            if high_value_target_list_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("highValueTargetListId"));
                            }
                            high_value_target_list_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HighValueTargetDescriptionId => {
                            if high_value_target_description_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("highValueTargetDescriptionId"));
                            }
                            high_value_target_description_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HighValueTargetMatch {
                    high_value_target_list_id: high_value_target_list_id__.unwrap_or_default(),
                    high_value_target_description_id: high_value_target_description_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.HighValueTargetMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Indicators {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.simulated.is_some() {
            len += 1;
        }
        if self.exercise.is_some() {
            len += 1;
        }
        if self.emergency.is_some() {
            len += 1;
        }
        if self.c2.is_some() {
            len += 1;
        }
        if self.egressable.is_some() {
            len += 1;
        }
        if self.starred.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Indicators", len)?;
        if let Some(v) = self.simulated.as_ref() {
            struct_ser.serialize_field("simulated", v)?;
        }
        if let Some(v) = self.exercise.as_ref() {
            struct_ser.serialize_field("exercise", v)?;
        }
        if let Some(v) = self.emergency.as_ref() {
            struct_ser.serialize_field("emergency", v)?;
        }
        if let Some(v) = self.c2.as_ref() {
            struct_ser.serialize_field("c2", v)?;
        }
        if let Some(v) = self.egressable.as_ref() {
            struct_ser.serialize_field("egressable", v)?;
        }
        if let Some(v) = self.starred.as_ref() {
            struct_ser.serialize_field("starred", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Indicators {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "simulated",
            "exercise",
            "emergency",
            "c2",
            "egressable",
            "starred",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Simulated,
            Exercise,
            Emergency,
            C2,
            Egressable,
            Starred,
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
                            "simulated" => Ok(GeneratedField::Simulated),
                            "exercise" => Ok(GeneratedField::Exercise),
                            "emergency" => Ok(GeneratedField::Emergency),
                            "c2" => Ok(GeneratedField::C2),
                            "egressable" => Ok(GeneratedField::Egressable),
                            "starred" => Ok(GeneratedField::Starred),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Indicators;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Indicators")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Indicators, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut simulated__ = None;
                let mut exercise__ = None;
                let mut emergency__ = None;
                let mut c2__ = None;
                let mut egressable__ = None;
                let mut starred__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Simulated => {
                            if simulated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simulated"));
                            }
                            simulated__ = map_.next_value()?;
                        }
                        GeneratedField::Exercise => {
                            if exercise__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exercise"));
                            }
                            exercise__ = map_.next_value()?;
                        }
                        GeneratedField::Emergency => {
                            if emergency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emergency"));
                            }
                            emergency__ = map_.next_value()?;
                        }
                        GeneratedField::C2 => {
                            if c2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("c2"));
                            }
                            c2__ = map_.next_value()?;
                        }
                        GeneratedField::Egressable => {
                            if egressable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("egressable"));
                            }
                            egressable__ = map_.next_value()?;
                        }
                        GeneratedField::Starred => {
                            if starred__.is_some() {
                                return Err(serde::de::Error::duplicate_field("starred"));
                            }
                            starred__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Indicators {
                    simulated: simulated__,
                    exercise: exercise__,
                    emergency: emergency__,
                    c2: c2__,
                    egressable: egressable__,
                    starred: starred__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Indicators", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterrogationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "INTERROGATION_RESPONSE_INVALID",
            Self::Correct => "INTERROGATION_RESPONSE_CORRECT",
            Self::Incorrect => "INTERROGATION_RESPONSE_INCORRECT",
            Self::NoResponse => "INTERROGATION_RESPONSE_NO_RESPONSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for InterrogationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INTERROGATION_RESPONSE_INVALID",
            "INTERROGATION_RESPONSE_CORRECT",
            "INTERROGATION_RESPONSE_INCORRECT",
            "INTERROGATION_RESPONSE_NO_RESPONSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterrogationResponse;

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
                    "INTERROGATION_RESPONSE_INVALID" => Ok(InterrogationResponse::Invalid),
                    "INTERROGATION_RESPONSE_CORRECT" => Ok(InterrogationResponse::Correct),
                    "INTERROGATION_RESPONSE_INCORRECT" => Ok(InterrogationResponse::Incorrect),
                    "INTERROGATION_RESPONSE_NO_RESPONSE" => Ok(InterrogationResponse::NoResponse),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IntersectionComparator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.comparison.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.IntersectionComparator", len)?;
        if let Some(v) = self.comparison.as_ref() {
            match v {
                intersection_comparator::Comparison::WithinComparison(v) => {
                    struct_ser.serialize_field("withinComparison", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntersectionComparator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "within_comparison",
            "withinComparison",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithinComparison,
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
                            "withinComparison" | "within_comparison" => Ok(GeneratedField::WithinComparison),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntersectionComparator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.IntersectionComparator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntersectionComparator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut comparison__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithinComparison => {
                            if comparison__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withinComparison"));
                            }
                            comparison__ = map_.next_value::<::std::option::Option<_>>()?.map(intersection_comparator::Comparison::WithinComparison)
;
                        }
                    }
                }
                Ok(IntersectionComparator {
                    comparison: comparison__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.IntersectionComparator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LineOfBearing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.angle_of_arrival.is_some() {
            len += 1;
        }
        if self.detection_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.LineOfBearing", len)?;
        if let Some(v) = self.angle_of_arrival.as_ref() {
            struct_ser.serialize_field("angleOfArrival", v)?;
        }
        if let Some(v) = self.detection_range.as_ref() {
            match v {
                line_of_bearing::DetectionRange::RangeEstimateM(v) => {
                    struct_ser.serialize_field("rangeEstimateM", v)?;
                }
                line_of_bearing::DetectionRange::MaxRangeM(v) => {
                    struct_ser.serialize_field("maxRangeM", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineOfBearing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "angle_of_arrival",
            "angleOfArrival",
            "range_estimate_m",
            "rangeEstimateM",
            "max_range_m",
            "maxRangeM",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AngleOfArrival,
            RangeEstimateM,
            MaxRangeM,
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
                            "angleOfArrival" | "angle_of_arrival" => Ok(GeneratedField::AngleOfArrival),
                            "rangeEstimateM" | "range_estimate_m" => Ok(GeneratedField::RangeEstimateM),
                            "maxRangeM" | "max_range_m" => Ok(GeneratedField::MaxRangeM),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LineOfBearing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.LineOfBearing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LineOfBearing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut angle_of_arrival__ = None;
                let mut detection_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AngleOfArrival => {
                            if angle_of_arrival__.is_some() {
                                return Err(serde::de::Error::duplicate_field("angleOfArrival"));
                            }
                            angle_of_arrival__ = map_.next_value()?;
                        }
                        GeneratedField::RangeEstimateM => {
                            if detection_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rangeEstimateM"));
                            }
                            detection_range__ = map_.next_value::<::std::option::Option<_>>()?.map(line_of_bearing::DetectionRange::RangeEstimateM)
;
                        }
                        GeneratedField::MaxRangeM => {
                            if detection_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRangeM"));
                            }
                            detection_range__ = map_.next_value::<::std::option::Option<_>>()?.map(line_of_bearing::DetectionRange::MaxRangeM)
;
                        }
                    }
                }
                Ok(LineOfBearing {
                    angle_of_arrival: angle_of_arrival__,
                    detection_range: detection_range__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.LineOfBearing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinearRing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.positions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.LinearRing", len)?;
        if !self.positions.is_empty() {
            struct_ser.serialize_field("positions", &self.positions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinearRing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "positions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Positions,
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
                            "positions" => Ok(GeneratedField::Positions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinearRing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.LinearRing")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinearRing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut positions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Positions => {
                            if positions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positions"));
                            }
                            positions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LinearRing {
                    positions: positions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.LinearRing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListComparator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "LIST_COMPARATOR_INVALID",
            Self::AnyOf => "LIST_COMPARATOR_ANY_OF",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ListComparator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LIST_COMPARATOR_INVALID",
            "LIST_COMPARATOR_ANY_OF",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListComparator;

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
                    "LIST_COMPARATOR_INVALID" => Ok(ListComparator::Invalid),
                    "LIST_COMPARATOR_ANY_OF" => Ok(ListComparator::AnyOf),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.list_path.is_empty() {
            len += 1;
        }
        if self.list_comparator != 0 {
            len += 1;
        }
        if self.statement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ListOperation", len)?;
        if !self.list_path.is_empty() {
            struct_ser.serialize_field("listPath", &self.list_path)?;
        }
        if self.list_comparator != 0 {
            let v = ListComparator::try_from(self.list_comparator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.list_comparator)))?;
            struct_ser.serialize_field("listComparator", &v)?;
        }
        if let Some(v) = self.statement.as_ref() {
            struct_ser.serialize_field("statement", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "list_path",
            "listPath",
            "list_comparator",
            "listComparator",
            "statement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ListPath,
            ListComparator,
            Statement,
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
                            "listPath" | "list_path" => Ok(GeneratedField::ListPath),
                            "listComparator" | "list_comparator" => Ok(GeneratedField::ListComparator),
                            "statement" => Ok(GeneratedField::Statement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ListOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut list_path__ = None;
                let mut list_comparator__ = None;
                let mut statement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ListPath => {
                            if list_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listPath"));
                            }
                            list_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ListComparator => {
                            if list_comparator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listComparator"));
                            }
                            list_comparator__ = Some(map_.next_value::<ListComparator>()? as i32);
                        }
                        GeneratedField::Statement => {
                            if statement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statement"));
                            }
                            statement__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOperation {
                    list_path: list_path__.unwrap_or_default(),
                    list_comparator: list_comparator__.unwrap_or_default(),
                    statement: statement__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ListOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ListType", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ListType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListType {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ListType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Location {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.position.is_some() {
            len += 1;
        }
        if self.velocity_enu.is_some() {
            len += 1;
        }
        if self.speed_mps.is_some() {
            len += 1;
        }
        if self.acceleration.is_some() {
            len += 1;
        }
        if self.attitude_enu.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Location", len)?;
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if let Some(v) = self.velocity_enu.as_ref() {
            struct_ser.serialize_field("velocityEnu", v)?;
        }
        if let Some(v) = self.speed_mps.as_ref() {
            struct_ser.serialize_field("speedMps", v)?;
        }
        if let Some(v) = self.acceleration.as_ref() {
            struct_ser.serialize_field("acceleration", v)?;
        }
        if let Some(v) = self.attitude_enu.as_ref() {
            struct_ser.serialize_field("attitudeEnu", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Location {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "position",
            "velocity_enu",
            "velocityEnu",
            "speed_mps",
            "speedMps",
            "acceleration",
            "attitude_enu",
            "attitudeEnu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Position,
            VelocityEnu,
            SpeedMps,
            Acceleration,
            AttitudeEnu,
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
                            "position" => Ok(GeneratedField::Position),
                            "velocityEnu" | "velocity_enu" => Ok(GeneratedField::VelocityEnu),
                            "speedMps" | "speed_mps" => Ok(GeneratedField::SpeedMps),
                            "acceleration" => Ok(GeneratedField::Acceleration),
                            "attitudeEnu" | "attitude_enu" => Ok(GeneratedField::AttitudeEnu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Location")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Location, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut position__ = None;
                let mut velocity_enu__ = None;
                let mut speed_mps__ = None;
                let mut acceleration__ = None;
                let mut attitude_enu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                        GeneratedField::VelocityEnu => {
                            if velocity_enu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("velocityEnu"));
                            }
                            velocity_enu__ = map_.next_value()?;
                        }
                        GeneratedField::SpeedMps => {
                            if speed_mps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speedMps"));
                            }
                            speed_mps__ = map_.next_value()?;
                        }
                        GeneratedField::Acceleration => {
                            if acceleration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceleration"));
                            }
                            acceleration__ = map_.next_value()?;
                        }
                        GeneratedField::AttitudeEnu => {
                            if attitude_enu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attitudeEnu"));
                            }
                            attitude_enu__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Location {
                    position: position__,
                    velocity_enu: velocity_enu__,
                    speed_mps: speed_mps__,
                    acceleration: acceleration__,
                    attitude_enu: attitude_enu__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Location", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocationUncertainty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.position_enu_cov.is_some() {
            len += 1;
        }
        if self.velocity_enu_cov.is_some() {
            len += 1;
        }
        if self.position_error_ellipse.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.LocationUncertainty", len)?;
        if let Some(v) = self.position_enu_cov.as_ref() {
            struct_ser.serialize_field("positionEnuCov", v)?;
        }
        if let Some(v) = self.velocity_enu_cov.as_ref() {
            struct_ser.serialize_field("velocityEnuCov", v)?;
        }
        if let Some(v) = self.position_error_ellipse.as_ref() {
            struct_ser.serialize_field("positionErrorEllipse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocationUncertainty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "position_enu_cov",
            "positionEnuCov",
            "velocity_enu_cov",
            "velocityEnuCov",
            "position_error_ellipse",
            "positionErrorEllipse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PositionEnuCov,
            VelocityEnuCov,
            PositionErrorEllipse,
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
                            "positionEnuCov" | "position_enu_cov" => Ok(GeneratedField::PositionEnuCov),
                            "velocityEnuCov" | "velocity_enu_cov" => Ok(GeneratedField::VelocityEnuCov),
                            "positionErrorEllipse" | "position_error_ellipse" => Ok(GeneratedField::PositionErrorEllipse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocationUncertainty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.LocationUncertainty")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocationUncertainty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut position_enu_cov__ = None;
                let mut velocity_enu_cov__ = None;
                let mut position_error_ellipse__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PositionEnuCov => {
                            if position_enu_cov__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionEnuCov"));
                            }
                            position_enu_cov__ = map_.next_value()?;
                        }
                        GeneratedField::VelocityEnuCov => {
                            if velocity_enu_cov__.is_some() {
                                return Err(serde::de::Error::duplicate_field("velocityEnuCov"));
                            }
                            velocity_enu_cov__ = map_.next_value()?;
                        }
                        GeneratedField::PositionErrorEllipse => {
                            if position_error_ellipse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionErrorEllipse"));
                            }
                            position_error_ellipse__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LocationUncertainty {
                    position_enu_cov: position_enu_cov__,
                    velocity_enu_cov: velocity_enu_cov__,
                    position_error_ellipse: position_error_ellipse__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.LocationUncertainty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Measurement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        if self.sigma.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Measurement", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.sigma.as_ref() {
            struct_ser.serialize_field("sigma", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Measurement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "sigma",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Sigma,
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
                            "value" => Ok(GeneratedField::Value),
                            "sigma" => Ok(GeneratedField::Sigma),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Measurement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Measurement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Measurement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut sigma__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Sigma => {
                            if sigma__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigma"));
                            }
                            sigma__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Measurement {
                    value: value__,
                    sigma: sigma__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Measurement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Media {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Media", len)?;
        if !self.media.is_empty() {
            struct_ser.serialize_field("media", &self.media)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Media {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Media,
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
                            "media" => Ok(GeneratedField::Media),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Media;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Media")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Media, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Media => {
                            if media__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            media__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Media {
                    media: media__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Media", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.relative_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.MediaItem", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if self.r#type != 0 {
            let v = MediaType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.relative_path.is_empty() {
            struct_ser.serialize_field("relativePath", &self.relative_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MediaItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "type",
            "relative_path",
            "relativePath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            Type,
            RelativePath,
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
                            "url" => Ok(GeneratedField::Url),
                            "type" => Ok(GeneratedField::Type),
                            "relativePath" | "relative_path" => Ok(GeneratedField::RelativePath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.MediaItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut r#type__ = None;
                let mut relative_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<MediaType>()? as i32);
                        }
                        GeneratedField::RelativePath => {
                            if relative_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativePath"));
                            }
                            relative_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MediaItem {
                    url: url__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    relative_path: relative_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.MediaItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "MEDIA_TYPE_INVALID",
            Self::Thumbnail => "MEDIA_TYPE_THUMBNAIL",
            Self::Image => "MEDIA_TYPE_IMAGE",
            Self::Video => "MEDIA_TYPE_VIDEO",
            Self::SlippyTiles => "MEDIA_TYPE_SLIPPY_TILES",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MediaType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEDIA_TYPE_INVALID",
            "MEDIA_TYPE_THUMBNAIL",
            "MEDIA_TYPE_IMAGE",
            "MEDIA_TYPE_VIDEO",
            "MEDIA_TYPE_SLIPPY_TILES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaType;

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
                    "MEDIA_TYPE_INVALID" => Ok(MediaType::Invalid),
                    "MEDIA_TYPE_THUMBNAIL" => Ok(MediaType::Thumbnail),
                    "MEDIA_TYPE_IMAGE" => Ok(MediaType::Image),
                    "MEDIA_TYPE_VIDEO" => Ok(MediaType::Video),
                    "MEDIA_TYPE_SLIPPY_TILES" => Ok(MediaType::SlippyTiles),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MergedFrom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.MergedFrom", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MergedFrom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MergedFrom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.MergedFrom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MergedFrom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MergedFrom {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.MergedFrom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MilView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.disposition != 0 {
            len += 1;
        }
        if self.environment != 0 {
            len += 1;
        }
        if self.nationality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.MilView", len)?;
        if self.disposition != 0 {
            let v = super::super::ontology::v1::Disposition::try_from(self.disposition)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.disposition)))?;
            struct_ser.serialize_field("disposition", &v)?;
        }
        if self.environment != 0 {
            let v = super::super::ontology::v1::Environment::try_from(self.environment)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.environment)))?;
            struct_ser.serialize_field("environment", &v)?;
        }
        if self.nationality != 0 {
            let v = super::super::ontology::v1::Nationality::try_from(self.nationality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.nationality)))?;
            struct_ser.serialize_field("nationality", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MilView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disposition",
            "environment",
            "nationality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disposition,
            Environment,
            Nationality,
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
                            "disposition" => Ok(GeneratedField::Disposition),
                            "environment" => Ok(GeneratedField::Environment),
                            "nationality" => Ok(GeneratedField::Nationality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MilView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.MilView")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MilView, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disposition__ = None;
                let mut environment__ = None;
                let mut nationality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Disposition => {
                            if disposition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disposition"));
                            }
                            disposition__ = Some(map_.next_value::<super::super::ontology::v1::Disposition>()? as i32);
                        }
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(map_.next_value::<super::super::ontology::v1::Environment>()? as i32);
                        }
                        GeneratedField::Nationality => {
                            if nationality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nationality"));
                            }
                            nationality__ = Some(map_.next_value::<super::super::ontology::v1::Nationality>()? as i32);
                        }
                    }
                }
                Ok(MilView {
                    disposition: disposition__.unwrap_or_default(),
                    environment: environment__.unwrap_or_default(),
                    nationality: nationality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.MilView", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Mode5 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode5_interrogation_response != 0 {
            len += 1;
        }
        if self.mode5 != 0 {
            len += 1;
        }
        if self.mode5_platform_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Mode5", len)?;
        if self.mode5_interrogation_response != 0 {
            let v = InterrogationResponse::try_from(self.mode5_interrogation_response)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode5_interrogation_response)))?;
            struct_ser.serialize_field("mode5InterrogationResponse", &v)?;
        }
        if self.mode5 != 0 {
            struct_ser.serialize_field("mode5", &self.mode5)?;
        }
        if self.mode5_platform_id != 0 {
            struct_ser.serialize_field("mode5PlatformId", &self.mode5_platform_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Mode5 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mode5_interrogation_response",
            "mode5InterrogationResponse",
            "mode5",
            "mode5_platform_id",
            "mode5PlatformId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode5InterrogationResponse,
            Mode5,
            Mode5PlatformId,
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
                            "mode5InterrogationResponse" | "mode5_interrogation_response" => Ok(GeneratedField::Mode5InterrogationResponse),
                            "mode5" => Ok(GeneratedField::Mode5),
                            "mode5PlatformId" | "mode5_platform_id" => Ok(GeneratedField::Mode5PlatformId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Mode5;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Mode5")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Mode5, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mode5_interrogation_response__ = None;
                let mut mode5__ = None;
                let mut mode5_platform_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mode5InterrogationResponse => {
                            if mode5_interrogation_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode5InterrogationResponse"));
                            }
                            mode5_interrogation_response__ = Some(map_.next_value::<InterrogationResponse>()? as i32);
                        }
                        GeneratedField::Mode5 => {
                            if mode5__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode5"));
                            }
                            mode5__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mode5PlatformId => {
                            if mode5_platform_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode5PlatformId"));
                            }
                            mode5_platform_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Mode5 {
                    mode5_interrogation_response: mode5_interrogation_response__.unwrap_or_default(),
                    mode5: mode5__.unwrap_or_default(),
                    mode5_platform_id: mode5_platform_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Mode5", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModeS {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.address != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ModeS", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.address != 0 {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModeS {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Address,
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
                            "id" => Ok(GeneratedField::Id),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModeS;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ModeS")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModeS, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ModeS {
                    id: id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ModeS", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NonPrimaryMembership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.NonPrimaryMembership", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NonPrimaryMembership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NonPrimaryMembership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.NonPrimaryMembership")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NonPrimaryMembership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NonPrimaryMembership {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.NonPrimaryMembership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.child.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.NotOperation", len)?;
        if let Some(v) = self.child.as_ref() {
            match v {
                not_operation::Child::Predicate(v) => {
                    struct_ser.serialize_field("predicate", v)?;
                }
                not_operation::Child::Statement(v) => {
                    struct_ser.serialize_field("statement", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate",
            "statement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicate,
            Statement,
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
                            "predicate" => Ok(GeneratedField::Predicate),
                            "statement" => Ok(GeneratedField::Statement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.NotOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut child__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Predicate => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            child__ = map_.next_value::<::std::option::Option<_>>()?.map(not_operation::Child::Predicate)
;
                        }
                        GeneratedField::Statement => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statement"));
                            }
                            child__ = map_.next_value::<::std::option::Option<_>>()?.map(not_operation::Child::Statement)
;
                        }
                    }
                }
                Ok(NotOperation {
                    child: child__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.NotOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NumericType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.NumericType", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                numeric_type::Value::DoubleValue(v) => {
                    struct_ser.serialize_field("doubleValue", v)?;
                }
                numeric_type::Value::FloatValue(v) => {
                    struct_ser.serialize_field("floatValue", v)?;
                }
                numeric_type::Value::Int32Value(v) => {
                    struct_ser.serialize_field("int32Value", v)?;
                }
                numeric_type::Value::Int64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("int64Value", ToString::to_string(&v).as_str())?;
                }
                numeric_type::Value::Uint32Value(v) => {
                    struct_ser.serialize_field("uint32Value", v)?;
                }
                numeric_type::Value::Uint64Value(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("uint64Value", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NumericType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "double_value",
            "doubleValue",
            "float_value",
            "floatValue",
            "int32_value",
            "int32Value",
            "int64_value",
            "int64Value",
            "uint32_value",
            "uint32Value",
            "uint64_value",
            "uint64Value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DoubleValue,
            FloatValue,
            Int32Value,
            Int64Value,
            Uint32Value,
            Uint64Value,
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
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "floatValue" | "float_value" => Ok(GeneratedField::FloatValue),
                            "int32Value" | "int32_value" => Ok(GeneratedField::Int32Value),
                            "int64Value" | "int64_value" => Ok(GeneratedField::Int64Value),
                            "uint32Value" | "uint32_value" => Ok(GeneratedField::Uint32Value),
                            "uint64Value" | "uint64_value" => Ok(GeneratedField::Uint64Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NumericType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.NumericType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NumericType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DoubleValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::DoubleValue(x.0));
                        }
                        GeneratedField::FloatValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::FloatValue(x.0));
                        }
                        GeneratedField::Int32Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int32Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::Int32Value(x.0));
                        }
                        GeneratedField::Int64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::Int64Value(x.0));
                        }
                        GeneratedField::Uint32Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint32Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::Uint32Value(x.0));
                        }
                        GeneratedField::Uint64Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64Value"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| numeric_type::Value::Uint64Value(x.0));
                        }
                    }
                }
                Ok(NumericType {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.NumericType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ontology {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.platform_type.is_empty() {
            len += 1;
        }
        if !self.specific_type.is_empty() {
            len += 1;
        }
        if self.template != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Ontology", len)?;
        if !self.platform_type.is_empty() {
            struct_ser.serialize_field("platformType", &self.platform_type)?;
        }
        if !self.specific_type.is_empty() {
            struct_ser.serialize_field("specificType", &self.specific_type)?;
        }
        if self.template != 0 {
            let v = Template::try_from(self.template)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.template)))?;
            struct_ser.serialize_field("template", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ontology {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "platform_type",
            "platformType",
            "specific_type",
            "specificType",
            "template",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlatformType,
            SpecificType,
            Template,
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
                            "platformType" | "platform_type" => Ok(GeneratedField::PlatformType),
                            "specificType" | "specific_type" => Ok(GeneratedField::SpecificType),
                            "template" => Ok(GeneratedField::Template),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ontology;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Ontology")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ontology, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut platform_type__ = None;
                let mut specific_type__ = None;
                let mut template__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlatformType => {
                            if platform_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformType"));
                            }
                            platform_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpecificType => {
                            if specific_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specificType"));
                            }
                            specific_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Template => {
                            if template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            template__ = Some(map_.next_value::<Template>()? as i32);
                        }
                    }
                }
                Ok(Ontology {
                    platform_type: platform_type__.unwrap_or_default(),
                    specific_type: specific_type__.unwrap_or_default(),
                    template: template__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Ontology", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationalState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "OPERATIONAL_STATE_INVALID",
            Self::Off => "OPERATIONAL_STATE_OFF",
            Self::NonOperational => "OPERATIONAL_STATE_NON_OPERATIONAL",
            Self::Degraded => "OPERATIONAL_STATE_DEGRADED",
            Self::Operational => "OPERATIONAL_STATE_OPERATIONAL",
            Self::Denied => "OPERATIONAL_STATE_DENIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OperationalState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPERATIONAL_STATE_INVALID",
            "OPERATIONAL_STATE_OFF",
            "OPERATIONAL_STATE_NON_OPERATIONAL",
            "OPERATIONAL_STATE_DEGRADED",
            "OPERATIONAL_STATE_OPERATIONAL",
            "OPERATIONAL_STATE_DENIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationalState;

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
                    "OPERATIONAL_STATE_INVALID" => Ok(OperationalState::Invalid),
                    "OPERATIONAL_STATE_OFF" => Ok(OperationalState::Off),
                    "OPERATIONAL_STATE_NON_OPERATIONAL" => Ok(OperationalState::NonOperational),
                    "OPERATIONAL_STATE_DEGRADED" => Ok(OperationalState::Degraded),
                    "OPERATIONAL_STATE_OPERATIONAL" => Ok(OperationalState::Operational),
                    "OPERATIONAL_STATE_DENIED" => Ok(OperationalState::Denied),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OrOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.children.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.OrOperation", len)?;
        if let Some(v) = self.children.as_ref() {
            match v {
                or_operation::Children::PredicateSet(v) => {
                    struct_ser.serialize_field("predicateSet", v)?;
                }
                or_operation::Children::StatementSet(v) => {
                    struct_ser.serialize_field("statementSet", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate_set",
            "predicateSet",
            "statement_set",
            "statementSet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PredicateSet,
            StatementSet,
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
                            "predicateSet" | "predicate_set" => Ok(GeneratedField::PredicateSet),
                            "statementSet" | "statement_set" => Ok(GeneratedField::StatementSet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.OrOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut children__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PredicateSet => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicateSet"));
                            }
                            children__ = map_.next_value::<::std::option::Option<_>>()?.map(or_operation::Children::PredicateSet)
;
                        }
                        GeneratedField::StatementSet => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statementSet"));
                            }
                            children__ = map_.next_value::<::std::option::Option<_>>()?.map(or_operation::Children::StatementSet)
;
                        }
                    }
                }
                Ok(OrOperation {
                    children: children__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.OrOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Orbit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.orbit_mean_elements.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Orbit", len)?;
        if let Some(v) = self.orbit_mean_elements.as_ref() {
            struct_ser.serialize_field("orbitMeanElements", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Orbit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orbit_mean_elements",
            "orbitMeanElements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrbitMeanElements,
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
                            "orbitMeanElements" | "orbit_mean_elements" => Ok(GeneratedField::OrbitMeanElements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Orbit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Orbit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Orbit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orbit_mean_elements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrbitMeanElements => {
                            if orbit_mean_elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orbitMeanElements"));
                            }
                            orbit_mean_elements__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Orbit {
                    orbit_mean_elements: orbit_mean_elements__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Orbit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Override {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.masked_field_value.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.request_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Override", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if let Some(v) = self.masked_field_value.as_ref() {
            struct_ser.serialize_field("maskedFieldValue", v)?;
        }
        if self.status != 0 {
            let v = OverrideStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        if self.r#type != 0 {
            let v = OverrideType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.request_timestamp.as_ref() {
            struct_ser.serialize_field("requestTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Override {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "field_path",
            "fieldPath",
            "masked_field_value",
            "maskedFieldValue",
            "status",
            "provenance",
            "type",
            "request_timestamp",
            "requestTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            FieldPath,
            MaskedFieldValue,
            Status,
            Provenance,
            Type,
            RequestTimestamp,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "maskedFieldValue" | "masked_field_value" => Ok(GeneratedField::MaskedFieldValue),
                            "status" => Ok(GeneratedField::Status),
                            "provenance" => Ok(GeneratedField::Provenance),
                            "type" => Ok(GeneratedField::Type),
                            "requestTimestamp" | "request_timestamp" => Ok(GeneratedField::RequestTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Override;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Override")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Override, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut field_path__ = None;
                let mut masked_field_value__ = None;
                let mut status__ = None;
                let mut provenance__ = None;
                let mut r#type__ = None;
                let mut request_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaskedFieldValue => {
                            if masked_field_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maskedFieldValue"));
                            }
                            masked_field_value__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<OverrideStatus>()? as i32);
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<OverrideType>()? as i32);
                        }
                        GeneratedField::RequestTimestamp => {
                            if request_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTimestamp"));
                            }
                            request_timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Override {
                    request_id: request_id__.unwrap_or_default(),
                    field_path: field_path__.unwrap_or_default(),
                    masked_field_value: masked_field_value__,
                    status: status__.unwrap_or_default(),
                    provenance: provenance__,
                    r#type: r#type__.unwrap_or_default(),
                    request_timestamp: request_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Override", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideEntityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.OverrideEntityRequest", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverrideEntityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
            "field_path",
            "fieldPath",
            "provenance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
            FieldPath,
            Provenance,
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
                            "entity" => Ok(GeneratedField::Entity),
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "provenance" => Ok(GeneratedField::Provenance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideEntityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.OverrideEntityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverrideEntityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                let mut field_path__ = None;
                let mut provenance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OverrideEntityRequest {
                    entity: entity__,
                    field_path: field_path__.unwrap_or_default(),
                    provenance: provenance__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.OverrideEntityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideEntityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.OverrideEntityResponse", len)?;
        if self.status != 0 {
            let v = OverrideStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverrideEntityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideEntityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.OverrideEntityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverrideEntityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<OverrideStatus>()? as i32);
                        }
                    }
                }
                Ok(OverrideEntityResponse {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.OverrideEntityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideNotificationPayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.OverrideNotificationPayload", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverrideNotificationPayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
            "field_path",
            "fieldPath",
            "provenance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
            FieldPath,
            Provenance,
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
                            "entity" => Ok(GeneratedField::Entity),
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "provenance" => Ok(GeneratedField::Provenance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideNotificationPayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.OverrideNotificationPayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverrideNotificationPayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                let mut field_path__ = None;
                let mut provenance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OverrideNotificationPayload {
                    entity: entity__,
                    field_path: field_path__.unwrap_or_default(),
                    provenance: provenance__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.OverrideNotificationPayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "OVERRIDE_STATUS_INVALID",
            Self::Applied => "OVERRIDE_STATUS_APPLIED",
            Self::Pending => "OVERRIDE_STATUS_PENDING",
            Self::Timeout => "OVERRIDE_STATUS_TIMEOUT",
            Self::Rejected => "OVERRIDE_STATUS_REJECTED",
            Self::DeletionPending => "OVERRIDE_STATUS_DELETION_PENDING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OverrideStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERRIDE_STATUS_INVALID",
            "OVERRIDE_STATUS_APPLIED",
            "OVERRIDE_STATUS_PENDING",
            "OVERRIDE_STATUS_TIMEOUT",
            "OVERRIDE_STATUS_REJECTED",
            "OVERRIDE_STATUS_DELETION_PENDING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideStatus;

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
                    "OVERRIDE_STATUS_INVALID" => Ok(OverrideStatus::Invalid),
                    "OVERRIDE_STATUS_APPLIED" => Ok(OverrideStatus::Applied),
                    "OVERRIDE_STATUS_PENDING" => Ok(OverrideStatus::Pending),
                    "OVERRIDE_STATUS_TIMEOUT" => Ok(OverrideStatus::Timeout),
                    "OVERRIDE_STATUS_REJECTED" => Ok(OverrideStatus::Rejected),
                    "OVERRIDE_STATUS_DELETION_PENDING" => Ok(OverrideStatus::DeletionPending),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OverrideType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "OVERRIDE_TYPE_INVALID",
            Self::Live => "OVERRIDE_TYPE_LIVE",
            Self::PostExpiry => "OVERRIDE_TYPE_POST_EXPIRY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OverrideType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERRIDE_TYPE_INVALID",
            "OVERRIDE_TYPE_LIVE",
            "OVERRIDE_TYPE_POST_EXPIRY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverrideType;

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
                    "OVERRIDE_TYPE_INVALID" => Ok(OverrideType::Invalid),
                    "OVERRIDE_TYPE_LIVE" => Ok(OverrideType::Live),
                    "OVERRIDE_TYPE_POST_EXPIRY" => Ok(OverrideType::PostExpiry),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Overrides {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#override.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Overrides", len)?;
        if !self.r#override.is_empty() {
            struct_ser.serialize_field("override", &self.r#override)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Overrides {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "override",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Override,
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
                            "override" => Ok(GeneratedField::Override),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Overrides;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Overrides")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Overrides, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Override => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            r#override__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Overrides {
                    r#override: r#override__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Overrides", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Payload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Payload", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
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
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Payload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Payload {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Payload", FIELDS, GeneratedVisitor)
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
        if !self.effective_environment.is_empty() {
            len += 1;
        }
        if self.payload_operational_state != 0 {
            len += 1;
        }
        if !self.payload_description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PayloadConfiguration", len)?;
        if !self.capability_id.is_empty() {
            struct_ser.serialize_field("capabilityId", &self.capability_id)?;
        }
        if self.quantity != 0 {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.effective_environment.is_empty() {
            let v = self.effective_environment.iter().cloned().map(|v| {
                super::super::ontology::v1::Environment::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("effectiveEnvironment", &v)?;
        }
        if self.payload_operational_state != 0 {
            let v = PayloadOperationalState::try_from(self.payload_operational_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.payload_operational_state)))?;
            struct_ser.serialize_field("payloadOperationalState", &v)?;
        }
        if !self.payload_description.is_empty() {
            struct_ser.serialize_field("payloadDescription", &self.payload_description)?;
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
            "effective_environment",
            "effectiveEnvironment",
            "payload_operational_state",
            "payloadOperationalState",
            "payload_description",
            "payloadDescription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CapabilityId,
            Quantity,
            EffectiveEnvironment,
            PayloadOperationalState,
            PayloadDescription,
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
                            "effectiveEnvironment" | "effective_environment" => Ok(GeneratedField::EffectiveEnvironment),
                            "payloadOperationalState" | "payload_operational_state" => Ok(GeneratedField::PayloadOperationalState),
                            "payloadDescription" | "payload_description" => Ok(GeneratedField::PayloadDescription),
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
                formatter.write_str("struct anduril.entitymanager.v1.PayloadConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayloadConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut capability_id__ = None;
                let mut quantity__ = None;
                let mut effective_environment__ = None;
                let mut payload_operational_state__ = None;
                let mut payload_description__ = None;
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
                        GeneratedField::EffectiveEnvironment => {
                            if effective_environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveEnvironment"));
                            }
                            effective_environment__ = Some(map_.next_value::<Vec<super::super::ontology::v1::Environment>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::PayloadOperationalState => {
                            if payload_operational_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadOperationalState"));
                            }
                            payload_operational_state__ = Some(map_.next_value::<PayloadOperationalState>()? as i32);
                        }
                        GeneratedField::PayloadDescription => {
                            if payload_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadDescription"));
                            }
                            payload_description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PayloadConfiguration {
                    capability_id: capability_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    effective_environment: effective_environment__.unwrap_or_default(),
                    payload_operational_state: payload_operational_state__.unwrap_or_default(),
                    payload_description: payload_description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PayloadConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayloadOperationalState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "PAYLOAD_OPERATIONAL_STATE_INVALID",
            Self::Off => "PAYLOAD_OPERATIONAL_STATE_OFF",
            Self::NonOperational => "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL",
            Self::Degraded => "PAYLOAD_OPERATIONAL_STATE_DEGRADED",
            Self::Operational => "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL",
            Self::OutOfService => "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE",
            Self::Unknown => "PAYLOAD_OPERATIONAL_STATE_UNKNOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PayloadOperationalState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAYLOAD_OPERATIONAL_STATE_INVALID",
            "PAYLOAD_OPERATIONAL_STATE_OFF",
            "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL",
            "PAYLOAD_OPERATIONAL_STATE_DEGRADED",
            "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL",
            "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE",
            "PAYLOAD_OPERATIONAL_STATE_UNKNOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayloadOperationalState;

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
                    "PAYLOAD_OPERATIONAL_STATE_INVALID" => Ok(PayloadOperationalState::Invalid),
                    "PAYLOAD_OPERATIONAL_STATE_OFF" => Ok(PayloadOperationalState::Off),
                    "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL" => Ok(PayloadOperationalState::NonOperational),
                    "PAYLOAD_OPERATIONAL_STATE_DEGRADED" => Ok(PayloadOperationalState::Degraded),
                    "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL" => Ok(PayloadOperationalState::Operational),
                    "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE" => Ok(PayloadOperationalState::OutOfService),
                    "PAYLOAD_OPERATIONAL_STATE_UNKNOWN" => Ok(PayloadOperationalState::Unknown),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Payloads {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.payload_configurations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Payloads", len)?;
        if !self.payload_configurations.is_empty() {
            struct_ser.serialize_field("payloadConfigurations", &self.payload_configurations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payloads {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "payload_configurations",
            "payloadConfigurations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PayloadConfigurations,
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
                            "payloadConfigurations" | "payload_configurations" => Ok(GeneratedField::PayloadConfigurations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payloads;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Payloads")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payloads, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload_configurations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PayloadConfigurations => {
                            if payload_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadConfigurations"));
                            }
                            payload_configurations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Payloads {
                    payload_configurations: payload_configurations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Payloads", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Pose {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pos.is_some() {
            len += 1;
        }
        if self.orientation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Pose", len)?;
        if let Some(v) = self.pos.as_ref() {
            struct_ser.serialize_field("pos", v)?;
        }
        if let Some(v) = self.orientation.as_ref() {
            struct_ser.serialize_field("orientation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pose {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pos",
            "orientation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pos,
            Orientation,
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
                            "pos" => Ok(GeneratedField::Pos),
                            "orientation" => Ok(GeneratedField::Orientation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Pose;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Pose")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Pose, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pos__ = None;
                let mut orientation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pos => {
                            if pos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pos"));
                            }
                            pos__ = map_.next_value()?;
                        }
                        GeneratedField::Orientation => {
                            if orientation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orientation"));
                            }
                            orientation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Pose {
                    pos: pos__,
                    orientation: orientation__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Pose", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Position {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.latitude_degrees != 0. {
            len += 1;
        }
        if self.longitude_degrees != 0. {
            len += 1;
        }
        if self.altitude_hae_meters.is_some() {
            len += 1;
        }
        if self.altitude_agl_meters.is_some() {
            len += 1;
        }
        if self.altitude_asf_meters.is_some() {
            len += 1;
        }
        if self.pressure_depth_meters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Position", len)?;
        if self.latitude_degrees != 0. {
            struct_ser.serialize_field("latitudeDegrees", &self.latitude_degrees)?;
        }
        if self.longitude_degrees != 0. {
            struct_ser.serialize_field("longitudeDegrees", &self.longitude_degrees)?;
        }
        if let Some(v) = self.altitude_hae_meters.as_ref() {
            struct_ser.serialize_field("altitudeHaeMeters", v)?;
        }
        if let Some(v) = self.altitude_agl_meters.as_ref() {
            struct_ser.serialize_field("altitudeAglMeters", v)?;
        }
        if let Some(v) = self.altitude_asf_meters.as_ref() {
            struct_ser.serialize_field("altitudeAsfMeters", v)?;
        }
        if let Some(v) = self.pressure_depth_meters.as_ref() {
            struct_ser.serialize_field("pressureDepthMeters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latitude_degrees",
            "latitudeDegrees",
            "longitude_degrees",
            "longitudeDegrees",
            "altitude_hae_meters",
            "altitudeHaeMeters",
            "altitude_agl_meters",
            "altitudeAglMeters",
            "altitude_asf_meters",
            "altitudeAsfMeters",
            "pressure_depth_meters",
            "pressureDepthMeters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LatitudeDegrees,
            LongitudeDegrees,
            AltitudeHaeMeters,
            AltitudeAglMeters,
            AltitudeAsfMeters,
            PressureDepthMeters,
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
                            "latitudeDegrees" | "latitude_degrees" => Ok(GeneratedField::LatitudeDegrees),
                            "longitudeDegrees" | "longitude_degrees" => Ok(GeneratedField::LongitudeDegrees),
                            "altitudeHaeMeters" | "altitude_hae_meters" => Ok(GeneratedField::AltitudeHaeMeters),
                            "altitudeAglMeters" | "altitude_agl_meters" => Ok(GeneratedField::AltitudeAglMeters),
                            "altitudeAsfMeters" | "altitude_asf_meters" => Ok(GeneratedField::AltitudeAsfMeters),
                            "pressureDepthMeters" | "pressure_depth_meters" => Ok(GeneratedField::PressureDepthMeters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut latitude_degrees__ = None;
                let mut longitude_degrees__ = None;
                let mut altitude_hae_meters__ = None;
                let mut altitude_agl_meters__ = None;
                let mut altitude_asf_meters__ = None;
                let mut pressure_depth_meters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LatitudeDegrees => {
                            if latitude_degrees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitudeDegrees"));
                            }
                            latitude_degrees__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LongitudeDegrees => {
                            if longitude_degrees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitudeDegrees"));
                            }
                            longitude_degrees__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AltitudeHaeMeters => {
                            if altitude_hae_meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitudeHaeMeters"));
                            }
                            altitude_hae_meters__ = map_.next_value()?;
                        }
                        GeneratedField::AltitudeAglMeters => {
                            if altitude_agl_meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitudeAglMeters"));
                            }
                            altitude_agl_meters__ = map_.next_value()?;
                        }
                        GeneratedField::AltitudeAsfMeters => {
                            if altitude_asf_meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitudeAsfMeters"));
                            }
                            altitude_asf_meters__ = map_.next_value()?;
                        }
                        GeneratedField::PressureDepthMeters => {
                            if pressure_depth_meters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pressureDepthMeters"));
                            }
                            pressure_depth_meters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Position {
                    latitude_degrees: latitude_degrees__.unwrap_or_default(),
                    longitude_degrees: longitude_degrees__.unwrap_or_default(),
                    altitude_hae_meters: altitude_hae_meters__,
                    altitude_agl_meters: altitude_agl_meters__,
                    altitude_asf_meters: altitude_asf_meters__,
                    pressure_depth_meters: pressure_depth_meters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PositionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PositionType", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PositionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PositionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PositionType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PositionType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PositionType {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PositionType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PowerLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.capacity != 0. {
            len += 1;
        }
        if self.remaining != 0. {
            len += 1;
        }
        if self.percent_remaining != 0. {
            len += 1;
        }
        if self.voltage.is_some() {
            len += 1;
        }
        if self.current_amps.is_some() {
            len += 1;
        }
        if self.run_time_to_empty_mins.is_some() {
            len += 1;
        }
        if self.consumption_rate_l_per_s.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PowerLevel", len)?;
        if self.capacity != 0. {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.remaining != 0. {
            struct_ser.serialize_field("remaining", &self.remaining)?;
        }
        if self.percent_remaining != 0. {
            struct_ser.serialize_field("percentRemaining", &self.percent_remaining)?;
        }
        if let Some(v) = self.voltage.as_ref() {
            struct_ser.serialize_field("voltage", v)?;
        }
        if let Some(v) = self.current_amps.as_ref() {
            struct_ser.serialize_field("currentAmps", v)?;
        }
        if let Some(v) = self.run_time_to_empty_mins.as_ref() {
            struct_ser.serialize_field("runTimeToEmptyMins", v)?;
        }
        if let Some(v) = self.consumption_rate_l_per_s.as_ref() {
            struct_ser.serialize_field("consumptionRateLPerS", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PowerLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "capacity",
            "remaining",
            "percent_remaining",
            "percentRemaining",
            "voltage",
            "current_amps",
            "currentAmps",
            "run_time_to_empty_mins",
            "runTimeToEmptyMins",
            "consumption_rate_l_per_s",
            "consumptionRateLPerS",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Capacity,
            Remaining,
            PercentRemaining,
            Voltage,
            CurrentAmps,
            RunTimeToEmptyMins,
            ConsumptionRateLPerS,
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
                            "capacity" => Ok(GeneratedField::Capacity),
                            "remaining" => Ok(GeneratedField::Remaining),
                            "percentRemaining" | "percent_remaining" => Ok(GeneratedField::PercentRemaining),
                            "voltage" => Ok(GeneratedField::Voltage),
                            "currentAmps" | "current_amps" => Ok(GeneratedField::CurrentAmps),
                            "runTimeToEmptyMins" | "run_time_to_empty_mins" => Ok(GeneratedField::RunTimeToEmptyMins),
                            "consumptionRateLPerS" | "consumption_rate_l_per_s" => Ok(GeneratedField::ConsumptionRateLPerS),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PowerLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PowerLevel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PowerLevel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut capacity__ = None;
                let mut remaining__ = None;
                let mut percent_remaining__ = None;
                let mut voltage__ = None;
                let mut current_amps__ = None;
                let mut run_time_to_empty_mins__ = None;
                let mut consumption_rate_l_per_s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Remaining => {
                            if remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remaining"));
                            }
                            remaining__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PercentRemaining => {
                            if percent_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentRemaining"));
                            }
                            percent_remaining__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Voltage => {
                            if voltage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voltage"));
                            }
                            voltage__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentAmps => {
                            if current_amps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentAmps"));
                            }
                            current_amps__ = map_.next_value()?;
                        }
                        GeneratedField::RunTimeToEmptyMins => {
                            if run_time_to_empty_mins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runTimeToEmptyMins"));
                            }
                            run_time_to_empty_mins__ = map_.next_value()?;
                        }
                        GeneratedField::ConsumptionRateLPerS => {
                            if consumption_rate_l_per_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumptionRateLPerS"));
                            }
                            consumption_rate_l_per_s__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PowerLevel {
                    capacity: capacity__.unwrap_or_default(),
                    remaining: remaining__.unwrap_or_default(),
                    percent_remaining: percent_remaining__.unwrap_or_default(),
                    voltage: voltage__,
                    current_amps: current_amps__,
                    run_time_to_empty_mins: run_time_to_empty_mins__,
                    consumption_rate_l_per_s: consumption_rate_l_per_s__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PowerLevel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PowerSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.power_status != 0 {
            len += 1;
        }
        if self.power_type != 0 {
            len += 1;
        }
        if self.power_level.is_some() {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        if self.offloadable.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PowerSource", len)?;
        if self.power_status != 0 {
            let v = PowerStatus::try_from(self.power_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.power_status)))?;
            struct_ser.serialize_field("powerStatus", &v)?;
        }
        if self.power_type != 0 {
            let v = PowerType::try_from(self.power_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.power_type)))?;
            struct_ser.serialize_field("powerType", &v)?;
        }
        if let Some(v) = self.power_level.as_ref() {
            struct_ser.serialize_field("powerLevel", v)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if let Some(v) = self.offloadable.as_ref() {
            struct_ser.serialize_field("offloadable", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PowerSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "power_status",
            "powerStatus",
            "power_type",
            "powerType",
            "power_level",
            "powerLevel",
            "messages",
            "offloadable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PowerStatus,
            PowerType,
            PowerLevel,
            Messages,
            Offloadable,
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
                            "powerStatus" | "power_status" => Ok(GeneratedField::PowerStatus),
                            "powerType" | "power_type" => Ok(GeneratedField::PowerType),
                            "powerLevel" | "power_level" => Ok(GeneratedField::PowerLevel),
                            "messages" => Ok(GeneratedField::Messages),
                            "offloadable" => Ok(GeneratedField::Offloadable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PowerSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PowerSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PowerSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut power_status__ = None;
                let mut power_type__ = None;
                let mut power_level__ = None;
                let mut messages__ = None;
                let mut offloadable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PowerStatus => {
                            if power_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("powerStatus"));
                            }
                            power_status__ = Some(map_.next_value::<PowerStatus>()? as i32);
                        }
                        GeneratedField::PowerType => {
                            if power_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("powerType"));
                            }
                            power_type__ = Some(map_.next_value::<PowerType>()? as i32);
                        }
                        GeneratedField::PowerLevel => {
                            if power_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("powerLevel"));
                            }
                            power_level__ = map_.next_value()?;
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offloadable => {
                            if offloadable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offloadable"));
                            }
                            offloadable__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PowerSource {
                    power_status: power_status__.unwrap_or_default(),
                    power_type: power_type__.unwrap_or_default(),
                    power_level: power_level__,
                    messages: messages__.unwrap_or_default(),
                    offloadable: offloadable__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PowerSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PowerState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_id_to_state.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PowerState", len)?;
        if !self.source_id_to_state.is_empty() {
            struct_ser.serialize_field("sourceIdToState", &self.source_id_to_state)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PowerState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_id_to_state",
            "sourceIdToState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIdToState,
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
                            "sourceIdToState" | "source_id_to_state" => Ok(GeneratedField::SourceIdToState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PowerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PowerState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PowerState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_id_to_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceIdToState => {
                            if source_id_to_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIdToState"));
                            }
                            source_id_to_state__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(PowerState {
                    source_id_to_state: source_id_to_state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PowerState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PowerStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "POWER_STATUS_INVALID",
            Self::Unknown => "POWER_STATUS_UNKNOWN",
            Self::NotPresent => "POWER_STATUS_NOT_PRESENT",
            Self::Operating => "POWER_STATUS_OPERATING",
            Self::Disabled => "POWER_STATUS_DISABLED",
            Self::Error => "POWER_STATUS_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PowerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "POWER_STATUS_INVALID",
            "POWER_STATUS_UNKNOWN",
            "POWER_STATUS_NOT_PRESENT",
            "POWER_STATUS_OPERATING",
            "POWER_STATUS_DISABLED",
            "POWER_STATUS_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PowerStatus;

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
                    "POWER_STATUS_INVALID" => Ok(PowerStatus::Invalid),
                    "POWER_STATUS_UNKNOWN" => Ok(PowerStatus::Unknown),
                    "POWER_STATUS_NOT_PRESENT" => Ok(PowerStatus::NotPresent),
                    "POWER_STATUS_OPERATING" => Ok(PowerStatus::Operating),
                    "POWER_STATUS_DISABLED" => Ok(PowerStatus::Disabled),
                    "POWER_STATUS_ERROR" => Ok(PowerStatus::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PowerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "POWER_TYPE_INVALID",
            Self::Unknown => "POWER_TYPE_UNKNOWN",
            Self::Gas => "POWER_TYPE_GAS",
            Self::Battery => "POWER_TYPE_BATTERY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PowerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "POWER_TYPE_INVALID",
            "POWER_TYPE_UNKNOWN",
            "POWER_TYPE_GAS",
            "POWER_TYPE_BATTERY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PowerType;

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
                    "POWER_TYPE_INVALID" => Ok(PowerType::Invalid),
                    "POWER_TYPE_UNKNOWN" => Ok(PowerType::Unknown),
                    "POWER_TYPE_GAS" => Ok(PowerType::Gas),
                    "POWER_TYPE_BATTERY" => Ok(PowerType::Battery),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Predicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_path.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.comparator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Predicate", len)?;
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.comparator != 0 {
            let v = Comparator::try_from(self.comparator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.comparator)))?;
            struct_ser.serialize_field("comparator", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Predicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_path",
            "fieldPath",
            "value",
            "comparator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldPath,
            Value,
            Comparator,
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
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            "value" => Ok(GeneratedField::Value),
                            "comparator" => Ok(GeneratedField::Comparator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Predicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Predicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Predicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_path__ = None;
                let mut value__ = None;
                let mut comparator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Comparator => {
                            if comparator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparator"));
                            }
                            comparator__ = Some(map_.next_value::<Comparator>()? as i32);
                        }
                    }
                }
                Ok(Predicate {
                    field_path: field_path__.unwrap_or_default(),
                    value: value__,
                    comparator: comparator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Predicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PredicateSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.predicates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PredicateSet", len)?;
        if !self.predicates.is_empty() {
            struct_ser.serialize_field("predicates", &self.predicates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PredicateSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicates,
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
                            "predicates" => Ok(GeneratedField::Predicates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PredicateSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PredicateSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PredicateSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicates__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Predicates => {
                            if predicates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicates"));
                            }
                            predicates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PredicateSet {
                    predicates: predicates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PredicateSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrimaryCorrelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.secondary_entity_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PrimaryCorrelation", len)?;
        if !self.secondary_entity_ids.is_empty() {
            struct_ser.serialize_field("secondaryEntityIds", &self.secondary_entity_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrimaryCorrelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secondary_entity_ids",
            "secondaryEntityIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SecondaryEntityIds,
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
                            "secondaryEntityIds" | "secondary_entity_ids" => Ok(GeneratedField::SecondaryEntityIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrimaryCorrelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PrimaryCorrelation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrimaryCorrelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secondary_entity_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SecondaryEntityIds => {
                            if secondary_entity_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryEntityIds"));
                            }
                            secondary_entity_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PrimaryCorrelation {
                    secondary_entity_ids: secondary_entity_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PrimaryCorrelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrimaryMembership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PrimaryMembership", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrimaryMembership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrimaryMembership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PrimaryMembership")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrimaryMembership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PrimaryMembership {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PrimaryMembership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectedFrustum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.upper_left.is_some() {
            len += 1;
        }
        if self.upper_right.is_some() {
            len += 1;
        }
        if self.bottom_right.is_some() {
            len += 1;
        }
        if self.bottom_left.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ProjectedFrustum", len)?;
        if let Some(v) = self.upper_left.as_ref() {
            struct_ser.serialize_field("upperLeft", v)?;
        }
        if let Some(v) = self.upper_right.as_ref() {
            struct_ser.serialize_field("upperRight", v)?;
        }
        if let Some(v) = self.bottom_right.as_ref() {
            struct_ser.serialize_field("bottomRight", v)?;
        }
        if let Some(v) = self.bottom_left.as_ref() {
            struct_ser.serialize_field("bottomLeft", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectedFrustum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upper_left",
            "upperLeft",
            "upper_right",
            "upperRight",
            "bottom_right",
            "bottomRight",
            "bottom_left",
            "bottomLeft",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpperLeft,
            UpperRight,
            BottomRight,
            BottomLeft,
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
                            "upperLeft" | "upper_left" => Ok(GeneratedField::UpperLeft),
                            "upperRight" | "upper_right" => Ok(GeneratedField::UpperRight),
                            "bottomRight" | "bottom_right" => Ok(GeneratedField::BottomRight),
                            "bottomLeft" | "bottom_left" => Ok(GeneratedField::BottomLeft),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectedFrustum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ProjectedFrustum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectedFrustum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upper_left__ = None;
                let mut upper_right__ = None;
                let mut bottom_right__ = None;
                let mut bottom_left__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpperLeft => {
                            if upper_left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperLeft"));
                            }
                            upper_left__ = map_.next_value()?;
                        }
                        GeneratedField::UpperRight => {
                            if upper_right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperRight"));
                            }
                            upper_right__ = map_.next_value()?;
                        }
                        GeneratedField::BottomRight => {
                            if bottom_right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottomRight"));
                            }
                            bottom_right__ = map_.next_value()?;
                        }
                        GeneratedField::BottomLeft => {
                            if bottom_left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottomLeft"));
                            }
                            bottom_left__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProjectedFrustum {
                    upper_left: upper_left__,
                    upper_right: upper_right__,
                    bottom_right: bottom_right__,
                    bottom_left: bottom_left__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ProjectedFrustum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Provenance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.integration_name.is_empty() {
            len += 1;
        }
        if !self.data_type.is_empty() {
            len += 1;
        }
        if !self.source_id.is_empty() {
            len += 1;
        }
        if self.source_update_time.is_some() {
            len += 1;
        }
        if !self.source_description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Provenance", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.data_type.is_empty() {
            struct_ser.serialize_field("dataType", &self.data_type)?;
        }
        if !self.source_id.is_empty() {
            struct_ser.serialize_field("sourceId", &self.source_id)?;
        }
        if let Some(v) = self.source_update_time.as_ref() {
            struct_ser.serialize_field("sourceUpdateTime", v)?;
        }
        if !self.source_description.is_empty() {
            struct_ser.serialize_field("sourceDescription", &self.source_description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Provenance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "data_type",
            "dataType",
            "source_id",
            "sourceId",
            "source_update_time",
            "sourceUpdateTime",
            "source_description",
            "sourceDescription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            DataType,
            SourceId,
            SourceUpdateTime,
            SourceDescription,
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
                            "integrationName" | "integration_name" => Ok(GeneratedField::IntegrationName),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "sourceId" | "source_id" => Ok(GeneratedField::SourceId),
                            "sourceUpdateTime" | "source_update_time" => Ok(GeneratedField::SourceUpdateTime),
                            "sourceDescription" | "source_description" => Ok(GeneratedField::SourceDescription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Provenance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Provenance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Provenance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut data_type__ = None;
                let mut source_id__ = None;
                let mut source_update_time__ = None;
                let mut source_description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceId => {
                            if source_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceId"));
                            }
                            source_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceUpdateTime => {
                            if source_update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceUpdateTime"));
                            }
                            source_update_time__ = map_.next_value()?;
                        }
                        GeneratedField::SourceDescription => {
                            if source_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDescription"));
                            }
                            source_description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Provenance {
                    integration_name: integration_name__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    source_id: source_id__.unwrap_or_default(),
                    source_update_time: source_update_time__,
                    source_description: source_description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Provenance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublishEntitiesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PublishEntitiesRequest", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublishEntitiesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
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
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublishEntitiesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PublishEntitiesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublishEntitiesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PublishEntitiesRequest {
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PublishEntitiesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublishEntitiesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PublishEntitiesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublishEntitiesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublishEntitiesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PublishEntitiesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublishEntitiesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PublishEntitiesResponse {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PublishEntitiesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublishEntityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PublishEntityRequest", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublishEntityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
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
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublishEntityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PublishEntityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublishEntityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PublishEntityRequest {
                    entity: entity__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PublishEntityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublishEntityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PublishEntityResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublishEntityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublishEntityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PublishEntityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublishEntityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PublishEntityResponse {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PublishEntityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PulseRepetitionInterval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pulse_repetition_interval_s.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.PulseRepetitionInterval", len)?;
        if let Some(v) = self.pulse_repetition_interval_s.as_ref() {
            struct_ser.serialize_field("pulseRepetitionIntervalS", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PulseRepetitionInterval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pulse_repetition_interval_s",
            "pulseRepetitionIntervalS",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PulseRepetitionIntervalS,
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
                            "pulseRepetitionIntervalS" | "pulse_repetition_interval_s" => Ok(GeneratedField::PulseRepetitionIntervalS),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PulseRepetitionInterval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.PulseRepetitionInterval")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PulseRepetitionInterval, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pulse_repetition_interval_s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PulseRepetitionIntervalS => {
                            if pulse_repetition_interval_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pulseRepetitionIntervalS"));
                            }
                            pulse_repetition_interval_s__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PulseRepetitionInterval {
                    pulse_repetition_interval_s: pulse_repetition_interval_s__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.PulseRepetitionInterval", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RfConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.frequency_range_hz.is_empty() {
            len += 1;
        }
        if !self.bandwidth_range_hz.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RFConfiguration", len)?;
        if !self.frequency_range_hz.is_empty() {
            struct_ser.serialize_field("frequencyRangeHz", &self.frequency_range_hz)?;
        }
        if !self.bandwidth_range_hz.is_empty() {
            struct_ser.serialize_field("bandwidthRangeHz", &self.bandwidth_range_hz)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RfConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "frequency_range_hz",
            "frequencyRangeHz",
            "bandwidth_range_hz",
            "bandwidthRangeHz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FrequencyRangeHz,
            BandwidthRangeHz,
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
                            "frequencyRangeHz" | "frequency_range_hz" => Ok(GeneratedField::FrequencyRangeHz),
                            "bandwidthRangeHz" | "bandwidth_range_hz" => Ok(GeneratedField::BandwidthRangeHz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RfConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RFConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RfConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut frequency_range_hz__ = None;
                let mut bandwidth_range_hz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FrequencyRangeHz => {
                            if frequency_range_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frequencyRangeHz"));
                            }
                            frequency_range_hz__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BandwidthRangeHz => {
                            if bandwidth_range_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandwidthRangeHz"));
                            }
                            bandwidth_range_hz__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RfConfiguration {
                    frequency_range_hz: frequency_range_hz__.unwrap_or_default(),
                    bandwidth_range_hz: bandwidth_range_hz__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RFConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RangeRings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_distance_m.is_some() {
            len += 1;
        }
        if self.max_distance_m.is_some() {
            len += 1;
        }
        if self.ring_count != 0 {
            len += 1;
        }
        if self.ring_line_color.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RangeRings", len)?;
        if let Some(v) = self.min_distance_m.as_ref() {
            struct_ser.serialize_field("minDistanceM", v)?;
        }
        if let Some(v) = self.max_distance_m.as_ref() {
            struct_ser.serialize_field("maxDistanceM", v)?;
        }
        if self.ring_count != 0 {
            struct_ser.serialize_field("ringCount", &self.ring_count)?;
        }
        if let Some(v) = self.ring_line_color.as_ref() {
            struct_ser.serialize_field("ringLineColor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RangeRings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_distance_m",
            "minDistanceM",
            "max_distance_m",
            "maxDistanceM",
            "ring_count",
            "ringCount",
            "ring_line_color",
            "ringLineColor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinDistanceM,
            MaxDistanceM,
            RingCount,
            RingLineColor,
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
                            "minDistanceM" | "min_distance_m" => Ok(GeneratedField::MinDistanceM),
                            "maxDistanceM" | "max_distance_m" => Ok(GeneratedField::MaxDistanceM),
                            "ringCount" | "ring_count" => Ok(GeneratedField::RingCount),
                            "ringLineColor" | "ring_line_color" => Ok(GeneratedField::RingLineColor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RangeRings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RangeRings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RangeRings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_distance_m__ = None;
                let mut max_distance_m__ = None;
                let mut ring_count__ = None;
                let mut ring_line_color__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinDistanceM => {
                            if min_distance_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDistanceM"));
                            }
                            min_distance_m__ = map_.next_value()?;
                        }
                        GeneratedField::MaxDistanceM => {
                            if max_distance_m__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDistanceM"));
                            }
                            max_distance_m__ = map_.next_value()?;
                        }
                        GeneratedField::RingCount => {
                            if ring_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ringCount"));
                            }
                            ring_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RingLineColor => {
                            if ring_line_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ringLineColor"));
                            }
                            ring_line_color__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RangeRings {
                    min_distance_m: min_distance_m__,
                    max_distance_m: max_distance_m__,
                    ring_count: ring_count__.unwrap_or_default(),
                    ring_line_color: ring_line_color__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RangeRings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RangeType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RangeType", len)?;
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RangeType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RangeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RangeType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RangeType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RangeType {
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RangeType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update_per_entity_limit_ms != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RateLimit", len)?;
        if self.update_per_entity_limit_ms != 0 {
            struct_ser.serialize_field("updatePerEntityLimitMs", &self.update_per_entity_limit_ms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "update_per_entity_limit_ms",
            "updatePerEntityLimitMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdatePerEntityLimitMs,
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
                            "updatePerEntityLimitMs" | "update_per_entity_limit_ms" => Ok(GeneratedField::UpdatePerEntityLimitMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update_per_entity_limit_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpdatePerEntityLimitMs => {
                            if update_per_entity_limit_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatePerEntityLimitMs"));
                            }
                            update_per_entity_limit_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimit {
                    update_per_entity_limit_ms: update_per_entity_limit_ms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.related_entity_id.is_empty() {
            len += 1;
        }
        if !self.relationship_id.is_empty() {
            len += 1;
        }
        if self.relationship_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Relationship", len)?;
        if !self.related_entity_id.is_empty() {
            struct_ser.serialize_field("relatedEntityId", &self.related_entity_id)?;
        }
        if !self.relationship_id.is_empty() {
            struct_ser.serialize_field("relationshipId", &self.relationship_id)?;
        }
        if let Some(v) = self.relationship_type.as_ref() {
            struct_ser.serialize_field("relationshipType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "related_entity_id",
            "relatedEntityId",
            "relationship_id",
            "relationshipId",
            "relationship_type",
            "relationshipType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelatedEntityId,
            RelationshipId,
            RelationshipType,
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
                            "relatedEntityId" | "related_entity_id" => Ok(GeneratedField::RelatedEntityId),
                            "relationshipId" | "relationship_id" => Ok(GeneratedField::RelationshipId),
                            "relationshipType" | "relationship_type" => Ok(GeneratedField::RelationshipType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relationship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Relationship")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Relationship, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut related_entity_id__ = None;
                let mut relationship_id__ = None;
                let mut relationship_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RelatedEntityId => {
                            if related_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedEntityId"));
                            }
                            related_entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RelationshipId => {
                            if relationship_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipId"));
                            }
                            relationship_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RelationshipType => {
                            if relationship_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipType"));
                            }
                            relationship_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Relationship {
                    related_entity_id: related_entity_id__.unwrap_or_default(),
                    relationship_id: relationship_id__.unwrap_or_default(),
                    relationship_type: relationship_type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Relationship", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationshipType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RelationshipType", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                relationship_type::Type::TrackedBy(v) => {
                    struct_ser.serialize_field("trackedBy", v)?;
                }
                relationship_type::Type::GroupChild(v) => {
                    struct_ser.serialize_field("groupChild", v)?;
                }
                relationship_type::Type::GroupParent(v) => {
                    struct_ser.serialize_field("groupParent", v)?;
                }
                relationship_type::Type::MergedFrom(v) => {
                    struct_ser.serialize_field("mergedFrom", v)?;
                }
                relationship_type::Type::ActiveTarget(v) => {
                    struct_ser.serialize_field("activeTarget", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationshipType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tracked_by",
            "trackedBy",
            "group_child",
            "groupChild",
            "group_parent",
            "groupParent",
            "merged_from",
            "mergedFrom",
            "active_target",
            "activeTarget",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrackedBy,
            GroupChild,
            GroupParent,
            MergedFrom,
            ActiveTarget,
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
                            "trackedBy" | "tracked_by" => Ok(GeneratedField::TrackedBy),
                            "groupChild" | "group_child" => Ok(GeneratedField::GroupChild),
                            "groupParent" | "group_parent" => Ok(GeneratedField::GroupParent),
                            "mergedFrom" | "merged_from" => Ok(GeneratedField::MergedFrom),
                            "activeTarget" | "active_target" => Ok(GeneratedField::ActiveTarget),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationshipType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RelationshipType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RelationshipType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrackedBy => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackedBy"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(relationship_type::Type::TrackedBy)
;
                        }
                        GeneratedField::GroupChild => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupChild"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(relationship_type::Type::GroupChild)
;
                        }
                        GeneratedField::GroupParent => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupParent"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(relationship_type::Type::GroupParent)
;
                        }
                        GeneratedField::MergedFrom => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergedFrom"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(relationship_type::Type::MergedFrom)
;
                        }
                        GeneratedField::ActiveTarget => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeTarget"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(relationship_type::Type::ActiveTarget)
;
                        }
                    }
                }
                Ok(RelationshipType {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RelationshipType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relationships {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relationships.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Relationships", len)?;
        if !self.relationships.is_empty() {
            struct_ser.serialize_field("relationships", &self.relationships)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relationships {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relationships",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relationships,
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
                            "relationships" => Ok(GeneratedField::Relationships),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relationships;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Relationships")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Relationships, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relationships__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Relationships {
                    relationships: relationships__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Relationships", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveEntityOverrideRequest {
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
        if !self.field_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RemoveEntityOverrideRequest", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.field_path.is_empty() {
            struct_ser.serialize_field("fieldPath", &self.field_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveEntityOverrideRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "field_path",
            "fieldPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            FieldPath,
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
                            "fieldPath" | "field_path" => Ok(GeneratedField::FieldPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveEntityOverrideRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RemoveEntityOverrideRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveEntityOverrideRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut field_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldPath => {
                            if field_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldPath"));
                            }
                            field_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveEntityOverrideRequest {
                    entity_id: entity_id__.unwrap_or_default(),
                    field_path: field_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RemoveEntityOverrideRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveEntityOverrideResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RemoveEntityOverrideResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveEntityOverrideResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveEntityOverrideResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RemoveEntityOverrideResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveEntityOverrideResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveEntityOverrideResponse {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RemoveEntityOverrideResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.destination_name.is_empty() {
            len += 1;
        }
        if self.estimated_arrival_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.RouteDetails", len)?;
        if !self.destination_name.is_empty() {
            struct_ser.serialize_field("destinationName", &self.destination_name)?;
        }
        if let Some(v) = self.estimated_arrival_time.as_ref() {
            struct_ser.serialize_field("estimatedArrivalTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "destination_name",
            "destinationName",
            "estimated_arrival_time",
            "estimatedArrivalTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationName,
            EstimatedArrivalTime,
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
                            "destinationName" | "destination_name" => Ok(GeneratedField::DestinationName),
                            "estimatedArrivalTime" | "estimated_arrival_time" => Ok(GeneratedField::EstimatedArrivalTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.RouteDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RouteDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut destination_name__ = None;
                let mut estimated_arrival_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DestinationName => {
                            if destination_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationName"));
                            }
                            destination_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EstimatedArrivalTime => {
                            if estimated_arrival_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedArrivalTime"));
                            }
                            estimated_arrival_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RouteDetails {
                    destination_name: destination_name__.unwrap_or_default(),
                    estimated_arrival_time: estimated_arrival_time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.RouteDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanCharacteristics {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scan_type != 0 {
            len += 1;
        }
        if self.scan_period_s.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.ScanCharacteristics", len)?;
        if self.scan_type != 0 {
            let v = ScanType::try_from(self.scan_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.scan_type)))?;
            struct_ser.serialize_field("scanType", &v)?;
        }
        if let Some(v) = self.scan_period_s.as_ref() {
            struct_ser.serialize_field("scanPeriodS", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScanCharacteristics {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scan_type",
            "scanType",
            "scan_period_s",
            "scanPeriodS",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScanType,
            ScanPeriodS,
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
                            "scanType" | "scan_type" => Ok(GeneratedField::ScanType),
                            "scanPeriodS" | "scan_period_s" => Ok(GeneratedField::ScanPeriodS),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanCharacteristics;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.ScanCharacteristics")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScanCharacteristics, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scan_type__ = None;
                let mut scan_period_s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ScanType => {
                            if scan_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanType"));
                            }
                            scan_type__ = Some(map_.next_value::<ScanType>()? as i32);
                        }
                        GeneratedField::ScanPeriodS => {
                            if scan_period_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanPeriodS"));
                            }
                            scan_period_s__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ScanCharacteristics {
                    scan_type: scan_type__.unwrap_or_default(),
                    scan_period_s: scan_period_s__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.ScanCharacteristics", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScanType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SCAN_TYPE_INVALID",
            Self::Circular => "SCAN_TYPE_CIRCULAR",
            Self::BidirectionalHorizontalSector => "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR",
            Self::BidirectionalVerticalSector => "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR",
            Self::NonScanning => "SCAN_TYPE_NON_SCANNING",
            Self::Irregular => "SCAN_TYPE_IRREGULAR",
            Self::Conical => "SCAN_TYPE_CONICAL",
            Self::LobeSwitching => "SCAN_TYPE_LOBE_SWITCHING",
            Self::Raster => "SCAN_TYPE_RASTER",
            Self::CircularVerticalSector => "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR",
            Self::CircularConical => "SCAN_TYPE_CIRCULAR_CONICAL",
            Self::SectorConical => "SCAN_TYPE_SECTOR_CONICAL",
            Self::AgileBeam => "SCAN_TYPE_AGILE_BEAM",
            Self::UnidirectionalVerticalSector => "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR",
            Self::UnidirectionalHorizontalSector => "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR",
            Self::UnidirectionalSector => "SCAN_TYPE_UNIDIRECTIONAL_SECTOR",
            Self::BidirectionalSector => "SCAN_TYPE_BIDIRECTIONAL_SECTOR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ScanType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCAN_TYPE_INVALID",
            "SCAN_TYPE_CIRCULAR",
            "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR",
            "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR",
            "SCAN_TYPE_NON_SCANNING",
            "SCAN_TYPE_IRREGULAR",
            "SCAN_TYPE_CONICAL",
            "SCAN_TYPE_LOBE_SWITCHING",
            "SCAN_TYPE_RASTER",
            "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR",
            "SCAN_TYPE_CIRCULAR_CONICAL",
            "SCAN_TYPE_SECTOR_CONICAL",
            "SCAN_TYPE_AGILE_BEAM",
            "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR",
            "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR",
            "SCAN_TYPE_UNIDIRECTIONAL_SECTOR",
            "SCAN_TYPE_BIDIRECTIONAL_SECTOR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScanType;

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
                    "SCAN_TYPE_INVALID" => Ok(ScanType::Invalid),
                    "SCAN_TYPE_CIRCULAR" => Ok(ScanType::Circular),
                    "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR" => Ok(ScanType::BidirectionalHorizontalSector),
                    "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR" => Ok(ScanType::BidirectionalVerticalSector),
                    "SCAN_TYPE_NON_SCANNING" => Ok(ScanType::NonScanning),
                    "SCAN_TYPE_IRREGULAR" => Ok(ScanType::Irregular),
                    "SCAN_TYPE_CONICAL" => Ok(ScanType::Conical),
                    "SCAN_TYPE_LOBE_SWITCHING" => Ok(ScanType::LobeSwitching),
                    "SCAN_TYPE_RASTER" => Ok(ScanType::Raster),
                    "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR" => Ok(ScanType::CircularVerticalSector),
                    "SCAN_TYPE_CIRCULAR_CONICAL" => Ok(ScanType::CircularConical),
                    "SCAN_TYPE_SECTOR_CONICAL" => Ok(ScanType::SectorConical),
                    "SCAN_TYPE_AGILE_BEAM" => Ok(ScanType::AgileBeam),
                    "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR" => Ok(ScanType::UnidirectionalVerticalSector),
                    "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR" => Ok(ScanType::UnidirectionalHorizontalSector),
                    "SCAN_TYPE_UNIDIRECTIONAL_SECTOR" => Ok(ScanType::UnidirectionalSector),
                    "SCAN_TYPE_BIDIRECTIONAL_SECTOR" => Ok(ScanType::BidirectionalSector),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Schedule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.windows.is_empty() {
            len += 1;
        }
        if !self.schedule_id.is_empty() {
            len += 1;
        }
        if self.schedule_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Schedule", len)?;
        if !self.windows.is_empty() {
            struct_ser.serialize_field("windows", &self.windows)?;
        }
        if !self.schedule_id.is_empty() {
            struct_ser.serialize_field("scheduleId", &self.schedule_id)?;
        }
        if self.schedule_type != 0 {
            let v = ScheduleType::try_from(self.schedule_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.schedule_type)))?;
            struct_ser.serialize_field("scheduleType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Schedule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "windows",
            "schedule_id",
            "scheduleId",
            "schedule_type",
            "scheduleType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Windows,
            ScheduleId,
            ScheduleType,
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
                            "windows" => Ok(GeneratedField::Windows),
                            "scheduleId" | "schedule_id" => Ok(GeneratedField::ScheduleId),
                            "scheduleType" | "schedule_type" => Ok(GeneratedField::ScheduleType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Schedule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Schedule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Schedule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut windows__ = None;
                let mut schedule_id__ = None;
                let mut schedule_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Windows => {
                            if windows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windows"));
                            }
                            windows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScheduleId => {
                            if schedule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduleId"));
                            }
                            schedule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScheduleType => {
                            if schedule_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduleType"));
                            }
                            schedule_type__ = Some(map_.next_value::<ScheduleType>()? as i32);
                        }
                    }
                }
                Ok(Schedule {
                    windows: windows__.unwrap_or_default(),
                    schedule_id: schedule_id__.unwrap_or_default(),
                    schedule_type: schedule_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Schedule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScheduleType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SCHEDULE_TYPE_INVALID",
            Self::ZoneEnabled => "SCHEDULE_TYPE_ZONE_ENABLED",
            Self::ZoneTempEnabled => "SCHEDULE_TYPE_ZONE_TEMP_ENABLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ScheduleType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCHEDULE_TYPE_INVALID",
            "SCHEDULE_TYPE_ZONE_ENABLED",
            "SCHEDULE_TYPE_ZONE_TEMP_ENABLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScheduleType;

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
                    "SCHEDULE_TYPE_INVALID" => Ok(ScheduleType::Invalid),
                    "SCHEDULE_TYPE_ZONE_ENABLED" => Ok(ScheduleType::ZoneEnabled),
                    "SCHEDULE_TYPE_ZONE_TEMP_ENABLED" => Ok(ScheduleType::ZoneTempEnabled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Schedules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schedules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Schedules", len)?;
        if !self.schedules.is_empty() {
            struct_ser.serialize_field("schedules", &self.schedules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Schedules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schedules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schedules,
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
                            "schedules" => Ok(GeneratedField::Schedules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Schedules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Schedules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Schedules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schedules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schedules => {
                            if schedules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedules"));
                            }
                            schedules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Schedules {
                    schedules: schedules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Schedules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScoreInterpretation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SCORE_INTERPRETATION_INVALID",
            Self::Unlikely => "SCORE_INTERPRETATION_UNLIKELY",
            Self::Likely => "SCORE_INTERPRETATION_LIKELY",
            Self::VeryLikely => "SCORE_INTERPRETATION_VERY_LIKELY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ScoreInterpretation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SCORE_INTERPRETATION_INVALID",
            "SCORE_INTERPRETATION_UNLIKELY",
            "SCORE_INTERPRETATION_LIKELY",
            "SCORE_INTERPRETATION_VERY_LIKELY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScoreInterpretation;

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
                    "SCORE_INTERPRETATION_INVALID" => Ok(ScoreInterpretation::Invalid),
                    "SCORE_INTERPRETATION_UNLIKELY" => Ok(ScoreInterpretation::Unlikely),
                    "SCORE_INTERPRETATION_LIKELY" => Ok(ScoreInterpretation::Likely),
                    "SCORE_INTERPRETATION_VERY_LIKELY" => Ok(ScoreInterpretation::VeryLikely),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SecondaryCorrelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.primary_entity_id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.SecondaryCorrelation", len)?;
        if !self.primary_entity_id.is_empty() {
            struct_ser.serialize_field("primaryEntityId", &self.primary_entity_id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecondaryCorrelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "primary_entity_id",
            "primaryEntityId",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrimaryEntityId,
            Metadata,
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
                            "primaryEntityId" | "primary_entity_id" => Ok(GeneratedField::PrimaryEntityId),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecondaryCorrelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.SecondaryCorrelation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecondaryCorrelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut primary_entity_id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrimaryEntityId => {
                            if primary_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryEntityId"));
                            }
                            primary_entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SecondaryCorrelation {
                    primary_entity_id: primary_entity_id__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.SecondaryCorrelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Sensor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sensor_id.is_empty() {
            len += 1;
        }
        if self.operational_state != 0 {
            len += 1;
        }
        if self.sensor_type != 0 {
            len += 1;
        }
        if !self.sensor_description.is_empty() {
            len += 1;
        }
        if self.rf_configuraton.is_some() {
            len += 1;
        }
        if self.last_detection_timestamp.is_some() {
            len += 1;
        }
        if !self.fields_of_view.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Sensor", len)?;
        if !self.sensor_id.is_empty() {
            struct_ser.serialize_field("sensorId", &self.sensor_id)?;
        }
        if self.operational_state != 0 {
            let v = OperationalState::try_from(self.operational_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.operational_state)))?;
            struct_ser.serialize_field("operationalState", &v)?;
        }
        if self.sensor_type != 0 {
            let v = SensorType::try_from(self.sensor_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sensor_type)))?;
            struct_ser.serialize_field("sensorType", &v)?;
        }
        if !self.sensor_description.is_empty() {
            struct_ser.serialize_field("sensorDescription", &self.sensor_description)?;
        }
        if let Some(v) = self.rf_configuraton.as_ref() {
            struct_ser.serialize_field("rfConfiguraton", v)?;
        }
        if let Some(v) = self.last_detection_timestamp.as_ref() {
            struct_ser.serialize_field("lastDetectionTimestamp", v)?;
        }
        if !self.fields_of_view.is_empty() {
            struct_ser.serialize_field("fieldsOfView", &self.fields_of_view)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sensor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sensor_id",
            "sensorId",
            "operational_state",
            "operationalState",
            "sensor_type",
            "sensorType",
            "sensor_description",
            "sensorDescription",
            "rf_configuraton",
            "rfConfiguraton",
            "last_detection_timestamp",
            "lastDetectionTimestamp",
            "fields_of_view",
            "fieldsOfView",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SensorId,
            OperationalState,
            SensorType,
            SensorDescription,
            RfConfiguraton,
            LastDetectionTimestamp,
            FieldsOfView,
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
                            "sensorId" | "sensor_id" => Ok(GeneratedField::SensorId),
                            "operationalState" | "operational_state" => Ok(GeneratedField::OperationalState),
                            "sensorType" | "sensor_type" => Ok(GeneratedField::SensorType),
                            "sensorDescription" | "sensor_description" => Ok(GeneratedField::SensorDescription),
                            "rfConfiguraton" | "rf_configuraton" => Ok(GeneratedField::RfConfiguraton),
                            "lastDetectionTimestamp" | "last_detection_timestamp" => Ok(GeneratedField::LastDetectionTimestamp),
                            "fieldsOfView" | "fields_of_view" => Ok(GeneratedField::FieldsOfView),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Sensor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Sensor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Sensor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sensor_id__ = None;
                let mut operational_state__ = None;
                let mut sensor_type__ = None;
                let mut sensor_description__ = None;
                let mut rf_configuraton__ = None;
                let mut last_detection_timestamp__ = None;
                let mut fields_of_view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SensorId => {
                            if sensor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensorId"));
                            }
                            sensor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OperationalState => {
                            if operational_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationalState"));
                            }
                            operational_state__ = Some(map_.next_value::<OperationalState>()? as i32);
                        }
                        GeneratedField::SensorType => {
                            if sensor_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensorType"));
                            }
                            sensor_type__ = Some(map_.next_value::<SensorType>()? as i32);
                        }
                        GeneratedField::SensorDescription => {
                            if sensor_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensorDescription"));
                            }
                            sensor_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RfConfiguraton => {
                            if rf_configuraton__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfConfiguraton"));
                            }
                            rf_configuraton__ = map_.next_value()?;
                        }
                        GeneratedField::LastDetectionTimestamp => {
                            if last_detection_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastDetectionTimestamp"));
                            }
                            last_detection_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::FieldsOfView => {
                            if fields_of_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldsOfView"));
                            }
                            fields_of_view__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Sensor {
                    sensor_id: sensor_id__.unwrap_or_default(),
                    operational_state: operational_state__.unwrap_or_default(),
                    sensor_type: sensor_type__.unwrap_or_default(),
                    sensor_description: sensor_description__.unwrap_or_default(),
                    rf_configuraton: rf_configuraton__,
                    last_detection_timestamp: last_detection_timestamp__,
                    fields_of_view: fields_of_view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Sensor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SensorMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SENSOR_MODE_INVALID",
            Self::Search => "SENSOR_MODE_SEARCH",
            Self::Track => "SENSOR_MODE_TRACK",
            Self::WeaponSupport => "SENSOR_MODE_WEAPON_SUPPORT",
            Self::Auto => "SENSOR_MODE_AUTO",
            Self::Mute => "SENSOR_MODE_MUTE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SensorMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SENSOR_MODE_INVALID",
            "SENSOR_MODE_SEARCH",
            "SENSOR_MODE_TRACK",
            "SENSOR_MODE_WEAPON_SUPPORT",
            "SENSOR_MODE_AUTO",
            "SENSOR_MODE_MUTE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SensorMode;

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
                    "SENSOR_MODE_INVALID" => Ok(SensorMode::Invalid),
                    "SENSOR_MODE_SEARCH" => Ok(SensorMode::Search),
                    "SENSOR_MODE_TRACK" => Ok(SensorMode::Track),
                    "SENSOR_MODE_WEAPON_SUPPORT" => Ok(SensorMode::WeaponSupport),
                    "SENSOR_MODE_AUTO" => Ok(SensorMode::Auto),
                    "SENSOR_MODE_MUTE" => Ok(SensorMode::Mute),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SensorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SENSOR_TYPE_INVALID",
            Self::Radar => "SENSOR_TYPE_RADAR",
            Self::Camera => "SENSOR_TYPE_CAMERA",
            Self::Transponder => "SENSOR_TYPE_TRANSPONDER",
            Self::Rf => "SENSOR_TYPE_RF",
            Self::Gps => "SENSOR_TYPE_GPS",
            Self::PtuPos => "SENSOR_TYPE_PTU_POS",
            Self::Perimeter => "SENSOR_TYPE_PERIMETER",
            Self::Sonar => "SENSOR_TYPE_SONAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SensorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SENSOR_TYPE_INVALID",
            "SENSOR_TYPE_RADAR",
            "SENSOR_TYPE_CAMERA",
            "SENSOR_TYPE_TRANSPONDER",
            "SENSOR_TYPE_RF",
            "SENSOR_TYPE_GPS",
            "SENSOR_TYPE_PTU_POS",
            "SENSOR_TYPE_PERIMETER",
            "SENSOR_TYPE_SONAR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SensorType;

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
                    "SENSOR_TYPE_INVALID" => Ok(SensorType::Invalid),
                    "SENSOR_TYPE_RADAR" => Ok(SensorType::Radar),
                    "SENSOR_TYPE_CAMERA" => Ok(SensorType::Camera),
                    "SENSOR_TYPE_TRANSPONDER" => Ok(SensorType::Transponder),
                    "SENSOR_TYPE_RF" => Ok(SensorType::Rf),
                    "SENSOR_TYPE_GPS" => Ok(SensorType::Gps),
                    "SENSOR_TYPE_PTU_POS" => Ok(SensorType::PtuPos),
                    "SENSOR_TYPE_PERIMETER" => Ok(SensorType::Perimeter),
                    "SENSOR_TYPE_SONAR" => Ok(SensorType::Sonar),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Sensors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sensors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Sensors", len)?;
        if !self.sensors.is_empty() {
            struct_ser.serialize_field("sensors", &self.sensors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sensors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sensors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sensors,
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
                            "sensors" => Ok(GeneratedField::Sensors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Sensors;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Sensors")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Sensors, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sensors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sensors => {
                            if sensors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensors"));
                            }
                            sensors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Sensors {
                    sensors: sensors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Sensors", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Signal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bandwidth_hz.is_some() {
            len += 1;
        }
        if self.signal_to_noise_ratio.is_some() {
            len += 1;
        }
        if !self.emitter_notations.is_empty() {
            len += 1;
        }
        if self.pulse_width_s.is_some() {
            len += 1;
        }
        if self.pulse_repetition_interval.is_some() {
            len += 1;
        }
        if self.scan_characteristics.is_some() {
            len += 1;
        }
        if self.frequency_measurement.is_some() {
            len += 1;
        }
        if self.report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Signal", len)?;
        if let Some(v) = self.bandwidth_hz.as_ref() {
            struct_ser.serialize_field("bandwidthHz", v)?;
        }
        if let Some(v) = self.signal_to_noise_ratio.as_ref() {
            struct_ser.serialize_field("signalToNoiseRatio", v)?;
        }
        if !self.emitter_notations.is_empty() {
            struct_ser.serialize_field("emitterNotations", &self.emitter_notations)?;
        }
        if let Some(v) = self.pulse_width_s.as_ref() {
            struct_ser.serialize_field("pulseWidthS", v)?;
        }
        if let Some(v) = self.pulse_repetition_interval.as_ref() {
            struct_ser.serialize_field("pulseRepetitionInterval", v)?;
        }
        if let Some(v) = self.scan_characteristics.as_ref() {
            struct_ser.serialize_field("scanCharacteristics", v)?;
        }
        if let Some(v) = self.frequency_measurement.as_ref() {
            match v {
                signal::FrequencyMeasurement::FrequencyCenter(v) => {
                    struct_ser.serialize_field("frequencyCenter", v)?;
                }
                signal::FrequencyMeasurement::FrequencyRange(v) => {
                    struct_ser.serialize_field("frequencyRange", v)?;
                }
            }
        }
        if let Some(v) = self.report.as_ref() {
            match v {
                signal::Report::LineOfBearing(v) => {
                    struct_ser.serialize_field("lineOfBearing", v)?;
                }
                signal::Report::Fixed(v) => {
                    struct_ser.serialize_field("fixed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Signal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bandwidth_hz",
            "bandwidthHz",
            "signal_to_noise_ratio",
            "signalToNoiseRatio",
            "emitter_notations",
            "emitterNotations",
            "pulse_width_s",
            "pulseWidthS",
            "pulse_repetition_interval",
            "pulseRepetitionInterval",
            "scan_characteristics",
            "scanCharacteristics",
            "frequency_center",
            "frequencyCenter",
            "frequency_range",
            "frequencyRange",
            "line_of_bearing",
            "lineOfBearing",
            "fixed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BandwidthHz,
            SignalToNoiseRatio,
            EmitterNotations,
            PulseWidthS,
            PulseRepetitionInterval,
            ScanCharacteristics,
            FrequencyCenter,
            FrequencyRange,
            LineOfBearing,
            Fixed,
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
                            "bandwidthHz" | "bandwidth_hz" => Ok(GeneratedField::BandwidthHz),
                            "signalToNoiseRatio" | "signal_to_noise_ratio" => Ok(GeneratedField::SignalToNoiseRatio),
                            "emitterNotations" | "emitter_notations" => Ok(GeneratedField::EmitterNotations),
                            "pulseWidthS" | "pulse_width_s" => Ok(GeneratedField::PulseWidthS),
                            "pulseRepetitionInterval" | "pulse_repetition_interval" => Ok(GeneratedField::PulseRepetitionInterval),
                            "scanCharacteristics" | "scan_characteristics" => Ok(GeneratedField::ScanCharacteristics),
                            "frequencyCenter" | "frequency_center" => Ok(GeneratedField::FrequencyCenter),
                            "frequencyRange" | "frequency_range" => Ok(GeneratedField::FrequencyRange),
                            "lineOfBearing" | "line_of_bearing" => Ok(GeneratedField::LineOfBearing),
                            "fixed" => Ok(GeneratedField::Fixed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Signal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Signal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Signal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bandwidth_hz__ = None;
                let mut signal_to_noise_ratio__ = None;
                let mut emitter_notations__ = None;
                let mut pulse_width_s__ = None;
                let mut pulse_repetition_interval__ = None;
                let mut scan_characteristics__ = None;
                let mut frequency_measurement__ = None;
                let mut report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BandwidthHz => {
                            if bandwidth_hz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandwidthHz"));
                            }
                            bandwidth_hz__ = map_.next_value()?;
                        }
                        GeneratedField::SignalToNoiseRatio => {
                            if signal_to_noise_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signalToNoiseRatio"));
                            }
                            signal_to_noise_ratio__ = map_.next_value()?;
                        }
                        GeneratedField::EmitterNotations => {
                            if emitter_notations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitterNotations"));
                            }
                            emitter_notations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PulseWidthS => {
                            if pulse_width_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pulseWidthS"));
                            }
                            pulse_width_s__ = map_.next_value()?;
                        }
                        GeneratedField::PulseRepetitionInterval => {
                            if pulse_repetition_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pulseRepetitionInterval"));
                            }
                            pulse_repetition_interval__ = map_.next_value()?;
                        }
                        GeneratedField::ScanCharacteristics => {
                            if scan_characteristics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scanCharacteristics"));
                            }
                            scan_characteristics__ = map_.next_value()?;
                        }
                        GeneratedField::FrequencyCenter => {
                            if frequency_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frequencyCenter"));
                            }
                            frequency_measurement__ = map_.next_value::<::std::option::Option<_>>()?.map(signal::FrequencyMeasurement::FrequencyCenter)
;
                        }
                        GeneratedField::FrequencyRange => {
                            if frequency_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frequencyRange"));
                            }
                            frequency_measurement__ = map_.next_value::<::std::option::Option<_>>()?.map(signal::FrequencyMeasurement::FrequencyRange)
;
                        }
                        GeneratedField::LineOfBearing => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineOfBearing"));
                            }
                            report__ = map_.next_value::<::std::option::Option<_>>()?.map(signal::Report::LineOfBearing)
;
                        }
                        GeneratedField::Fixed => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixed"));
                            }
                            report__ = map_.next_value::<::std::option::Option<_>>()?.map(signal::Report::Fixed)
;
                        }
                    }
                }
                Ok(Signal {
                    bandwidth_hz: bandwidth_hz__,
                    signal_to_noise_ratio: signal_to_noise_ratio__,
                    emitter_notations: emitter_notations__.unwrap_or_default(),
                    pulse_width_s: pulse_width_s__,
                    pulse_repetition_interval: pulse_repetition_interval__,
                    scan_characteristics: scan_characteristics__,
                    frequency_measurement: frequency_measurement__,
                    report: report__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Signal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Statement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Statement", len)?;
        if let Some(v) = self.operation.as_ref() {
            match v {
                statement::Operation::And(v) => {
                    struct_ser.serialize_field("and", v)?;
                }
                statement::Operation::Or(v) => {
                    struct_ser.serialize_field("or", v)?;
                }
                statement::Operation::Not(v) => {
                    struct_ser.serialize_field("not", v)?;
                }
                statement::Operation::List(v) => {
                    struct_ser.serialize_field("list", v)?;
                }
                statement::Operation::Predicate(v) => {
                    struct_ser.serialize_field("predicate", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Statement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "and",
            "or",
            "not",
            "list",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            And,
            Or,
            Not,
            List,
            Predicate,
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
                            "and" => Ok(GeneratedField::And),
                            "or" => Ok(GeneratedField::Or),
                            "not" => Ok(GeneratedField::Not),
                            "list" => Ok(GeneratedField::List),
                            "predicate" => Ok(GeneratedField::Predicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Statement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Statement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Statement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::And => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("and"));
                            }
                            operation__ = map_.next_value::<::std::option::Option<_>>()?.map(statement::Operation::And)
;
                        }
                        GeneratedField::Or => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("or"));
                            }
                            operation__ = map_.next_value::<::std::option::Option<_>>()?.map(statement::Operation::Or)
;
                        }
                        GeneratedField::Not => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("not"));
                            }
                            operation__ = map_.next_value::<::std::option::Option<_>>()?.map(statement::Operation::Not)
;
                        }
                        GeneratedField::List => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("list"));
                            }
                            operation__ = map_.next_value::<::std::option::Option<_>>()?.map(statement::Operation::List)
;
                        }
                        GeneratedField::Predicate => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            operation__ = map_.next_value::<::std::option::Option<_>>()?.map(statement::Operation::Predicate)
;
                        }
                    }
                }
                Ok(Statement {
                    operation: operation__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Statement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatementSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.statements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.StatementSet", len)?;
        if !self.statements.is_empty() {
            struct_ser.serialize_field("statements", &self.statements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatementSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "statements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Statements,
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
                            "statements" => Ok(GeneratedField::Statements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatementSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.StatementSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatementSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut statements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Statements => {
                            if statements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statements"));
                            }
                            statements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StatementSet {
                    statements: statements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.StatementSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.platform_activity.is_empty() {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Status", len)?;
        if !self.platform_activity.is_empty() {
            struct_ser.serialize_field("platformActivity", &self.platform_activity)?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "platform_activity",
            "platformActivity",
            "role",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlatformActivity,
            Role,
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
                            "platformActivity" | "platform_activity" => Ok(GeneratedField::PlatformActivity),
                            "role" => Ok(GeneratedField::Role),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Status")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Status, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut platform_activity__ = None;
                let mut role__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlatformActivity => {
                            if platform_activity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformActivity"));
                            }
                            platform_activity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Status {
                    platform_activity: platform_activity__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Status", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamEntityComponentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.components_to_include.is_empty() {
            len += 1;
        }
        if self.include_all_components {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.rate_limit.is_some() {
            len += 1;
        }
        if self.heartbeat_period_millis != 0 {
            len += 1;
        }
        if self.preexisting_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.StreamEntityComponentsRequest", len)?;
        if !self.components_to_include.is_empty() {
            struct_ser.serialize_field("componentsToInclude", &self.components_to_include)?;
        }
        if self.include_all_components {
            struct_ser.serialize_field("includeAllComponents", &self.include_all_components)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.rate_limit.as_ref() {
            struct_ser.serialize_field("rateLimit", v)?;
        }
        if self.heartbeat_period_millis != 0 {
            struct_ser.serialize_field("heartbeatPeriodMillis", &self.heartbeat_period_millis)?;
        }
        if self.preexisting_only {
            struct_ser.serialize_field("preexistingOnly", &self.preexisting_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamEntityComponentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "components_to_include",
            "componentsToInclude",
            "include_all_components",
            "includeAllComponents",
            "filter",
            "rate_limit",
            "rateLimit",
            "heartbeat_period_millis",
            "heartbeatPeriodMillis",
            "preexisting_only",
            "preexistingOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ComponentsToInclude,
            IncludeAllComponents,
            Filter,
            RateLimit,
            HeartbeatPeriodMillis,
            PreexistingOnly,
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
                            "componentsToInclude" | "components_to_include" => Ok(GeneratedField::ComponentsToInclude),
                            "includeAllComponents" | "include_all_components" => Ok(GeneratedField::IncludeAllComponents),
                            "filter" => Ok(GeneratedField::Filter),
                            "rateLimit" | "rate_limit" => Ok(GeneratedField::RateLimit),
                            "heartbeatPeriodMillis" | "heartbeat_period_millis" => Ok(GeneratedField::HeartbeatPeriodMillis),
                            "preexistingOnly" | "preexisting_only" => Ok(GeneratedField::PreexistingOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamEntityComponentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.StreamEntityComponentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamEntityComponentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut components_to_include__ = None;
                let mut include_all_components__ = None;
                let mut filter__ = None;
                let mut rate_limit__ = None;
                let mut heartbeat_period_millis__ = None;
                let mut preexisting_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ComponentsToInclude => {
                            if components_to_include__.is_some() {
                                return Err(serde::de::Error::duplicate_field("componentsToInclude"));
                            }
                            components_to_include__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeAllComponents => {
                            if include_all_components__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeAllComponents"));
                            }
                            include_all_components__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::RateLimit => {
                            if rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimit"));
                            }
                            rate_limit__ = map_.next_value()?;
                        }
                        GeneratedField::HeartbeatPeriodMillis => {
                            if heartbeat_period_millis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeatPeriodMillis"));
                            }
                            heartbeat_period_millis__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PreexistingOnly => {
                            if preexisting_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preexistingOnly"));
                            }
                            preexisting_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamEntityComponentsRequest {
                    components_to_include: components_to_include__.unwrap_or_default(),
                    include_all_components: include_all_components__.unwrap_or_default(),
                    filter: filter__,
                    rate_limit: rate_limit__,
                    heartbeat_period_millis: heartbeat_period_millis__.unwrap_or_default(),
                    preexisting_only: preexisting_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.StreamEntityComponentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamEntityComponentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_event.is_some() {
            len += 1;
        }
        if self.heartbeat.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.StreamEntityComponentsResponse", len)?;
        if let Some(v) = self.entity_event.as_ref() {
            struct_ser.serialize_field("entityEvent", v)?;
        }
        if let Some(v) = self.heartbeat.as_ref() {
            struct_ser.serialize_field("heartbeat", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamEntityComponentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_event",
            "entityEvent",
            "heartbeat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityEvent,
            Heartbeat,
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
                            "entityEvent" | "entity_event" => Ok(GeneratedField::EntityEvent),
                            "heartbeat" => Ok(GeneratedField::Heartbeat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamEntityComponentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.StreamEntityComponentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamEntityComponentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_event__ = None;
                let mut heartbeat__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityEvent => {
                            if entity_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityEvent"));
                            }
                            entity_event__ = map_.next_value()?;
                        }
                        GeneratedField::Heartbeat => {
                            if heartbeat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeat"));
                            }
                            heartbeat__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamEntityComponentsResponse {
                    entity_event: entity_event__,
                    heartbeat: heartbeat__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.StreamEntityComponentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.StringType", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.StringType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StringType {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.StringType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Supplies {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fuel.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Supplies", len)?;
        if !self.fuel.is_empty() {
            struct_ser.serialize_field("fuel", &self.fuel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Supplies {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fuel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fuel,
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
                            "fuel" => Ok(GeneratedField::Fuel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Supplies;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Supplies")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Supplies, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fuel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fuel => {
                            if fuel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fuel"));
                            }
                            fuel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Supplies {
                    fuel: fuel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Supplies", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TMat3 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mxx != 0. {
            len += 1;
        }
        if self.mxy != 0. {
            len += 1;
        }
        if self.mxz != 0. {
            len += 1;
        }
        if self.myy != 0. {
            len += 1;
        }
        if self.myz != 0. {
            len += 1;
        }
        if self.mzz != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.TMat3", len)?;
        if self.mxx != 0. {
            struct_ser.serialize_field("mxx", &self.mxx)?;
        }
        if self.mxy != 0. {
            struct_ser.serialize_field("mxy", &self.mxy)?;
        }
        if self.mxz != 0. {
            struct_ser.serialize_field("mxz", &self.mxz)?;
        }
        if self.myy != 0. {
            struct_ser.serialize_field("myy", &self.myy)?;
        }
        if self.myz != 0. {
            struct_ser.serialize_field("myz", &self.myz)?;
        }
        if self.mzz != 0. {
            struct_ser.serialize_field("mzz", &self.mzz)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TMat3 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mxx",
            "mxy",
            "mxz",
            "myy",
            "myz",
            "mzz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mxx,
            Mxy,
            Mxz,
            Myy,
            Myz,
            Mzz,
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
                            "mxx" => Ok(GeneratedField::Mxx),
                            "mxy" => Ok(GeneratedField::Mxy),
                            "mxz" => Ok(GeneratedField::Mxz),
                            "myy" => Ok(GeneratedField::Myy),
                            "myz" => Ok(GeneratedField::Myz),
                            "mzz" => Ok(GeneratedField::Mzz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TMat3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.TMat3")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TMat3, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mxx__ = None;
                let mut mxy__ = None;
                let mut mxz__ = None;
                let mut myy__ = None;
                let mut myz__ = None;
                let mut mzz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mxx => {
                            if mxx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mxx"));
                            }
                            mxx__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mxy => {
                            if mxy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mxy"));
                            }
                            mxy__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mxz => {
                            if mxz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mxz"));
                            }
                            mxz__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Myy => {
                            if myy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("myy"));
                            }
                            myy__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Myz => {
                            if myz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("myz"));
                            }
                            myz__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mzz => {
                            if mzz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mzz"));
                            }
                            mzz__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TMat3 {
                    mxx: mxx__.unwrap_or_default(),
                    mxy: mxy__.unwrap_or_default(),
                    mxz: mxz__.unwrap_or_default(),
                    myy: myy__.unwrap_or_default(),
                    myz: myz__.unwrap_or_default(),
                    mzz: mzz__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.TMat3", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetPriority {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.high_value_target.is_some() {
            len += 1;
        }
        if self.threat.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.TargetPriority", len)?;
        if let Some(v) = self.high_value_target.as_ref() {
            struct_ser.serialize_field("highValueTarget", v)?;
        }
        if let Some(v) = self.threat.as_ref() {
            struct_ser.serialize_field("threat", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetPriority {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "high_value_target",
            "highValueTarget",
            "threat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HighValueTarget,
            Threat,
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
                            "highValueTarget" | "high_value_target" => Ok(GeneratedField::HighValueTarget),
                            "threat" => Ok(GeneratedField::Threat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetPriority;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.TargetPriority")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetPriority, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut high_value_target__ = None;
                let mut threat__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HighValueTarget => {
                            if high_value_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("highValueTarget"));
                            }
                            high_value_target__ = map_.next_value()?;
                        }
                        GeneratedField::Threat => {
                            if threat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threat"));
                            }
                            threat__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TargetPriority {
                    high_value_target: high_value_target__,
                    threat: threat__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.TargetPriority", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Template {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "TEMPLATE_INVALID",
            Self::Track => "TEMPLATE_TRACK",
            Self::SensorPointOfInterest => "TEMPLATE_SENSOR_POINT_OF_INTEREST",
            Self::Asset => "TEMPLATE_ASSET",
            Self::Geo => "TEMPLATE_GEO",
            Self::SignalOfInterest => "TEMPLATE_SIGNAL_OF_INTEREST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Template {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEMPLATE_INVALID",
            "TEMPLATE_TRACK",
            "TEMPLATE_SENSOR_POINT_OF_INTEREST",
            "TEMPLATE_ASSET",
            "TEMPLATE_GEO",
            "TEMPLATE_SIGNAL_OF_INTEREST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Template;

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
                    "TEMPLATE_INVALID" => Ok(Template::Invalid),
                    "TEMPLATE_TRACK" => Ok(Template::Track),
                    "TEMPLATE_SENSOR_POINT_OF_INTEREST" => Ok(Template::SensorPointOfInterest),
                    "TEMPLATE_ASSET" => Ok(Template::Asset),
                    "TEMPLATE_GEO" => Ok(Template::Geo),
                    "TEMPLATE_SIGNAL_OF_INTEREST" => Ok(Template::SignalOfInterest),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Threat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_threat {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Threat", len)?;
        if self.is_threat {
            struct_ser.serialize_field("isThreat", &self.is_threat)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Threat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_threat",
            "isThreat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsThreat,
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
                            "isThreat" | "is_threat" => Ok(GeneratedField::IsThreat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Threat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Threat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Threat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_threat__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsThreat => {
                            if is_threat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isThreat"));
                            }
                            is_threat__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Threat {
                    is_threat: is_threat__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Threat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.TimestampType", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.TimestampType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TimestampType {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.TimestampType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tracked {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.track_quality_wrapper.is_some() {
            len += 1;
        }
        if self.sensor_hits.is_some() {
            len += 1;
        }
        if self.number_of_objects.is_some() {
            len += 1;
        }
        if self.radar_cross_section.is_some() {
            len += 1;
        }
        if self.last_measurement_time.is_some() {
            len += 1;
        }
        if self.line_of_bearing.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Tracked", len)?;
        if let Some(v) = self.track_quality_wrapper.as_ref() {
            struct_ser.serialize_field("trackQualityWrapper", v)?;
        }
        if let Some(v) = self.sensor_hits.as_ref() {
            struct_ser.serialize_field("sensorHits", v)?;
        }
        if let Some(v) = self.number_of_objects.as_ref() {
            struct_ser.serialize_field("numberOfObjects", v)?;
        }
        if let Some(v) = self.radar_cross_section.as_ref() {
            struct_ser.serialize_field("radarCrossSection", v)?;
        }
        if let Some(v) = self.last_measurement_time.as_ref() {
            struct_ser.serialize_field("lastMeasurementTime", v)?;
        }
        if let Some(v) = self.line_of_bearing.as_ref() {
            struct_ser.serialize_field("lineOfBearing", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tracked {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "track_quality_wrapper",
            "trackQualityWrapper",
            "sensor_hits",
            "sensorHits",
            "number_of_objects",
            "numberOfObjects",
            "radar_cross_section",
            "radarCrossSection",
            "last_measurement_time",
            "lastMeasurementTime",
            "line_of_bearing",
            "lineOfBearing",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrackQualityWrapper,
            SensorHits,
            NumberOfObjects,
            RadarCrossSection,
            LastMeasurementTime,
            LineOfBearing,
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
                            "trackQualityWrapper" | "track_quality_wrapper" => Ok(GeneratedField::TrackQualityWrapper),
                            "sensorHits" | "sensor_hits" => Ok(GeneratedField::SensorHits),
                            "numberOfObjects" | "number_of_objects" => Ok(GeneratedField::NumberOfObjects),
                            "radarCrossSection" | "radar_cross_section" => Ok(GeneratedField::RadarCrossSection),
                            "lastMeasurementTime" | "last_measurement_time" => Ok(GeneratedField::LastMeasurementTime),
                            "lineOfBearing" | "line_of_bearing" => Ok(GeneratedField::LineOfBearing),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tracked;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Tracked")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tracked, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut track_quality_wrapper__ = None;
                let mut sensor_hits__ = None;
                let mut number_of_objects__ = None;
                let mut radar_cross_section__ = None;
                let mut last_measurement_time__ = None;
                let mut line_of_bearing__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrackQualityWrapper => {
                            if track_quality_wrapper__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackQualityWrapper"));
                            }
                            track_quality_wrapper__ = map_.next_value()?;
                        }
                        GeneratedField::SensorHits => {
                            if sensor_hits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensorHits"));
                            }
                            sensor_hits__ = map_.next_value()?;
                        }
                        GeneratedField::NumberOfObjects => {
                            if number_of_objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfObjects"));
                            }
                            number_of_objects__ = map_.next_value()?;
                        }
                        GeneratedField::RadarCrossSection => {
                            if radar_cross_section__.is_some() {
                                return Err(serde::de::Error::duplicate_field("radarCrossSection"));
                            }
                            radar_cross_section__ = map_.next_value()?;
                        }
                        GeneratedField::LastMeasurementTime => {
                            if last_measurement_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMeasurementTime"));
                            }
                            last_measurement_time__ = map_.next_value()?;
                        }
                        GeneratedField::LineOfBearing => {
                            if line_of_bearing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineOfBearing"));
                            }
                            line_of_bearing__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Tracked {
                    track_quality_wrapper: track_quality_wrapper__,
                    sensor_hits: sensor_hits__,
                    number_of_objects: number_of_objects__,
                    radar_cross_section: radar_cross_section__,
                    last_measurement_time: last_measurement_time__,
                    line_of_bearing: line_of_bearing__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Tracked", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrackedBy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.actively_tracking_sensors.is_some() {
            len += 1;
        }
        if self.last_measurement_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.TrackedBy", len)?;
        if let Some(v) = self.actively_tracking_sensors.as_ref() {
            struct_ser.serialize_field("activelyTrackingSensors", v)?;
        }
        if let Some(v) = self.last_measurement_timestamp.as_ref() {
            struct_ser.serialize_field("lastMeasurementTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrackedBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actively_tracking_sensors",
            "activelyTrackingSensors",
            "last_measurement_timestamp",
            "lastMeasurementTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActivelyTrackingSensors,
            LastMeasurementTimestamp,
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
                            "activelyTrackingSensors" | "actively_tracking_sensors" => Ok(GeneratedField::ActivelyTrackingSensors),
                            "lastMeasurementTimestamp" | "last_measurement_timestamp" => Ok(GeneratedField::LastMeasurementTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrackedBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.TrackedBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TrackedBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actively_tracking_sensors__ = None;
                let mut last_measurement_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActivelyTrackingSensors => {
                            if actively_tracking_sensors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activelyTrackingSensors"));
                            }
                            actively_tracking_sensors__ = map_.next_value()?;
                        }
                        GeneratedField::LastMeasurementTimestamp => {
                            if last_measurement_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMeasurementTimestamp"));
                            }
                            last_measurement_timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TrackedBy {
                    actively_tracking_sensors: actively_tracking_sensors__,
                    last_measurement_timestamp: last_measurement_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.TrackedBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransponderCodes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode1 != 0 {
            len += 1;
        }
        if self.mode2 != 0 {
            len += 1;
        }
        if self.mode3 != 0 {
            len += 1;
        }
        if self.mode4_interrogation_response != 0 {
            len += 1;
        }
        if self.mode5.is_some() {
            len += 1;
        }
        if self.mode_s.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.TransponderCodes", len)?;
        if self.mode1 != 0 {
            struct_ser.serialize_field("mode1", &self.mode1)?;
        }
        if self.mode2 != 0 {
            struct_ser.serialize_field("mode2", &self.mode2)?;
        }
        if self.mode3 != 0 {
            struct_ser.serialize_field("mode3", &self.mode3)?;
        }
        if self.mode4_interrogation_response != 0 {
            let v = InterrogationResponse::try_from(self.mode4_interrogation_response)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode4_interrogation_response)))?;
            struct_ser.serialize_field("mode4InterrogationResponse", &v)?;
        }
        if let Some(v) = self.mode5.as_ref() {
            struct_ser.serialize_field("mode5", v)?;
        }
        if let Some(v) = self.mode_s.as_ref() {
            struct_ser.serialize_field("modeS", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransponderCodes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mode1",
            "mode2",
            "mode3",
            "mode4_interrogation_response",
            "mode4InterrogationResponse",
            "mode5",
            "mode_s",
            "modeS",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode1,
            Mode2,
            Mode3,
            Mode4InterrogationResponse,
            Mode5,
            ModeS,
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
                            "mode1" => Ok(GeneratedField::Mode1),
                            "mode2" => Ok(GeneratedField::Mode2),
                            "mode3" => Ok(GeneratedField::Mode3),
                            "mode4InterrogationResponse" | "mode4_interrogation_response" => Ok(GeneratedField::Mode4InterrogationResponse),
                            "mode5" => Ok(GeneratedField::Mode5),
                            "modeS" | "mode_s" => Ok(GeneratedField::ModeS),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransponderCodes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.TransponderCodes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransponderCodes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mode1__ = None;
                let mut mode2__ = None;
                let mut mode3__ = None;
                let mut mode4_interrogation_response__ = None;
                let mut mode5__ = None;
                let mut mode_s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mode1 => {
                            if mode1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode1"));
                            }
                            mode1__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mode2 => {
                            if mode2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode2"));
                            }
                            mode2__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mode3 => {
                            if mode3__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode3"));
                            }
                            mode3__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mode4InterrogationResponse => {
                            if mode4_interrogation_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode4InterrogationResponse"));
                            }
                            mode4_interrogation_response__ = Some(map_.next_value::<InterrogationResponse>()? as i32);
                        }
                        GeneratedField::Mode5 => {
                            if mode5__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode5"));
                            }
                            mode5__ = map_.next_value()?;
                        }
                        GeneratedField::ModeS => {
                            if mode_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modeS"));
                            }
                            mode_s__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TransponderCodes {
                    mode1: mode1__.unwrap_or_default(),
                    mode2: mode2__.unwrap_or_default(),
                    mode3: mode3__.unwrap_or_default(),
                    mode4_interrogation_response: mode4_interrogation_response__.unwrap_or_default(),
                    mode5: mode5__,
                    mode_s: mode_s__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.TransponderCodes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UInt32Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lower_bound != 0 {
            len += 1;
        }
        if self.upper_bound != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.UInt32Range", len)?;
        if self.lower_bound != 0 {
            struct_ser.serialize_field("lowerBound", &self.lower_bound)?;
        }
        if self.upper_bound != 0 {
            struct_ser.serialize_field("upperBound", &self.upper_bound)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UInt32Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lower_bound",
            "lowerBound",
            "upper_bound",
            "upperBound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LowerBound,
            UpperBound,
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
                            "lowerBound" | "lower_bound" => Ok(GeneratedField::LowerBound),
                            "upperBound" | "upper_bound" => Ok(GeneratedField::UpperBound),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UInt32Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.UInt32Range")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UInt32Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lower_bound__ = None;
                let mut upper_bound__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LowerBound => {
                            if lower_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowerBound"));
                            }
                            lower_bound__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpperBound => {
                            if upper_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperBound"));
                            }
                            upper_bound__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UInt32Range {
                    lower_bound: lower_bound__.unwrap_or_default(),
                    upper_bound: upper_bound__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.UInt32Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Value {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.Value", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                value::Type::BooleanType(v) => {
                    struct_ser.serialize_field("booleanType", v)?;
                }
                value::Type::NumericType(v) => {
                    struct_ser.serialize_field("numericType", v)?;
                }
                value::Type::StringType(v) => {
                    struct_ser.serialize_field("stringType", v)?;
                }
                value::Type::EnumType(v) => {
                    struct_ser.serialize_field("enumType", v)?;
                }
                value::Type::TimestampType(v) => {
                    struct_ser.serialize_field("timestampType", v)?;
                }
                value::Type::BoundedShapeType(v) => {
                    struct_ser.serialize_field("boundedShapeType", v)?;
                }
                value::Type::PositionType(v) => {
                    struct_ser.serialize_field("positionType", v)?;
                }
                value::Type::HeadingType(v) => {
                    struct_ser.serialize_field("headingType", v)?;
                }
                value::Type::ListType(v) => {
                    struct_ser.serialize_field("listType", v)?;
                }
                value::Type::RangeType(v) => {
                    struct_ser.serialize_field("rangeType", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "boolean_type",
            "booleanType",
            "numeric_type",
            "numericType",
            "string_type",
            "stringType",
            "enum_type",
            "enumType",
            "timestamp_type",
            "timestampType",
            "bounded_shape_type",
            "boundedShapeType",
            "position_type",
            "positionType",
            "heading_type",
            "headingType",
            "list_type",
            "listType",
            "range_type",
            "rangeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BooleanType,
            NumericType,
            StringType,
            EnumType,
            TimestampType,
            BoundedShapeType,
            PositionType,
            HeadingType,
            ListType,
            RangeType,
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
                            "booleanType" | "boolean_type" => Ok(GeneratedField::BooleanType),
                            "numericType" | "numeric_type" => Ok(GeneratedField::NumericType),
                            "stringType" | "string_type" => Ok(GeneratedField::StringType),
                            "enumType" | "enum_type" => Ok(GeneratedField::EnumType),
                            "timestampType" | "timestamp_type" => Ok(GeneratedField::TimestampType),
                            "boundedShapeType" | "bounded_shape_type" => Ok(GeneratedField::BoundedShapeType),
                            "positionType" | "position_type" => Ok(GeneratedField::PositionType),
                            "headingType" | "heading_type" => Ok(GeneratedField::HeadingType),
                            "listType" | "list_type" => Ok(GeneratedField::ListType),
                            "rangeType" | "range_type" => Ok(GeneratedField::RangeType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.Value")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BooleanType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::BooleanType)
;
                        }
                        GeneratedField::NumericType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numericType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::NumericType)
;
                        }
                        GeneratedField::StringType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::StringType)
;
                        }
                        GeneratedField::EnumType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::EnumType)
;
                        }
                        GeneratedField::TimestampType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::TimestampType)
;
                        }
                        GeneratedField::BoundedShapeType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boundedShapeType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::BoundedShapeType)
;
                        }
                        GeneratedField::PositionType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::PositionType)
;
                        }
                        GeneratedField::HeadingType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headingType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::HeadingType)
;
                        }
                        GeneratedField::ListType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::ListType)
;
                        }
                        GeneratedField::RangeType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rangeType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(value::Type::RangeType)
;
                        }
                    }
                }
                Ok(Value {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.Value", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VisualDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.range_rings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.VisualDetails", len)?;
        if let Some(v) = self.range_rings.as_ref() {
            struct_ser.serialize_field("rangeRings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VisualDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range_rings",
            "rangeRings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RangeRings,
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
                            "rangeRings" | "range_rings" => Ok(GeneratedField::RangeRings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VisualDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.VisualDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VisualDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range_rings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RangeRings => {
                            if range_rings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rangeRings"));
                            }
                            range_rings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VisualDetails {
                    range_rings: range_rings__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.VisualDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WithinComparison {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entitymanager.v1.WithinComparison", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WithinComparison {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WithinComparison;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entitymanager.v1.WithinComparison")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WithinComparison, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WithinComparison {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entitymanager.v1.WithinComparison", FIELDS, GeneratedVisitor)
    }
}
