pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MilViewNationality {
    NationalityInvalid,
    NationalityAlbania,
    NationalityAlgeria,
    NationalityArgentina,
    NationalityArmenia,
    NationalityAustralia,
    NationalityAustria,
    NationalityAzerbaijan,
    NationalityBelarus,
    NationalityBelgium,
    NationalityBolivia,
    NationalityBosniaAndHerzegovina,
    NationalityBrazil,
    NationalityBulgaria,
    NationalityCambodia,
    NationalityCanada,
    NationalityChile,
    NationalityChina,
    NationalityColombia,
    NationalityCroatia,
    NationalityCuba,
    NationalityCyprus,
    NationalityCzechRepublic,
    NationalityDemocraticPeoplesRepublicOfKorea,
    NationalityDenmark,
    NationalityDominicanRepublic,
    NationalityEcuador,
    NationalityEgypt,
    NationalityEstonia,
    NationalityEthiopia,
    NationalityFinland,
    NationalityFrance,
    NationalityGeorgia,
    NationalityGermany,
    NationalityGreece,
    NationalityGuatemala,
    NationalityGuinea,
    NationalityHungary,
    NationalityIceland,
    NationalityIndia,
    NationalityIndonesia,
    NationalityInternationalRedCross,
    NationalityIraq,
    NationalityIreland,
    NationalityIslamicRepublicOfIran,
    NationalityIsrael,
    NationalityItaly,
    NationalityJamaica,
    NationalityJapan,
    NationalityJordan,
    NationalityKazakhstan,
    NationalityKuwait,
    NationalityKyrghyzRepublic,
    NationalityLaoPeoplesDemocraticRepublic,
    NationalityLatvia,
    NationalityLebanon,
    NationalityLiberia,
    NationalityLithuania,
    NationalityLuxembourg,
    NationalityMadagascar,
    NationalityMalaysia,
    NationalityMalta,
    NationalityMexico,
    NationalityMoldova,
    NationalityMontenegro,
    NationalityMorocco,
    NationalityMyanmar,
    NationalityNato,
    NationalityNetherlands,
    NationalityNewZealand,
    NationalityNicaragua,
    NationalityNigeria,
    NationalityNorway,
    NationalityPakistan,
    NationalityPanama,
    NationalityParaguay,
    NationalityPeru,
    NationalityPhilippines,
    NationalityPoland,
    NationalityPortugal,
    NationalityRepublicOfKorea,
    NationalityRomania,
    NationalityRussia,
    NationalitySaudiArabia,
    NationalitySenegal,
    NationalitySerbia,
    NationalitySingapore,
    NationalitySlovakia,
    NationalitySlovenia,
    NationalitySouthAfrica,
    NationalitySpain,
    NationalitySudan,
    NationalitySweden,
    NationalitySwitzerland,
    NationalitySyrianArabRepublic,
    NationalityTaiwan,
    NationalityTajikistan,
    NationalityThailand,
    NationalityTheFormerYugoslavRepublicOfMacedonia,
    NationalityTunisia,
    NationalityTurkey,
    NationalityTurkmenistan,
    NationalityUganda,
    NationalityUkraine,
    NationalityUnitedKingdom,
    NationalityUnitedNations,
    NationalityUnitedRepublicOfTanzania,
    NationalityUnitedStatesOfAmerica,
    NationalityUruguay,
    NationalityUzbekistan,
    NationalityVenezuela,
    NationalityVietnam,
    NationalityYemen,
    NationalityZimbabwe,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MilViewNationality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NationalityInvalid => serializer.serialize_str("NATIONALITY_INVALID"),
            Self::NationalityAlbania => serializer.serialize_str("NATIONALITY_ALBANIA"),
            Self::NationalityAlgeria => serializer.serialize_str("NATIONALITY_ALGERIA"),
            Self::NationalityArgentina => serializer.serialize_str("NATIONALITY_ARGENTINA"),
            Self::NationalityArmenia => serializer.serialize_str("NATIONALITY_ARMENIA"),
            Self::NationalityAustralia => serializer.serialize_str("NATIONALITY_AUSTRALIA"),
            Self::NationalityAustria => serializer.serialize_str("NATIONALITY_AUSTRIA"),
            Self::NationalityAzerbaijan => serializer.serialize_str("NATIONALITY_AZERBAIJAN"),
            Self::NationalityBelarus => serializer.serialize_str("NATIONALITY_BELARUS"),
            Self::NationalityBelgium => serializer.serialize_str("NATIONALITY_BELGIUM"),
            Self::NationalityBolivia => serializer.serialize_str("NATIONALITY_BOLIVIA"),
            Self::NationalityBosniaAndHerzegovina => {
                serializer.serialize_str("NATIONALITY_BOSNIA_AND_HERZEGOVINA")
            }
            Self::NationalityBrazil => serializer.serialize_str("NATIONALITY_BRAZIL"),
            Self::NationalityBulgaria => serializer.serialize_str("NATIONALITY_BULGARIA"),
            Self::NationalityCambodia => serializer.serialize_str("NATIONALITY_CAMBODIA"),
            Self::NationalityCanada => serializer.serialize_str("NATIONALITY_CANADA"),
            Self::NationalityChile => serializer.serialize_str("NATIONALITY_CHILE"),
            Self::NationalityChina => serializer.serialize_str("NATIONALITY_CHINA"),
            Self::NationalityColombia => serializer.serialize_str("NATIONALITY_COLOMBIA"),
            Self::NationalityCroatia => serializer.serialize_str("NATIONALITY_CROATIA"),
            Self::NationalityCuba => serializer.serialize_str("NATIONALITY_CUBA"),
            Self::NationalityCyprus => serializer.serialize_str("NATIONALITY_CYPRUS"),
            Self::NationalityCzechRepublic => {
                serializer.serialize_str("NATIONALITY_CZECH_REPUBLIC")
            }
            Self::NationalityDemocraticPeoplesRepublicOfKorea => {
                serializer.serialize_str("NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA")
            }
            Self::NationalityDenmark => serializer.serialize_str("NATIONALITY_DENMARK"),
            Self::NationalityDominicanRepublic => {
                serializer.serialize_str("NATIONALITY_DOMINICAN_REPUBLIC")
            }
            Self::NationalityEcuador => serializer.serialize_str("NATIONALITY_ECUADOR"),
            Self::NationalityEgypt => serializer.serialize_str("NATIONALITY_EGYPT"),
            Self::NationalityEstonia => serializer.serialize_str("NATIONALITY_ESTONIA"),
            Self::NationalityEthiopia => serializer.serialize_str("NATIONALITY_ETHIOPIA"),
            Self::NationalityFinland => serializer.serialize_str("NATIONALITY_FINLAND"),
            Self::NationalityFrance => serializer.serialize_str("NATIONALITY_FRANCE"),
            Self::NationalityGeorgia => serializer.serialize_str("NATIONALITY_GEORGIA"),
            Self::NationalityGermany => serializer.serialize_str("NATIONALITY_GERMANY"),
            Self::NationalityGreece => serializer.serialize_str("NATIONALITY_GREECE"),
            Self::NationalityGuatemala => serializer.serialize_str("NATIONALITY_GUATEMALA"),
            Self::NationalityGuinea => serializer.serialize_str("NATIONALITY_GUINEA"),
            Self::NationalityHungary => serializer.serialize_str("NATIONALITY_HUNGARY"),
            Self::NationalityIceland => serializer.serialize_str("NATIONALITY_ICELAND"),
            Self::NationalityIndia => serializer.serialize_str("NATIONALITY_INDIA"),
            Self::NationalityIndonesia => serializer.serialize_str("NATIONALITY_INDONESIA"),
            Self::NationalityInternationalRedCross => {
                serializer.serialize_str("NATIONALITY_INTERNATIONAL_RED_CROSS")
            }
            Self::NationalityIraq => serializer.serialize_str("NATIONALITY_IRAQ"),
            Self::NationalityIreland => serializer.serialize_str("NATIONALITY_IRELAND"),
            Self::NationalityIslamicRepublicOfIran => {
                serializer.serialize_str("NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN")
            }
            Self::NationalityIsrael => serializer.serialize_str("NATIONALITY_ISRAEL"),
            Self::NationalityItaly => serializer.serialize_str("NATIONALITY_ITALY"),
            Self::NationalityJamaica => serializer.serialize_str("NATIONALITY_JAMAICA"),
            Self::NationalityJapan => serializer.serialize_str("NATIONALITY_JAPAN"),
            Self::NationalityJordan => serializer.serialize_str("NATIONALITY_JORDAN"),
            Self::NationalityKazakhstan => serializer.serialize_str("NATIONALITY_KAZAKHSTAN"),
            Self::NationalityKuwait => serializer.serialize_str("NATIONALITY_KUWAIT"),
            Self::NationalityKyrghyzRepublic => {
                serializer.serialize_str("NATIONALITY_KYRGHYZ_REPUBLIC")
            }
            Self::NationalityLaoPeoplesDemocraticRepublic => {
                serializer.serialize_str("NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC")
            }
            Self::NationalityLatvia => serializer.serialize_str("NATIONALITY_LATVIA"),
            Self::NationalityLebanon => serializer.serialize_str("NATIONALITY_LEBANON"),
            Self::NationalityLiberia => serializer.serialize_str("NATIONALITY_LIBERIA"),
            Self::NationalityLithuania => serializer.serialize_str("NATIONALITY_LITHUANIA"),
            Self::NationalityLuxembourg => serializer.serialize_str("NATIONALITY_LUXEMBOURG"),
            Self::NationalityMadagascar => serializer.serialize_str("NATIONALITY_MADAGASCAR"),
            Self::NationalityMalaysia => serializer.serialize_str("NATIONALITY_MALAYSIA"),
            Self::NationalityMalta => serializer.serialize_str("NATIONALITY_MALTA"),
            Self::NationalityMexico => serializer.serialize_str("NATIONALITY_MEXICO"),
            Self::NationalityMoldova => serializer.serialize_str("NATIONALITY_MOLDOVA"),
            Self::NationalityMontenegro => serializer.serialize_str("NATIONALITY_MONTENEGRO"),
            Self::NationalityMorocco => serializer.serialize_str("NATIONALITY_MOROCCO"),
            Self::NationalityMyanmar => serializer.serialize_str("NATIONALITY_MYANMAR"),
            Self::NationalityNato => serializer.serialize_str("NATIONALITY_NATO"),
            Self::NationalityNetherlands => serializer.serialize_str("NATIONALITY_NETHERLANDS"),
            Self::NationalityNewZealand => serializer.serialize_str("NATIONALITY_NEW_ZEALAND"),
            Self::NationalityNicaragua => serializer.serialize_str("NATIONALITY_NICARAGUA"),
            Self::NationalityNigeria => serializer.serialize_str("NATIONALITY_NIGERIA"),
            Self::NationalityNorway => serializer.serialize_str("NATIONALITY_NORWAY"),
            Self::NationalityPakistan => serializer.serialize_str("NATIONALITY_PAKISTAN"),
            Self::NationalityPanama => serializer.serialize_str("NATIONALITY_PANAMA"),
            Self::NationalityParaguay => serializer.serialize_str("NATIONALITY_PARAGUAY"),
            Self::NationalityPeru => serializer.serialize_str("NATIONALITY_PERU"),
            Self::NationalityPhilippines => serializer.serialize_str("NATIONALITY_PHILIPPINES"),
            Self::NationalityPoland => serializer.serialize_str("NATIONALITY_POLAND"),
            Self::NationalityPortugal => serializer.serialize_str("NATIONALITY_PORTUGAL"),
            Self::NationalityRepublicOfKorea => {
                serializer.serialize_str("NATIONALITY_REPUBLIC_OF_KOREA")
            }
            Self::NationalityRomania => serializer.serialize_str("NATIONALITY_ROMANIA"),
            Self::NationalityRussia => serializer.serialize_str("NATIONALITY_RUSSIA"),
            Self::NationalitySaudiArabia => serializer.serialize_str("NATIONALITY_SAUDI_ARABIA"),
            Self::NationalitySenegal => serializer.serialize_str("NATIONALITY_SENEGAL"),
            Self::NationalitySerbia => serializer.serialize_str("NATIONALITY_SERBIA"),
            Self::NationalitySingapore => serializer.serialize_str("NATIONALITY_SINGAPORE"),
            Self::NationalitySlovakia => serializer.serialize_str("NATIONALITY_SLOVAKIA"),
            Self::NationalitySlovenia => serializer.serialize_str("NATIONALITY_SLOVENIA"),
            Self::NationalitySouthAfrica => serializer.serialize_str("NATIONALITY_SOUTH_AFRICA"),
            Self::NationalitySpain => serializer.serialize_str("NATIONALITY_SPAIN"),
            Self::NationalitySudan => serializer.serialize_str("NATIONALITY_SUDAN"),
            Self::NationalitySweden => serializer.serialize_str("NATIONALITY_SWEDEN"),
            Self::NationalitySwitzerland => serializer.serialize_str("NATIONALITY_SWITZERLAND"),
            Self::NationalitySyrianArabRepublic => {
                serializer.serialize_str("NATIONALITY_SYRIAN_ARAB_REPUBLIC")
            }
            Self::NationalityTaiwan => serializer.serialize_str("NATIONALITY_TAIWAN"),
            Self::NationalityTajikistan => serializer.serialize_str("NATIONALITY_TAJIKISTAN"),
            Self::NationalityThailand => serializer.serialize_str("NATIONALITY_THAILAND"),
            Self::NationalityTheFormerYugoslavRepublicOfMacedonia => {
                serializer.serialize_str("NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA")
            }
            Self::NationalityTunisia => serializer.serialize_str("NATIONALITY_TUNISIA"),
            Self::NationalityTurkey => serializer.serialize_str("NATIONALITY_TURKEY"),
            Self::NationalityTurkmenistan => serializer.serialize_str("NATIONALITY_TURKMENISTAN"),
            Self::NationalityUganda => serializer.serialize_str("NATIONALITY_UGANDA"),
            Self::NationalityUkraine => serializer.serialize_str("NATIONALITY_UKRAINE"),
            Self::NationalityUnitedKingdom => {
                serializer.serialize_str("NATIONALITY_UNITED_KINGDOM")
            }
            Self::NationalityUnitedNations => {
                serializer.serialize_str("NATIONALITY_UNITED_NATIONS")
            }
            Self::NationalityUnitedRepublicOfTanzania => {
                serializer.serialize_str("NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA")
            }
            Self::NationalityUnitedStatesOfAmerica => {
                serializer.serialize_str("NATIONALITY_UNITED_STATES_OF_AMERICA")
            }
            Self::NationalityUruguay => serializer.serialize_str("NATIONALITY_URUGUAY"),
            Self::NationalityUzbekistan => serializer.serialize_str("NATIONALITY_UZBEKISTAN"),
            Self::NationalityVenezuela => serializer.serialize_str("NATIONALITY_VENEZUELA"),
            Self::NationalityVietnam => serializer.serialize_str("NATIONALITY_VIETNAM"),
            Self::NationalityYemen => serializer.serialize_str("NATIONALITY_YEMEN"),
            Self::NationalityZimbabwe => serializer.serialize_str("NATIONALITY_ZIMBABWE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MilViewNationality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NATIONALITY_INVALID" => Ok(Self::NationalityInvalid),
            "NATIONALITY_ALBANIA" => Ok(Self::NationalityAlbania),
            "NATIONALITY_ALGERIA" => Ok(Self::NationalityAlgeria),
            "NATIONALITY_ARGENTINA" => Ok(Self::NationalityArgentina),
            "NATIONALITY_ARMENIA" => Ok(Self::NationalityArmenia),
            "NATIONALITY_AUSTRALIA" => Ok(Self::NationalityAustralia),
            "NATIONALITY_AUSTRIA" => Ok(Self::NationalityAustria),
            "NATIONALITY_AZERBAIJAN" => Ok(Self::NationalityAzerbaijan),
            "NATIONALITY_BELARUS" => Ok(Self::NationalityBelarus),
            "NATIONALITY_BELGIUM" => Ok(Self::NationalityBelgium),
            "NATIONALITY_BOLIVIA" => Ok(Self::NationalityBolivia),
            "NATIONALITY_BOSNIA_AND_HERZEGOVINA" => Ok(Self::NationalityBosniaAndHerzegovina),
            "NATIONALITY_BRAZIL" => Ok(Self::NationalityBrazil),
            "NATIONALITY_BULGARIA" => Ok(Self::NationalityBulgaria),
            "NATIONALITY_CAMBODIA" => Ok(Self::NationalityCambodia),
            "NATIONALITY_CANADA" => Ok(Self::NationalityCanada),
            "NATIONALITY_CHILE" => Ok(Self::NationalityChile),
            "NATIONALITY_CHINA" => Ok(Self::NationalityChina),
            "NATIONALITY_COLOMBIA" => Ok(Self::NationalityColombia),
            "NATIONALITY_CROATIA" => Ok(Self::NationalityCroatia),
            "NATIONALITY_CUBA" => Ok(Self::NationalityCuba),
            "NATIONALITY_CYPRUS" => Ok(Self::NationalityCyprus),
            "NATIONALITY_CZECH_REPUBLIC" => Ok(Self::NationalityCzechRepublic),
            "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA" => {
                Ok(Self::NationalityDemocraticPeoplesRepublicOfKorea)
            }
            "NATIONALITY_DENMARK" => Ok(Self::NationalityDenmark),
            "NATIONALITY_DOMINICAN_REPUBLIC" => Ok(Self::NationalityDominicanRepublic),
            "NATIONALITY_ECUADOR" => Ok(Self::NationalityEcuador),
            "NATIONALITY_EGYPT" => Ok(Self::NationalityEgypt),
            "NATIONALITY_ESTONIA" => Ok(Self::NationalityEstonia),
            "NATIONALITY_ETHIOPIA" => Ok(Self::NationalityEthiopia),
            "NATIONALITY_FINLAND" => Ok(Self::NationalityFinland),
            "NATIONALITY_FRANCE" => Ok(Self::NationalityFrance),
            "NATIONALITY_GEORGIA" => Ok(Self::NationalityGeorgia),
            "NATIONALITY_GERMANY" => Ok(Self::NationalityGermany),
            "NATIONALITY_GREECE" => Ok(Self::NationalityGreece),
            "NATIONALITY_GUATEMALA" => Ok(Self::NationalityGuatemala),
            "NATIONALITY_GUINEA" => Ok(Self::NationalityGuinea),
            "NATIONALITY_HUNGARY" => Ok(Self::NationalityHungary),
            "NATIONALITY_ICELAND" => Ok(Self::NationalityIceland),
            "NATIONALITY_INDIA" => Ok(Self::NationalityIndia),
            "NATIONALITY_INDONESIA" => Ok(Self::NationalityIndonesia),
            "NATIONALITY_INTERNATIONAL_RED_CROSS" => Ok(Self::NationalityInternationalRedCross),
            "NATIONALITY_IRAQ" => Ok(Self::NationalityIraq),
            "NATIONALITY_IRELAND" => Ok(Self::NationalityIreland),
            "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN" => Ok(Self::NationalityIslamicRepublicOfIran),
            "NATIONALITY_ISRAEL" => Ok(Self::NationalityIsrael),
            "NATIONALITY_ITALY" => Ok(Self::NationalityItaly),
            "NATIONALITY_JAMAICA" => Ok(Self::NationalityJamaica),
            "NATIONALITY_JAPAN" => Ok(Self::NationalityJapan),
            "NATIONALITY_JORDAN" => Ok(Self::NationalityJordan),
            "NATIONALITY_KAZAKHSTAN" => Ok(Self::NationalityKazakhstan),
            "NATIONALITY_KUWAIT" => Ok(Self::NationalityKuwait),
            "NATIONALITY_KYRGHYZ_REPUBLIC" => Ok(Self::NationalityKyrghyzRepublic),
            "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC" => {
                Ok(Self::NationalityLaoPeoplesDemocraticRepublic)
            }
            "NATIONALITY_LATVIA" => Ok(Self::NationalityLatvia),
            "NATIONALITY_LEBANON" => Ok(Self::NationalityLebanon),
            "NATIONALITY_LIBERIA" => Ok(Self::NationalityLiberia),
            "NATIONALITY_LITHUANIA" => Ok(Self::NationalityLithuania),
            "NATIONALITY_LUXEMBOURG" => Ok(Self::NationalityLuxembourg),
            "NATIONALITY_MADAGASCAR" => Ok(Self::NationalityMadagascar),
            "NATIONALITY_MALAYSIA" => Ok(Self::NationalityMalaysia),
            "NATIONALITY_MALTA" => Ok(Self::NationalityMalta),
            "NATIONALITY_MEXICO" => Ok(Self::NationalityMexico),
            "NATIONALITY_MOLDOVA" => Ok(Self::NationalityMoldova),
            "NATIONALITY_MONTENEGRO" => Ok(Self::NationalityMontenegro),
            "NATIONALITY_MOROCCO" => Ok(Self::NationalityMorocco),
            "NATIONALITY_MYANMAR" => Ok(Self::NationalityMyanmar),
            "NATIONALITY_NATO" => Ok(Self::NationalityNato),
            "NATIONALITY_NETHERLANDS" => Ok(Self::NationalityNetherlands),
            "NATIONALITY_NEW_ZEALAND" => Ok(Self::NationalityNewZealand),
            "NATIONALITY_NICARAGUA" => Ok(Self::NationalityNicaragua),
            "NATIONALITY_NIGERIA" => Ok(Self::NationalityNigeria),
            "NATIONALITY_NORWAY" => Ok(Self::NationalityNorway),
            "NATIONALITY_PAKISTAN" => Ok(Self::NationalityPakistan),
            "NATIONALITY_PANAMA" => Ok(Self::NationalityPanama),
            "NATIONALITY_PARAGUAY" => Ok(Self::NationalityParaguay),
            "NATIONALITY_PERU" => Ok(Self::NationalityPeru),
            "NATIONALITY_PHILIPPINES" => Ok(Self::NationalityPhilippines),
            "NATIONALITY_POLAND" => Ok(Self::NationalityPoland),
            "NATIONALITY_PORTUGAL" => Ok(Self::NationalityPortugal),
            "NATIONALITY_REPUBLIC_OF_KOREA" => Ok(Self::NationalityRepublicOfKorea),
            "NATIONALITY_ROMANIA" => Ok(Self::NationalityRomania),
            "NATIONALITY_RUSSIA" => Ok(Self::NationalityRussia),
            "NATIONALITY_SAUDI_ARABIA" => Ok(Self::NationalitySaudiArabia),
            "NATIONALITY_SENEGAL" => Ok(Self::NationalitySenegal),
            "NATIONALITY_SERBIA" => Ok(Self::NationalitySerbia),
            "NATIONALITY_SINGAPORE" => Ok(Self::NationalitySingapore),
            "NATIONALITY_SLOVAKIA" => Ok(Self::NationalitySlovakia),
            "NATIONALITY_SLOVENIA" => Ok(Self::NationalitySlovenia),
            "NATIONALITY_SOUTH_AFRICA" => Ok(Self::NationalitySouthAfrica),
            "NATIONALITY_SPAIN" => Ok(Self::NationalitySpain),
            "NATIONALITY_SUDAN" => Ok(Self::NationalitySudan),
            "NATIONALITY_SWEDEN" => Ok(Self::NationalitySweden),
            "NATIONALITY_SWITZERLAND" => Ok(Self::NationalitySwitzerland),
            "NATIONALITY_SYRIAN_ARAB_REPUBLIC" => Ok(Self::NationalitySyrianArabRepublic),
            "NATIONALITY_TAIWAN" => Ok(Self::NationalityTaiwan),
            "NATIONALITY_TAJIKISTAN" => Ok(Self::NationalityTajikistan),
            "NATIONALITY_THAILAND" => Ok(Self::NationalityThailand),
            "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA" => {
                Ok(Self::NationalityTheFormerYugoslavRepublicOfMacedonia)
            }
            "NATIONALITY_TUNISIA" => Ok(Self::NationalityTunisia),
            "NATIONALITY_TURKEY" => Ok(Self::NationalityTurkey),
            "NATIONALITY_TURKMENISTAN" => Ok(Self::NationalityTurkmenistan),
            "NATIONALITY_UGANDA" => Ok(Self::NationalityUganda),
            "NATIONALITY_UKRAINE" => Ok(Self::NationalityUkraine),
            "NATIONALITY_UNITED_KINGDOM" => Ok(Self::NationalityUnitedKingdom),
            "NATIONALITY_UNITED_NATIONS" => Ok(Self::NationalityUnitedNations),
            "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA" => {
                Ok(Self::NationalityUnitedRepublicOfTanzania)
            }
            "NATIONALITY_UNITED_STATES_OF_AMERICA" => Ok(Self::NationalityUnitedStatesOfAmerica),
            "NATIONALITY_URUGUAY" => Ok(Self::NationalityUruguay),
            "NATIONALITY_UZBEKISTAN" => Ok(Self::NationalityUzbekistan),
            "NATIONALITY_VENEZUELA" => Ok(Self::NationalityVenezuela),
            "NATIONALITY_VIETNAM" => Ok(Self::NationalityVietnam),
            "NATIONALITY_YEMEN" => Ok(Self::NationalityYemen),
            "NATIONALITY_ZIMBABWE" => Ok(Self::NationalityZimbabwe),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MilViewNationality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NationalityInvalid => write!(f, "NATIONALITY_INVALID"),
            Self::NationalityAlbania => write!(f, "NATIONALITY_ALBANIA"),
            Self::NationalityAlgeria => write!(f, "NATIONALITY_ALGERIA"),
            Self::NationalityArgentina => write!(f, "NATIONALITY_ARGENTINA"),
            Self::NationalityArmenia => write!(f, "NATIONALITY_ARMENIA"),
            Self::NationalityAustralia => write!(f, "NATIONALITY_AUSTRALIA"),
            Self::NationalityAustria => write!(f, "NATIONALITY_AUSTRIA"),
            Self::NationalityAzerbaijan => write!(f, "NATIONALITY_AZERBAIJAN"),
            Self::NationalityBelarus => write!(f, "NATIONALITY_BELARUS"),
            Self::NationalityBelgium => write!(f, "NATIONALITY_BELGIUM"),
            Self::NationalityBolivia => write!(f, "NATIONALITY_BOLIVIA"),
            Self::NationalityBosniaAndHerzegovina => {
                write!(f, "NATIONALITY_BOSNIA_AND_HERZEGOVINA")
            }
            Self::NationalityBrazil => write!(f, "NATIONALITY_BRAZIL"),
            Self::NationalityBulgaria => write!(f, "NATIONALITY_BULGARIA"),
            Self::NationalityCambodia => write!(f, "NATIONALITY_CAMBODIA"),
            Self::NationalityCanada => write!(f, "NATIONALITY_CANADA"),
            Self::NationalityChile => write!(f, "NATIONALITY_CHILE"),
            Self::NationalityChina => write!(f, "NATIONALITY_CHINA"),
            Self::NationalityColombia => write!(f, "NATIONALITY_COLOMBIA"),
            Self::NationalityCroatia => write!(f, "NATIONALITY_CROATIA"),
            Self::NationalityCuba => write!(f, "NATIONALITY_CUBA"),
            Self::NationalityCyprus => write!(f, "NATIONALITY_CYPRUS"),
            Self::NationalityCzechRepublic => write!(f, "NATIONALITY_CZECH_REPUBLIC"),
            Self::NationalityDemocraticPeoplesRepublicOfKorea => {
                write!(f, "NATIONALITY_DEMOCRATIC_PEOPLES_REPUBLIC_OF_KOREA")
            }
            Self::NationalityDenmark => write!(f, "NATIONALITY_DENMARK"),
            Self::NationalityDominicanRepublic => write!(f, "NATIONALITY_DOMINICAN_REPUBLIC"),
            Self::NationalityEcuador => write!(f, "NATIONALITY_ECUADOR"),
            Self::NationalityEgypt => write!(f, "NATIONALITY_EGYPT"),
            Self::NationalityEstonia => write!(f, "NATIONALITY_ESTONIA"),
            Self::NationalityEthiopia => write!(f, "NATIONALITY_ETHIOPIA"),
            Self::NationalityFinland => write!(f, "NATIONALITY_FINLAND"),
            Self::NationalityFrance => write!(f, "NATIONALITY_FRANCE"),
            Self::NationalityGeorgia => write!(f, "NATIONALITY_GEORGIA"),
            Self::NationalityGermany => write!(f, "NATIONALITY_GERMANY"),
            Self::NationalityGreece => write!(f, "NATIONALITY_GREECE"),
            Self::NationalityGuatemala => write!(f, "NATIONALITY_GUATEMALA"),
            Self::NationalityGuinea => write!(f, "NATIONALITY_GUINEA"),
            Self::NationalityHungary => write!(f, "NATIONALITY_HUNGARY"),
            Self::NationalityIceland => write!(f, "NATIONALITY_ICELAND"),
            Self::NationalityIndia => write!(f, "NATIONALITY_INDIA"),
            Self::NationalityIndonesia => write!(f, "NATIONALITY_INDONESIA"),
            Self::NationalityInternationalRedCross => {
                write!(f, "NATIONALITY_INTERNATIONAL_RED_CROSS")
            }
            Self::NationalityIraq => write!(f, "NATIONALITY_IRAQ"),
            Self::NationalityIreland => write!(f, "NATIONALITY_IRELAND"),
            Self::NationalityIslamicRepublicOfIran => {
                write!(f, "NATIONALITY_ISLAMIC_REPUBLIC_OF_IRAN")
            }
            Self::NationalityIsrael => write!(f, "NATIONALITY_ISRAEL"),
            Self::NationalityItaly => write!(f, "NATIONALITY_ITALY"),
            Self::NationalityJamaica => write!(f, "NATIONALITY_JAMAICA"),
            Self::NationalityJapan => write!(f, "NATIONALITY_JAPAN"),
            Self::NationalityJordan => write!(f, "NATIONALITY_JORDAN"),
            Self::NationalityKazakhstan => write!(f, "NATIONALITY_KAZAKHSTAN"),
            Self::NationalityKuwait => write!(f, "NATIONALITY_KUWAIT"),
            Self::NationalityKyrghyzRepublic => write!(f, "NATIONALITY_KYRGHYZ_REPUBLIC"),
            Self::NationalityLaoPeoplesDemocraticRepublic => {
                write!(f, "NATIONALITY_LAO_PEOPLES_DEMOCRATIC_REPUBLIC")
            }
            Self::NationalityLatvia => write!(f, "NATIONALITY_LATVIA"),
            Self::NationalityLebanon => write!(f, "NATIONALITY_LEBANON"),
            Self::NationalityLiberia => write!(f, "NATIONALITY_LIBERIA"),
            Self::NationalityLithuania => write!(f, "NATIONALITY_LITHUANIA"),
            Self::NationalityLuxembourg => write!(f, "NATIONALITY_LUXEMBOURG"),
            Self::NationalityMadagascar => write!(f, "NATIONALITY_MADAGASCAR"),
            Self::NationalityMalaysia => write!(f, "NATIONALITY_MALAYSIA"),
            Self::NationalityMalta => write!(f, "NATIONALITY_MALTA"),
            Self::NationalityMexico => write!(f, "NATIONALITY_MEXICO"),
            Self::NationalityMoldova => write!(f, "NATIONALITY_MOLDOVA"),
            Self::NationalityMontenegro => write!(f, "NATIONALITY_MONTENEGRO"),
            Self::NationalityMorocco => write!(f, "NATIONALITY_MOROCCO"),
            Self::NationalityMyanmar => write!(f, "NATIONALITY_MYANMAR"),
            Self::NationalityNato => write!(f, "NATIONALITY_NATO"),
            Self::NationalityNetherlands => write!(f, "NATIONALITY_NETHERLANDS"),
            Self::NationalityNewZealand => write!(f, "NATIONALITY_NEW_ZEALAND"),
            Self::NationalityNicaragua => write!(f, "NATIONALITY_NICARAGUA"),
            Self::NationalityNigeria => write!(f, "NATIONALITY_NIGERIA"),
            Self::NationalityNorway => write!(f, "NATIONALITY_NORWAY"),
            Self::NationalityPakistan => write!(f, "NATIONALITY_PAKISTAN"),
            Self::NationalityPanama => write!(f, "NATIONALITY_PANAMA"),
            Self::NationalityParaguay => write!(f, "NATIONALITY_PARAGUAY"),
            Self::NationalityPeru => write!(f, "NATIONALITY_PERU"),
            Self::NationalityPhilippines => write!(f, "NATIONALITY_PHILIPPINES"),
            Self::NationalityPoland => write!(f, "NATIONALITY_POLAND"),
            Self::NationalityPortugal => write!(f, "NATIONALITY_PORTUGAL"),
            Self::NationalityRepublicOfKorea => write!(f, "NATIONALITY_REPUBLIC_OF_KOREA"),
            Self::NationalityRomania => write!(f, "NATIONALITY_ROMANIA"),
            Self::NationalityRussia => write!(f, "NATIONALITY_RUSSIA"),
            Self::NationalitySaudiArabia => write!(f, "NATIONALITY_SAUDI_ARABIA"),
            Self::NationalitySenegal => write!(f, "NATIONALITY_SENEGAL"),
            Self::NationalitySerbia => write!(f, "NATIONALITY_SERBIA"),
            Self::NationalitySingapore => write!(f, "NATIONALITY_SINGAPORE"),
            Self::NationalitySlovakia => write!(f, "NATIONALITY_SLOVAKIA"),
            Self::NationalitySlovenia => write!(f, "NATIONALITY_SLOVENIA"),
            Self::NationalitySouthAfrica => write!(f, "NATIONALITY_SOUTH_AFRICA"),
            Self::NationalitySpain => write!(f, "NATIONALITY_SPAIN"),
            Self::NationalitySudan => write!(f, "NATIONALITY_SUDAN"),
            Self::NationalitySweden => write!(f, "NATIONALITY_SWEDEN"),
            Self::NationalitySwitzerland => write!(f, "NATIONALITY_SWITZERLAND"),
            Self::NationalitySyrianArabRepublic => write!(f, "NATIONALITY_SYRIAN_ARAB_REPUBLIC"),
            Self::NationalityTaiwan => write!(f, "NATIONALITY_TAIWAN"),
            Self::NationalityTajikistan => write!(f, "NATIONALITY_TAJIKISTAN"),
            Self::NationalityThailand => write!(f, "NATIONALITY_THAILAND"),
            Self::NationalityTheFormerYugoslavRepublicOfMacedonia => {
                write!(f, "NATIONALITY_THE_FORMER_YUGOSLAV_REPUBLIC_OF_MACEDONIA")
            }
            Self::NationalityTunisia => write!(f, "NATIONALITY_TUNISIA"),
            Self::NationalityTurkey => write!(f, "NATIONALITY_TURKEY"),
            Self::NationalityTurkmenistan => write!(f, "NATIONALITY_TURKMENISTAN"),
            Self::NationalityUganda => write!(f, "NATIONALITY_UGANDA"),
            Self::NationalityUkraine => write!(f, "NATIONALITY_UKRAINE"),
            Self::NationalityUnitedKingdom => write!(f, "NATIONALITY_UNITED_KINGDOM"),
            Self::NationalityUnitedNations => write!(f, "NATIONALITY_UNITED_NATIONS"),
            Self::NationalityUnitedRepublicOfTanzania => {
                write!(f, "NATIONALITY_UNITED_REPUBLIC_OF_TANZANIA")
            }
            Self::NationalityUnitedStatesOfAmerica => {
                write!(f, "NATIONALITY_UNITED_STATES_OF_AMERICA")
            }
            Self::NationalityUruguay => write!(f, "NATIONALITY_URUGUAY"),
            Self::NationalityUzbekistan => write!(f, "NATIONALITY_UZBEKISTAN"),
            Self::NationalityVenezuela => write!(f, "NATIONALITY_VENEZUELA"),
            Self::NationalityVietnam => write!(f, "NATIONALITY_VIETNAM"),
            Self::NationalityYemen => write!(f, "NATIONALITY_YEMEN"),
            Self::NationalityZimbabwe => write!(f, "NATIONALITY_ZIMBABWE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
