pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UploadObjectRequestDistributionMode {
    #[serde(rename = "force")]
    Force,
}
impl fmt::Display for UploadObjectRequestDistributionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Force => "force",
        };
        write!(f, "{}", s)
    }
}
