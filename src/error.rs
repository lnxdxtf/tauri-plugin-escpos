use eco_print::EcoPrintError;
use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),

    /// Error to getting store state on the backend tauri
    #[error("Invalid store state. Error getting the state (State of: {0}) on the backend tauri")]
    StoreState(String),
    #[error("Error feature not enabled: {0}")]
    FeatureNotEnabled(String),
    #[error("Error: {0}")]
    EcoPrint(EcoPrintError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let msg = format!("{}", self.to_string());
        error!("{}", msg);
        serializer.serialize_str(&msg)
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
