use std::time::Duration;

use eco_print::escpos::finder::ble::FinderBLE;
use tauri::{AppHandle, Manager, Runtime, State, Window};

use crate::{desktop, Error, Escpos, PermissionResponse, PermissionState, PrinterStore, Result};

#[tauri::command]
pub(crate) fn request_permissions<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    escpos.request_permissions();
    Ok(())
}

#[tauri::command]
pub(crate) async fn permissions_ok<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<bool> {
    let state = escpos.permissions_state()?;
    if state.bluetooth == PermissionState::Granted
        && state.bluetooth_scan == PermissionState::Granted
        && state.bluetooth_connect == PermissionState::Granted
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub(crate) async fn start<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
    type_conn: String,
) -> Result<()> {
    let printer_store = _app.state::<PrinterStore>().inner();
    let devices = if type_conn == "bluetooth" {
        let adapter = FinderBLE::get_adapter()
            .await
            .map_err(|_| Error::AdapterBluetoothNotFound)?;
        let time = Duration::from_secs(5);

        FinderBLE::scan(&adapter, vec![], time)
            .await
            .map_err(|_| Error::ErrorGetDevicesBle)?
    } else if type_conn == "usb" {
        todo!()
    } else {
        return Err(Error::InvalidConnectionType);
    };

    match printer_store.printer.lock() {
        Ok(mut printerMutex) => {}
        Err(_) => {
            return Err(Error::InvalidPrinterState);
        }
    };

    Ok(())
}

#[tauri::command]
pub(crate) async fn start_scan<R: Runtime>(
    _app: AppHandle<R>,
    escpos: State<'_, Escpos<R>>,
) -> Result<()> {
    todo!()
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
