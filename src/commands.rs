#![allow(unused)]

use eco_print::escpos::{
    btleplug::api::Peripheral,
    finder::{ble::FinderBLE, uuid::Uuid},
    printers::printer_ble::{PrinterESCPOSBLE, THERMAL_PRINTER_SERVICE},
};
use std::{sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, Runtime, State};
use tokio::sync::Mutex;

use crate::{
    permission::{PermissionResponse, PermissionState},
    ConnectionType, Device, Error, Escpos, EscposExt, Printer, PrinterStore, Result,
};

#[tauri::command]
pub(crate) fn request_permissions<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    let state = escpos.permissions_state()?;
    if state.bluetooth != PermissionState::Granted
        || state.bluetooth_scan != PermissionState::Granted
        || state.bluetooth_connect != PermissionState::Granted
        || state.bluetooth_admin != PermissionState::Granted
    {
        info!(
            "Requesting Permissions. Permissions not Granted: {:?}",
            state
        );
        let _ = escpos.request_permissions();
    }
    Ok(())
}

#[tauri::command]
pub(crate) fn permissions_status<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<PermissionResponse> {
    let state = escpos.permissions_state()?;
    Ok(state)
}

/// After getting the permissions, we can start the adapter
/// Start command, will set the adapter on the printer store state
/// After executing this command, the adapter will be available to use
/// in the printer store state
#[tauri::command]
pub(crate) async fn start<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
    conn: ConnectionType,
) -> Result<()> {
    let mut printer_store = _printer_store.lock().await;

    match conn {
        ConnectionType::BLE => {
            #[cfg(feature = "ble")]
            {
                #[cfg(target_os = "android")]
                {}
            }
            #[cfg(not(feature = "ble"))]
            {
                return Err(Error::BLEFeatureNotEnabled);
            }
        }
        ConnectionType::USB => {
            #[cfg(feature = "usb")]
            {}
            #[cfg(not(feature = "usb"))]
            {
                return Err(Error::USBFeatureNotEnabled);
            }
        }
    }

    printer_store.connection = conn;
    Ok(())
}

#[tauri::command]
pub(crate) async fn check_store_state(
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
) -> Result<PrinterStore> {
    let mut printer_store = _printer_store.lock().await;
    // The adapter, devices_ble, devices_usb and printer are not serialized, so we need to remove them.
    Ok(PrinterStore {
        connection: printer_store.connection,
        connected: printer_store.connected,
        devices_ble: printer_store.devices_ble.clone(),
        devices_usb: printer_store.devices_usb.clone(),
        ..Default::default()
    })
}

#[tauri::command]
pub(crate) async fn start_scan<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
    time: u32,
) -> Result<()> {
    let printer_store = Arc::clone(&_printer_store);
    let conn = printer_store.lock().await.connection;

    // Clear the devices list
    printer_store.lock().await.devices_ble.clear();
    printer_store.lock().await.devices_usb.clear();

    match conn {
        ConnectionType::BLE => {
            #[cfg(feature = "ble")]
            {
                #[cfg(target_os = "android")]
                {
                    _app.escpos()
                        .btleplug_context_spawn(async move {
                            let mut printer_store_android = printer_store.lock().await;
                            let adapter = FinderBLE::get_adapter()
                                .await
                                .map_err(|_| Error::BLEAdapterNotFound)?;
                            let scanned_devices =
                                FinderBLE::scan(&adapter, vec![], Duration::from_secs(time as u64))
                                    .await
                                    .map_err(|_| Error::BLEScan)?;
                            for device in scanned_devices {
                                info!("getting prop from Device: {:?}", device);
                                if let Some(prop) = device.properties().await.unwrap() {
                                    printer_store_android.devices_ble.push(Device {
                                        name: prop.local_name.unwrap_or("Unknown".to_string()),
                                        address: device.address().to_string(),
                                        conn: ConnectionType::BLE,
                                        services_ble: prop
                                            .services
                                            .iter()
                                            .map(|s| s.to_string())
                                            .collect(),
                                    })
                                }
                            }
                            Ok::<(), Error>(())
                        })
                        .await
                        .map_err(|_err| Error::BLEbtleplugContextSpawn(_err.to_string()))?;
                }
            }
            #[cfg(not(feature = "ble"))]
            {
                return Err(Error::BLEFeatureNotEnabled);
            }
        }
        ConnectionType::USB => {
            #[cfg(feature = "usb")]
            {
                todo!()
            }
            #[cfg(not(feature = "usb"))]
            {
                return Err(Error::USBFeatureNotEnabled);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub(crate) async fn connect<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
    time: u64,
    device: Device,
) -> Result<()> {
    let printer_store = Arc::clone(&_printer_store);
    let conn = device.conn;

    info!("Connecting to device: {:?}", device);

    match conn {
        ConnectionType::BLE => {
            #[cfg(feature = "ble")]
            {
                #[cfg(target_os = "android")]
                {
                    _app.escpos()
                        .btleplug_context_spawn(async move {
                            let mut printer_store_android = printer_store.lock().await;
                            let adapter = FinderBLE::get_adapter()
                                .await
                                .map_err(|_| Error::BLEAdapterNotFound)?;
                            let scanned_devices =
                                FinderBLE::scan(&adapter, vec![], Duration::from_secs(time as u64))
                                    .await
                                    .map_err(|_| Error::BLEScan)?;

                            if let Some(device_found) = scanned_devices
                                .into_iter()
                                .find(|d| d.address().to_string() == device.address)
                            {
                                info!("Connecting to device: {:?}", device_found);
                                let device_peripheral = FinderBLE::connect(device_found)
                                    .await
                                    .map_err(|_| Error::BLEConnect(device.address))?;
                                info!("Connected to device: {:?}", device_peripheral);
                                let mut printer = PrinterESCPOSBLE::new(device_peripheral)
                                    .await
                                    .map_err(|_| Error::BLEPrinterInstance)?;
                                info!("Printer instance created");
                                printer_store_android.printer = Printer::BLE(printer);
                                printer_store_android.connected = true;
                            }

                            Ok::<(), Error>(())
                        })
                        .await
                        .map_err(|_err| Error::BLEbtleplugContextSpawn(_err.to_string()))?;
                }
            }
            #[cfg(not(feature = "ble"))]
            {
                return Err(Error::BLEFeatureNotEnabled);
            }
        }
        ConnectionType::USB => {
            #[cfg(feature = "usb")]
            {
                todo!()
            }
            #[cfg(not(feature = "usb"))]
            {
                return Err(Error::USBFeatureNotEnabled);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub(crate) async fn disconnect<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
) -> Result<()> {
    let printer_store = Arc::clone(&_printer_store);
    Ok(())
}

#[tauri::command]
pub(crate) async fn print<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
    data: String,
) -> Result<()> {
    let printer_store = Arc::clone(&_printer_store);
    Ok(())
}
