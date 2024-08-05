// @generated
impl serde::Serialize for BlobAnnouncement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.BlobAnnouncement", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobAnnouncement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "key",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Key,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "key" => Ok(GeneratedField::Key),
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
            type Value = BlobAnnouncement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.BlobAnnouncement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobAnnouncement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut key__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BlobAnnouncement {
                    client_id: client_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.BlobAnnouncement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlobMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.created_time.is_some() {
            len += 1;
        }
        if self.retention_time.is_some() {
            len += 1;
        }
        if self.size_bytes != 0 {
            len += 1;
        }
        if !self.md5.is_empty() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.BlobMetadata", len)?;
        if let Some(v) = self.created_time.as_ref() {
            struct_ser.serialize_field("createdTime", v)?;
        }
        if let Some(v) = self.retention_time.as_ref() {
            struct_ser.serialize_field("retentionTime", v)?;
        }
        if self.size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sizeBytes", ToString::to_string(&self.size_bytes).as_str())?;
        }
        if !self.md5.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("md5", pbjson::private::base64::encode(&self.md5).as_str())?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_time",
            "createdTime",
            "retention_time",
            "retentionTime",
            "size_bytes",
            "sizeBytes",
            "md5",
            "content_type",
            "contentType",
            "provenance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedTime,
            RetentionTime,
            SizeBytes,
            Md5,
            ContentType,
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
                            "createdTime" | "created_time" => Ok(GeneratedField::CreatedTime),
                            "retentionTime" | "retention_time" => Ok(GeneratedField::RetentionTime),
                            "sizeBytes" | "size_bytes" => Ok(GeneratedField::SizeBytes),
                            "md5" => Ok(GeneratedField::Md5),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
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
            type Value = BlobMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.BlobMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_time__ = None;
                let mut retention_time__ = None;
                let mut size_bytes__ = None;
                let mut md5__ = None;
                let mut content_type__ = None;
                let mut provenance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedTime => {
                            if created_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdTime"));
                            }
                            created_time__ = map_.next_value()?;
                        }
                        GeneratedField::RetentionTime => {
                            if retention_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionTime"));
                            }
                            retention_time__ = map_.next_value()?;
                        }
                        GeneratedField::SizeBytes => {
                            if size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeBytes"));
                            }
                            size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Md5 => {
                            if md5__.is_some() {
                                return Err(serde::de::Error::duplicate_field("md5"));
                            }
                            md5__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BlobMetadata {
                    created_time: created_time__,
                    retention_time: retention_time__,
                    size_bytes: size_bytes__.unwrap_or_default(),
                    md5: md5__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    provenance: provenance__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.BlobMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlobProvenance {
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
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.BlobProvenance", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.data_type.is_empty() {
            struct_ser.serialize_field("dataType", &self.data_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobProvenance {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            DataType,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlobProvenance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.BlobProvenance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobProvenance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut data_type__ = None;
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
                    }
                }
                Ok(BlobProvenance {
                    integration_name: integration_name__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.BlobProvenance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBlobChunkedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.retention_time.is_some() {
            len += 1;
        }
        if !self.md5.is_empty() {
            len += 1;
        }
        if !self.chunk_contents.is_empty() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.CreateBlobChunkedRequest", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.retention_time.as_ref() {
            struct_ser.serialize_field("retentionTime", v)?;
        }
        if !self.md5.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("md5", pbjson::private::base64::encode(&self.md5).as_str())?;
        }
        if !self.chunk_contents.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("chunkContents", pbjson::private::base64::encode(&self.chunk_contents).as_str())?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBlobChunkedRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "key",
            "retention_time",
            "retentionTime",
            "md5",
            "chunk_contents",
            "chunkContents",
            "provenance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Key,
            RetentionTime,
            Md5,
            ChunkContents,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "key" => Ok(GeneratedField::Key),
                            "retentionTime" | "retention_time" => Ok(GeneratedField::RetentionTime),
                            "md5" => Ok(GeneratedField::Md5),
                            "chunkContents" | "chunk_contents" => Ok(GeneratedField::ChunkContents),
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
            type Value = CreateBlobChunkedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.CreateBlobChunkedRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBlobChunkedRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut key__ = None;
                let mut retention_time__ = None;
                let mut md5__ = None;
                let mut chunk_contents__ = None;
                let mut provenance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetentionTime => {
                            if retention_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionTime"));
                            }
                            retention_time__ = map_.next_value()?;
                        }
                        GeneratedField::Md5 => {
                            if md5__.is_some() {
                                return Err(serde::de::Error::duplicate_field("md5"));
                            }
                            md5__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChunkContents => {
                            if chunk_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkContents"));
                            }
                            chunk_contents__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateBlobChunkedRequest {
                    client_id: client_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    retention_time: retention_time__,
                    md5: md5__.unwrap_or_default(),
                    chunk_contents: chunk_contents__.unwrap_or_default(),
                    provenance: provenance__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.CreateBlobChunkedRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBlobChunkedResponse {
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
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.CreateBlobChunkedResponse", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBlobChunkedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Url,
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
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBlobChunkedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.CreateBlobChunkedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBlobChunkedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateBlobChunkedResponse {
                    metadata: metadata__,
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.CreateBlobChunkedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBlobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.retention_time.is_some() {
            len += 1;
        }
        if !self.md5.is_empty() {
            len += 1;
        }
        if !self.contents.is_empty() {
            len += 1;
        }
        if self.provenance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.CreateBlobRequest", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.retention_time.as_ref() {
            struct_ser.serialize_field("retentionTime", v)?;
        }
        if !self.md5.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("md5", pbjson::private::base64::encode(&self.md5).as_str())?;
        }
        if !self.contents.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("contents", pbjson::private::base64::encode(&self.contents).as_str())?;
        }
        if let Some(v) = self.provenance.as_ref() {
            struct_ser.serialize_field("provenance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBlobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "key",
            "retention_time",
            "retentionTime",
            "md5",
            "contents",
            "provenance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Key,
            RetentionTime,
            Md5,
            Contents,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "key" => Ok(GeneratedField::Key),
                            "retentionTime" | "retention_time" => Ok(GeneratedField::RetentionTime),
                            "md5" => Ok(GeneratedField::Md5),
                            "contents" => Ok(GeneratedField::Contents),
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
            type Value = CreateBlobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.CreateBlobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBlobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut key__ = None;
                let mut retention_time__ = None;
                let mut md5__ = None;
                let mut contents__ = None;
                let mut provenance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetentionTime => {
                            if retention_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionTime"));
                            }
                            retention_time__ = map_.next_value()?;
                        }
                        GeneratedField::Md5 => {
                            if md5__.is_some() {
                                return Err(serde::de::Error::duplicate_field("md5"));
                            }
                            md5__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Provenance => {
                            if provenance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provenance"));
                            }
                            provenance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateBlobRequest {
                    client_id: client_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    retention_time: retention_time__,
                    md5: md5__.unwrap_or_default(),
                    contents: contents__.unwrap_or_default(),
                    provenance: provenance__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.CreateBlobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBlobResponse {
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
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.CreateBlobResponse", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBlobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Url,
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
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBlobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.CreateBlobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBlobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateBlobResponse {
                    metadata: metadata__,
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.CreateBlobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldMatcher {
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
        if !self.mime_type.is_empty() {
            len += 1;
        }
        if !self.data_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.FieldMatcher", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.mime_type.is_empty() {
            struct_ser.serialize_field("mimeType", &self.mime_type)?;
        }
        if !self.data_type.is_empty() {
            struct_ser.serialize_field("dataType", &self.data_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "mime_type",
            "mimeType",
            "data_type",
            "dataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            MimeType,
            DataType,
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
                            "mimeType" | "mime_type" => Ok(GeneratedField::MimeType),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.FieldMatcher")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut mime_type__ = None;
                let mut data_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MimeType => {
                            if mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mimeType"));
                            }
                            mime_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldMatcher {
                    integration_name: integration_name__.unwrap_or_default(),
                    mime_type: mime_type__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.FieldMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBlobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.GetBlobRequest", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBlobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Key,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBlobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.GetBlobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBlobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBlobRequest {
                    client_id: client_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.GetBlobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBlobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.contents.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.GetBlobResponse", len)?;
        if !self.contents.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("contents", pbjson::private::base64::encode(&self.contents).as_str())?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBlobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contents",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Contents,
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
                            "contents" => Ok(GeneratedField::Contents),
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
            type Value = GetBlobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.GetBlobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBlobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contents__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetBlobResponse {
                    contents: contents__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.GetBlobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeadBlobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.HeadBlobRequest", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeadBlobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Key,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeadBlobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.HeadBlobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeadBlobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HeadBlobRequest {
                    client_id: client_id__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.HeadBlobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeadBlobResponse {
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
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.HeadBlobResponse", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeadBlobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Url,
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
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeadBlobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.HeadBlobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HeadBlobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HeadBlobResponse {
                    metadata: metadata__,
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.HeadBlobResponse", FIELDS, GeneratedVisitor)
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
        if self.r#match.is_some() {
            len += 1;
        }
        if !self.not_match.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.Statement", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if !self.not_match.is_empty() {
            struct_ser.serialize_field("notMatch", &self.not_match)?;
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
            "match",
            "not_match",
            "notMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
            NotMatch,
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
                            "match" => Ok(GeneratedField::Match),
                            "notMatch" | "not_match" => Ok(GeneratedField::NotMatch),
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
                formatter.write_str("struct anduril.blobs.v1.Statement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Statement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut not_match__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map_.next_value()?;
                        }
                        GeneratedField::NotMatch => {
                            if not_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notMatch"));
                            }
                            not_match__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Statement {
                    r#match: r#match__,
                    not_match: not_match__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.Statement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamBlobMetadataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.StreamBlobMetadataRequest", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamBlobMetadataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamBlobMetadataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.StreamBlobMetadataRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamBlobMetadataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamBlobMetadataRequest {
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.StreamBlobMetadataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamBlobMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.announcements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.blobs.v1.StreamBlobMetadataResponse", len)?;
        if !self.announcements.is_empty() {
            struct_ser.serialize_field("announcements", &self.announcements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamBlobMetadataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "announcements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Announcements,
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
                            "announcements" => Ok(GeneratedField::Announcements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamBlobMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.blobs.v1.StreamBlobMetadataResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamBlobMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut announcements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Announcements => {
                            if announcements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("announcements"));
                            }
                            announcements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamBlobMetadataResponse {
                    announcements: announcements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.blobs.v1.StreamBlobMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
