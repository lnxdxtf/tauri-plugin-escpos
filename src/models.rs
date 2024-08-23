use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub enum Printer {
    #[cfg(feature = "ble")]
    BLE(eco_print::ble::ESCPOSPrinterBLE),
    #[cfg(feature = "usb")]
    USB(eco_print::printers::printer_usb::PrinterESCPOSUSB),
    #[default]
    NONE,
}

#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug)]
pub enum ConnectionType {
    #[default]
    BLE,
    USB,
}

#[derive(Default, Serialize, Clone)]
pub struct PrinterStore {
    /// Connection type, bluetooth or usb connection | This is set when the start command is executed.
    /// Default is BLE connection
    pub connection: ConnectionType,

    /// Connected to the printer
    pub connected: bool,

    /// When connected, the printer is set here
    #[serde(skip)]
    pub printer: Printer,

    /// Devices Found | Serialized to export to the frontend
    pub devices_ble: Vec<Device>,

    /// Devices Found | Serialized to export to the frontend
    pub devices_usb: Vec<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub name: String,
    pub address: String,
    #[cfg(feature = "ble")]
    pub services_ble: Vec<String>,
    /// Connection type, bluetooth or usb connection
    pub conn: ConnectionType,
}
