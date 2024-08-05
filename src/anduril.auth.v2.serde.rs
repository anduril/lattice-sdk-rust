// @generated
impl serde::Serialize for ExternalIdp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.sso_url.is_empty() {
            len += 1;
        }
        if !self.unique_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.ExternalIDP", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.sso_url.is_empty() {
            struct_ser.serialize_field("ssoUrl", &self.sso_url)?;
        }
        if !self.unique_id.is_empty() {
            struct_ser.serialize_field("uniqueId", &self.unique_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalIdp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "sso_url",
            "ssoUrl",
            "unique_id",
            "uniqueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SsoUrl,
            UniqueId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "ssoUrl" | "sso_url" => Ok(GeneratedField::SsoUrl),
                            "uniqueId" | "unique_id" => Ok(GeneratedField::UniqueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalIdp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.ExternalIDP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternalIdp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut sso_url__ = None;
                let mut unique_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SsoUrl => {
                            if sso_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ssoUrl"));
                            }
                            sso_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UniqueId => {
                            if unique_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueId"));
                            }
                            unique_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExternalIdp {
                    name: name__.unwrap_or_default(),
                    sso_url: sso_url__.unwrap_or_default(),
                    unique_id: unique_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.ExternalIDP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSsourlRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.redirect_url.is_empty() {
            len += 1;
        }
        if self.append_token {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.GetSSOURLRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.redirect_url.is_empty() {
            struct_ser.serialize_field("redirectUrl", &self.redirect_url)?;
        }
        if self.append_token {
            struct_ser.serialize_field("appendToken", &self.append_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSsourlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "redirect_url",
            "redirectUrl",
            "append_token",
            "appendToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            RedirectUrl,
            AppendToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "redirectUrl" | "redirect_url" => Ok(GeneratedField::RedirectUrl),
                            "appendToken" | "append_token" => Ok(GeneratedField::AppendToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSsourlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.GetSSOURLRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSsourlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut redirect_url__ = None;
                let mut append_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedirectUrl => {
                            if redirect_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectUrl"));
                            }
                            redirect_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppendToken => {
                            if append_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendToken"));
                            }
                            append_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSsourlRequest {
                    email: email__.unwrap_or_default(),
                    redirect_url: redirect_url__.unwrap_or_default(),
                    append_token: append_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.GetSSOURLRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSsourlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.GetSSOURLResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                get_ssourl_response::Response::InvalidEmail(v) => {
                    struct_ser.serialize_field("invalidEmail", v)?;
                }
                get_ssourl_response::Response::ValidDomain(v) => {
                    struct_ser.serialize_field("validDomain", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSsourlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "invalid_email",
            "invalidEmail",
            "valid_domain",
            "validDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InvalidEmail,
            ValidDomain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "invalidEmail" | "invalid_email" => Ok(GeneratedField::InvalidEmail),
                            "validDomain" | "valid_domain" => Ok(GeneratedField::ValidDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSsourlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.GetSSOURLResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSsourlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InvalidEmail => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invalidEmail"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_ssourl_response::Response::InvalidEmail)
;
                        }
                        GeneratedField::ValidDomain => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validDomain"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_ssourl_response::Response::ValidDomain)
;
                        }
                    }
                }
                Ok(GetSsourlResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.GetSSOURLResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_ssourl_response::InvalidEmail {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.GetSSOURLResponse.InvalidEmail", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_ssourl_response::InvalidEmail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_ssourl_response::InvalidEmail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.GetSSOURLResponse.InvalidEmail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_ssourl_response::InvalidEmail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(get_ssourl_response::InvalidEmail {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.GetSSOURLResponse.InvalidEmail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_ssourl_response::ValidDomain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.idps.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.GetSSOURLResponse.ValidDomain", len)?;
        if !self.idps.is_empty() {
            struct_ser.serialize_field("idps", &self.idps)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_ssourl_response::ValidDomain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Idps,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "idps" => Ok(GeneratedField::Idps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_ssourl_response::ValidDomain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.GetSSOURLResponse.ValidDomain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_ssourl_response::ValidDomain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Idps => {
                            if idps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idps"));
                            }
                            idps__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(get_ssourl_response::ValidDomain {
                    idps: idps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.GetSSOURLResponse.ValidDomain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Idp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.idp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.Idp", len)?;
        if let Some(v) = self.idp.as_ref() {
            match v {
                idp::Idp::Internal(v) => {
                    struct_ser.serialize_field("internal", v)?;
                }
                idp::Idp::External(v) => {
                    struct_ser.serialize_field("external", v)?;
                }
                idp::Idp::Piv(v) => {
                    struct_ser.serialize_field("piv", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Idp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "internal",
            "external",
            "piv",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Internal,
            External,
            Piv,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "internal" => Ok(GeneratedField::Internal),
                            "external" => Ok(GeneratedField::External),
                            "piv" => Ok(GeneratedField::Piv),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Idp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.Idp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Idp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Internal => {
                            if idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internal"));
                            }
                            idp__ = map_.next_value::<::std::option::Option<_>>()?.map(idp::Idp::Internal)
;
                        }
                        GeneratedField::External => {
                            if idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("external"));
                            }
                            idp__ = map_.next_value::<::std::option::Option<_>>()?.map(idp::Idp::External)
;
                        }
                        GeneratedField::Piv => {
                            if idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("piv"));
                            }
                            idp__ = map_.next_value::<::std::option::Option<_>>()?.map(idp::Idp::Piv)
;
                        }
                    }
                }
                Ok(Idp {
                    idp: idp__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.Idp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InternalIdp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.auth.v2.InternalIDP", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalIdp {
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
            type Value = InternalIdp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.InternalIDP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InternalIdp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InternalIdp {
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.InternalIDP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Piv {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("anduril.auth.v2.PIV", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Piv {
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
            type Value = Piv;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.PIV")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Piv, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Piv {
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.PIV", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshOidcTokensRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.RefreshOidcTokensRequest", len)?;
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshOidcTokensRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_token",
            "refreshToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshOidcTokensRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.RefreshOidcTokensRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshOidcTokensRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshOidcTokensRequest {
                    refresh_token: refresh_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.RefreshOidcTokensRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshOidcTokensResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token_validity_duration.is_some() {
            len += 1;
        }
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        if !self.id_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.auth.v2.RefreshOidcTokensResponse", len)?;
        if let Some(v) = self.token_validity_duration.as_ref() {
            struct_ser.serialize_field("tokenValidityDuration", v)?;
        }
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        if !self.id_token.is_empty() {
            struct_ser.serialize_field("idToken", &self.id_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshOidcTokensResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_validity_duration",
            "tokenValidityDuration",
            "refresh_token",
            "refreshToken",
            "id_token",
            "idToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenValidityDuration,
            RefreshToken,
            IdToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokenValidityDuration" | "token_validity_duration" => Ok(GeneratedField::TokenValidityDuration),
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            "idToken" | "id_token" => Ok(GeneratedField::IdToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshOidcTokensResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.auth.v2.RefreshOidcTokensResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshOidcTokensResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_validity_duration__ = None;
                let mut refresh_token__ = None;
                let mut id_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TokenValidityDuration => {
                            if token_validity_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenValidityDuration"));
                            }
                            token_validity_duration__ = map_.next_value()?;
                        }
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdToken => {
                            if id_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idToken"));
                            }
                            id_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshOidcTokensResponse {
                    token_validity_duration: token_validity_duration__,
                    refresh_token: refresh_token__.unwrap_or_default(),
                    id_token: id_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.auth.v2.RefreshOidcTokensResponse", FIELDS, GeneratedVisitor)
    }
}
