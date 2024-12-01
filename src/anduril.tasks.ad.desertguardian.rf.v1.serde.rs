// @generated
impl serde::Serialize for EmissionControlState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "EMISSION_CONTROL_STATE_INVALID",
            Self::Allowed => "EMISSION_CONTROL_STATE_ALLOWED",
            Self::NotAllowed => "EMISSION_CONTROL_STATE_NOT_ALLOWED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EmissionControlState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EMISSION_CONTROL_STATE_INVALID",
            "EMISSION_CONTROL_STATE_ALLOWED",
            "EMISSION_CONTROL_STATE_NOT_ALLOWED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmissionControlState;

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
                    "EMISSION_CONTROL_STATE_INVALID" => Ok(EmissionControlState::Invalid),
                    "EMISSION_CONTROL_STATE_ALLOWED" => Ok(EmissionControlState::Allowed),
                    "EMISSION_CONTROL_STATE_NOT_ALLOWED" => Ok(EmissionControlState::NotAllowed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SetEmissionControlState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.emcon_state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetEmissionControlState", len)?;
        if self.emcon_state != 0 {
            let v = EmissionControlState::try_from(self.emcon_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.emcon_state)))?;
            struct_ser.serialize_field("emconState", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetEmissionControlState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "emcon_state",
            "emconState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmconState,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "emconState" | "emcon_state" => Ok(GeneratedField::EmconState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetEmissionControlState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.ad.desertguardian.rf.v1.SetEmissionControlState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetEmissionControlState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut emcon_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmconState => {
                            if emcon_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emconState"));
                            }
                            emcon_state__ = Some(map_.next_value::<EmissionControlState>()? as i32);
                        }
                    }
                }
                Ok(SetEmissionControlState {
                    emcon_state: emcon_state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetEmissionControlState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSurveillanceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.surveillance_state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetSurveillanceState", len)?;
        if self.surveillance_state != 0 {
            let v = SurveillanceState::try_from(self.surveillance_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.surveillance_state)))?;
            struct_ser.serialize_field("surveillanceState", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSurveillanceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "surveillance_state",
            "surveillanceState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SurveillanceState,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "surveillanceState" | "surveillance_state" => Ok(GeneratedField::SurveillanceState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSurveillanceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.ad.desertguardian.rf.v1.SetSurveillanceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSurveillanceState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut surveillance_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SurveillanceState => {
                            if surveillance_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("surveillanceState"));
                            }
                            surveillance_state__ = Some(map_.next_value::<SurveillanceState>()? as i32);
                        }
                    }
                }
                Ok(SetSurveillanceState {
                    surveillance_state: surveillance_state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetSurveillanceState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetTransmitState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transmit_state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetTransmitState", len)?;
        if self.transmit_state != 0 {
            let v = TransmitState::try_from(self.transmit_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.transmit_state)))?;
            struct_ser.serialize_field("transmitState", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetTransmitState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transmit_state",
            "transmitState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransmitState,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transmitState" | "transmit_state" => Ok(GeneratedField::TransmitState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetTransmitState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.tasks.ad.desertguardian.rf.v1.SetTransmitState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetTransmitState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transmit_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransmitState => {
                            if transmit_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transmitState"));
                            }
                            transmit_state__ = Some(map_.next_value::<TransmitState>()? as i32);
                        }
                    }
                }
                Ok(SetTransmitState {
                    transmit_state: transmit_state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.tasks.ad.desertguardian.rf.v1.SetTransmitState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SurveillanceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "SURVEILLANCE_STATE_INVALID",
            Self::Surveilling => "SURVEILLANCE_STATE_SURVEILLING",
            Self::NotSurveilling => "SURVEILLANCE_STATE_NOT_SURVEILLING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SurveillanceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SURVEILLANCE_STATE_INVALID",
            "SURVEILLANCE_STATE_SURVEILLING",
            "SURVEILLANCE_STATE_NOT_SURVEILLING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SurveillanceState;

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
                    "SURVEILLANCE_STATE_INVALID" => Ok(SurveillanceState::Invalid),
                    "SURVEILLANCE_STATE_SURVEILLING" => Ok(SurveillanceState::Surveilling),
                    "SURVEILLANCE_STATE_NOT_SURVEILLING" => Ok(SurveillanceState::NotSurveilling),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TransmitState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "TRANSMIT_STATE_INVALID",
            Self::Transmitting => "TRANSMIT_STATE_TRANSMITTING",
            Self::NotTransmitting => "TRANSMIT_STATE_NOT_TRANSMITTING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransmitState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRANSMIT_STATE_INVALID",
            "TRANSMIT_STATE_TRANSMITTING",
            "TRANSMIT_STATE_NOT_TRANSMITTING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransmitState;

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
                    "TRANSMIT_STATE_INVALID" => Ok(TransmitState::Invalid),
                    "TRANSMIT_STATE_TRANSMITTING" => Ok(TransmitState::Transmitting),
                    "TRANSMIT_STATE_NOT_TRANSMITTING" => Ok(TransmitState::NotTransmitting),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
