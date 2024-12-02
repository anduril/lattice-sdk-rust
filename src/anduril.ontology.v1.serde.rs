// @generated
impl serde::Serialize for Disposition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "DISPOSITION_UNKNOWN",
            Self::Friendly => "DISPOSITION_FRIENDLY",
            Self::Hostile => "DISPOSITION_HOSTILE",
            Self::Suspicious => "DISPOSITION_SUSPICIOUS",
            Self::AssumedFriendly => "DISPOSITION_ASSUMED_FRIENDLY",
            Self::Neutral => "DISPOSITION_NEUTRAL",
            Self::Pending => "DISPOSITION_PENDING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Disposition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DISPOSITION_UNKNOWN",
            "DISPOSITION_FRIENDLY",
            "DISPOSITION_HOSTILE",
            "DISPOSITION_SUSPICIOUS",
            "DISPOSITION_ASSUMED_FRIENDLY",
            "DISPOSITION_NEUTRAL",
            "DISPOSITION_PENDING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Disposition;

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
                    "DISPOSITION_UNKNOWN" => Ok(Disposition::Unknown),
                    "DISPOSITION_FRIENDLY" => Ok(Disposition::Friendly),
                    "DISPOSITION_HOSTILE" => Ok(Disposition::Hostile),
                    "DISPOSITION_SUSPICIOUS" => Ok(Disposition::Suspicious),
                    "DISPOSITION_ASSUMED_FRIENDLY" => Ok(Disposition::AssumedFriendly),
                    "DISPOSITION_NEUTRAL" => Ok(Disposition::Neutral),
                    "DISPOSITION_PENDING" => Ok(Disposition::Pending),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Environment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ENVIRONMENT_UNKNOWN",
            Self::Air => "ENVIRONMENT_AIR",
            Self::Surface => "ENVIRONMENT_SURFACE",
            Self::SubSurface => "ENVIRONMENT_SUB_SURFACE",
            Self::Land => "ENVIRONMENT_LAND",
            Self::Space => "ENVIRONMENT_SPACE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Environment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENVIRONMENT_UNKNOWN",
            "ENVIRONMENT_AIR",
            "ENVIRONMENT_SURFACE",
            "ENVIRONMENT_SUB_SURFACE",
            "ENVIRONMENT_LAND",
            "ENVIRONMENT_SPACE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Environment;

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
                    "ENVIRONMENT_UNKNOWN" => Ok(Environment::Unknown),
                    "ENVIRONMENT_AIR" => Ok(Environment::Air),
                    "ENVIRONMENT_SURFACE" => Ok(Environment::Surface),
                    "ENVIRONMENT_SUB_SURFACE" => Ok(Environment::SubSurface),
                    "ENVIRONMENT_LAND" => Ok(Environment::Land),
                    "ENVIRONMENT_SPACE" => Ok(Environment::Space),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Nationality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Invalid => "NATIONALITY_INVALID",
            Self::Albania => "NATIONALITY_ALBANIA",
            Self::Algeria => "NATIONALITY_ALGERIA",
            Self::Argentina => "NATIONALITY_ARGENTINA",
            Self::Armenia => "NATIONALITY_ARMENIA",
            Self::Australia => "NATIONALITY_AUSTRALIA",
            Self::Austria => "NATIONALITY_AUSTRIA",
            Self::Azerbaijan => "NATIONALITY_AZERBAIJAN",
            Self::Belarus => "NATIONALITY_BELARUS",
            Self::Belgium => "NATIONALITY_BELGIUM",
            Self::Bolivia => "NATIONALITY_BOLIVIA",
            Self::BosniaAndHerzegovina => "NATIONALITY_BOSNIA_AND_HERZEGOVINA",
            Self::Brazil => "NATIONALITY_BRAZIL",
            Self::Bulgaria => "NATIONALITY_BULGARIA",
            Self::Cambodia => "NATIONALITY_CAMBODIA",
            Self::Canada => "NATIONALITY_CANADA",
            Self::Chile => "NATIONALITY_CHILE",
            Self::China => "NATIONALITY_CHINA",
            Self::Colombia => "NATIONALITY_COLOMBIA",
            Self::Croatia => "NATIONALITY_CROATIA",
            Self::Cuba => "NATIONALITY_CUBA",
            Self::Cyprus => "NATIONALITY_CYPRUS",
            Self::CzechRepublic => "NATIONALITY_CZECH_REPUBLIC",
            Self::DemocraticPeoplesRepublicOfKorea => "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA",
            Self::Denmark => "NATIONALITY_DENMARK",
            Self::DominicanRepublic => "NATIONALITY_DOMINICAN_REPUBLIC",
            Self::Ecuador => "NATIONALITY_ECUADOR",
            Self::Egypt => "NATIONALITY_EGYPT",
            Self::Estonia => "NATIONALITY_ESTONIA",
            Self::Ethiopia => "NATIONALITY_ETHIOPIA",
            Self::Finland => "NATIONALITY_FINLAND",
            Self::France => "NATIONALITY_FRANCE",
            Self::Georgia => "NATIONALITY_GEORGIA",
            Self::Germany => "NATIONALITY_GERMANY",
            Self::Greece => "NATIONALITY_GREECE",
            Self::Guatemala => "NATIONALITY_GUATEMALA",
            Self::Guinea => "NATIONALITY_GUINEA",
            Self::Hungary => "NATIONALITY_HUNGARY",
            Self::Iceland => "NATIONALITY_ICELAND",
            Self::India => "NATIONALITY_INDIA",
            Self::Indonesia => "NATIONALITY_INDONESIA",
            Self::InternationalRedCross => "NATIONALITY_INTERNATIONAL_RED_CROSS",
            Self::Iraq => "NATIONALITY_IRAQ",
            Self::Ireland => "NATIONALITY_IRELAND",
            Self::IslamicRepublicOfIran => "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN",
            Self::Israel => "NATIONALITY_ISRAEL",
            Self::Italy => "NATIONALITY_ITALY",
            Self::Jamaica => "NATIONALITY_JAMAICA",
            Self::Japan => "NATIONALITY_JAPAN",
            Self::Jordan => "NATIONALITY_JORDAN",
            Self::Kazakhstan => "NATIONALITY_KAZAKHSTAN",
            Self::Kuwait => "NATIONALITY_KUWAIT",
            Self::KyrghyzRepublic => "NATIONALITY_KYRGHYZ_REPUBLIC",
            Self::LaoPeoplesDemocraticRepublic => "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC",
            Self::Latvia => "NATIONALITY_LATVIA",
            Self::Lebanon => "NATIONALITY_LEBANON",
            Self::Liberia => "NATIONALITY_LIBERIA",
            Self::Lithuania => "NATIONALITY_LITHUANIA",
            Self::Luxembourg => "NATIONALITY_LUXEMBOURG",
            Self::Madagascar => "NATIONALITY_MADAGASCAR",
            Self::Malaysia => "NATIONALITY_MALAYSIA",
            Self::Malta => "NATIONALITY_MALTA",
            Self::Mexico => "NATIONALITY_MEXICO",
            Self::Moldova => "NATIONALITY_MOLDOVA",
            Self::Montenegro => "NATIONALITY_MONTENEGRO",
            Self::Morocco => "NATIONALITY_MOROCCO",
            Self::Myanmar => "NATIONALITY_MYANMAR",
            Self::Nato => "NATIONALITY_NATO",
            Self::Netherlands => "NATIONALITY_NETHERLANDS",
            Self::NewZealand => "NATIONALITY_NEW_ZEALAND",
            Self::Nicaragua => "NATIONALITY_NICARAGUA",
            Self::Nigeria => "NATIONALITY_NIGERIA",
            Self::Norway => "NATIONALITY_NORWAY",
            Self::Pakistan => "NATIONALITY_PAKISTAN",
            Self::Panama => "NATIONALITY_PANAMA",
            Self::Paraguay => "NATIONALITY_PARAGUAY",
            Self::Peru => "NATIONALITY_PERU",
            Self::Philippines => "NATIONALITY_PHILIPPINES",
            Self::Poland => "NATIONALITY_POLAND",
            Self::Portugal => "NATIONALITY_PORTUGAL",
            Self::RepublicOfKorea => "NATIONALITY_REPUBLIC_OF_KOREA",
            Self::Romania => "NATIONALITY_ROMANIA",
            Self::Russia => "NATIONALITY_RUSSIA",
            Self::SaudiArabia => "NATIONALITY_SAUDI_ARABIA",
            Self::Senegal => "NATIONALITY_SENEGAL",
            Self::Serbia => "NATIONALITY_SERBIA",
            Self::Singapore => "NATIONALITY_SINGAPORE",
            Self::Slovakia => "NATIONALITY_SLOVAKIA",
            Self::Slovenia => "NATIONALITY_SLOVENIA",
            Self::SouthAfrica => "NATIONALITY_SOUTH_AFRICA",
            Self::Spain => "NATIONALITY_SPAIN",
            Self::Sudan => "NATIONALITY_SUDAN",
            Self::Sweden => "NATIONALITY_SWEDEN",
            Self::Switzerland => "NATIONALITY_SWITZERLAND",
            Self::SyrianArabRepublic => "NATIONALITY_SYRIAN_ARAB_REPUBLIC",
            Self::Taiwan => "NATIONALITY_TAIWAN",
            Self::Tajikistan => "NATIONALITY_TAJIKISTAN",
            Self::Thailand => "NATIONALITY_THAILAND",
            Self::TheFormerYugoslavRepublicOfMacedonia => "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA",
            Self::Tunisia => "NATIONALITY_TUNISIA",
            Self::Turkey => "NATIONALITY_TURKEY",
            Self::Turkmenistan => "NATIONALITY_TURKMENISTAN",
            Self::Uganda => "NATIONALITY_UGANDA",
            Self::Ukraine => "NATIONALITY_UKRAINE",
            Self::UnitedKingdom => "NATIONALITY_UNITED_KINGDOM",
            Self::UnitedNations => "NATIONALITY_UNITED_NATIONS",
            Self::UnitedRepublicOfTanzania => "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA",
            Self::UnitedStatesOfAmerica => "NATIONALITY_UNITED_STATES_OF_AMERICA",
            Self::Uruguay => "NATIONALITY_URUGUAY",
            Self::Uzbekistan => "NATIONALITY_UZBEKISTAN",
            Self::Venezuela => "NATIONALITY_VENEZUELA",
            Self::Vietnam => "NATIONALITY_VIETNAM",
            Self::Yemen => "NATIONALITY_YEMEN",
            Self::Zimbabwe => "NATIONALITY_ZIMBABWE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Nationality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NATIONALITY_INVALID",
            "NATIONALITY_ALBANIA",
            "NATIONALITY_ALGERIA",
            "NATIONALITY_ARGENTINA",
            "NATIONALITY_ARMENIA",
            "NATIONALITY_AUSTRALIA",
            "NATIONALITY_AUSTRIA",
            "NATIONALITY_AZERBAIJAN",
            "NATIONALITY_BELARUS",
            "NATIONALITY_BELGIUM",
            "NATIONALITY_BOLIVIA",
            "NATIONALITY_BOSNIA_AND_HERZEGOVINA",
            "NATIONALITY_BRAZIL",
            "NATIONALITY_BULGARIA",
            "NATIONALITY_CAMBODIA",
            "NATIONALITY_CANADA",
            "NATIONALITY_CHILE",
            "NATIONALITY_CHINA",
            "NATIONALITY_COLOMBIA",
            "NATIONALITY_CROATIA",
            "NATIONALITY_CUBA",
            "NATIONALITY_CYPRUS",
            "NATIONALITY_CZECH_REPUBLIC",
            "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA",
            "NATIONALITY_DENMARK",
            "NATIONALITY_DOMINICAN_REPUBLIC",
            "NATIONALITY_ECUADOR",
            "NATIONALITY_EGYPT",
            "NATIONALITY_ESTONIA",
            "NATIONALITY_ETHIOPIA",
            "NATIONALITY_FINLAND",
            "NATIONALITY_FRANCE",
            "NATIONALITY_GEORGIA",
            "NATIONALITY_GERMANY",
            "NATIONALITY_GREECE",
            "NATIONALITY_GUATEMALA",
            "NATIONALITY_GUINEA",
            "NATIONALITY_HUNGARY",
            "NATIONALITY_ICELAND",
            "NATIONALITY_INDIA",
            "NATIONALITY_INDONESIA",
            "NATIONALITY_INTERNATIONAL_RED_CROSS",
            "NATIONALITY_IRAQ",
            "NATIONALITY_IRELAND",
            "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN",
            "NATIONALITY_ISRAEL",
            "NATIONALITY_ITALY",
            "NATIONALITY_JAMAICA",
            "NATIONALITY_JAPAN",
            "NATIONALITY_JORDAN",
            "NATIONALITY_KAZAKHSTAN",
            "NATIONALITY_KUWAIT",
            "NATIONALITY_KYRGHYZ_REPUBLIC",
            "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC",
            "NATIONALITY_LATVIA",
            "NATIONALITY_LEBANON",
            "NATIONALITY_LIBERIA",
            "NATIONALITY_LITHUANIA",
            "NATIONALITY_LUXEMBOURG",
            "NATIONALITY_MADAGASCAR",
            "NATIONALITY_MALAYSIA",
            "NATIONALITY_MALTA",
            "NATIONALITY_MEXICO",
            "NATIONALITY_MOLDOVA",
            "NATIONALITY_MONTENEGRO",
            "NATIONALITY_MOROCCO",
            "NATIONALITY_MYANMAR",
            "NATIONALITY_NATO",
            "NATIONALITY_NETHERLANDS",
            "NATIONALITY_NEW_ZEALAND",
            "NATIONALITY_NICARAGUA",
            "NATIONALITY_NIGERIA",
            "NATIONALITY_NORWAY",
            "NATIONALITY_PAKISTAN",
            "NATIONALITY_PANAMA",
            "NATIONALITY_PARAGUAY",
            "NATIONALITY_PERU",
            "NATIONALITY_PHILIPPINES",
            "NATIONALITY_POLAND",
            "NATIONALITY_PORTUGAL",
            "NATIONALITY_REPUBLIC_OF_KOREA",
            "NATIONALITY_ROMANIA",
            "NATIONALITY_RUSSIA",
            "NATIONALITY_SAUDI_ARABIA",
            "NATIONALITY_SENEGAL",
            "NATIONALITY_SERBIA",
            "NATIONALITY_SINGAPORE",
            "NATIONALITY_SLOVAKIA",
            "NATIONALITY_SLOVENIA",
            "NATIONALITY_SOUTH_AFRICA",
            "NATIONALITY_SPAIN",
            "NATIONALITY_SUDAN",
            "NATIONALITY_SWEDEN",
            "NATIONALITY_SWITZERLAND",
            "NATIONALITY_SYRIAN_ARAB_REPUBLIC",
            "NATIONALITY_TAIWAN",
            "NATIONALITY_TAJIKISTAN",
            "NATIONALITY_THAILAND",
            "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA",
            "NATIONALITY_TUNISIA",
            "NATIONALITY_TURKEY",
            "NATIONALITY_TURKMENISTAN",
            "NATIONALITY_UGANDA",
            "NATIONALITY_UKRAINE",
            "NATIONALITY_UNITED_KINGDOM",
            "NATIONALITY_UNITED_NATIONS",
            "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA",
            "NATIONALITY_UNITED_STATES_OF_AMERICA",
            "NATIONALITY_URUGUAY",
            "NATIONALITY_UZBEKISTAN",
            "NATIONALITY_VENEZUELA",
            "NATIONALITY_VIETNAM",
            "NATIONALITY_YEMEN",
            "NATIONALITY_ZIMBABWE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Nationality;

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
                    "NATIONALITY_INVALID" => Ok(Nationality::Invalid),
                    "NATIONALITY_ALBANIA" => Ok(Nationality::Albania),
                    "NATIONALITY_ALGERIA" => Ok(Nationality::Algeria),
                    "NATIONALITY_ARGENTINA" => Ok(Nationality::Argentina),
                    "NATIONALITY_ARMENIA" => Ok(Nationality::Armenia),
                    "NATIONALITY_AUSTRALIA" => Ok(Nationality::Australia),
                    "NATIONALITY_AUSTRIA" => Ok(Nationality::Austria),
                    "NATIONALITY_AZERBAIJAN" => Ok(Nationality::Azerbaijan),
                    "NATIONALITY_BELARUS" => Ok(Nationality::Belarus),
                    "NATIONALITY_BELGIUM" => Ok(Nationality::Belgium),
                    "NATIONALITY_BOLIVIA" => Ok(Nationality::Bolivia),
                    "NATIONALITY_BOSNIA_AND_HERZEGOVINA" => Ok(Nationality::BosniaAndHerzegovina),
                    "NATIONALITY_BRAZIL" => Ok(Nationality::Brazil),
                    "NATIONALITY_BULGARIA" => Ok(Nationality::Bulgaria),
                    "NATIONALITY_CAMBODIA" => Ok(Nationality::Cambodia),
                    "NATIONALITY_CANADA" => Ok(Nationality::Canada),
                    "NATIONALITY_CHILE" => Ok(Nationality::Chile),
                    "NATIONALITY_CHINA" => Ok(Nationality::China),
                    "NATIONALITY_COLOMBIA" => Ok(Nationality::Colombia),
                    "NATIONALITY_CROATIA" => Ok(Nationality::Croatia),
                    "NATIONALITY_CUBA" => Ok(Nationality::Cuba),
                    "NATIONALITY_CYPRUS" => Ok(Nationality::Cyprus),
                    "NATIONALITY_CZECH_REPUBLIC" => Ok(Nationality::CzechRepublic),
                    "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA" => Ok(Nationality::DemocraticPeoplesRepublicOfKorea),
                    "NATIONALITY_DENMARK" => Ok(Nationality::Denmark),
                    "NATIONALITY_DOMINICAN_REPUBLIC" => Ok(Nationality::DominicanRepublic),
                    "NATIONALITY_ECUADOR" => Ok(Nationality::Ecuador),
                    "NATIONALITY_EGYPT" => Ok(Nationality::Egypt),
                    "NATIONALITY_ESTONIA" => Ok(Nationality::Estonia),
                    "NATIONALITY_ETHIOPIA" => Ok(Nationality::Ethiopia),
                    "NATIONALITY_FINLAND" => Ok(Nationality::Finland),
                    "NATIONALITY_FRANCE" => Ok(Nationality::France),
                    "NATIONALITY_GEORGIA" => Ok(Nationality::Georgia),
                    "NATIONALITY_GERMANY" => Ok(Nationality::Germany),
                    "NATIONALITY_GREECE" => Ok(Nationality::Greece),
                    "NATIONALITY_GUATEMALA" => Ok(Nationality::Guatemala),
                    "NATIONALITY_GUINEA" => Ok(Nationality::Guinea),
                    "NATIONALITY_HUNGARY" => Ok(Nationality::Hungary),
                    "NATIONALITY_ICELAND" => Ok(Nationality::Iceland),
                    "NATIONALITY_INDIA" => Ok(Nationality::India),
                    "NATIONALITY_INDONESIA" => Ok(Nationality::Indonesia),
                    "NATIONALITY_INTERNATIONAL_RED_CROSS" => Ok(Nationality::InternationalRedCross),
                    "NATIONALITY_IRAQ" => Ok(Nationality::Iraq),
                    "NATIONALITY_IRELAND" => Ok(Nationality::Ireland),
                    "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN" => Ok(Nationality::IslamicRepublicOfIran),
                    "NATIONALITY_ISRAEL" => Ok(Nationality::Israel),
                    "NATIONALITY_ITALY" => Ok(Nationality::Italy),
                    "NATIONALITY_JAMAICA" => Ok(Nationality::Jamaica),
                    "NATIONALITY_JAPAN" => Ok(Nationality::Japan),
                    "NATIONALITY_JORDAN" => Ok(Nationality::Jordan),
                    "NATIONALITY_KAZAKHSTAN" => Ok(Nationality::Kazakhstan),
                    "NATIONALITY_KUWAIT" => Ok(Nationality::Kuwait),
                    "NATIONALITY_KYRGHYZ_REPUBLIC" => Ok(Nationality::KyrghyzRepublic),
                    "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC" => Ok(Nationality::LaoPeoplesDemocraticRepublic),
                    "NATIONALITY_LATVIA" => Ok(Nationality::Latvia),
                    "NATIONALITY_LEBANON" => Ok(Nationality::Lebanon),
                    "NATIONALITY_LIBERIA" => Ok(Nationality::Liberia),
                    "NATIONALITY_LITHUANIA" => Ok(Nationality::Lithuania),
                    "NATIONALITY_LUXEMBOURG" => Ok(Nationality::Luxembourg),
                    "NATIONALITY_MADAGASCAR" => Ok(Nationality::Madagascar),
                    "NATIONALITY_MALAYSIA" => Ok(Nationality::Malaysia),
                    "NATIONALITY_MALTA" => Ok(Nationality::Malta),
                    "NATIONALITY_MEXICO" => Ok(Nationality::Mexico),
                    "NATIONALITY_MOLDOVA" => Ok(Nationality::Moldova),
                    "NATIONALITY_MONTENEGRO" => Ok(Nationality::Montenegro),
                    "NATIONALITY_MOROCCO" => Ok(Nationality::Morocco),
                    "NATIONALITY_MYANMAR" => Ok(Nationality::Myanmar),
                    "NATIONALITY_NATO" => Ok(Nationality::Nato),
                    "NATIONALITY_NETHERLANDS" => Ok(Nationality::Netherlands),
                    "NATIONALITY_NEW_ZEALAND" => Ok(Nationality::NewZealand),
                    "NATIONALITY_NICARAGUA" => Ok(Nationality::Nicaragua),
                    "NATIONALITY_NIGERIA" => Ok(Nationality::Nigeria),
                    "NATIONALITY_NORWAY" => Ok(Nationality::Norway),
                    "NATIONALITY_PAKISTAN" => Ok(Nationality::Pakistan),
                    "NATIONALITY_PANAMA" => Ok(Nationality::Panama),
                    "NATIONALITY_PARAGUAY" => Ok(Nationality::Paraguay),
                    "NATIONALITY_PERU" => Ok(Nationality::Peru),
                    "NATIONALITY_PHILIPPINES" => Ok(Nationality::Philippines),
                    "NATIONALITY_POLAND" => Ok(Nationality::Poland),
                    "NATIONALITY_PORTUGAL" => Ok(Nationality::Portugal),
                    "NATIONALITY_REPUBLIC_OF_KOREA" => Ok(Nationality::RepublicOfKorea),
                    "NATIONALITY_ROMANIA" => Ok(Nationality::Romania),
                    "NATIONALITY_RUSSIA" => Ok(Nationality::Russia),
                    "NATIONALITY_SAUDI_ARABIA" => Ok(Nationality::SaudiArabia),
                    "NATIONALITY_SENEGAL" => Ok(Nationality::Senegal),
                    "NATIONALITY_SERBIA" => Ok(Nationality::Serbia),
                    "NATIONALITY_SINGAPORE" => Ok(Nationality::Singapore),
                    "NATIONALITY_SLOVAKIA" => Ok(Nationality::Slovakia),
                    "NATIONALITY_SLOVENIA" => Ok(Nationality::Slovenia),
                    "NATIONALITY_SOUTH_AFRICA" => Ok(Nationality::SouthAfrica),
                    "NATIONALITY_SPAIN" => Ok(Nationality::Spain),
                    "NATIONALITY_SUDAN" => Ok(Nationality::Sudan),
                    "NATIONALITY_SWEDEN" => Ok(Nationality::Sweden),
                    "NATIONALITY_SWITZERLAND" => Ok(Nationality::Switzerland),
                    "NATIONALITY_SYRIAN_ARAB_REPUBLIC" => Ok(Nationality::SyrianArabRepublic),
                    "NATIONALITY_TAIWAN" => Ok(Nationality::Taiwan),
                    "NATIONALITY_TAJIKISTAN" => Ok(Nationality::Tajikistan),
                    "NATIONALITY_THAILAND" => Ok(Nationality::Thailand),
                    "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA" => Ok(Nationality::TheFormerYugoslavRepublicOfMacedonia),
                    "NATIONALITY_TUNISIA" => Ok(Nationality::Tunisia),
                    "NATIONALITY_TURKEY" => Ok(Nationality::Turkey),
                    "NATIONALITY_TURKMENISTAN" => Ok(Nationality::Turkmenistan),
                    "NATIONALITY_UGANDA" => Ok(Nationality::Uganda),
                    "NATIONALITY_UKRAINE" => Ok(Nationality::Ukraine),
                    "NATIONALITY_UNITED_KINGDOM" => Ok(Nationality::UnitedKingdom),
                    "NATIONALITY_UNITED_NATIONS" => Ok(Nationality::UnitedNations),
                    "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA" => Ok(Nationality::UnitedRepublicOfTanzania),
                    "NATIONALITY_UNITED_STATES_OF_AMERICA" => Ok(Nationality::UnitedStatesOfAmerica),
                    "NATIONALITY_URUGUAY" => Ok(Nationality::Uruguay),
                    "NATIONALITY_UZBEKISTAN" => Ok(Nationality::Uzbekistan),
                    "NATIONALITY_VENEZUELA" => Ok(Nationality::Venezuela),
                    "NATIONALITY_VIETNAM" => Ok(Nationality::Vietnam),
                    "NATIONALITY_YEMEN" => Ok(Nationality::Yemen),
                    "NATIONALITY_ZIMBABWE" => Ok(Nationality::Zimbabwe),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
