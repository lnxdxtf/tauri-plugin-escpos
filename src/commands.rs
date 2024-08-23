#![allow(unused)]

use eco_print::{
    ble::ESCPOSPrinterBLE,
    btleplug::api::Peripheral,
    commands::command::{ESCPOSBuilder, ESCPOSBuilderTrait, ESCPOSCommand, ESCPOSDataBuilder},
    uuid::Uuid,
    FinderTrait, PrinterTrait,
};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tauri::{AppHandle, Emitter as _, Manager, Runtime, State};
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
    let mut printer_store = &mut _printer_store.lock().await;

    #[cfg(not(any(feature = "ble", feature = "usb")))]
    {
        let _f = if !cfg!(feature = "ble") {
            "ble"
        } else if !cfg!(feature = "usb") {
            "usb"
        } else {
            return Err(Error::FeatureNotEnabled(
                "ble and usb feature not enabled".to_string(),
            ));
        };

        return Err(Error::FeatureNotEnabled(format!(
            "Feature {} is not enabled",
            _f
        )));
    }

    let printer: Printer;

    match conn {
        ConnectionType::BLE => {
            let mut _printer = ESCPOSPrinterBLE::new().map_err(|_err| Error::EcoPrint(_err))?;
            _printer.start().await.map_err(Error::EcoPrint)?;
            _printer.scan().await.map_err(Error::EcoPrint)?;
            printer = Printer::BLE(_printer);
        }
        ConnectionType::USB => todo!(),
    }

    printer_store.connection = conn;
    printer_store.printer = printer;

    // Task to emit the state of the store to the frontend
    tokio::task::spawn({
        let printer_store_clone = Arc::clone(&_printer_store);
        async move {
            loop {
                let printer_store = printer_store_clone.lock().await;
                let _ = _app.emit(
                    "store_state_update",
                    PrinterStore {
                        connection: printer_store.connection,
                        connected: printer_store.connected,
                        devices_ble: printer_store.devices_ble.clone(),
                        devices_usb: printer_store.devices_usb.clone(),
                        ..Default::default()
                    },
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    Ok(())
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
            let mut printer_store = printer_store.lock().await;

            let printer_wrapper = &printer_store.printer;
            let mut scanned_devices: Vec<eco_print::btleplug::platform::Peripheral> = vec![];
            match printer_wrapper {
                Printer::BLE(printer) => {
                    scanned_devices = printer.get_devices().await;
                }
                Printer::NONE => todo!(),
            };

            for device in scanned_devices {
                info!("getting prop from Device: {:?}", device);
                if let Some(prop) = device.properties().await.unwrap() {
                    printer_store.devices_ble.push(Device {
                        name: prop.local_name.unwrap_or("Unknown".to_string()),
                        address: device.address().to_string(),
                        conn: ConnectionType::BLE,
                        services_ble: prop.services.iter().map(|s| s.to_string()).collect(),
                    })
                }
            }
        }

        ConnectionType::USB => {
            todo!()
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
            let printer_wrapper = &mut printer_store.lock().await.printer;
            match printer_wrapper {
                Printer::BLE(printer) => {
                    let scanned_devices = printer.get_devices().await;
                    let device = scanned_devices
                        .iter()
                        .find(|d| d.address().to_string() == device.address)
                        .unwrap();

                    printer
                        .connect(device.to_owned())
                        .await
                        .map_err(Error::EcoPrint)?;

                    let mut commands: ESCPOSBuilder = ESCPOSBuilder::default();
                    commands.add_commands(vec![
                        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
                        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
                        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
                        ESCPOSDataBuilder::Text("Only a Test LOL ".into()),
                    ]);

                    printer
                        .print(commands.to_escpos())
                        .await
                        .map_err(Error::EcoPrint)?;
                }
                Printer::NONE => {
                    return Err(Error::StoreState(format!("Printer not seted")));
                }
            };
        }

        ConnectionType::USB => {
            todo!()
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
