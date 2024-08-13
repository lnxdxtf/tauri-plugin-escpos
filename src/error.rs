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

    /// Only used to differentiate the type of connection that the user wants to use. USB OR BLE
    #[error("Invalid connection type: {0}. Only supports bluetooth and usb")]
    ConnectionType(String),

    // BLE/Bluetooth errors
    #[error("Bluetooth/BLE feature not enabled on Cargo.toml")]
    BLEFeatureNotEnabled,
    #[error("Adapter Bluetooth/BLE don't found")]
    BLEAdapterNotFound,
    #[error("Bluetooth/BLE don't enabled")]
    BLENotEnabled,
    #[error("Error getting target device{0} on store state. {1}")]
    BLEGetDevice(String, String),
    #[error("Error scanning for devices on Bluetooth/BLE")]
    BLEScan,
    #[error("Error connecting to device {0}")]
    BLEConnect(String),
    #[error("Error btleplug context spawn: {0}")]
    BLEbtleplugContextSpawn(String),

    // USB errors
    #[error("USB feature not enabled on Cargo.toml")]
    USBFeatureNotEnabled,

    // Printer errors
    #[error("Error creating BLE printer instance")]
    BLEPrinterInstance,
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
