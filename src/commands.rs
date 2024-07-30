use std::{collections::HashMap, time::Duration};

use eco_print::escpos::finder::ble::FinderBLE;
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{Error, Escpos, PermissionState, PrinterStore, Result};

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
    _escpos: State<'_, Escpos<R>>,
    conn: String,
) -> Result<()> {
    let printer_store = _app.state::<PrinterStore>().inner();

    match printer_store.connection.lock() {
        Ok(mut connection_mutex) => {
            *connection_mutex = Some(conn.to_string());
        }
        Err(_) => return Err(Error::InvalidPrinterState),
    }

    if conn == "bluetooth" {
        let adapter = FinderBLE::get_adapter()
            .await
            .map_err(|_| Error::AdapterBluetoothNotFound)?;

        match printer_store.adapter.lock() {
            Ok(mut adapter_mutex) => {
                *adapter_mutex = Some(adapter);
            }
            Err(_) => return Err(Error::InvalidPrinterState),
        }
    } else if conn == "usb" {
        todo!("usb is not implemented yet");
    } else {
        return Err(Error::InvalidConnectionType);
    };

    Ok(())
}

#[tauri::command]
pub(crate) async fn check_store_state<R: Runtime>(
    _app: AppHandle<R>,
    _escpos: State<'_, Escpos<R>>,
) -> Result<HashMap<String, bool>> {
    let mut seted = HashMap::new();
    let printer_store = _app.state::<PrinterStore>().inner();
    match printer_store.adapter.lock() {
        Ok(adapter_opt) => {
            if let Some(_) = &*adapter_opt {
                seted.insert("adapter".to_string(), true);
            } else {
                seted.insert("adapter".to_string(), false);
            }
        }
        Err(_) => return Err(Error::InvalidPrinterState),
    }
    match printer_store.connection.lock() {
        Ok(conn_opt) => {
            if let Some(conn) = &*conn_opt {
                seted.insert(format!("connection {}", conn), true);
            } else {
                seted.insert(format!("connection"), false);
            }
        }
        Err(_) => return Err(Error::InvalidPrinterState),
    }
    match printer_store.printer.lock() {
        Ok(printer_opt) => {
            if let Some(_) = &*printer_opt {
                seted.insert("printer".to_string(), true);
            } else {
                seted.insert("printer".to_string(), false);
            }
        }
        Err(_) => return Err(Error::InvalidPrinterState),
    }

    Ok(seted)
}

#[tauri::command]
pub(crate) async fn start_scan<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
    time_secs: Option<u64>,
) -> Result<()> {
    let time = Duration::from_secs(time_secs.unwrap_or(10));
    let printer_store = _app.state::<PrinterStore>().inner();
    match printer_store.adapter.lock() {
        Ok(adapterOpt) => {
            if let Some(adapter) = &*adapterOpt {
                match printer_store.connection.lock() {
                    Ok(conn) => todo!(),
                    Err(_) => todo!(),
                }
            } else {
                return Err(Error::AdapterNotSet);
            }
        }
        Err(_) => return Err(Error::InvalidPrinterState),
    }
    Ok(())
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
