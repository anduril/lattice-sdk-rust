// @generated
impl serde::Serialize for BackfillUpdate {
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
        if self.time_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.BackfillUpdate", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if let Some(v) = self.time_range.as_ref() {
            struct_ser.serialize_field("timeRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackfillUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "time_range",
            "timeRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            TimeRange,
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
                            "timeRange" | "time_range" => Ok(GeneratedField::TimeRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BackfillUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.BackfillUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BackfillUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut time_range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeRange => {
                            if time_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            time_range__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BackfillUpdate {
                    entity_id: entity_id__.unwrap_or_default(),
                    time_range: time_range__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.BackfillUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompositePage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.snapshots.is_empty() {
            len += 1;
        }
        if !self.trails.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.CompositePage", len)?;
        if !self.snapshots.is_empty() {
            struct_ser.serialize_field("snapshots", &self.snapshots)?;
        }
        if !self.trails.is_empty() {
            struct_ser.serialize_field("trails", &self.trails)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompositePage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshots",
            "trails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snapshots,
            Trails,
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
                            "snapshots" => Ok(GeneratedField::Snapshots),
                            "trails" => Ok(GeneratedField::Trails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompositePage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.CompositePage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CompositePage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshots__ = None;
                let mut trails__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snapshots => {
                            if snapshots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshots"));
                            }
                            snapshots__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Trails => {
                            if trails__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trails"));
                            }
                            trails__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CompositePage {
                    snapshots: snapshots__.unwrap_or_default(),
                    trails: trails__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.CompositePage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Downsample {
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
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.Downsample", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                downsample::Type::DownsampleDuration(v) => {
                    struct_ser.serialize_field("downsampleDuration", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Downsample {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "downsample_duration",
            "downsampleDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownsampleDuration,
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
                            "downsampleDuration" | "downsample_duration" => Ok(GeneratedField::DownsampleDuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Downsample;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.Downsample")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Downsample, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownsampleDuration => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downsampleDuration"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(downsample::Type::DownsampleDuration)
;
                        }
                    }
                }
                Ok(Downsample {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.Downsample", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownsampleDuration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.duration_ms != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.DownsampleDuration", len)?;
        if self.duration_ms != 0 {
            struct_ser.serialize_field("durationMs", &self.duration_ms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownsampleDuration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "duration_ms",
            "durationMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DurationMs,
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
                            "durationMs" | "duration_ms" => Ok(GeneratedField::DurationMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownsampleDuration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.DownsampleDuration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DownsampleDuration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut duration_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DurationMs => {
                            if duration_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationMs"));
                            }
                            duration_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DownsampleDuration {
                    duration_ms: duration_ms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.DownsampleDuration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityChangeEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.from.is_some() {
            len += 1;
        }
        if self.to.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.EntityChangeEvent", len)?;
        if let Some(v) = self.from.as_ref() {
            struct_ser.serialize_field("from", v)?;
        }
        if let Some(v) = self.to.as_ref() {
            struct_ser.serialize_field("to", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityChangeEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
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
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityChangeEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.EntityChangeEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityChangeEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = map_.next_value()?;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EntityChangeEvent {
                    from: from__,
                    to: to__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.EntityChangeEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistoryPage {
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
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.HistoryPage", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                history_page::Type::TrailPage(v) => {
                    struct_ser.serialize_field("trailPage", v)?;
                }
                history_page::Type::SnapshotPage(v) => {
                    struct_ser.serialize_field("snapshotPage", v)?;
                }
                history_page::Type::CompositePage(v) => {
                    struct_ser.serialize_field("compositePage", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HistoryPage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trail_page",
            "trailPage",
            "snapshot_page",
            "snapshotPage",
            "composite_page",
            "compositePage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrailPage,
            SnapshotPage,
            CompositePage,
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
                            "trailPage" | "trail_page" => Ok(GeneratedField::TrailPage),
                            "snapshotPage" | "snapshot_page" => Ok(GeneratedField::SnapshotPage),
                            "compositePage" | "composite_page" => Ok(GeneratedField::CompositePage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistoryPage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.HistoryPage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HistoryPage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrailPage => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailPage"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(history_page::Type::TrailPage)
;
                        }
                        GeneratedField::SnapshotPage => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotPage"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(history_page::Type::SnapshotPage)
;
                        }
                        GeneratedField::CompositePage => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compositePage"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(history_page::Type::CompositePage)
;
                        }
                    }
                }
                Ok(HistoryPage {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.HistoryPage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistoryPageToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query_id.is_empty() {
            len += 1;
        }
        if self.page_num != 0 {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.storage_system != 0 {
            len += 1;
        }
        if !self.start_time.is_empty() {
            len += 1;
        }
        if self.is_complete {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.HistoryPageToken", len)?;
        if !self.query_id.is_empty() {
            struct_ser.serialize_field("queryId", &self.query_id)?;
        }
        if self.page_num != 0 {
            struct_ser.serialize_field("pageNum", &self.page_num)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if self.storage_system != 0 {
            let v = StorageSystem::try_from(self.storage_system)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.storage_system)))?;
            struct_ser.serialize_field("storageSystem", &v)?;
        }
        if !self.start_time.is_empty() {
            struct_ser.serialize_field("startTime", &self.start_time)?;
        }
        if self.is_complete {
            struct_ser.serialize_field("isComplete", &self.is_complete)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HistoryPageToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query_id",
            "queryId",
            "page_num",
            "pageNum",
            "entity_id",
            "entityId",
            "storage_system",
            "storageSystem",
            "start_time",
            "startTime",
            "is_complete",
            "isComplete",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueryId,
            PageNum,
            EntityId,
            StorageSystem,
            StartTime,
            IsComplete,
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
                            "queryId" | "query_id" => Ok(GeneratedField::QueryId),
                            "pageNum" | "page_num" => Ok(GeneratedField::PageNum),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "storageSystem" | "storage_system" => Ok(GeneratedField::StorageSystem),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "isComplete" | "is_complete" => Ok(GeneratedField::IsComplete),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistoryPageToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.HistoryPageToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HistoryPageToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query_id__ = None;
                let mut page_num__ = None;
                let mut entity_id__ = None;
                let mut storage_system__ = None;
                let mut start_time__ = None;
                let mut is_complete__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueryId => {
                            if query_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryId"));
                            }
                            query_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageNum => {
                            if page_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageNum"));
                            }
                            page_num__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageSystem => {
                            if storage_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageSystem"));
                            }
                            storage_system__ = Some(map_.next_value::<StorageSystem>()? as i32);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsComplete => {
                            if is_complete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isComplete"));
                            }
                            is_complete__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HistoryPageToken {
                    query_id: query_id__.unwrap_or_default(),
                    page_num: page_num__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    storage_system: storage_system__.unwrap_or_default(),
                    start_time: start_time__.unwrap_or_default(),
                    is_complete: is_complete__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.HistoryPageToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistoryQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.history_type != 0 {
            len += 1;
        }
        if self.time_range.is_some() {
            len += 1;
        }
        if self.downsample.is_some() {
            len += 1;
        }
        if self.statement.is_some() {
            len += 1;
        }
        if !self.entity_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.HistoryQuery", len)?;
        if self.history_type != 0 {
            let v = HistoryType::try_from(self.history_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.history_type)))?;
            struct_ser.serialize_field("historyType", &v)?;
        }
        if let Some(v) = self.time_range.as_ref() {
            struct_ser.serialize_field("timeRange", v)?;
        }
        if let Some(v) = self.downsample.as_ref() {
            struct_ser.serialize_field("downsample", v)?;
        }
        if let Some(v) = self.statement.as_ref() {
            struct_ser.serialize_field("statement", v)?;
        }
        if !self.entity_ids.is_empty() {
            struct_ser.serialize_field("entityIds", &self.entity_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HistoryQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "history_type",
            "historyType",
            "time_range",
            "timeRange",
            "downsample",
            "statement",
            "entity_ids",
            "entityIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HistoryType,
            TimeRange,
            Downsample,
            Statement,
            EntityIds,
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
                            "historyType" | "history_type" => Ok(GeneratedField::HistoryType),
                            "timeRange" | "time_range" => Ok(GeneratedField::TimeRange),
                            "downsample" => Ok(GeneratedField::Downsample),
                            "statement" => Ok(GeneratedField::Statement),
                            "entityIds" | "entity_ids" => Ok(GeneratedField::EntityIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistoryQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.HistoryQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HistoryQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut history_type__ = None;
                let mut time_range__ = None;
                let mut downsample__ = None;
                let mut statement__ = None;
                let mut entity_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HistoryType => {
                            if history_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyType"));
                            }
                            history_type__ = Some(map_.next_value::<HistoryType>()? as i32);
                        }
                        GeneratedField::TimeRange => {
                            if time_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            time_range__ = map_.next_value()?;
                        }
                        GeneratedField::Downsample => {
                            if downsample__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downsample"));
                            }
                            downsample__ = map_.next_value()?;
                        }
                        GeneratedField::Statement => {
                            if statement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statement"));
                            }
                            statement__ = map_.next_value()?;
                        }
                        GeneratedField::EntityIds => {
                            if entity_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityIds"));
                            }
                            entity_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HistoryQuery {
                    history_type: history_type__.unwrap_or_default(),
                    time_range: time_range__,
                    downsample: downsample__,
                    statement: statement__,
                    entity_ids: entity_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.HistoryQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HistoryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "HISTORY_TYPE_INVALID",
            Self::Trail => "HISTORY_TYPE_TRAIL",
            Self::Snapshot => "HISTORY_TYPE_SNAPSHOT",
            Self::Composite => "HISTORY_TYPE_COMPOSITE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HistoryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HISTORY_TYPE_INVALID",
            "HISTORY_TYPE_TRAIL",
            "HISTORY_TYPE_SNAPSHOT",
            "HISTORY_TYPE_COMPOSITE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HistoryType;

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
                    "HISTORY_TYPE_INVALID" => Ok(HistoryType::Invalid),
                    "HISTORY_TYPE_TRAIL" => Ok(HistoryType::Trail),
                    "HISTORY_TYPE_SNAPSHOT" => Ok(HistoryType::Snapshot),
                    "HISTORY_TYPE_COMPOSITE" => Ok(HistoryType::Composite),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListHistoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.history_query.is_some() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.history_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.ListHistoryRequest", len)?;
        if let Some(v) = self.history_query.as_ref() {
            struct_ser.serialize_field("historyQuery", v)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if let Some(v) = self.history_page_token.as_ref() {
            struct_ser.serialize_field("historyPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListHistoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "history_query",
            "historyQuery",
            "page_token",
            "pageToken",
            "history_page_token",
            "historyPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HistoryQuery,
            PageToken,
            HistoryPageToken,
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
                            "historyQuery" | "history_query" => Ok(GeneratedField::HistoryQuery),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "historyPageToken" | "history_page_token" => Ok(GeneratedField::HistoryPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListHistoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.ListHistoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListHistoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut history_query__ = None;
                let mut page_token__ = None;
                let mut history_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HistoryQuery => {
                            if history_query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyQuery"));
                            }
                            history_query__ = map_.next_value()?;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistoryPageToken => {
                            if history_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyPageToken"));
                            }
                            history_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListHistoryRequest {
                    history_query: history_query__,
                    page_token: page_token__.unwrap_or_default(),
                    history_page_token: history_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.ListHistoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListHistoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.history_page.is_some() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        if self.next_history_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.ListHistoryResponse", len)?;
        if let Some(v) = self.history_page.as_ref() {
            struct_ser.serialize_field("historyPage", v)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        if let Some(v) = self.next_history_page_token.as_ref() {
            struct_ser.serialize_field("nextHistoryPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListHistoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "history_page",
            "historyPage",
            "next_page_token",
            "nextPageToken",
            "next_history_page_token",
            "nextHistoryPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HistoryPage,
            NextPageToken,
            NextHistoryPageToken,
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
                            "historyPage" | "history_page" => Ok(GeneratedField::HistoryPage),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            "nextHistoryPageToken" | "next_history_page_token" => Ok(GeneratedField::NextHistoryPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListHistoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.ListHistoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListHistoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut history_page__ = None;
                let mut next_page_token__ = None;
                let mut next_history_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HistoryPage => {
                            if history_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyPage"));
                            }
                            history_page__ = map_.next_value()?;
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextHistoryPageToken => {
                            if next_history_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHistoryPageToken"));
                            }
                            next_history_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListHistoryResponse {
                    history_page: history_page__,
                    next_page_token: next_page_token__.unwrap_or_default(),
                    next_history_page_token: next_history_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.ListHistoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Snapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.latest_state.is_some() {
            len += 1;
        }
        if self.original_state.is_some() {
            len += 1;
        }
        if !self.snapshot_events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.Snapshot", len)?;
        if let Some(v) = self.latest_state.as_ref() {
            struct_ser.serialize_field("latestState", v)?;
        }
        if let Some(v) = self.original_state.as_ref() {
            struct_ser.serialize_field("originalState", v)?;
        }
        if !self.snapshot_events.is_empty() {
            struct_ser.serialize_field("snapshotEvents", &self.snapshot_events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Snapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latest_state",
            "latestState",
            "original_state",
            "originalState",
            "snapshot_events",
            "snapshotEvents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LatestState,
            OriginalState,
            SnapshotEvents,
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
                            "latestState" | "latest_state" => Ok(GeneratedField::LatestState),
                            "originalState" | "original_state" => Ok(GeneratedField::OriginalState),
                            "snapshotEvents" | "snapshot_events" => Ok(GeneratedField::SnapshotEvents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.Snapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Snapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut latest_state__ = None;
                let mut original_state__ = None;
                let mut snapshot_events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LatestState => {
                            if latest_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestState"));
                            }
                            latest_state__ = map_.next_value()?;
                        }
                        GeneratedField::OriginalState => {
                            if original_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalState"));
                            }
                            original_state__ = map_.next_value()?;
                        }
                        GeneratedField::SnapshotEvents => {
                            if snapshot_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotEvents"));
                            }
                            snapshot_events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Snapshot {
                    latest_state: latest_state__,
                    original_state: original_state__,
                    snapshot_events: snapshot_events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.Snapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnapshotEvent {
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
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.SnapshotEvent", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                snapshot_event::Type::EntityChangeEvent(v) => {
                    struct_ser.serialize_field("entityChangeEvent", v)?;
                }
                snapshot_event::Type::EntityState(v) => {
                    struct_ser.serialize_field("entityState", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "entity_change_event",
            "entityChangeEvent",
            "entity_state",
            "entityState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            EntityChangeEvent,
            EntityState,
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
                            "entityChangeEvent" | "entity_change_event" => Ok(GeneratedField::EntityChangeEvent),
                            "entityState" | "entity_state" => Ok(GeneratedField::EntityState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnapshotEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.SnapshotEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnapshotEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::EntityChangeEvent => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityChangeEvent"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(snapshot_event::Type::EntityChangeEvent)
;
                        }
                        GeneratedField::EntityState => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityState"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(snapshot_event::Type::EntityState)
;
                        }
                    }
                }
                Ok(SnapshotEvent {
                    timestamp: timestamp__,
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.SnapshotEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnapshotPage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.snapshots.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.SnapshotPage", len)?;
        if !self.snapshots.is_empty() {
            struct_ser.serialize_field("snapshots", &self.snapshots)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotPage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshots",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snapshots,
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
                            "snapshots" => Ok(GeneratedField::Snapshots),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnapshotPage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.SnapshotPage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnapshotPage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshots__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snapshots => {
                            if snapshots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshots"));
                            }
                            snapshots__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SnapshotPage {
                    snapshots: snapshots__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.SnapshotPage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StorageSystem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "STORAGE_SYSTEM_INVALID",
            Self::Persistent => "STORAGE_SYSTEM_PERSISTENT",
            Self::Ephemeral => "STORAGE_SYSTEM_EPHEMERAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StorageSystem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STORAGE_SYSTEM_INVALID",
            "STORAGE_SYSTEM_PERSISTENT",
            "STORAGE_SYSTEM_EPHEMERAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageSystem;

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
                    "STORAGE_SYSTEM_INVALID" => Ok(StorageSystem::Invalid),
                    "STORAGE_SYSTEM_PERSISTENT" => Ok(StorageSystem::Persistent),
                    "STORAGE_SYSTEM_EPHEMERAL" => Ok(StorageSystem::Ephemeral),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamBackfillUpdatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.StreamBackfillUpdatesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamBackfillUpdatesRequest {
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
            type Value = StreamBackfillUpdatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.StreamBackfillUpdatesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamBackfillUpdatesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StreamBackfillUpdatesRequest {
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.StreamBackfillUpdatesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamBackfillUpdatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backfill_updates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.StreamBackfillUpdatesResponse", len)?;
        if !self.backfill_updates.is_empty() {
            struct_ser.serialize_field("backfillUpdates", &self.backfill_updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamBackfillUpdatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "backfill_updates",
            "backfillUpdates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BackfillUpdates,
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
                            "backfillUpdates" | "backfill_updates" => Ok(GeneratedField::BackfillUpdates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamBackfillUpdatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.StreamBackfillUpdatesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamBackfillUpdatesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut backfill_updates__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BackfillUpdates => {
                            if backfill_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backfillUpdates"));
                            }
                            backfill_updates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamBackfillUpdatesResponse {
                    backfill_updates: backfill_updates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.StreamBackfillUpdatesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lower_bound_inc.is_some() {
            len += 1;
        }
        if self.upper_bound_exc.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.TimeRange", len)?;
        if let Some(v) = self.lower_bound_inc.as_ref() {
            struct_ser.serialize_field("lowerBoundInc", v)?;
        }
        if let Some(v) = self.upper_bound_exc.as_ref() {
            struct_ser.serialize_field("upperBoundExc", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lower_bound_inc",
            "lowerBoundInc",
            "upper_bound_exc",
            "upperBoundExc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LowerBoundInc,
            UpperBoundExc,
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
                            "lowerBoundInc" | "lower_bound_inc" => Ok(GeneratedField::LowerBoundInc),
                            "upperBoundExc" | "upper_bound_exc" => Ok(GeneratedField::UpperBoundExc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.TimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lower_bound_inc__ = None;
                let mut upper_bound_exc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LowerBoundInc => {
                            if lower_bound_inc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowerBoundInc"));
                            }
                            lower_bound_inc__ = map_.next_value()?;
                        }
                        GeneratedField::UpperBoundExc => {
                            if upper_bound_exc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperBoundExc"));
                            }
                            upper_bound_exc__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TimeRange {
                    lower_bound_inc: lower_bound_inc__,
                    upper_bound_exc: upper_bound_exc__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.TimeRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Trail {
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
        if !self.trail_points.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.Trail", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.trail_points.is_empty() {
            struct_ser.serialize_field("trailPoints", &self.trail_points)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Trail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "trail_points",
            "trailPoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            TrailPoints,
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
                            "trailPoints" | "trail_points" => Ok(GeneratedField::TrailPoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Trail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.Trail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Trail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut trail_points__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrailPoints => {
                            if trail_points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailPoints"));
                            }
                            trail_points__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Trail {
                    entity_id: entity_id__.unwrap_or_default(),
                    trail_points: trail_points__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.Trail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrailPage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trails.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.TrailPage", len)?;
        if !self.trails.is_empty() {
            struct_ser.serialize_field("trails", &self.trails)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrailPage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Trails,
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
                            "trails" => Ok(GeneratedField::Trails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrailPage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.TrailPage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TrailPage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trails__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Trails => {
                            if trails__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trails"));
                            }
                            trails__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TrailPage {
                    trails: trails__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.TrailPage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrailPoint {
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
        if self.location.is_some() {
            len += 1;
        }
        if self.location_uncertainty.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.entityhistory.v1.TrailPoint", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.location.as_ref() {
            struct_ser.serialize_field("location", v)?;
        }
        if let Some(v) = self.location_uncertainty.as_ref() {
            struct_ser.serialize_field("locationUncertainty", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrailPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "location",
            "location_uncertainty",
            "locationUncertainty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Location,
            LocationUncertainty,
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
                            "location" => Ok(GeneratedField::Location),
                            "locationUncertainty" | "location_uncertainty" => Ok(GeneratedField::LocationUncertainty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrailPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.entityhistory.v1.TrailPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TrailPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut location__ = None;
                let mut location_uncertainty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
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
                    }
                }
                Ok(TrailPoint {
                    timestamp: timestamp__,
                    location: location__,
                    location_uncertainty: location_uncertainty__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.entityhistory.v1.TrailPoint", FIELDS, GeneratedVisitor)
    }
}
