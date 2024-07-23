use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),

    #[error("Invalid connection type")]
    InvalidConnectionType,
    #[error("Invalid printer type")]
    InvalidPrinterType,
    #[error("Invalid printer state")]
    InvalidPrinterState,
    #[error("Adapter Bluetooth not found")]
    AdapterBluetoothNotFound,
    #[error("Bluetooth not enabled")]
    BluetoothNotEnabled,

    #[error("Error getting devices ble")]
    ErrorGetDevicesBle,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
