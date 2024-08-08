use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration, vec};

use eco_print::escpos::{
    btleplug::api::Peripheral,
    finder::{ble::FinderBLE, uuid::Uuid},
    printers::printer_ble::THERMAL_PRINTER_SERVICE,
};
use tauri::{AppHandle, Manager, Runtime, State};
use tokio::sync::Mutex;

use crate::{permission::PermissionState, Error, Escpos, EscposExt, PrinterStore, Result};

#[tauri::command]
pub(crate) fn request_permissions<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    let _ = escpos.request_permissions();
    Ok(())
}

#[tauri::command]
pub(crate) async fn permissions_ok<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<bool> {
    let state = escpos.permissions_state()?;
    Ok(state.bluetooth == PermissionState::Granted
        && state.bluetooth_scan == PermissionState::Granted
        && state.bluetooth_connect == PermissionState::Granted)
}

/// After getting the permissions, we can start the adapter
/// Start command, will set the adapter on the printer store state
/// After executing this command, the adapter will be available to use
/// in the printer store state
#[tauri::command]
pub(crate) async fn start<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
    conn: String,
) -> Result<()> {
    if conn == "bluetooth" {
        #[cfg(target_os = "android")]
        {
            let mut printer_store_clone = Arc::clone(&_printer_store);
            _app.escpos().btleplug_context_spawn(async move {
                let mut printer_store = printer_store_clone.lock().await;

                let adapter = FinderBLE::get_adapter()
                    .await
                    .map_err(|_| Error::AdapterBluetoothNotFound)
                    .unwrap();
                printer_store.connection = Some(conn);
                printer_store.adapter = Some(adapter);
            })
        }
        #[cfg(not(target_os = "android"))]
        {
            let adapter = FinderBLE::get_adapter()
                .await
                .map_err(|_| Error::AdapterBluetoothNotFound)?;
            let mut printer_store = _printer_store.lock().await;
            printer_store.connection = Some(conn);
            printer_store.adapter = Some(adapter);
        }
    } else if conn == "usb" {
        todo!("usb is not implemented yet");
    } else {
        return Err(Error::InvalidConnectionType);
    };

    Ok(())
}

#[tauri::command]
pub(crate) async fn check_store_state(
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
) -> Result<HashMap<String, bool>> {
    let mut printer_store_arc: Arc<Mutex<PrinterStore>> = Arc::clone(&_printer_store);
    let mut printer_store = printer_store_arc.lock().await;

    let mut seted = HashMap::new();

    if let Some(_) = printer_store.adapter {
        seted.insert("adapter".to_string(), true);
    } else {
        seted.insert("adapter".to_string(), false);
    }

    if let Some(conn) = &printer_store.connection {
        seted.insert(format!("connection {}", conn), true);
    } else {
        seted.insert(format!("connection"), false);
    }

    if let Some(_) = printer_store.printer {
        seted.insert("printer".to_string(), true);
    } else {
        seted.insert("printer".to_string(), false);
    }

    Ok(seted)
}

#[tauri::command]
pub(crate) async fn start_scan<R: Runtime>(
    _app: AppHandle<R>,
    _printer_store: State<'_, Arc<Mutex<PrinterStore>>>,
) -> Result<Vec<String>> {
    let mut devices_data: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    #[cfg(target_os = "android")]
    {
        let printer_store_clone = Arc::clone(&_printer_store);
        let device_data_clone: Arc<Mutex<Vec<String>>> = Arc::clone(&devices_data);
        _app.escpos()
            .btleplug_context_spawn(async move {
                let time = Duration::from_secs(5 as u64);
                let mut printer_store = printer_store_clone.lock().await;
                if let Some(adapter) = &printer_store.adapter {
                    let services_filter = vec![];
                    match FinderBLE::scan(adapter, services_filter, time)
                        .await
                        .map_err(|_| Error::ScanError)
                    {
                        Ok(dvc) => {
                            log::info!("Devices found: {:?}", dvc);
                            let mut devices = device_data_clone.lock().await;
                            // devices = dvc.into_iter().map(|d| d.id().to_string()).collect();
                            for d in dvc {
                                devices.push(d.address().to_string());
                            }
                        }
                        Err(e) => {
                            log::info!("Error scanning devices: {:?}", e);
                        }
                    };
                }
            })
            .await
            .expect("Error btleplug ctx");
    }

    let data = devices_data.lock().await;
    Ok(data.clone())
}

#[tauri::command]
pub(crate) async fn connect<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    todo!()
}

#[tauri::command]
pub(crate) async fn disconnect<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    todo!()
}

#[tauri::command]
pub(crate) async fn print<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
    data: String,
) -> Result<()> {
    todo!()
}
