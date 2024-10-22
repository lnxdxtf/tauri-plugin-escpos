use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub bluetooth: PermissionState,
    pub bluetooth_scan: PermissionState,
    pub bluetooth_connect: PermissionState,
    pub bluetooth_admin: PermissionState,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissions {
    pub bluetooth: bool,
    pub bluetooth_scan: bool,
    pub bluetooth_connect: bool,
    pub bluetooth_admin: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Granted,
    Denied,
    Unknown,
}

impl std::fmt::Display for PermissionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Granted => write!(f, "Granted"),
            Self::Denied => write!(f, "Denied"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Serialize for PermissionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> Deserialize<'de> for PermissionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "granted" => Ok(Self::Granted),
            "denied" => Ok(Self::Denied),
            "prompt" => Ok(Self::Unknown),
            _ => Err(de::Error::custom(format!("unknown permission state '{s}'"))),
        }
    }
}
