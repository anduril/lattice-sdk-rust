// @generated
impl serde::Serialize for AllNodes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.AllNodes", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllNodes {
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
            type Value = AllNodes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.AllNodes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllNodes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AllNodes {
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.AllNodes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlobDistributionRuleDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.blob_filter.is_some() {
            len += 1;
        }
        if !self.blob_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.BlobDistributionRuleDetails", len)?;
        if let Some(v) = self.blob_filter.as_ref() {
            struct_ser.serialize_field("blobFilter", v)?;
        }
        if !self.blob_filters.is_empty() {
            struct_ser.serialize_field("blobFilters", &self.blob_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobDistributionRuleDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blob_filter",
            "blobFilter",
            "blob_filters",
            "blobFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlobFilter,
            BlobFilters,
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
                            "blobFilter" | "blob_filter" => Ok(GeneratedField::BlobFilter),
                            "blobFilters" | "blob_filters" => Ok(GeneratedField::BlobFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlobDistributionRuleDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.BlobDistributionRuleDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobDistributionRuleDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blob_filter__ = None;
                let mut blob_filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlobFilter => {
                            if blob_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blobFilter"));
                            }
                            blob_filter__ = map_.next_value()?;
                        }
                        GeneratedField::BlobFilters => {
                            if blob_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blobFilters"));
                            }
                            blob_filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BlobDistributionRuleDetails {
                    blob_filter: blob_filter__,
                    blob_filters: blob_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.BlobDistributionRuleDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlobIntegrationRuleDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.BlobIntegrationRuleDetails", len)?;
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobIntegrationRuleDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlobIntegrationRuleDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.BlobIntegrationRuleDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobIntegrationRuleDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BlobIntegrationRuleDetails {
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.BlobIntegrationRuleDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlobsIntegrationDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.BlobsIntegrationDetails", len)?;
        if !self.data_types.is_empty() {
            struct_ser.serialize_field("dataTypes", &self.data_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlobsIntegrationDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_types",
            "dataTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataTypes,
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
                            "dataTypes" | "data_types" => Ok(GeneratedField::DataTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlobsIntegrationDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.BlobsIntegrationDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlobsIntegrationDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataTypes => {
                            if data_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataTypes"));
                            }
                            data_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BlobsIntegrationDetails {
                    data_types: data_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.BlobsIntegrationDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CdsDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_cds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.CdsDestination", len)?;
        if let Some(v) = self.registered_cds.as_ref() {
            struct_ser.serialize_field("registeredCds", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CdsDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_cds",
            "registeredCds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredCds,
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
                            "registeredCds" | "registered_cds" => Ok(GeneratedField::RegisteredCds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CdsDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.CdsDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CdsDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_cds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RegisteredCds => {
                            if registered_cds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredCds"));
                            }
                            registered_cds__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CdsDestination {
                    registered_cds: registered_cds__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.CdsDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommunicationsManagerError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.CommunicationsManagerError", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommunicationsManagerError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CommunicationsManagerError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.CommunicationsManagerError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommunicationsManagerError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CommunicationsManagerError {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.CommunicationsManagerError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CrossDomainRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cds_destination.is_some() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.CrossDomainRule", len)?;
        if let Some(v) = self.cds_destination.as_ref() {
            struct_ser.serialize_field("cdsDestination", v)?;
        }
        if let Some(v) = self.details.as_ref() {
            match v {
                cross_domain_rule::Details::EntityDetails(v) => {
                    struct_ser.serialize_field("entityDetails", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CrossDomainRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cds_destination",
            "cdsDestination",
            "entity_details",
            "entityDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CdsDestination,
            EntityDetails,
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
                            "cdsDestination" | "cds_destination" => Ok(GeneratedField::CdsDestination),
                            "entityDetails" | "entity_details" => Ok(GeneratedField::EntityDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CrossDomainRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.CrossDomainRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CrossDomainRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cds_destination__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CdsDestination => {
                            if cds_destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdsDestination"));
                            }
                            cds_destination__ = map_.next_value()?;
                        }
                        GeneratedField::EntityDetails => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityDetails"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(cross_domain_rule::Details::EntityDetails)
;
                        }
                    }
                }
                Ok(CrossDomainRule {
                    cds_destination: cds_destination__,
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.CrossDomainRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        if self.distribution_rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DeleteRuleRequest", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        if let Some(v) = self.distribution_rule.as_ref() {
            struct_ser.serialize_field("distributionRule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "distribution_rule",
            "distributionRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            DistributionRule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DeleteRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut distribution_rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                        GeneratedField::DistributionRule => {
                            if distribution_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            distribution_rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteRuleRequest {
                    rule: rule__,
                    distribution_rule: distribution_rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DeleteRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if self.distribution_rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DeleteRuleResponse", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.distribution_rule.as_ref() {
            struct_ser.serialize_field("distributionRule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "error",
            "distribution_rule",
            "distributionRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            Error,
            DistributionRule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            "error" => Ok(GeneratedField::Error),
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DeleteRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut error__ = None;
                let mut distribution_rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::DistributionRule => {
                            if distribution_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            distribution_rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteRuleResponse {
                    rule: rule__,
                    error: error__,
                    distribution_rule: distribution_rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DeleteRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeregisterIntegrationRequest {
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
        if !self.node_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DeregisterIntegrationRequest", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.node_id.is_empty() {
            struct_ser.serialize_field("nodeId", &self.node_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeregisterIntegrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "node_id",
            "nodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            NodeId,
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
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeregisterIntegrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DeregisterIntegrationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeregisterIntegrationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut node_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeregisterIntegrationRequest {
                    integration_name: integration_name__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DeregisterIntegrationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeregisterIntegrationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DeregisterIntegrationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeregisterIntegrationResponse {
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
            type Value = DeregisterIntegrationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DeregisterIntegrationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeregisterIntegrationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeregisterIntegrationResponse {
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DeregisterIntegrationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Destination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.destination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.Destination", len)?;
        if let Some(v) = self.destination.as_ref() {
            match v {
                destination::Destination::AssetId(v) => {
                    struct_ser.serialize_field("assetId", v)?;
                }
                destination::Destination::HostId(v) => {
                    struct_ser.serialize_field("hostId", v)?;
                }
                destination::Destination::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Destination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "host_id",
            "hostId",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            HostId,
            All,
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
                            "hostId" | "host_id" => Ok(GeneratedField::HostId),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Destination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.Destination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Destination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut destination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            destination__ = map_.next_value::<::std::option::Option<_>>()?.map(destination::Destination::AssetId);
                        }
                        GeneratedField::HostId => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostId"));
                            }
                            destination__ = map_.next_value::<::std::option::Option<_>>()?.map(destination::Destination::HostId);
                        }
                        GeneratedField::All => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            destination__ = map_.next_value::<::std::option::Option<_>>()?.map(destination::Destination::All);
                        }
                    }
                }
                Ok(Destination {
                    destination: destination__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.Destination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DistributionRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_id.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        if self.destination.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.version_num != 0 {
            len += 1;
        }
        if self.source_destination_directional_control.is_some() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DistributionRule", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if let Some(v) = self.destination.as_ref() {
            struct_ser.serialize_field("destination", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.version_num != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("versionNum", ToString::to_string(&self.version_num).as_str())?;
        }
        if let Some(v) = self.source_destination_directional_control.as_ref() {
            struct_ser.serialize_field("sourceDestinationDirectionalControl", v)?;
        }
        if let Some(v) = self.details.as_ref() {
            match v {
                distribution_rule::Details::EntityDetails(v) => {
                    struct_ser.serialize_field("entityDetails", v)?;
                }
                distribution_rule::Details::BlobDetails(v) => {
                    struct_ser.serialize_field("blobDetails", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DistributionRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "version",
            "enabled",
            "source",
            "sources",
            "destination",
            "priority",
            "description",
            "name",
            "version_num",
            "versionNum",
            "source_destination_directional_control",
            "sourceDestinationDirectionalControl",
            "entity_details",
            "entityDetails",
            "blob_details",
            "blobDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            Version,
            Enabled,
            Source,
            Sources,
            Destination,
            Priority,
            Description,
            Name,
            VersionNum,
            SourceDestinationDirectionalControl,
            EntityDetails,
            BlobDetails,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "version" => Ok(GeneratedField::Version),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "source" => Ok(GeneratedField::Source),
                            "sources" => Ok(GeneratedField::Sources),
                            "destination" => Ok(GeneratedField::Destination),
                            "priority" => Ok(GeneratedField::Priority),
                            "description" => Ok(GeneratedField::Description),
                            "name" => Ok(GeneratedField::Name),
                            "versionNum" | "version_num" => Ok(GeneratedField::VersionNum),
                            "sourceDestinationDirectionalControl" | "source_destination_directional_control" => Ok(GeneratedField::SourceDestinationDirectionalControl),
                            "entityDetails" | "entity_details" => Ok(GeneratedField::EntityDetails),
                            "blobDetails" | "blob_details" => Ok(GeneratedField::BlobDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DistributionRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DistributionRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DistributionRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut version__ = None;
                let mut enabled__ = None;
                let mut source__ = None;
                let mut sources__ = None;
                let mut destination__ = None;
                let mut priority__ = None;
                let mut description__ = None;
                let mut name__ = None;
                let mut version_num__ = None;
                let mut source_destination_directional_control__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value()?;
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionNum => {
                            if version_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNum"));
                            }
                            version_num__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SourceDestinationDirectionalControl => {
                            if source_destination_directional_control__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceDestinationDirectionalControl"));
                            }
                            source_destination_directional_control__ = map_.next_value()?;
                        }
                        GeneratedField::EntityDetails => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityDetails"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution_rule::Details::EntityDetails)
;
                        }
                        GeneratedField::BlobDetails => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blobDetails"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(distribution_rule::Details::BlobDetails)
;
                        }
                    }
                }
                Ok(DistributionRule {
                    rule_id: rule_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                    source: source__,
                    sources: sources__.unwrap_or_default(),
                    destination: destination__,
                    priority: priority__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    version_num: version_num__.unwrap_or_default(),
                    source_destination_directional_control: source_destination_directional_control__,
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DistributionRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DistributionRuleEvent {
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
        if self.distribution_rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.DistributionRuleEvent", len)?;
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.distribution_rule.as_ref() {
            struct_ser.serialize_field("distributionRule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DistributionRuleEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_type",
            "eventType",
            "time",
            "distribution_rule",
            "distributionRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventType,
            Time,
            DistributionRule,
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
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DistributionRuleEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.DistributionRuleEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DistributionRuleEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_type__ = None;
                let mut time__ = None;
                let mut distribution_rule__ = None;
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
                        GeneratedField::DistributionRule => {
                            if distribution_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            distribution_rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DistributionRuleEvent {
                    event_type: event_type__.unwrap_or_default(),
                    time: time__,
                    distribution_rule: distribution_rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.DistributionRuleEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityDataVendor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vendor_name.is_empty() {
            len += 1;
        }
        if !self.data_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.EntityDataVendor", len)?;
        if !self.vendor_name.is_empty() {
            struct_ser.serialize_field("vendorName", &self.vendor_name)?;
        }
        if !self.data_type.is_empty() {
            struct_ser.serialize_field("dataType", &self.data_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityDataVendor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vendor_name",
            "vendorName",
            "data_type",
            "dataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VendorName,
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
                            "vendorName" | "vendor_name" => Ok(GeneratedField::VendorName),
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
            type Value = EntityDataVendor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.EntityDataVendor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityDataVendor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vendor_name__ = None;
                let mut data_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VendorName => {
                            if vendor_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendorName"));
                            }
                            vendor_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EntityDataVendor {
                    vendor_name: vendor_name__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.EntityDataVendor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityDistributionRuleDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_filter.is_some() {
            len += 1;
        }
        if self.entity_filter_selection.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.EntityDistributionRuleDetails", len)?;
        if let Some(v) = self.entity_filter.as_ref() {
            struct_ser.serialize_field("entityFilter", v)?;
        }
        if let Some(v) = self.entity_filter_selection.as_ref() {
            match v {
                entity_distribution_rule_details::EntityFilterSelection::StaticStatement(v) => {
                    struct_ser.serialize_field("staticStatement", v)?;
                }
                entity_distribution_rule_details::EntityFilterSelection::DynamicStatement(v) => {
                    struct_ser.serialize_field("dynamicStatement", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityDistributionRuleDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_filter",
            "entityFilter",
            "static_statement",
            "staticStatement",
            "dynamic_statement",
            "dynamicStatement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityFilter,
            StaticStatement,
            DynamicStatement,
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
                            "entityFilter" | "entity_filter" => Ok(GeneratedField::EntityFilter),
                            "staticStatement" | "static_statement" => Ok(GeneratedField::StaticStatement),
                            "dynamicStatement" | "dynamic_statement" => Ok(GeneratedField::DynamicStatement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityDistributionRuleDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.EntityDistributionRuleDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityDistributionRuleDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_filter__ = None;
                let mut entity_filter_selection__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityFilter => {
                            if entity_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityFilter"));
                            }
                            entity_filter__ = map_.next_value()?;
                        }
                        GeneratedField::StaticStatement => {
                            if entity_filter_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticStatement"));
                            }
                            entity_filter_selection__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_distribution_rule_details::EntityFilterSelection::StaticStatement)
;
                        }
                        GeneratedField::DynamicStatement => {
                            if entity_filter_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicStatement"));
                            }
                            entity_filter_selection__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_distribution_rule_details::EntityFilterSelection::DynamicStatement)
;
                        }
                    }
                }
                Ok(EntityDistributionRuleDetails {
                    entity_filter: entity_filter__,
                    entity_filter_selection: entity_filter_selection__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.EntityDistributionRuleDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityIntegrationDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vendors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.EntityIntegrationDetails", len)?;
        if !self.vendors.is_empty() {
            struct_ser.serialize_field("vendors", &self.vendors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityIntegrationDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vendors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vendors,
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
                            "vendors" => Ok(GeneratedField::Vendors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityIntegrationDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.EntityIntegrationDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityIntegrationDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vendors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vendors => {
                            if vendors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendors"));
                            }
                            vendors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EntityIntegrationDetails {
                    vendors: vendors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.EntityIntegrationDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityIntegrationRuleDetails {
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
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.EntityIntegrationRuleDetails", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityIntegrationRuleDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityIntegrationRuleDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.EntityIntegrationRuleDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityIntegrationRuleDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EntityIntegrationRuleDetails {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.EntityIntegrationRuleDetails", FIELDS, GeneratedVisitor)
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
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetIntegrationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.GetIntegrationsRequest", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetIntegrationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
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
                            "nodes" => Ok(GeneratedField::Nodes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetIntegrationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.GetIntegrationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIntegrationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetIntegrationsRequest {
                    nodes: nodes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.GetIntegrationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetIntegrationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.integration.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.GetIntegrationsResponse", len)?;
        if !self.integration.is_empty() {
            struct_ser.serialize_field("integration", &self.integration)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetIntegrationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Integration,
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
                            "integration" => Ok(GeneratedField::Integration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetIntegrationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.GetIntegrationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIntegrationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Integration => {
                            if integration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integration"));
                            }
                            integration__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetIntegrationsResponse {
                    integration: integration__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.GetIntegrationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InboundRuleSourceDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sources.is_empty() {
            len += 1;
        }
        if self.destination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.InboundRuleSourceDestination", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if let Some(v) = self.destination.as_ref() {
            struct_ser.serialize_field("destination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InboundRuleSourceDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sources",
            "destination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
            Destination,
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
                            "sources" => Ok(GeneratedField::Sources),
                            "destination" => Ok(GeneratedField::Destination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InboundRuleSourceDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.InboundRuleSourceDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InboundRuleSourceDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sources__ = None;
                let mut destination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InboundRuleSourceDestination {
                    sources: sources__.unwrap_or_default(),
                    destination: destination__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.InboundRuleSourceDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_details.is_some() {
            len += 1;
        }
        if self.blob_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationDetails", len)?;
        if let Some(v) = self.entity_details.as_ref() {
            struct_ser.serialize_field("entityDetails", v)?;
        }
        if let Some(v) = self.blob_details.as_ref() {
            struct_ser.serialize_field("blobDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_details",
            "entityDetails",
            "blob_details",
            "blobDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityDetails,
            BlobDetails,
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
                            "entityDetails" | "entity_details" => Ok(GeneratedField::EntityDetails),
                            "blobDetails" | "blob_details" => Ok(GeneratedField::BlobDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_details__ = None;
                let mut blob_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityDetails => {
                            if entity_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityDetails"));
                            }
                            entity_details__ = map_.next_value()?;
                        }
                        GeneratedField::BlobDetails => {
                            if blob_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blobDetails"));
                            }
                            blob_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IntegrationDetails {
                    entity_details: entity_details__,
                    blob_details: blob_details__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationEvent {
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
        if self.integration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationEvent", len)?;
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.integration.as_ref() {
            struct_ser.serialize_field("integration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_type",
            "eventType",
            "time",
            "integration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventType,
            Time,
            Integration,
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
                            "integration" => Ok(GeneratedField::Integration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_type__ = None;
                let mut time__ = None;
                let mut integration__ = None;
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
                        GeneratedField::Integration => {
                            if integration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integration"));
                            }
                            integration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IntegrationEvent {
                    event_type: event_type__.unwrap_or_default(),
                    time: time__,
                    integration: integration__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationHealthState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.health_status != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationHealthState", len)?;
        if self.health_status != 0 {
            let v = IntegrationHealthStatus::try_from(self.health_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("healthStatus", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationHealthState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_status",
            "healthStatus",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthStatus,
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
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
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
            type Value = IntegrationHealthState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationHealthState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationHealthState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_status__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map_.next_value::<IntegrationHealthStatus>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IntegrationHealthState {
                    health_status: health_status__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationHealthState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationHealthStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "INTEGRATION_HEALTH_STATUS_INVALID",
            Self::Unknown => "INTEGRATION_HEALTH_STATUS_UNKNOWN",
            Self::Healthy => "INTEGRATION_HEALTH_STATUS_HEALTHY",
            Self::Unhealthy => "INTEGRATION_HEALTH_STATUS_UNHEALTHY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationHealthStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INTEGRATION_HEALTH_STATUS_INVALID",
            "INTEGRATION_HEALTH_STATUS_UNKNOWN",
            "INTEGRATION_HEALTH_STATUS_HEALTHY",
            "INTEGRATION_HEALTH_STATUS_UNHEALTHY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationHealthStatus;

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
                    "INTEGRATION_HEALTH_STATUS_INVALID" => Ok(IntegrationHealthStatus::Invalid),
                    "INTEGRATION_HEALTH_STATUS_UNKNOWN" => Ok(IntegrationHealthStatus::Unknown),
                    "INTEGRATION_HEALTH_STATUS_HEALTHY" => Ok(IntegrationHealthStatus::Healthy),
                    "INTEGRATION_HEALTH_STATUS_UNHEALTHY" => Ok(IntegrationHealthStatus::Unhealthy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationHealthUpdateRequest {
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
        if self.health_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationHealthUpdateRequest", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if let Some(v) = self.health_state.as_ref() {
            struct_ser.serialize_field("healthState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationHealthUpdateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "health_state",
            "healthState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            HealthState,
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
                            "healthState" | "health_state" => Ok(GeneratedField::HealthState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationHealthUpdateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationHealthUpdateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationHealthUpdateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut health_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HealthState => {
                            if health_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthState"));
                            }
                            health_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IntegrationHealthUpdateRequest {
                    integration_name: integration_name__.unwrap_or_default(),
                    health_state: health_state__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationHealthUpdateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationHealthUpdateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationHealthUpdateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationHealthUpdateResponse {
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
            type Value = IntegrationHealthUpdateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationHealthUpdateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationHealthUpdateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(IntegrationHealthUpdateResponse {
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationHealthUpdateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationRule {
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
        if !self.host_id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.IntegrationRule", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.host_id.is_empty() {
            struct_ser.serialize_field("hostId", &self.host_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            match v {
                integration_rule::Details::EntityIntegrationRuleDetails(v) => {
                    struct_ser.serialize_field("entityIntegrationRuleDetails", v)?;
                }
                integration_rule::Details::BlobIntegrationRuleDetails(v) => {
                    struct_ser.serialize_field("blobIntegrationRuleDetails", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "host_id",
            "hostId",
            "entity_integration_rule_details",
            "entityIntegrationRuleDetails",
            "blob_integration_rule_details",
            "blobIntegrationRuleDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            HostId,
            EntityIntegrationRuleDetails,
            BlobIntegrationRuleDetails,
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
                            "hostId" | "host_id" => Ok(GeneratedField::HostId),
                            "entityIntegrationRuleDetails" | "entity_integration_rule_details" => Ok(GeneratedField::EntityIntegrationRuleDetails),
                            "blobIntegrationRuleDetails" | "blob_integration_rule_details" => Ok(GeneratedField::BlobIntegrationRuleDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.IntegrationRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegrationRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut host_id__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostId => {
                            if host_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostId"));
                            }
                            host_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityIntegrationRuleDetails => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityIntegrationRuleDetails"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(integration_rule::Details::EntityIntegrationRuleDetails)
;
                        }
                        GeneratedField::BlobIntegrationRuleDetails => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blobIntegrationRuleDetails"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(integration_rule::Details::BlobIntegrationRuleDetails)
;
                        }
                    }
                }
                Ok(IntegrationRule {
                    integration_name: integration_name__.unwrap_or_default(),
                    host_id: host_id__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.IntegrationRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.LocalNode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalNode {
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
            type Value = LocalNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.LocalNode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalNode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LocalNode {
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.LocalNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutboundRuleSourceDestination {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.destinations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.OutboundRuleSourceDestination", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.destinations.is_empty() {
            struct_ser.serialize_field("destinations", &self.destinations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutboundRuleSourceDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "destinations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Destinations,
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
                            "source" => Ok(GeneratedField::Source),
                            "destinations" => Ok(GeneratedField::Destinations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutboundRuleSourceDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.OutboundRuleSourceDestination")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OutboundRuleSourceDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut destinations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value()?;
                        }
                        GeneratedField::Destinations => {
                            if destinations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinations"));
                            }
                            destinations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OutboundRuleSourceDestination {
                    source: source__,
                    destinations: destinations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.OutboundRuleSourceDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutRuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        if self.distribution_rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.PutRuleRequest", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        if let Some(v) = self.distribution_rule.as_ref() {
            struct_ser.serialize_field("distributionRule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "distribution_rule",
            "distributionRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            DistributionRule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.PutRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut distribution_rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                        GeneratedField::DistributionRule => {
                            if distribution_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            distribution_rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutRuleRequest {
                    rule: rule__,
                    distribution_rule: distribution_rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.PutRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if self.distribution_rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.PutRuleResponse", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.distribution_rule.as_ref() {
            struct_ser.serialize_field("distributionRule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "error",
            "distribution_rule",
            "distributionRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            Error,
            DistributionRule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            "error" => Ok(GeneratedField::Error),
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.PutRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PutRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut error__ = None;
                let mut distribution_rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::DistributionRule => {
                            if distribution_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            distribution_rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PutRuleResponse {
                    rule: rule__,
                    error: error__,
                    distribution_rule: distribution_rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.PutRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterIntegrationRequest {
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
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.RegisterIntegrationRequest", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterIntegrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            Details,
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
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterIntegrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.RegisterIntegrationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterIntegrationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisterIntegrationRequest {
                    integration_name: integration_name__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.RegisterIntegrationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredCds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_id.is_empty() {
            len += 1;
        }
        if !self.cds_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.RegisteredCds", len)?;
        if !self.host_id.is_empty() {
            struct_ser.serialize_field("hostId", &self.host_id)?;
        }
        if !self.cds_name.is_empty() {
            struct_ser.serialize_field("cdsName", &self.cds_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredCds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_id",
            "hostId",
            "cds_name",
            "cdsName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostId,
            CdsName,
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
                            "hostId" | "host_id" => Ok(GeneratedField::HostId),
                            "cdsName" | "cds_name" => Ok(GeneratedField::CdsName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredCds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.RegisteredCds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredCds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_id__ = None;
                let mut cds_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostId => {
                            if host_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostId"));
                            }
                            host_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CdsName => {
                            if cds_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdsName"));
                            }
                            cds_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisteredCds {
                    host_id: host_id__.unwrap_or_default(),
                    cds_name: cds_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.RegisteredCds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredIntegration {
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
        if !self.node_id.is_empty() {
            len += 1;
        }
        if self.version_num != 0 {
            len += 1;
        }
        if self.integration_details.is_some() {
            len += 1;
        }
        if self.health_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.RegisteredIntegration", len)?;
        if !self.integration_name.is_empty() {
            struct_ser.serialize_field("integrationName", &self.integration_name)?;
        }
        if !self.node_id.is_empty() {
            struct_ser.serialize_field("nodeId", &self.node_id)?;
        }
        if self.version_num != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("versionNum", ToString::to_string(&self.version_num).as_str())?;
        }
        if let Some(v) = self.integration_details.as_ref() {
            struct_ser.serialize_field("integrationDetails", v)?;
        }
        if let Some(v) = self.health_state.as_ref() {
            struct_ser.serialize_field("healthState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredIntegration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_name",
            "integrationName",
            "node_id",
            "nodeId",
            "version_num",
            "versionNum",
            "integration_details",
            "integrationDetails",
            "health_state",
            "healthState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationName,
            NodeId,
            VersionNum,
            IntegrationDetails,
            HealthState,
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
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            "versionNum" | "version_num" => Ok(GeneratedField::VersionNum),
                            "integrationDetails" | "integration_details" => Ok(GeneratedField::IntegrationDetails),
                            "healthState" | "health_state" => Ok(GeneratedField::HealthState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredIntegration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.RegisteredIntegration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredIntegration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_name__ = None;
                let mut node_id__ = None;
                let mut version_num__ = None;
                let mut integration_details__ = None;
                let mut health_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationName => {
                            if integration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationName"));
                            }
                            integration_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionNum => {
                            if version_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNum"));
                            }
                            version_num__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IntegrationDetails => {
                            if integration_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationDetails"));
                            }
                            integration_details__ = map_.next_value()?;
                        }
                        GeneratedField::HealthState => {
                            if health_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthState"));
                            }
                            health_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisteredIntegration {
                    integration_name: integration_name__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                    version_num: version_num__.unwrap_or_default(),
                    integration_details: integration_details__,
                    health_state: health_state__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.RegisteredIntegration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_id.is_empty() {
            len += 1;
        }
        if self.version_num != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.last_modified_by_user.is_empty() {
            len += 1;
        }
        if self.last_modified_time_unix_epoch_ms != 0 {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.Rule", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if self.version_num != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("versionNum", ToString::to_string(&self.version_num).as_str())?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.last_modified_by_user.is_empty() {
            struct_ser.serialize_field("lastModifiedByUser", &self.last_modified_by_user)?;
        }
        if self.last_modified_time_unix_epoch_ms != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("lastModifiedTimeUnixEpochMs", ToString::to_string(&self.last_modified_time_unix_epoch_ms).as_str())?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                rule::Type::DistributionRule(v) => {
                    struct_ser.serialize_field("distributionRule", v)?;
                }
                rule::Type::IntegrationRule(v) => {
                    struct_ser.serialize_field("integrationRule", v)?;
                }
                rule::Type::CrossDomainRule(v) => {
                    struct_ser.serialize_field("crossDomainRule", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "version_num",
            "versionNum",
            "enabled",
            "name",
            "last_modified_by_user",
            "lastModifiedByUser",
            "last_modified_time_unix_epoch_ms",
            "lastModifiedTimeUnixEpochMs",
            "distribution_rule",
            "distributionRule",
            "integration_rule",
            "integrationRule",
            "cross_domain_rule",
            "crossDomainRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            VersionNum,
            Enabled,
            Name,
            LastModifiedByUser,
            LastModifiedTimeUnixEpochMs,
            DistributionRule,
            IntegrationRule,
            CrossDomainRule,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "versionNum" | "version_num" => Ok(GeneratedField::VersionNum),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "name" => Ok(GeneratedField::Name),
                            "lastModifiedByUser" | "last_modified_by_user" => Ok(GeneratedField::LastModifiedByUser),
                            "lastModifiedTimeUnixEpochMs" | "last_modified_time_unix_epoch_ms" => Ok(GeneratedField::LastModifiedTimeUnixEpochMs),
                            "distributionRule" | "distribution_rule" => Ok(GeneratedField::DistributionRule),
                            "integrationRule" | "integration_rule" => Ok(GeneratedField::IntegrationRule),
                            "crossDomainRule" | "cross_domain_rule" => Ok(GeneratedField::CrossDomainRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.Rule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut version_num__ = None;
                let mut enabled__ = None;
                let mut name__ = None;
                let mut last_modified_by_user__ = None;
                let mut last_modified_time_unix_epoch_ms__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionNum => {
                            if version_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNum"));
                            }
                            version_num__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastModifiedByUser => {
                            if last_modified_by_user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastModifiedByUser"));
                            }
                            last_modified_by_user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastModifiedTimeUnixEpochMs => {
                            if last_modified_time_unix_epoch_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastModifiedTimeUnixEpochMs"));
                            }
                            last_modified_time_unix_epoch_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DistributionRule => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRule"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(rule::Type::DistributionRule)
;
                        }
                        GeneratedField::IntegrationRule => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationRule"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(rule::Type::IntegrationRule)
;
                        }
                        GeneratedField::CrossDomainRule => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("crossDomainRule"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(rule::Type::CrossDomainRule)
;
                        }
                    }
                }
                Ok(Rule {
                    rule_id: rule_id__.unwrap_or_default(),
                    version_num: version_num__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    last_modified_by_user: last_modified_by_user__.unwrap_or_default(),
                    last_modified_time_unix_epoch_ms: last_modified_time_unix_epoch_ms__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleEvent {
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
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.RuleEvent", len)?;
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_type",
            "eventType",
            "time",
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventType,
            Time,
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.RuleEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_type__ = None;
                let mut time__ = None;
                let mut rule__ = None;
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
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RuleEvent {
                    event_type: event_type__.unwrap_or_default(),
                    time: time__,
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.RuleEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "RULE_TYPE_INVALID",
            Self::Entity => "RULE_TYPE_ENTITY",
            Self::Blob => "RULE_TYPE_BLOB",
            Self::Integration => "RULE_TYPE_INTEGRATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RuleType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RULE_TYPE_INVALID",
            "RULE_TYPE_ENTITY",
            "RULE_TYPE_BLOB",
            "RULE_TYPE_INTEGRATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleType;

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
                    "RULE_TYPE_INVALID" => Ok(RuleType::Invalid),
                    "RULE_TYPE_ENTITY" => Ok(RuleType::Entity),
                    "RULE_TYPE_BLOB" => Ok(RuleType::Blob),
                    "RULE_TYPE_INTEGRATION" => Ok(RuleType::Integration),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.Source", len)?;
        if let Some(v) = self.source.as_ref() {
            match v {
                source::Source::AssetId(v) => {
                    struct_ser.serialize_field("assetId", v)?;
                }
                source::Source::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
                source::Source::HostId(v) => {
                    struct_ser.serialize_field("hostId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "all",
            "host_id",
            "hostId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            All,
            HostId,
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
                            "all" => Ok(GeneratedField::All),
                            "hostId" | "host_id" => Ok(GeneratedField::HostId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.Source")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Source, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(source::Source::AssetId);
                        }
                        GeneratedField::All => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(source::Source::All);
                        }
                        GeneratedField::HostId => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostId"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(source::Source::HostId);
                        }
                    }
                }
                Ok(Source {
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.Source", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceDestinationDirectionalControl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_destination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.SourceDestinationDirectionalControl", len)?;
        if let Some(v) = self.source_destination.as_ref() {
            match v {
                source_destination_directional_control::SourceDestination::InboundRuleSourceDestination(v) => {
                    struct_ser.serialize_field("inboundRuleSourceDestination", v)?;
                }
                source_destination_directional_control::SourceDestination::OutboundRuleSourceDestination(v) => {
                    struct_ser.serialize_field("outboundRuleSourceDestination", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceDestinationDirectionalControl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inbound_rule_source_destination",
            "inboundRuleSourceDestination",
            "outbound_rule_source_destination",
            "outboundRuleSourceDestination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InboundRuleSourceDestination,
            OutboundRuleSourceDestination,
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
                            "inboundRuleSourceDestination" | "inbound_rule_source_destination" => Ok(GeneratedField::InboundRuleSourceDestination),
                            "outboundRuleSourceDestination" | "outbound_rule_source_destination" => Ok(GeneratedField::OutboundRuleSourceDestination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceDestinationDirectionalControl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.SourceDestinationDirectionalControl")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceDestinationDirectionalControl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_destination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InboundRuleSourceDestination => {
                            if source_destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inboundRuleSourceDestination"));
                            }
                            source_destination__ = map_.next_value::<::std::option::Option<_>>()?.map(source_destination_directional_control::SourceDestination::InboundRuleSourceDestination)
;
                        }
                        GeneratedField::OutboundRuleSourceDestination => {
                            if source_destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outboundRuleSourceDestination"));
                            }
                            source_destination__ = map_.next_value::<::std::option::Option<_>>()?.map(source_destination_directional_control::SourceDestination::OutboundRuleSourceDestination)
;
                        }
                    }
                }
                Ok(SourceDestinationDirectionalControl {
                    source_destination: source_destination__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.SourceDestinationDirectionalControl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIntegrationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preexisting_only {
            len += 1;
        }
        if self.owner_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.StreamIntegrationsRequest", len)?;
        if self.preexisting_only {
            struct_ser.serialize_field("preexistingOnly", &self.preexisting_only)?;
        }
        if let Some(v) = self.owner_filter.as_ref() {
            match v {
                stream_integrations_request::OwnerFilter::AllNodes(v) => {
                    struct_ser.serialize_field("allNodes", v)?;
                }
                stream_integrations_request::OwnerFilter::LocalNode(v) => {
                    struct_ser.serialize_field("localNode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIntegrationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preexisting_only",
            "preexistingOnly",
            "all_nodes",
            "allNodes",
            "local_node",
            "localNode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreexistingOnly,
            AllNodes,
            LocalNode,
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
                            "preexistingOnly" | "preexisting_only" => Ok(GeneratedField::PreexistingOnly),
                            "allNodes" | "all_nodes" => Ok(GeneratedField::AllNodes),
                            "localNode" | "local_node" => Ok(GeneratedField::LocalNode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIntegrationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.StreamIntegrationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamIntegrationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preexisting_only__ = None;
                let mut owner_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreexistingOnly => {
                            if preexisting_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preexistingOnly"));
                            }
                            preexisting_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllNodes => {
                            if owner_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allNodes"));
                            }
                            owner_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_integrations_request::OwnerFilter::AllNodes)
;
                        }
                        GeneratedField::LocalNode => {
                            if owner_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localNode"));
                            }
                            owner_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_integrations_request::OwnerFilter::LocalNode)
;
                        }
                    }
                }
                Ok(StreamIntegrationsRequest {
                    preexisting_only: preexisting_only__.unwrap_or_default(),
                    owner_filter: owner_filter__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.StreamIntegrationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIntegrationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.integration_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.StreamIntegrationsResponse", len)?;
        if let Some(v) = self.integration_event.as_ref() {
            struct_ser.serialize_field("integrationEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIntegrationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integration_event",
            "integrationEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegrationEvent,
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
                            "integrationEvent" | "integration_event" => Ok(GeneratedField::IntegrationEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIntegrationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.StreamIntegrationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamIntegrationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut integration_event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegrationEvent => {
                            if integration_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrationEvent"));
                            }
                            integration_event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamIntegrationsResponse {
                    integration_event: integration_event__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.StreamIntegrationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_type.is_empty() {
            len += 1;
        }
        if self.preexisting_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.StreamRulesRequest", len)?;
        if !self.rule_type.is_empty() {
            let v = self.rule_type.iter().cloned().map(|v| {
                RuleType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("ruleType", &v)?;
        }
        if self.preexisting_only {
            struct_ser.serialize_field("preexistingOnly", &self.preexisting_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_type",
            "ruleType",
            "preexisting_only",
            "preexistingOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleType,
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
                            "ruleType" | "rule_type" => Ok(GeneratedField::RuleType),
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
            type Value = StreamRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.StreamRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_type__ = None;
                let mut preexisting_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleType => {
                            if rule_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleType"));
                            }
                            rule_type__ = Some(map_.next_value::<Vec<RuleType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::PreexistingOnly => {
                            if preexisting_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preexistingOnly"));
                            }
                            preexisting_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamRulesRequest {
                    rule_type: rule_type__.unwrap_or_default(),
                    preexisting_only: preexisting_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.StreamRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule_event.is_some() {
            len += 1;
        }
        if self.distribution_rule_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.communicationsmanager.v1.StreamRulesResponse", len)?;
        if let Some(v) = self.rule_event.as_ref() {
            struct_ser.serialize_field("ruleEvent", v)?;
        }
        if let Some(v) = self.distribution_rule_event.as_ref() {
            struct_ser.serialize_field("distributionRuleEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_event",
            "ruleEvent",
            "distribution_rule_event",
            "distributionRuleEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleEvent,
            DistributionRuleEvent,
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
                            "ruleEvent" | "rule_event" => Ok(GeneratedField::RuleEvent),
                            "distributionRuleEvent" | "distribution_rule_event" => Ok(GeneratedField::DistributionRuleEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.communicationsmanager.v1.StreamRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_event__ = None;
                let mut distribution_rule_event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleEvent => {
                            if rule_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleEvent"));
                            }
                            rule_event__ = map_.next_value()?;
                        }
                        GeneratedField::DistributionRuleEvent => {
                            if distribution_rule_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distributionRuleEvent"));
                            }
                            distribution_rule_event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamRulesResponse {
                    rule_event: rule_event__,
                    distribution_rule_event: distribution_rule_event__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.communicationsmanager.v1.StreamRulesResponse", FIELDS, GeneratedVisitor)
    }
}
