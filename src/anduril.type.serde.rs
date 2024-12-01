// @generated
impl serde::Serialize for AerPolygon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.points.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.AERPolygon", len)?;
        if !self.points.is_empty() {
            struct_ser.serialize_field("points", &self.points)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AerPolygon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "points",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Points,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "points" => Ok(GeneratedField::Points),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AerPolygon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.AERPolygon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AerPolygon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut points__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Points => {
                            if points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("points"));
                            }
                            points__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AerPolygon {
                    points: points__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.AERPolygon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Attribution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Attribution", len)?;
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Attribution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
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
            type Value = Attribution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Attribution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Attribution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Attribution {
                    timestamp: timestamp__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Attribution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Color {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.red != 0. {
            len += 1;
        }
        if self.green != 0. {
            len += 1;
        }
        if self.blue != 0. {
            len += 1;
        }
        if self.alpha.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Color", len)?;
        if self.red != 0. {
            struct_ser.serialize_field("red", &self.red)?;
        }
        if self.green != 0. {
            struct_ser.serialize_field("green", &self.green)?;
        }
        if self.blue != 0. {
            struct_ser.serialize_field("blue", &self.blue)?;
        }
        if let Some(v) = self.alpha.as_ref() {
            struct_ser.serialize_field("alpha", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Color {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "red",
            "green",
            "blue",
            "alpha",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Red,
            Green,
            Blue,
            Alpha,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "red" => Ok(GeneratedField::Red),
                            "green" => Ok(GeneratedField::Green),
                            "blue" => Ok(GeneratedField::Blue),
                            "alpha" => Ok(GeneratedField::Alpha),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Color")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Color, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut red__ = None;
                let mut green__ = None;
                let mut blue__ = None;
                let mut alpha__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Red => {
                            if red__.is_some() {
                                return Err(serde::de::Error::duplicate_field("red"));
                            }
                            red__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Green => {
                            if green__.is_some() {
                                return Err(serde::de::Error::duplicate_field("green"));
                            }
                            green__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Blue => {
                            if blue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blue"));
                            }
                            blue__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Alpha => {
                            if alpha__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpha"));
                            }
                            alpha__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Color {
                    red: red__.unwrap_or_default(),
                    green: green__.unwrap_or_default(),
                    blue: blue__.unwrap_or_default(),
                    alpha: alpha__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Color", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleRange {
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
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.DoubleRange", len)?;
        if self.min != 0. {
            struct_ser.serialize_field("min", &self.min)?;
        }
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRange {
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
            type Value = DoubleRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.DoubleRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleRange, V::Error>
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
                Ok(DoubleRange {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.DoubleRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Eci {
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
        if self.z != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.ECI", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Eci {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "z" => Ok(GeneratedField::Z),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Eci;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.ECI")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Eci, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
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
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Eci {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.ECI", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Enu {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.e != 0. {
            len += 1;
        }
        if self.n != 0. {
            len += 1;
        }
        if self.u != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.ENU", len)?;
        if self.e != 0. {
            struct_ser.serialize_field("e", &self.e)?;
        }
        if self.n != 0. {
            struct_ser.serialize_field("n", &self.n)?;
        }
        if self.u != 0. {
            struct_ser.serialize_field("u", &self.u)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Enu {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "e",
            "n",
            "u",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            E,
            N,
            U,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "e" => Ok(GeneratedField::E),
                            "n" => Ok(GeneratedField::N),
                            "u" => Ok(GeneratedField::U),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Enu;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.ENU")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Enu, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut e__ = None;
                let mut n__ = None;
                let mut u__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::E => {
                            if e__.is_some() {
                                return Err(serde::de::Error::duplicate_field("e"));
                            }
                            e__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::N => {
                            if n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("n"));
                            }
                            n__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::U => {
                            if u__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u"));
                            }
                            u__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Enu {
                    e: e__.unwrap_or_default(),
                    n: n__.unwrap_or_default(),
                    u: u__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.ENU", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EciReferenceFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ECI_REFERENCE_FRAME_INVALID",
            Self::Teme => "ECI_REFERENCE_FRAME_TEME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EciReferenceFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ECI_REFERENCE_FRAME_INVALID",
            "ECI_REFERENCE_FRAME_TEME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EciReferenceFrame;

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
                    "ECI_REFERENCE_FRAME_INVALID" => Ok(EciReferenceFrame::Invalid),
                    "ECI_REFERENCE_FRAME_TEME" => Ok(EciReferenceFrame::Teme),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Grid {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bottom_left_pos.is_some() {
            len += 1;
        }
        if self.top_right_pos.is_some() {
            len += 1;
        }
        if self.grid_width != 0 {
            len += 1;
        }
        if self.grid_height != 0 {
            len += 1;
        }
        if !self.cell_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Grid", len)?;
        if let Some(v) = self.bottom_left_pos.as_ref() {
            struct_ser.serialize_field("bottomLeftPos", v)?;
        }
        if let Some(v) = self.top_right_pos.as_ref() {
            struct_ser.serialize_field("topRightPos", v)?;
        }
        if self.grid_width != 0 {
            struct_ser.serialize_field("gridWidth", &self.grid_width)?;
        }
        if self.grid_height != 0 {
            struct_ser.serialize_field("gridHeight", &self.grid_height)?;
        }
        if !self.cell_values.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cellValues", pbjson::private::base64::encode(&self.cell_values).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Grid {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bottom_left_pos",
            "bottomLeftPos",
            "top_right_pos",
            "topRightPos",
            "grid_width",
            "gridWidth",
            "grid_height",
            "gridHeight",
            "cell_values",
            "cellValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BottomLeftPos,
            TopRightPos,
            GridWidth,
            GridHeight,
            CellValues,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bottomLeftPos" | "bottom_left_pos" => Ok(GeneratedField::BottomLeftPos),
                            "topRightPos" | "top_right_pos" => Ok(GeneratedField::TopRightPos),
                            "gridWidth" | "grid_width" => Ok(GeneratedField::GridWidth),
                            "gridHeight" | "grid_height" => Ok(GeneratedField::GridHeight),
                            "cellValues" | "cell_values" => Ok(GeneratedField::CellValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Grid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Grid")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Grid, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bottom_left_pos__ = None;
                let mut top_right_pos__ = None;
                let mut grid_width__ = None;
                let mut grid_height__ = None;
                let mut cell_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BottomLeftPos => {
                            if bottom_left_pos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottomLeftPos"));
                            }
                            bottom_left_pos__ = map_.next_value()?;
                        }
                        GeneratedField::TopRightPos => {
                            if top_right_pos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topRightPos"));
                            }
                            top_right_pos__ = map_.next_value()?;
                        }
                        GeneratedField::GridWidth => {
                            if grid_width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gridWidth"));
                            }
                            grid_width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GridHeight => {
                            if grid_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gridHeight"));
                            }
                            grid_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CellValues => {
                            if cell_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellValues"));
                            }
                            cell_values__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Grid {
                    bottom_left_pos: bottom_left_pos__,
                    top_right_pos: top_right_pos__,
                    grid_width: grid_width__.unwrap_or_default(),
                    grid_height: grid_height__.unwrap_or_default(),
                    cell_values: cell_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Grid", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Lla {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lon != 0. {
            len += 1;
        }
        if self.lat != 0. {
            len += 1;
        }
        if self.alt != 0. {
            len += 1;
        }
        if self.is2d {
            len += 1;
        }
        if self.altitude_reference != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.LLA", len)?;
        if self.lon != 0. {
            struct_ser.serialize_field("lon", &self.lon)?;
        }
        if self.lat != 0. {
            struct_ser.serialize_field("lat", &self.lat)?;
        }
        if self.alt != 0. {
            struct_ser.serialize_field("alt", &self.alt)?;
        }
        if self.is2d {
            struct_ser.serialize_field("is2d", &self.is2d)?;
        }
        if self.altitude_reference != 0 {
            let v = lla::AltitudeReference::try_from(self.altitude_reference)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.altitude_reference)))?;
            struct_ser.serialize_field("altitudeReference", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Lla {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lon",
            "lat",
            "alt",
            "is2d",
            "altitude_reference",
            "altitudeReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lon,
            Lat,
            Alt,
            Is2d,
            AltitudeReference,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "lon" => Ok(GeneratedField::Lon),
                            "lat" => Ok(GeneratedField::Lat),
                            "alt" => Ok(GeneratedField::Alt),
                            "is2d" => Ok(GeneratedField::Is2d),
                            "altitudeReference" | "altitude_reference" => Ok(GeneratedField::AltitudeReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Lla;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.LLA")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Lla, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lon__ = None;
                let mut lat__ = None;
                let mut alt__ = None;
                let mut is2d__ = None;
                let mut altitude_reference__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Lon => {
                            if lon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lon"));
                            }
                            lon__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Lat => {
                            if lat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lat"));
                            }
                            lat__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Alt => {
                            if alt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alt"));
                            }
                            alt__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Is2d => {
                            if is2d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("is2d"));
                            }
                            is2d__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AltitudeReference => {
                            if altitude_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitudeReference"));
                            }
                            altitude_reference__ = Some(map_.next_value::<lla::AltitudeReference>()? as i32);
                        }
                    }
                }
                Ok(Lla {
                    lon: lon__.unwrap_or_default(),
                    lat: lat__.unwrap_or_default(),
                    alt: alt__.unwrap_or_default(),
                    is2d: is2d__.unwrap_or_default(),
                    altitude_reference: altitude_reference__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.LLA", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lla::AltitudeReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "ALTITUDE_REFERENCE_INVALID",
            Self::HeightAboveWgs84 => "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84",
            Self::HeightAboveEgm96 => "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96",
            Self::Unknown => "ALTITUDE_REFERENCE_UNKNOWN",
            Self::Barometric => "ALTITUDE_REFERENCE_BAROMETRIC",
            Self::AboveSeaFloor => "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR",
            Self::BelowSeaSurface => "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lla::AltitudeReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALTITUDE_REFERENCE_INVALID",
            "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84",
            "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96",
            "ALTITUDE_REFERENCE_UNKNOWN",
            "ALTITUDE_REFERENCE_BAROMETRIC",
            "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR",
            "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lla::AltitudeReference;

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
                    "ALTITUDE_REFERENCE_INVALID" => Ok(lla::AltitudeReference::Invalid),
                    "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84" => Ok(lla::AltitudeReference::HeightAboveWgs84),
                    "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96" => Ok(lla::AltitudeReference::HeightAboveEgm96),
                    "ALTITUDE_REFERENCE_UNKNOWN" => Ok(lla::AltitudeReference::Unknown),
                    "ALTITUDE_REFERENCE_BAROMETRIC" => Ok(lla::AltitudeReference::Barometric),
                    "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR" => Ok(lla::AltitudeReference::AboveSeaFloor),
                    "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE" => Ok(lla::AltitudeReference::BelowSeaSurface),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LlaPath {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.points.is_empty() {
            len += 1;
        }
        if self.r#loop {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.LLAPath", len)?;
        if !self.points.is_empty() {
            struct_ser.serialize_field("points", &self.points)?;
        }
        if self.r#loop {
            struct_ser.serialize_field("loop", &self.r#loop)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LlaPath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "points",
            "loop",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Points,
            Loop,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "points" => Ok(GeneratedField::Points),
                            "loop" => Ok(GeneratedField::Loop),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LlaPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.LLAPath")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LlaPath, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut points__ = None;
                let mut r#loop__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Points => {
                            if points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("points"));
                            }
                            points__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Loop => {
                            if r#loop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loop"));
                            }
                            r#loop__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LlaPath {
                    points: points__.unwrap_or_default(),
                    r#loop: r#loop__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.LLAPath", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LlaPolygon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.points.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.LLAPolygon", len)?;
        if !self.points.is_empty() {
            struct_ser.serialize_field("points", &self.points)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LlaPolygon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "points",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Points,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "points" => Ok(GeneratedField::Points),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LlaPolygon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.LLAPolygon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LlaPolygon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut points__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Points => {
                            if points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("points"));
                            }
                            points__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LlaPolygon {
                    points: points__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.LLAPolygon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeanElementTheory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "MEAN_ELEMENT_THEORY_INVALID",
            Self::Sgp4 => "MEAN_ELEMENT_THEORY_SGP4",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MeanElementTheory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEAN_ELEMENT_THEORY_INVALID",
            "MEAN_ELEMENT_THEORY_SGP4",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeanElementTheory;

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
                    "MEAN_ELEMENT_THEORY_INVALID" => Ok(MeanElementTheory::Invalid),
                    "MEAN_ELEMENT_THEORY_SGP4" => Ok(MeanElementTheory::Sgp4),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MeanKeplerianElements {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch.is_some() {
            len += 1;
        }
        if self.eccentricity != 0. {
            len += 1;
        }
        if self.inclination_deg != 0. {
            len += 1;
        }
        if self.ra_of_asc_node_deg != 0. {
            len += 1;
        }
        if self.arg_of_pericenter_deg != 0. {
            len += 1;
        }
        if self.mean_anomaly_deg != 0. {
            len += 1;
        }
        if self.gm.is_some() {
            len += 1;
        }
        if self.line2_field8.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.MeanKeplerianElements", len)?;
        if let Some(v) = self.epoch.as_ref() {
            struct_ser.serialize_field("epoch", v)?;
        }
        if self.eccentricity != 0. {
            struct_ser.serialize_field("eccentricity", &self.eccentricity)?;
        }
        if self.inclination_deg != 0. {
            struct_ser.serialize_field("inclinationDeg", &self.inclination_deg)?;
        }
        if self.ra_of_asc_node_deg != 0. {
            struct_ser.serialize_field("raOfAscNodeDeg", &self.ra_of_asc_node_deg)?;
        }
        if self.arg_of_pericenter_deg != 0. {
            struct_ser.serialize_field("argOfPericenterDeg", &self.arg_of_pericenter_deg)?;
        }
        if self.mean_anomaly_deg != 0. {
            struct_ser.serialize_field("meanAnomalyDeg", &self.mean_anomaly_deg)?;
        }
        if let Some(v) = self.gm.as_ref() {
            struct_ser.serialize_field("gm", v)?;
        }
        if let Some(v) = self.line2_field8.as_ref() {
            match v {
                mean_keplerian_elements::Line2Field8::SemiMajorAxisKm(v) => {
                    struct_ser.serialize_field("semiMajorAxisKm", v)?;
                }
                mean_keplerian_elements::Line2Field8::MeanMotion(v) => {
                    struct_ser.serialize_field("meanMotion", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeanKeplerianElements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
            "eccentricity",
            "inclination_deg",
            "inclinationDeg",
            "ra_of_asc_node_deg",
            "raOfAscNodeDeg",
            "arg_of_pericenter_deg",
            "argOfPericenterDeg",
            "mean_anomaly_deg",
            "meanAnomalyDeg",
            "gm",
            "semi_major_axis_km",
            "semiMajorAxisKm",
            "mean_motion",
            "meanMotion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            Eccentricity,
            InclinationDeg,
            RaOfAscNodeDeg,
            ArgOfPericenterDeg,
            MeanAnomalyDeg,
            Gm,
            SemiMajorAxisKm,
            MeanMotion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            "eccentricity" => Ok(GeneratedField::Eccentricity),
                            "inclinationDeg" | "inclination_deg" => Ok(GeneratedField::InclinationDeg),
                            "raOfAscNodeDeg" | "ra_of_asc_node_deg" => Ok(GeneratedField::RaOfAscNodeDeg),
                            "argOfPericenterDeg" | "arg_of_pericenter_deg" => Ok(GeneratedField::ArgOfPericenterDeg),
                            "meanAnomalyDeg" | "mean_anomaly_deg" => Ok(GeneratedField::MeanAnomalyDeg),
                            "gm" => Ok(GeneratedField::Gm),
                            "semiMajorAxisKm" | "semi_major_axis_km" => Ok(GeneratedField::SemiMajorAxisKm),
                            "meanMotion" | "mean_motion" => Ok(GeneratedField::MeanMotion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeanKeplerianElements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.MeanKeplerianElements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeanKeplerianElements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut eccentricity__ = None;
                let mut inclination_deg__ = None;
                let mut ra_of_asc_node_deg__ = None;
                let mut arg_of_pericenter_deg__ = None;
                let mut mean_anomaly_deg__ = None;
                let mut gm__ = None;
                let mut line2_field8__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = map_.next_value()?;
                        }
                        GeneratedField::Eccentricity => {
                            if eccentricity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eccentricity"));
                            }
                            eccentricity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InclinationDeg => {
                            if inclination_deg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inclinationDeg"));
                            }
                            inclination_deg__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RaOfAscNodeDeg => {
                            if ra_of_asc_node_deg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raOfAscNodeDeg"));
                            }
                            ra_of_asc_node_deg__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ArgOfPericenterDeg => {
                            if arg_of_pericenter_deg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("argOfPericenterDeg"));
                            }
                            arg_of_pericenter_deg__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MeanAnomalyDeg => {
                            if mean_anomaly_deg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanAnomalyDeg"));
                            }
                            mean_anomaly_deg__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Gm => {
                            if gm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gm"));
                            }
                            gm__ = map_.next_value()?;
                        }
                        GeneratedField::SemiMajorAxisKm => {
                            if line2_field8__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semiMajorAxisKm"));
                            }
                            line2_field8__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| mean_keplerian_elements::Line2Field8::SemiMajorAxisKm(x.0));
                        }
                        GeneratedField::MeanMotion => {
                            if line2_field8__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanMotion"));
                            }
                            line2_field8__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| mean_keplerian_elements::Line2Field8::MeanMotion(x.0));
                        }
                    }
                }
                Ok(MeanKeplerianElements {
                    epoch: epoch__,
                    eccentricity: eccentricity__.unwrap_or_default(),
                    inclination_deg: inclination_deg__.unwrap_or_default(),
                    ra_of_asc_node_deg: ra_of_asc_node_deg__.unwrap_or_default(),
                    arg_of_pericenter_deg: arg_of_pericenter_deg__.unwrap_or_default(),
                    mean_anomaly_deg: mean_anomaly_deg__.unwrap_or_default(),
                    gm: gm__,
                    line2_field8: line2_field8__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.MeanKeplerianElements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitMeanElements {
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
        if self.mean_keplerian_elements.is_some() {
            len += 1;
        }
        if self.tle_parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.OrbitMeanElements", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.mean_keplerian_elements.as_ref() {
            struct_ser.serialize_field("meanKeplerianElements", v)?;
        }
        if let Some(v) = self.tle_parameters.as_ref() {
            struct_ser.serialize_field("tleParameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrbitMeanElements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "mean_keplerian_elements",
            "meanKeplerianElements",
            "tle_parameters",
            "tleParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            MeanKeplerianElements,
            TleParameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "meanKeplerianElements" | "mean_keplerian_elements" => Ok(GeneratedField::MeanKeplerianElements),
                            "tleParameters" | "tle_parameters" => Ok(GeneratedField::TleParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitMeanElements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.OrbitMeanElements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrbitMeanElements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut mean_keplerian_elements__ = None;
                let mut tle_parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::MeanKeplerianElements => {
                            if mean_keplerian_elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanKeplerianElements"));
                            }
                            mean_keplerian_elements__ = map_.next_value()?;
                        }
                        GeneratedField::TleParameters => {
                            if tle_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tleParameters"));
                            }
                            tle_parameters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrbitMeanElements {
                    metadata: metadata__,
                    mean_keplerian_elements: mean_keplerian_elements__,
                    tle_parameters: tle_parameters__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.OrbitMeanElements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrbitMeanElementsMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.originator.is_some() {
            len += 1;
        }
        if self.message_id.is_some() {
            len += 1;
        }
        if self.ref_frame != 0 {
            len += 1;
        }
        if self.ref_frame_epoch.is_some() {
            len += 1;
        }
        if self.mean_element_theory != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.OrbitMeanElementsMetadata", len)?;
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.originator.as_ref() {
            struct_ser.serialize_field("originator", v)?;
        }
        if let Some(v) = self.message_id.as_ref() {
            struct_ser.serialize_field("messageId", v)?;
        }
        if self.ref_frame != 0 {
            let v = EciReferenceFrame::try_from(self.ref_frame)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.ref_frame)))?;
            struct_ser.serialize_field("refFrame", &v)?;
        }
        if let Some(v) = self.ref_frame_epoch.as_ref() {
            struct_ser.serialize_field("refFrameEpoch", v)?;
        }
        if self.mean_element_theory != 0 {
            let v = MeanElementTheory::try_from(self.mean_element_theory)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mean_element_theory)))?;
            struct_ser.serialize_field("meanElementTheory", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrbitMeanElementsMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_date",
            "creationDate",
            "originator",
            "message_id",
            "messageId",
            "ref_frame",
            "refFrame",
            "ref_frame_epoch",
            "refFrameEpoch",
            "mean_element_theory",
            "meanElementTheory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationDate,
            Originator,
            MessageId,
            RefFrame,
            RefFrameEpoch,
            MeanElementTheory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "originator" => Ok(GeneratedField::Originator),
                            "messageId" | "message_id" => Ok(GeneratedField::MessageId),
                            "refFrame" | "ref_frame" => Ok(GeneratedField::RefFrame),
                            "refFrameEpoch" | "ref_frame_epoch" => Ok(GeneratedField::RefFrameEpoch),
                            "meanElementTheory" | "mean_element_theory" => Ok(GeneratedField::MeanElementTheory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrbitMeanElementsMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.OrbitMeanElementsMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrbitMeanElementsMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut creation_date__ = None;
                let mut originator__ = None;
                let mut message_id__ = None;
                let mut ref_frame__ = None;
                let mut ref_frame_epoch__ = None;
                let mut mean_element_theory__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::Originator => {
                            if originator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originator"));
                            }
                            originator__ = map_.next_value()?;
                        }
                        GeneratedField::MessageId => {
                            if message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageId"));
                            }
                            message_id__ = map_.next_value()?;
                        }
                        GeneratedField::RefFrame => {
                            if ref_frame__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refFrame"));
                            }
                            ref_frame__ = Some(map_.next_value::<EciReferenceFrame>()? as i32);
                        }
                        GeneratedField::RefFrameEpoch => {
                            if ref_frame_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refFrameEpoch"));
                            }
                            ref_frame_epoch__ = map_.next_value()?;
                        }
                        GeneratedField::MeanElementTheory => {
                            if mean_element_theory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanElementTheory"));
                            }
                            mean_element_theory__ = Some(map_.next_value::<MeanElementTheory>()? as i32);
                        }
                    }
                }
                Ok(OrbitMeanElementsMetadata {
                    creation_date: creation_date__,
                    originator: originator__,
                    message_id: message_id__,
                    ref_frame: ref_frame__.unwrap_or_default(),
                    ref_frame_epoch: ref_frame_epoch__,
                    mean_element_theory: mean_element_theory__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.OrbitMeanElementsMetadata", FIELDS, GeneratedVisitor)
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
        if self.att_enu.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Pose", len)?;
        if let Some(v) = self.pos.as_ref() {
            struct_ser.serialize_field("pos", v)?;
        }
        if let Some(v) = self.att_enu.as_ref() {
            struct_ser.serialize_field("attEnu", v)?;
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
            "att_enu",
            "attEnu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pos,
            AttEnu,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "attEnu" | "att_enu" => Ok(GeneratedField::AttEnu),
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
                formatter.write_str("struct anduril.r#type.Pose")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Pose, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pos__ = None;
                let mut att_enu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pos => {
                            if pos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pos"));
                            }
                            pos__ = map_.next_value()?;
                        }
                        GeneratedField::AttEnu => {
                            if att_enu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attEnu"));
                            }
                            att_enu__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Pose {
                    pos: pos__,
                    att_enu: att_enu__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Pose", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quaternion {
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
        if self.z != 0. {
            len += 1;
        }
        if self.w != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Quaternion", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        if self.w != 0. {
            struct_ser.serialize_field("w", &self.w)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quaternion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
            "w",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
            W,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "z" => Ok(GeneratedField::Z),
                            "w" => Ok(GeneratedField::W),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quaternion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Quaternion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quaternion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
                let mut w__ = None;
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
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::W => {
                            if w__.is_some() {
                                return Err(serde::de::Error::duplicate_field("w"));
                            }
                            w__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Quaternion {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                    w: w__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Quaternion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RigidTransform {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rotation.is_some() {
            len += 1;
        }
        if self.translation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.RigidTransform", len)?;
        if let Some(v) = self.rotation.as_ref() {
            struct_ser.serialize_field("rotation", v)?;
        }
        if let Some(v) = self.translation.as_ref() {
            struct_ser.serialize_field("translation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RigidTransform {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rotation",
            "translation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rotation,
            Translation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rotation" => Ok(GeneratedField::Rotation),
                            "translation" => Ok(GeneratedField::Translation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RigidTransform;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.RigidTransform")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RigidTransform, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rotation__ = None;
                let mut translation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rotation => {
                            if rotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation__ = map_.next_value()?;
                        }
                        GeneratedField::Translation => {
                            if translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translation"));
                            }
                            translation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RigidTransform {
                    rotation: rotation__,
                    translation: translation__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.RigidTransform", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Spherical {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.az != 0. {
            len += 1;
        }
        if self.el != 0. {
            len += 1;
        }
        if self.range != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Spherical", len)?;
        if self.az != 0. {
            struct_ser.serialize_field("az", &self.az)?;
        }
        if self.el != 0. {
            struct_ser.serialize_field("el", &self.el)?;
        }
        if self.range != 0. {
            struct_ser.serialize_field("range", &self.range)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Spherical {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "az",
            "el",
            "range",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Az,
            El,
            Range,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "az" => Ok(GeneratedField::Az),
                            "el" => Ok(GeneratedField::El),
                            "range" => Ok(GeneratedField::Range),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Spherical;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Spherical")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Spherical, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut az__ = None;
                let mut el__ = None;
                let mut range__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Az => {
                            if az__.is_some() {
                                return Err(serde::de::Error::duplicate_field("az"));
                            }
                            az__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::El => {
                            if el__.is_some() {
                                return Err(serde::de::Error::duplicate_field("el"));
                            }
                            el__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Spherical {
                    az: az__.unwrap_or_default(),
                    el: el__.unwrap_or_default(),
                    range: range__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Spherical", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TMat2 {
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
        if self.myy != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.TMat2", len)?;
        if self.mxx != 0. {
            struct_ser.serialize_field("mxx", &self.mxx)?;
        }
        if self.mxy != 0. {
            struct_ser.serialize_field("mxy", &self.mxy)?;
        }
        if self.myy != 0. {
            struct_ser.serialize_field("myy", &self.myy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TMat2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mxx",
            "mxy",
            "myy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mxx,
            Mxy,
            Myy,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "myy" => Ok(GeneratedField::Myy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TMat2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.TMat2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TMat2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mxx__ = None;
                let mut mxy__ = None;
                let mut myy__ = None;
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
                        GeneratedField::Myy => {
                            if myy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("myy"));
                            }
                            myy__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TMat2 {
                    mxx: mxx__.unwrap_or_default(),
                    mxy: mxy__.unwrap_or_default(),
                    myy: myy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.TMat2", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.TMat3", len)?;
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
                formatter.write_str("struct anduril.r#type.TMat3")
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
        deserializer.deserialize_struct("anduril.r#type.TMat3", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TMat4f {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.m00 != 0. {
            len += 1;
        }
        if self.m01 != 0. {
            len += 1;
        }
        if self.m02 != 0. {
            len += 1;
        }
        if self.m03 != 0. {
            len += 1;
        }
        if self.m11 != 0. {
            len += 1;
        }
        if self.m12 != 0. {
            len += 1;
        }
        if self.m13 != 0. {
            len += 1;
        }
        if self.m22 != 0. {
            len += 1;
        }
        if self.m23 != 0. {
            len += 1;
        }
        if self.m33 != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.TMat4f", len)?;
        if self.m00 != 0. {
            struct_ser.serialize_field("m00", &self.m00)?;
        }
        if self.m01 != 0. {
            struct_ser.serialize_field("m01", &self.m01)?;
        }
        if self.m02 != 0. {
            struct_ser.serialize_field("m02", &self.m02)?;
        }
        if self.m03 != 0. {
            struct_ser.serialize_field("m03", &self.m03)?;
        }
        if self.m11 != 0. {
            struct_ser.serialize_field("m11", &self.m11)?;
        }
        if self.m12 != 0. {
            struct_ser.serialize_field("m12", &self.m12)?;
        }
        if self.m13 != 0. {
            struct_ser.serialize_field("m13", &self.m13)?;
        }
        if self.m22 != 0. {
            struct_ser.serialize_field("m22", &self.m22)?;
        }
        if self.m23 != 0. {
            struct_ser.serialize_field("m23", &self.m23)?;
        }
        if self.m33 != 0. {
            struct_ser.serialize_field("m33", &self.m33)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TMat4f {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "m00",
            "m01",
            "m02",
            "m03",
            "m11",
            "m12",
            "m13",
            "m22",
            "m23",
            "m33",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            M00,
            M01,
            M02,
            M03,
            M11,
            M12,
            M13,
            M22,
            M23,
            M33,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "m00" => Ok(GeneratedField::M00),
                            "m01" => Ok(GeneratedField::M01),
                            "m02" => Ok(GeneratedField::M02),
                            "m03" => Ok(GeneratedField::M03),
                            "m11" => Ok(GeneratedField::M11),
                            "m12" => Ok(GeneratedField::M12),
                            "m13" => Ok(GeneratedField::M13),
                            "m22" => Ok(GeneratedField::M22),
                            "m23" => Ok(GeneratedField::M23),
                            "m33" => Ok(GeneratedField::M33),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TMat4f;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.TMat4f")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TMat4f, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut m00__ = None;
                let mut m01__ = None;
                let mut m02__ = None;
                let mut m03__ = None;
                let mut m11__ = None;
                let mut m12__ = None;
                let mut m13__ = None;
                let mut m22__ = None;
                let mut m23__ = None;
                let mut m33__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::M00 => {
                            if m00__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m00"));
                            }
                            m00__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M01 => {
                            if m01__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m01"));
                            }
                            m01__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M02 => {
                            if m02__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m02"));
                            }
                            m02__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M03 => {
                            if m03__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m03"));
                            }
                            m03__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M11 => {
                            if m11__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m11"));
                            }
                            m11__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M12 => {
                            if m12__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m12"));
                            }
                            m12__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M13 => {
                            if m13__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m13"));
                            }
                            m13__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M22 => {
                            if m22__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m22"));
                            }
                            m22__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M23 => {
                            if m23__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m23"));
                            }
                            m23__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::M33 => {
                            if m33__.is_some() {
                                return Err(serde::de::Error::duplicate_field("m33"));
                            }
                            m33__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TMat4f {
                    m00: m00__.unwrap_or_default(),
                    m01: m01__.unwrap_or_default(),
                    m02: m02__.unwrap_or_default(),
                    m03: m03__.unwrap_or_default(),
                    m11: m11__.unwrap_or_default(),
                    m12: m12__.unwrap_or_default(),
                    m13: m13__.unwrap_or_default(),
                    m22: m22__.unwrap_or_default(),
                    m23: m23__.unwrap_or_default(),
                    m33: m33__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.TMat4f", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThetaPhi {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.theta != 0. {
            len += 1;
        }
        if self.phi != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.ThetaPhi", len)?;
        if self.theta != 0. {
            struct_ser.serialize_field("theta", &self.theta)?;
        }
        if self.phi != 0. {
            struct_ser.serialize_field("phi", &self.phi)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThetaPhi {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "theta",
            "phi",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Theta,
            Phi,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "theta" => Ok(GeneratedField::Theta),
                            "phi" => Ok(GeneratedField::Phi),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ThetaPhi;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.ThetaPhi")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ThetaPhi, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut theta__ = None;
                let mut phi__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Theta => {
                            if theta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("theta"));
                            }
                            theta__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Phi => {
                            if phi__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phi"));
                            }
                            phi__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ThetaPhi {
                    theta: theta__.unwrap_or_default(),
                    phi: phi__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.ThetaPhi", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TleParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ephemeris_type.is_some() {
            len += 1;
        }
        if self.classification_type.is_some() {
            len += 1;
        }
        if self.norad_cat_id.is_some() {
            len += 1;
        }
        if self.element_set_no.is_some() {
            len += 1;
        }
        if self.rev_at_epoch.is_some() {
            len += 1;
        }
        if self.mean_motion_dot.is_some() {
            len += 1;
        }
        if self.line1_field11.is_some() {
            len += 1;
        }
        if self.line1_field10.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.TleParameters", len)?;
        if let Some(v) = self.ephemeris_type.as_ref() {
            struct_ser.serialize_field("ephemerisType", v)?;
        }
        if let Some(v) = self.classification_type.as_ref() {
            struct_ser.serialize_field("classificationType", v)?;
        }
        if let Some(v) = self.norad_cat_id.as_ref() {
            struct_ser.serialize_field("noradCatId", v)?;
        }
        if let Some(v) = self.element_set_no.as_ref() {
            struct_ser.serialize_field("elementSetNo", v)?;
        }
        if let Some(v) = self.rev_at_epoch.as_ref() {
            struct_ser.serialize_field("revAtEpoch", v)?;
        }
        if let Some(v) = self.mean_motion_dot.as_ref() {
            struct_ser.serialize_field("meanMotionDot", v)?;
        }
        if let Some(v) = self.line1_field11.as_ref() {
            match v {
                tle_parameters::Line1Field11::Bstar(v) => {
                    struct_ser.serialize_field("bstar", v)?;
                }
                tle_parameters::Line1Field11::Bterm(v) => {
                    struct_ser.serialize_field("bterm", v)?;
                }
            }
        }
        if let Some(v) = self.line1_field10.as_ref() {
            match v {
                tle_parameters::Line1Field10::MeanMotionDdot(v) => {
                    struct_ser.serialize_field("meanMotionDdot", v)?;
                }
                tle_parameters::Line1Field10::Agom(v) => {
                    struct_ser.serialize_field("agom", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TleParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ephemeris_type",
            "ephemerisType",
            "classification_type",
            "classificationType",
            "norad_cat_id",
            "noradCatId",
            "element_set_no",
            "elementSetNo",
            "rev_at_epoch",
            "revAtEpoch",
            "mean_motion_dot",
            "meanMotionDot",
            "bstar",
            "bterm",
            "mean_motion_ddot",
            "meanMotionDdot",
            "agom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemerisType,
            ClassificationType,
            NoradCatId,
            ElementSetNo,
            RevAtEpoch,
            MeanMotionDot,
            Bstar,
            Bterm,
            MeanMotionDdot,
            Agom,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ephemerisType" | "ephemeris_type" => Ok(GeneratedField::EphemerisType),
                            "classificationType" | "classification_type" => Ok(GeneratedField::ClassificationType),
                            "noradCatId" | "norad_cat_id" => Ok(GeneratedField::NoradCatId),
                            "elementSetNo" | "element_set_no" => Ok(GeneratedField::ElementSetNo),
                            "revAtEpoch" | "rev_at_epoch" => Ok(GeneratedField::RevAtEpoch),
                            "meanMotionDot" | "mean_motion_dot" => Ok(GeneratedField::MeanMotionDot),
                            "bstar" => Ok(GeneratedField::Bstar),
                            "bterm" => Ok(GeneratedField::Bterm),
                            "meanMotionDdot" | "mean_motion_ddot" => Ok(GeneratedField::MeanMotionDdot),
                            "agom" => Ok(GeneratedField::Agom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TleParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.TleParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TleParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeris_type__ = None;
                let mut classification_type__ = None;
                let mut norad_cat_id__ = None;
                let mut element_set_no__ = None;
                let mut rev_at_epoch__ = None;
                let mut mean_motion_dot__ = None;
                let mut line1_field11__ = None;
                let mut line1_field10__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemerisType => {
                            if ephemeris_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemerisType"));
                            }
                            ephemeris_type__ = map_.next_value()?;
                        }
                        GeneratedField::ClassificationType => {
                            if classification_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classificationType"));
                            }
                            classification_type__ = map_.next_value()?;
                        }
                        GeneratedField::NoradCatId => {
                            if norad_cat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noradCatId"));
                            }
                            norad_cat_id__ = map_.next_value()?;
                        }
                        GeneratedField::ElementSetNo => {
                            if element_set_no__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elementSetNo"));
                            }
                            element_set_no__ = map_.next_value()?;
                        }
                        GeneratedField::RevAtEpoch => {
                            if rev_at_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revAtEpoch"));
                            }
                            rev_at_epoch__ = map_.next_value()?;
                        }
                        GeneratedField::MeanMotionDot => {
                            if mean_motion_dot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanMotionDot"));
                            }
                            mean_motion_dot__ = map_.next_value()?;
                        }
                        GeneratedField::Bstar => {
                            if line1_field11__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bstar"));
                            }
                            line1_field11__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| tle_parameters::Line1Field11::Bstar(x.0));
                        }
                        GeneratedField::Bterm => {
                            if line1_field11__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bterm"));
                            }
                            line1_field11__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| tle_parameters::Line1Field11::Bterm(x.0));
                        }
                        GeneratedField::MeanMotionDdot => {
                            if line1_field10__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meanMotionDdot"));
                            }
                            line1_field10__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| tle_parameters::Line1Field10::MeanMotionDdot(x.0));
                        }
                        GeneratedField::Agom => {
                            if line1_field10__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agom"));
                            }
                            line1_field10__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| tle_parameters::Line1Field10::Agom(x.0));
                        }
                    }
                }
                Ok(TleParameters {
                    ephemeris_type: ephemeris_type__,
                    classification_type: classification_type__,
                    norad_cat_id: norad_cat_id__,
                    element_set_no: element_set_no__,
                    rev_at_epoch: rev_at_epoch__,
                    mean_motion_dot: mean_motion_dot__,
                    line1_field11: line1_field11__,
                    line1_field10: line1_field10__,
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.TleParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Uint64Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0 {
            len += 1;
        }
        if self.max != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Uint64Range", len)?;
        if self.min != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("min", ToString::to_string(&self.min).as_str())?;
        }
        if self.max != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("max", ToString::to_string(&self.max).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Uint64Range {
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
            type Value = Uint64Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Uint64Range")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Uint64Range, V::Error>
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
                Ok(Uint64Range {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Uint64Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vec2 {
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
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Vec2", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vec2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Vec2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vec2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
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
                    }
                }
                Ok(Vec2 {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Vec2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vec2f {
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
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Vec2f", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vec2f {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vec2f;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Vec2f")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vec2f, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
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
                    }
                }
                Ok(Vec2f {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Vec2f", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vec3 {
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
        if self.z != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Vec3", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vec3 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "z" => Ok(GeneratedField::Z),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vec3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Vec3")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vec3, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
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
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Vec3 {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Vec3", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vec3f {
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
        if self.z != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.Vec3f", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vec3f {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "z" => Ok(GeneratedField::Z),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vec3f;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.Vec3f")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vec3f, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
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
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Vec3f {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.Vec3f", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ypr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.yaw != 0. {
            len += 1;
        }
        if self.pitch != 0. {
            len += 1;
        }
        if self.roll != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.YPR", len)?;
        if self.yaw != 0. {
            struct_ser.serialize_field("yaw", &self.yaw)?;
        }
        if self.pitch != 0. {
            struct_ser.serialize_field("pitch", &self.pitch)?;
        }
        if self.roll != 0. {
            struct_ser.serialize_field("roll", &self.roll)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ypr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "yaw",
            "pitch",
            "roll",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Yaw,
            Pitch,
            Roll,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "yaw" => Ok(GeneratedField::Yaw),
                            "pitch" => Ok(GeneratedField::Pitch),
                            "roll" => Ok(GeneratedField::Roll),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ypr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.YPR")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ypr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut yaw__ = None;
                let mut pitch__ = None;
                let mut roll__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Yaw => {
                            if yaw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("yaw"));
                            }
                            yaw__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pitch => {
                            if pitch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pitch"));
                            }
                            pitch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Roll => {
                            if roll__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roll"));
                            }
                            roll__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Ypr {
                    yaw: yaw__.unwrap_or_default(),
                    pitch: pitch__.unwrap_or_default(),
                    roll: roll__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.YPR", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for YawPitch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.yaw != 0. {
            len += 1;
        }
        if self.pitch != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("anduril.r#type.YawPitch", len)?;
        if self.yaw != 0. {
            struct_ser.serialize_field("yaw", &self.yaw)?;
        }
        if self.pitch != 0. {
            struct_ser.serialize_field("pitch", &self.pitch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for YawPitch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "yaw",
            "pitch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Yaw,
            Pitch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "yaw" => Ok(GeneratedField::Yaw),
                            "pitch" => Ok(GeneratedField::Pitch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = YawPitch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct anduril.r#type.YawPitch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<YawPitch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut yaw__ = None;
                let mut pitch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Yaw => {
                            if yaw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("yaw"));
                            }
                            yaw__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pitch => {
                            if pitch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pitch"));
                            }
                            pitch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(YawPitch {
                    yaw: yaw__.unwrap_or_default(),
                    pitch: pitch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("anduril.r#type.YawPitch", FIELDS, GeneratedVisitor)
    }
}
