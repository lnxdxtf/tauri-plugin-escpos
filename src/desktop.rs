use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Escpos<R>> {
    Ok(Escpos(app.clone()))
}

/// Access to the escpos APIs.
pub struct Escpos<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Escpos<R> {
    pub fn request_permissions(&self) -> crate::Result<PermissionResponse> {
        Ok(PermissionResponse {
            bluetooth: PermissionState::Granted,
            bluetooth_scan: PermissionState::Granted,
            bluetooth_connect: PermissionState::Granted,
        })
    }

    pub fn permissions_state(&self) -> crate::Result<PermissionResponse> {
        Ok(PermissionResponse {
            bluetooth: PermissionState::Granted,
            bluetooth_scan: PermissionState::Granted,
            bluetooth_connect: PermissionState::Granted,
        })
    }
}
