pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EchelonArmyEchelon {
    ArmyEchelonInvalid,
    ArmyEchelonFireTeam,
    ArmyEchelonSquad,
    ArmyEchelonPlatoon,
    ArmyEchelonCompany,
    ArmyEchelonBattalion,
    ArmyEchelonRegiment,
    ArmyEchelonBrigade,
    ArmyEchelonDivision,
    ArmyEchelonCorps,
    ArmyEchelonArmy,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EchelonArmyEchelon {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ArmyEchelonInvalid => serializer.serialize_str("ARMY_ECHELON_INVALID"),
            Self::ArmyEchelonFireTeam => serializer.serialize_str("ARMY_ECHELON_FIRE_TEAM"),
            Self::ArmyEchelonSquad => serializer.serialize_str("ARMY_ECHELON_SQUAD"),
            Self::ArmyEchelonPlatoon => serializer.serialize_str("ARMY_ECHELON_PLATOON"),
            Self::ArmyEchelonCompany => serializer.serialize_str("ARMY_ECHELON_COMPANY"),
            Self::ArmyEchelonBattalion => serializer.serialize_str("ARMY_ECHELON_BATTALION"),
            Self::ArmyEchelonRegiment => serializer.serialize_str("ARMY_ECHELON_REGIMENT"),
            Self::ArmyEchelonBrigade => serializer.serialize_str("ARMY_ECHELON_BRIGADE"),
            Self::ArmyEchelonDivision => serializer.serialize_str("ARMY_ECHELON_DIVISION"),
            Self::ArmyEchelonCorps => serializer.serialize_str("ARMY_ECHELON_CORPS"),
            Self::ArmyEchelonArmy => serializer.serialize_str("ARMY_ECHELON_ARMY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EchelonArmyEchelon {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ARMY_ECHELON_INVALID" => Ok(Self::ArmyEchelonInvalid),
            "ARMY_ECHELON_FIRE_TEAM" => Ok(Self::ArmyEchelonFireTeam),
            "ARMY_ECHELON_SQUAD" => Ok(Self::ArmyEchelonSquad),
            "ARMY_ECHELON_PLATOON" => Ok(Self::ArmyEchelonPlatoon),
            "ARMY_ECHELON_COMPANY" => Ok(Self::ArmyEchelonCompany),
            "ARMY_ECHELON_BATTALION" => Ok(Self::ArmyEchelonBattalion),
            "ARMY_ECHELON_REGIMENT" => Ok(Self::ArmyEchelonRegiment),
            "ARMY_ECHELON_BRIGADE" => Ok(Self::ArmyEchelonBrigade),
            "ARMY_ECHELON_DIVISION" => Ok(Self::ArmyEchelonDivision),
            "ARMY_ECHELON_CORPS" => Ok(Self::ArmyEchelonCorps),
            "ARMY_ECHELON_ARMY" => Ok(Self::ArmyEchelonArmy),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EchelonArmyEchelon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArmyEchelonInvalid => write!(f, "ARMY_ECHELON_INVALID"),
            Self::ArmyEchelonFireTeam => write!(f, "ARMY_ECHELON_FIRE_TEAM"),
            Self::ArmyEchelonSquad => write!(f, "ARMY_ECHELON_SQUAD"),
            Self::ArmyEchelonPlatoon => write!(f, "ARMY_ECHELON_PLATOON"),
            Self::ArmyEchelonCompany => write!(f, "ARMY_ECHELON_COMPANY"),
            Self::ArmyEchelonBattalion => write!(f, "ARMY_ECHELON_BATTALION"),
            Self::ArmyEchelonRegiment => write!(f, "ARMY_ECHELON_REGIMENT"),
            Self::ArmyEchelonBrigade => write!(f, "ARMY_ECHELON_BRIGADE"),
            Self::ArmyEchelonDivision => write!(f, "ARMY_ECHELON_DIVISION"),
            Self::ArmyEchelonCorps => write!(f, "ARMY_ECHELON_CORPS"),
            Self::ArmyEchelonArmy => write!(f, "ARMY_ECHELON_ARMY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
