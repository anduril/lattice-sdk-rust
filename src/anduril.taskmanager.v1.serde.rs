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
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Agent", len)?;
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
                formatter.write_str("struct anduril.taskmanager.v1.Agent")
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
        deserializer.deserialize_struct("anduril.taskmanager.v1.Agent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Allocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.active_agents.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Allocation", len)?;
        if !self.active_agents.is_empty() {
            struct_ser.serialize_field("activeAgents", &self.active_agents)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Allocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_agents",
            "activeAgents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveAgents,
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
                            "activeAgents" | "active_agents" => Ok(GeneratedField::ActiveAgents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Allocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Allocation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Allocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_agents__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActiveAgents => {
                            if active_agents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeAgents"));
                            }
                            active_agents__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Allocation {
                    active_agents: active_agents__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Allocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.specification.is_some() {
            len += 1;
        }
        if self.author.is_some() {
            len += 1;
        }
        if self.relations.is_some() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_executed_elsewhere {
            len += 1;
        }
        if !self.task_id.is_empty() {
            len += 1;
        }
        if !self.initial_entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.CreateTaskRequest", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.specification.as_ref() {
            struct_ser.serialize_field("specification", v)?;
        }
        if let Some(v) = self.author.as_ref() {
            struct_ser.serialize_field("author", v)?;
        }
        if let Some(v) = self.relations.as_ref() {
            struct_ser.serialize_field("relations", v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_executed_elsewhere {
            struct_ser.serialize_field("isExecutedElsewhere", &self.is_executed_elsewhere)?;
        }
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if !self.initial_entities.is_empty() {
            struct_ser.serialize_field("initialEntities", &self.initial_entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "specification",
            "author",
            "relations",
            "description",
            "is_executed_elsewhere",
            "isExecutedElsewhere",
            "task_id",
            "taskId",
            "initial_entities",
            "initialEntities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Specification,
            Author,
            Relations,
            Description,
            IsExecutedElsewhere,
            TaskId,
            InitialEntities,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "specification" => Ok(GeneratedField::Specification),
                            "author" => Ok(GeneratedField::Author),
                            "relations" => Ok(GeneratedField::Relations),
                            "description" => Ok(GeneratedField::Description),
                            "isExecutedElsewhere" | "is_executed_elsewhere" => Ok(GeneratedField::IsExecutedElsewhere),
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "initialEntities" | "initial_entities" => Ok(GeneratedField::InitialEntities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.CreateTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut specification__ = None;
                let mut author__ = None;
                let mut relations__ = None;
                let mut description__ = None;
                let mut is_executed_elsewhere__ = None;
                let mut task_id__ = None;
                let mut initial_entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Specification => {
                            if specification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specification"));
                            }
                            specification__ = map_.next_value()?;
                        }
                        GeneratedField::Author => {
                            if author__.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            author__ = map_.next_value()?;
                        }
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsExecutedElsewhere => {
                            if is_executed_elsewhere__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExecutedElsewhere"));
                            }
                            is_executed_elsewhere__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialEntities => {
                            if initial_entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialEntities"));
                            }
                            initial_entities__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTaskRequest {
                    display_name: display_name__.unwrap_or_default(),
                    specification: specification__,
                    author: author__,
                    relations: relations__,
                    description: description__.unwrap_or_default(),
                    is_executed_elsewhere: is_executed_elsewhere__.unwrap_or_default(),
                    task_id: task_id__.unwrap_or_default(),
                    initial_entities: initial_entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.CreateTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaskResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.CreateTaskResponse", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTaskResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
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
                            "task" => Ok(GeneratedField::Task),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTaskResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.CreateTaskResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTaskResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTaskResponse {
                    task: task__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.CreateTaskResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DefinitionUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.DefinitionUpdate", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DefinitionUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
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
                            "task" => Ok(GeneratedField::Task),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DefinitionUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.DefinitionUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DefinitionUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DefinitionUpdate {
                    task: task__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.DefinitionUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ERROR_CODE_INVALID",
            Self::Cancelled => "ERROR_CODE_CANCELLED",
            Self::Rejected => "ERROR_CODE_REJECTED",
            Self::Timeout => "ERROR_CODE_TIMEOUT",
            Self::Failed => "ERROR_CODE_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_CODE_INVALID",
            "ERROR_CODE_CANCELLED",
            "ERROR_CODE_REJECTED",
            "ERROR_CODE_TIMEOUT",
            "ERROR_CODE_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

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
                    "ERROR_CODE_INVALID" => Ok(ErrorCode::Invalid),
                    "ERROR_CODE_CANCELLED" => Ok(ErrorCode::Cancelled),
                    "ERROR_CODE_REJECTED" => Ok(ErrorCode::Rejected),
                    "ERROR_CODE_TIMEOUT" => Ok(ErrorCode::Timeout),
                    "ERROR_CODE_FAILED" => Ok(ErrorCode::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
            Self::Preexisting => "EVENT_TYPE_PREEXISTING",
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
            "EVENT_TYPE_PREEXISTING",
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
                    "EVENT_TYPE_PREEXISTING" => Ok(EventType::Preexisting),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_id.is_empty() {
            len += 1;
        }
        if self.definition_version != 0 {
            len += 1;
        }
        if self.task_view != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.GetTaskRequest", len)?;
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if self.definition_version != 0 {
            struct_ser.serialize_field("definitionVersion", &self.definition_version)?;
        }
        if self.task_view != 0 {
            let v = TaskView::try_from(self.task_view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.task_view)))?;
            struct_ser.serialize_field("taskView", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_id",
            "taskId",
            "definition_version",
            "definitionVersion",
            "task_view",
            "taskView",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskId,
            DefinitionVersion,
            TaskView,
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
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "definitionVersion" | "definition_version" => Ok(GeneratedField::DefinitionVersion),
                            "taskView" | "task_view" => Ok(GeneratedField::TaskView),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.GetTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_id__ = None;
                let mut definition_version__ = None;
                let mut task_view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefinitionVersion => {
                            if definition_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionVersion"));
                            }
                            definition_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TaskView => {
                            if task_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskView"));
                            }
                            task_view__ = Some(map_.next_value::<TaskView>()? as i32);
                        }
                    }
                }
                Ok(GetTaskRequest {
                    task_id: task_id__.unwrap_or_default(),
                    definition_version: definition_version__.unwrap_or_default(),
                    task_view: task_view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.GetTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaskResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.GetTaskResponse", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaskResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
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
                            "task" => Ok(GeneratedField::Task),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaskResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.GetTaskResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaskResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTaskResponse {
                    task: task__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.GetTaskResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Heartbeat", len)?;
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
                formatter.write_str("struct anduril.taskmanager.v1.Heartbeat")
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
        deserializer.deserialize_struct("anduril.taskmanager.v1.Heartbeat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Owner {
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
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Owner", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Owner {
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
            type Value = Owner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Owner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Owner, V::Error>
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
                Ok(Owner {
                    asset_id: asset_id__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Owner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Principal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.on_behalf_of.is_some() {
            len += 1;
        }
        if self.agent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Principal", len)?;
        if let Some(v) = self.on_behalf_of.as_ref() {
            struct_ser.serialize_field("onBehalfOf", v)?;
        }
        if let Some(v) = self.agent.as_ref() {
            match v {
                principal::Agent::System(v) => {
                    struct_ser.serialize_field("system", v)?;
                }
                principal::Agent::User(v) => {
                    struct_ser.serialize_field("user", v)?;
                }
                principal::Agent::Team(v) => {
                    struct_ser.serialize_field("team", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Principal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "on_behalf_of",
            "onBehalfOf",
            "system",
            "user",
            "team",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OnBehalfOf,
            System,
            User,
            Team,
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
                            "onBehalfOf" | "on_behalf_of" => Ok(GeneratedField::OnBehalfOf),
                            "system" => Ok(GeneratedField::System),
                            "user" => Ok(GeneratedField::User),
                            "team" => Ok(GeneratedField::Team),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Principal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Principal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Principal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut on_behalf_of__ = None;
                let mut agent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OnBehalfOf => {
                            if on_behalf_of__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                            }
                            on_behalf_of__ = map_.next_value()?;
                        }
                        GeneratedField::System => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("system"));
                            }
                            agent__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Agent::System)
;
                        }
                        GeneratedField::User => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            agent__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Agent::User)
;
                        }
                        GeneratedField::Team => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("team"));
                            }
                            agent__ = map_.next_value::<::std::option::Option<_>>()?.map(principal::Agent::Team)
;
                        }
                    }
                }
                Ok(Principal {
                    on_behalf_of: on_behalf_of__,
                    agent: agent__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Principal", FIELDS, GeneratedVisitor)
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
        if self.update_per_task_limit_ms != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.RateLimit", len)?;
        if self.update_per_task_limit_ms != 0 {
            struct_ser.serialize_field("updatePerTaskLimitMs", &self.update_per_task_limit_ms)?;
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
            "update_per_task_limit_ms",
            "updatePerTaskLimitMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdatePerTaskLimitMs,
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
                            "updatePerTaskLimitMs" | "update_per_task_limit_ms" => Ok(GeneratedField::UpdatePerTaskLimitMs),
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
                formatter.write_str("struct anduril.taskmanager.v1.RateLimit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update_per_task_limit_ms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpdatePerTaskLimitMs => {
                            if update_per_task_limit_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatePerTaskLimitMs"));
                            }
                            update_per_task_limit_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimit {
                    update_per_task_limit_ms: update_per_task_limit_ms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relations {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.assignee.is_some() {
            len += 1;
        }
        if !self.parent_task_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Relations", len)?;
        if let Some(v) = self.assignee.as_ref() {
            struct_ser.serialize_field("assignee", v)?;
        }
        if !self.parent_task_id.is_empty() {
            struct_ser.serialize_field("parentTaskId", &self.parent_task_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assignee",
            "parent_task_id",
            "parentTaskId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assignee,
            ParentTaskId,
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
                            "assignee" => Ok(GeneratedField::Assignee),
                            "parentTaskId" | "parent_task_id" => Ok(GeneratedField::ParentTaskId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relations;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Relations")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Relations, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assignee__ = None;
                let mut parent_task_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assignee => {
                            if assignee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignee"));
                            }
                            assignee__ = map_.next_value()?;
                        }
                        GeneratedField::ParentTaskId => {
                            if parent_task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentTaskId"));
                            }
                            parent_task_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Relations {
                    assignee: assignee__,
                    parent_task_id: parent_task_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Relations", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Replication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stale_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Replication", len)?;
        if let Some(v) = self.stale_time.as_ref() {
            struct_ser.serialize_field("staleTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Replication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stale_time",
            "staleTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaleTime,
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
                            "staleTime" | "stale_time" => Ok(GeneratedField::StaleTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Replication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Replication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Replication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stale_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StaleTime => {
                            if stale_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staleTime"));
                            }
                            stale_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Replication {
                    stale_time: stale_time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Replication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "STATUS_INVALID",
            Self::Created => "STATUS_CREATED",
            Self::ScheduledInManager => "STATUS_SCHEDULED_IN_MANAGER",
            Self::Sent => "STATUS_SENT",
            Self::MachineReceipt => "STATUS_MACHINE_RECEIPT",
            Self::Ack => "STATUS_ACK",
            Self::Wilco => "STATUS_WILCO",
            Self::Executing => "STATUS_EXECUTING",
            Self::WaitingForUpdate => "STATUS_WAITING_FOR_UPDATE",
            Self::DoneOk => "STATUS_DONE_OK",
            Self::DoneNotOk => "STATUS_DONE_NOT_OK",
            Self::Replaced => "STATUS_REPLACED",
            Self::CancelRequested => "STATUS_CANCEL_REQUESTED",
            Self::CompleteRequested => "STATUS_COMPLETE_REQUESTED",
            Self::VersionRejected => "STATUS_VERSION_REJECTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_INVALID",
            "STATUS_CREATED",
            "STATUS_SCHEDULED_IN_MANAGER",
            "STATUS_SENT",
            "STATUS_MACHINE_RECEIPT",
            "STATUS_ACK",
            "STATUS_WILCO",
            "STATUS_EXECUTING",
            "STATUS_WAITING_FOR_UPDATE",
            "STATUS_DONE_OK",
            "STATUS_DONE_NOT_OK",
            "STATUS_REPLACED",
            "STATUS_CANCEL_REQUESTED",
            "STATUS_COMPLETE_REQUESTED",
            "STATUS_VERSION_REJECTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

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
                    "STATUS_INVALID" => Ok(Status::Invalid),
                    "STATUS_CREATED" => Ok(Status::Created),
                    "STATUS_SCHEDULED_IN_MANAGER" => Ok(Status::ScheduledInManager),
                    "STATUS_SENT" => Ok(Status::Sent),
                    "STATUS_MACHINE_RECEIPT" => Ok(Status::MachineReceipt),
                    "STATUS_ACK" => Ok(Status::Ack),
                    "STATUS_WILCO" => Ok(Status::Wilco),
                    "STATUS_EXECUTING" => Ok(Status::Executing),
                    "STATUS_WAITING_FOR_UPDATE" => Ok(Status::WaitingForUpdate),
                    "STATUS_DONE_OK" => Ok(Status::DoneOk),
                    "STATUS_DONE_NOT_OK" => Ok(Status::DoneNotOk),
                    "STATUS_REPLACED" => Ok(Status::Replaced),
                    "STATUS_CANCEL_REQUESTED" => Ok(Status::CancelRequested),
                    "STATUS_COMPLETE_REQUESTED" => Ok(Status::CompleteRequested),
                    "STATUS_VERSION_REJECTED" => Ok(Status::VersionRejected),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StatusUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.author.is_some() {
            len += 1;
        }
        if self.scheduled_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.StatusUpdate", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.author.as_ref() {
            struct_ser.serialize_field("author", v)?;
        }
        if let Some(v) = self.scheduled_time.as_ref() {
            struct_ser.serialize_field("scheduledTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "status",
            "author",
            "scheduled_time",
            "scheduledTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Status,
            Author,
            ScheduledTime,
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
                            "version" => Ok(GeneratedField::Version),
                            "status" => Ok(GeneratedField::Status),
                            "author" => Ok(GeneratedField::Author),
                            "scheduledTime" | "scheduled_time" => Ok(GeneratedField::ScheduledTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.StatusUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StatusUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut status__ = None;
                let mut author__ = None;
                let mut scheduled_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Author => {
                            if author__.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            author__ = map_.next_value()?;
                        }
                        GeneratedField::ScheduledTime => {
                            if scheduled_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduledTime"));
                            }
                            scheduled_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StatusUpdate {
                    version: version__,
                    status: status__,
                    author: author__,
                    scheduled_time: scheduled_time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.StatusUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTasksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rate_limit.is_some() {
            len += 1;
        }
        if !self.views.is_empty() {
            len += 1;
        }
        if self.heartbeat_period_millis != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.StreamTasksRequest", len)?;
        if let Some(v) = self.rate_limit.as_ref() {
            struct_ser.serialize_field("rateLimit", v)?;
        }
        if !self.views.is_empty() {
            let v = self.views.iter().cloned().map(|v| {
                TaskView::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("views", &v)?;
        }
        if self.heartbeat_period_millis != 0 {
            struct_ser.serialize_field("heartbeatPeriodMillis", &self.heartbeat_period_millis)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTasksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate_limit",
            "rateLimit",
            "views",
            "heartbeat_period_millis",
            "heartbeatPeriodMillis",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RateLimit,
            Views,
            HeartbeatPeriodMillis,
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
                            "rateLimit" | "rate_limit" => Ok(GeneratedField::RateLimit),
                            "views" => Ok(GeneratedField::Views),
                            "heartbeatPeriodMillis" | "heartbeat_period_millis" => Ok(GeneratedField::HeartbeatPeriodMillis),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamTasksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.StreamTasksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamTasksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rate_limit__ = None;
                let mut views__ = None;
                let mut heartbeat_period_millis__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RateLimit => {
                            if rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimit"));
                            }
                            rate_limit__ = map_.next_value()?;
                        }
                        GeneratedField::Views => {
                            if views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("views"));
                            }
                            views__ = Some(map_.next_value::<Vec<TaskView>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::HeartbeatPeriodMillis => {
                            if heartbeat_period_millis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeatPeriodMillis"));
                            }
                            heartbeat_period_millis__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StreamTasksRequest {
                    rate_limit: rate_limit__,
                    views: views__.unwrap_or_default(),
                    heartbeat_period_millis: heartbeat_period_millis__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.StreamTasksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTasksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task_event.is_some() {
            len += 1;
        }
        if self.heartbeat.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.StreamTasksResponse", len)?;
        if let Some(v) = self.task_event.as_ref() {
            struct_ser.serialize_field("taskEvent", v)?;
        }
        if let Some(v) = self.heartbeat.as_ref() {
            struct_ser.serialize_field("heartbeat", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTasksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_event",
            "taskEvent",
            "heartbeat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskEvent,
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
                            "taskEvent" | "task_event" => Ok(GeneratedField::TaskEvent),
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
            type Value = StreamTasksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.StreamTasksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamTasksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_event__ = None;
                let mut heartbeat__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskEvent => {
                            if task_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskEvent"));
                            }
                            task_event__ = map_.next_value()?;
                        }
                        GeneratedField::Heartbeat => {
                            if heartbeat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heartbeat"));
                            }
                            heartbeat__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamTasksResponse {
                    task_event: task_event__,
                    heartbeat: heartbeat__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.StreamTasksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for System {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if self.manages_own_scheduling {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.System", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if self.manages_own_scheduling {
            struct_ser.serialize_field("managesOwnScheduling", &self.manages_own_scheduling)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for System {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "entity_id",
            "entityId",
            "asset_id",
            "assetId",
            "manages_own_scheduling",
            "managesOwnScheduling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            EntityId,
            AssetId,
            ManagesOwnScheduling,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "managesOwnScheduling" | "manages_own_scheduling" => Ok(GeneratedField::ManagesOwnScheduling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = System;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.System")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<System, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut entity_id__ = None;
                let mut asset_id__ = None;
                let mut manages_own_scheduling__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ManagesOwnScheduling => {
                            if manages_own_scheduling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("managesOwnScheduling"));
                            }
                            manages_own_scheduling__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(System {
                    service_name: service_name__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    manages_own_scheduling: manages_own_scheduling__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.System", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Task {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.specification.is_some() {
            len += 1;
        }
        if self.created_by.is_some() {
            len += 1;
        }
        if self.last_updated_by.is_some() {
            len += 1;
        }
        if self.last_update_time.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.scheduled_time.is_some() {
            len += 1;
        }
        if self.relations.is_some() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_executed_elsewhere {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.replication.is_some() {
            len += 1;
        }
        if !self.initial_entities.is_empty() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Task", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.specification.as_ref() {
            struct_ser.serialize_field("specification", v)?;
        }
        if let Some(v) = self.created_by.as_ref() {
            struct_ser.serialize_field("createdBy", v)?;
        }
        if let Some(v) = self.last_updated_by.as_ref() {
            struct_ser.serialize_field("lastUpdatedBy", v)?;
        }
        if let Some(v) = self.last_update_time.as_ref() {
            struct_ser.serialize_field("lastUpdateTime", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.scheduled_time.as_ref() {
            struct_ser.serialize_field("scheduledTime", v)?;
        }
        if let Some(v) = self.relations.as_ref() {
            struct_ser.serialize_field("relations", v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_executed_elsewhere {
            struct_ser.serialize_field("isExecutedElsewhere", &self.is_executed_elsewhere)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.replication.as_ref() {
            struct_ser.serialize_field("replication", v)?;
        }
        if !self.initial_entities.is_empty() {
            struct_ser.serialize_field("initialEntities", &self.initial_entities)?;
        }
        if let Some(v) = self.owner.as_ref() {
            struct_ser.serialize_field("owner", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Task {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "display_name",
            "displayName",
            "specification",
            "created_by",
            "createdBy",
            "last_updated_by",
            "lastUpdatedBy",
            "last_update_time",
            "lastUpdateTime",
            "status",
            "scheduled_time",
            "scheduledTime",
            "relations",
            "description",
            "is_executed_elsewhere",
            "isExecutedElsewhere",
            "create_time",
            "createTime",
            "replication",
            "initial_entities",
            "initialEntities",
            "owner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            DisplayName,
            Specification,
            CreatedBy,
            LastUpdatedBy,
            LastUpdateTime,
            Status,
            ScheduledTime,
            Relations,
            Description,
            IsExecutedElsewhere,
            CreateTime,
            Replication,
            InitialEntities,
            Owner,
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
                            "version" => Ok(GeneratedField::Version),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "specification" => Ok(GeneratedField::Specification),
                            "createdBy" | "created_by" => Ok(GeneratedField::CreatedBy),
                            "lastUpdatedBy" | "last_updated_by" => Ok(GeneratedField::LastUpdatedBy),
                            "lastUpdateTime" | "last_update_time" => Ok(GeneratedField::LastUpdateTime),
                            "status" => Ok(GeneratedField::Status),
                            "scheduledTime" | "scheduled_time" => Ok(GeneratedField::ScheduledTime),
                            "relations" => Ok(GeneratedField::Relations),
                            "description" => Ok(GeneratedField::Description),
                            "isExecutedElsewhere" | "is_executed_elsewhere" => Ok(GeneratedField::IsExecutedElsewhere),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "replication" => Ok(GeneratedField::Replication),
                            "initialEntities" | "initial_entities" => Ok(GeneratedField::InitialEntities),
                            "owner" => Ok(GeneratedField::Owner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Task;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Task")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Task, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut display_name__ = None;
                let mut specification__ = None;
                let mut created_by__ = None;
                let mut last_updated_by__ = None;
                let mut last_update_time__ = None;
                let mut status__ = None;
                let mut scheduled_time__ = None;
                let mut relations__ = None;
                let mut description__ = None;
                let mut is_executed_elsewhere__ = None;
                let mut create_time__ = None;
                let mut replication__ = None;
                let mut initial_entities__ = None;
                let mut owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Specification => {
                            if specification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("specification"));
                            }
                            specification__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdBy"));
                            }
                            created_by__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdatedBy => {
                            if last_updated_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdatedBy"));
                            }
                            last_updated_by__ = map_.next_value()?;
                        }
                        GeneratedField::LastUpdateTime => {
                            if last_update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdateTime"));
                            }
                            last_update_time__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::ScheduledTime => {
                            if scheduled_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduledTime"));
                            }
                            scheduled_time__ = map_.next_value()?;
                        }
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsExecutedElsewhere => {
                            if is_executed_elsewhere__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExecutedElsewhere"));
                            }
                            is_executed_elsewhere__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                        GeneratedField::Replication => {
                            if replication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replication"));
                            }
                            replication__ = map_.next_value()?;
                        }
                        GeneratedField::InitialEntities => {
                            if initial_entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialEntities"));
                            }
                            initial_entities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Task {
                    version: version__,
                    display_name: display_name__.unwrap_or_default(),
                    specification: specification__,
                    created_by: created_by__,
                    last_updated_by: last_updated_by__,
                    last_update_time: last_update_time__,
                    status: status__,
                    scheduled_time: scheduled_time__,
                    relations: relations__,
                    description: description__.unwrap_or_default(),
                    is_executed_elsewhere: is_executed_elsewhere__.unwrap_or_default(),
                    create_time: create_time__,
                    replication: replication__,
                    initial_entities: initial_entities__.unwrap_or_default(),
                    owner: owner__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Task", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskEntity {
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
        if self.snapshot {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.TaskEntity", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if self.snapshot {
            struct_ser.serialize_field("snapshot", &self.snapshot)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskEntity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
            Snapshot,
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
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskEntity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.TaskEntity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskEntity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TaskEntity {
                    entity: entity__,
                    snapshot: snapshot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.TaskEntity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.TaskError", len)?;
        if self.code != 0 {
            let v = ErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
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
            type Value = TaskError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.TaskError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<ErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TaskError {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.TaskError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskEvent {
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
        if self.task.is_some() {
            len += 1;
        }
        if self.task_view != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.TaskEvent", len)?;
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        if self.task_view != 0 {
            let v = TaskView::try_from(self.task_view)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.task_view)))?;
            struct_ser.serialize_field("taskView", &v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_type",
            "eventType",
            "task",
            "task_view",
            "taskView",
            "time",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventType,
            Task,
            TaskView,
            Time,
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
                            "task" => Ok(GeneratedField::Task),
                            "taskView" | "task_view" => Ok(GeneratedField::TaskView),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.TaskEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_type__ = None;
                let mut task__ = None;
                let mut task_view__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
                        }
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                        GeneratedField::TaskView => {
                            if task_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskView"));
                            }
                            task_view__ = Some(map_.next_value::<TaskView>()? as i32);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskEvent {
                    event_type: event_type__.unwrap_or_default(),
                    task: task__,
                    task_view: task_view__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.TaskEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskStatus {
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
        if self.task_error.is_some() {
            len += 1;
        }
        if self.progress.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.estimate.is_some() {
            len += 1;
        }
        if self.allocation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.TaskStatus", len)?;
        if self.status != 0 {
            let v = Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.task_error.as_ref() {
            struct_ser.serialize_field("taskError", v)?;
        }
        if let Some(v) = self.progress.as_ref() {
            struct_ser.serialize_field("progress", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.estimate.as_ref() {
            struct_ser.serialize_field("estimate", v)?;
        }
        if let Some(v) = self.allocation.as_ref() {
            struct_ser.serialize_field("allocation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "task_error",
            "taskError",
            "progress",
            "result",
            "start_time",
            "startTime",
            "estimate",
            "allocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            TaskError,
            Progress,
            Result,
            StartTime,
            Estimate,
            Allocation,
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
                            "taskError" | "task_error" => Ok(GeneratedField::TaskError),
                            "progress" => Ok(GeneratedField::Progress),
                            "result" => Ok(GeneratedField::Result),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "estimate" => Ok(GeneratedField::Estimate),
                            "allocation" => Ok(GeneratedField::Allocation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.TaskStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut task_error__ = None;
                let mut progress__ = None;
                let mut result__ = None;
                let mut start_time__ = None;
                let mut estimate__ = None;
                let mut allocation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<Status>()? as i32);
                        }
                        GeneratedField::TaskError => {
                            if task_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskError"));
                            }
                            task_error__ = map_.next_value()?;
                        }
                        GeneratedField::Progress => {
                            if progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progress"));
                            }
                            progress__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::Estimate => {
                            if estimate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimate"));
                            }
                            estimate__ = map_.next_value()?;
                        }
                        GeneratedField::Allocation => {
                            if allocation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocation"));
                            }
                            allocation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskStatus {
                    status: status__.unwrap_or_default(),
                    task_error: task_error__,
                    progress: progress__,
                    result: result__,
                    start_time: start_time__,
                    estimate: estimate__,
                    allocation: allocation__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.TaskStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_id.is_empty() {
            len += 1;
        }
        if self.definition_version != 0 {
            len += 1;
        }
        if self.status_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.TaskVersion", len)?;
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if self.definition_version != 0 {
            struct_ser.serialize_field("definitionVersion", &self.definition_version)?;
        }
        if self.status_version != 0 {
            struct_ser.serialize_field("statusVersion", &self.status_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_id",
            "taskId",
            "definition_version",
            "definitionVersion",
            "status_version",
            "statusVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskId,
            DefinitionVersion,
            StatusVersion,
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
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "definitionVersion" | "definition_version" => Ok(GeneratedField::DefinitionVersion),
                            "statusVersion" | "status_version" => Ok(GeneratedField::StatusVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.TaskVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_id__ = None;
                let mut definition_version__ = None;
                let mut status_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefinitionVersion => {
                            if definition_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionVersion"));
                            }
                            definition_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StatusVersion => {
                            if status_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusVersion"));
                            }
                            status_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TaskVersion {
                    task_id: task_id__.unwrap_or_default(),
                    definition_version: definition_version__.unwrap_or_default(),
                    status_version: status_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.TaskVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskView {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "TASK_VIEW_INVALID",
            Self::Manager => "TASK_VIEW_MANAGER",
            Self::Agent => "TASK_VIEW_AGENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TaskView {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TASK_VIEW_INVALID",
            "TASK_VIEW_MANAGER",
            "TASK_VIEW_AGENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskView;

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
                    "TASK_VIEW_INVALID" => Ok(TaskView::Invalid),
                    "TASK_VIEW_MANAGER" => Ok(TaskView::Manager),
                    "TASK_VIEW_AGENT" => Ok(TaskView::Agent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Team {
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
        if !self.members.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.Team", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Team {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "members",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            Members,
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
                            "members" => Ok(GeneratedField::Members),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Team;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.Team")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Team, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut members__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Team {
                    entity_id: entity_id__.unwrap_or_default(),
                    members: members__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.Team", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status_update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.UpdateStatusRequest", len)?;
        if let Some(v) = self.status_update.as_ref() {
            struct_ser.serialize_field("statusUpdate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status_update",
            "statusUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatusUpdate,
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
                            "statusUpdate" | "status_update" => Ok(GeneratedField::StatusUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.UpdateStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status_update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StatusUpdate => {
                            if status_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusUpdate"));
                            }
                            status_update__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateStatusRequest {
                    status_update: status_update__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.UpdateStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.UpdateStatusResponse", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
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
                            "task" => Ok(GeneratedField::Task),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.UpdateStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateStatusResponse {
                    task: task__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.UpdateStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        if self.is_executed_elsewhere {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.UpdateTaskRequest", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        if self.is_executed_elsewhere {
            struct_ser.serialize_field("isExecutedElsewhere", &self.is_executed_elsewhere)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
            "is_executed_elsewhere",
            "isExecutedElsewhere",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
            IsExecutedElsewhere,
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
                            "task" => Ok(GeneratedField::Task),
                            "isExecutedElsewhere" | "is_executed_elsewhere" => Ok(GeneratedField::IsExecutedElsewhere),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.UpdateTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                let mut is_executed_elsewhere__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                        GeneratedField::IsExecutedElsewhere => {
                            if is_executed_elsewhere__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExecutedElsewhere"));
                            }
                            is_executed_elsewhere__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateTaskRequest {
                    task: task__,
                    is_executed_elsewhere: is_executed_elsewhere__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.UpdateTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTaskResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.UpdateTaskResponse", len)?;
        if let Some(v) = self.task.as_ref() {
            struct_ser.serialize_field("task", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTaskResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
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
                            "task" => Ok(GeneratedField::Task),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTaskResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.UpdateTaskResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTaskResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            task__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTaskResponse {
                    task: task__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.UpdateTaskResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.taskmanager.v1.User", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.taskmanager.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(User {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.taskmanager.v1.User", FIELDS, GeneratedVisitor)
    }
}
