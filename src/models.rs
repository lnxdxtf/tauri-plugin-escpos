pub enum PrinterType {
    BLE(eco_print::escpos::printers::printer_ble::PrinterESCPOSBLE),
    #[cfg(desktop)]
    USB(eco_print::escpos::printers::printer_usb::PrinterESCPOSUSB),
}

#[derive(Default)]
pub struct PrinterStore {
    /// Connection type, bluetooth or usb connection | This is set when the start command is executed.
    pub connection: Option<String>,
    /// Adapter is the bluetooth adapter to use or the usb adapter, this is set when the start command is executed
    pub adapter: Option<eco_print::escpos::finder::ble::btleplug::platform::Adapter>,
    /// When connected, the printer is set here
    pub printer: Option<PrinterType>,
    /// Devices with uuid/id to connect
    pub devices: Vec<String>,
}
