use crate::permission::*;
use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.escpos";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_escpos);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Escpos<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "EscposPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_escpos)?;
    Ok(Escpos(handle))
}

/// Access to the escpos APIs.
pub struct Escpos<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Escpos<R> {
    pub fn request_permissions(&self) -> crate::Result<()> {
        let _ = self.0.run_mobile_plugin::<PermissionResponse>(
            "requestPermissions",
            RequestPermissions {
                bluetooth: true,
                bluetooth_scan: true,
                bluetooth_connect: true,
                bluetooth_admin: true,
            },
        );

        Ok(())
    }

    pub fn permissions_state(&self) -> crate::Result<PermissionResponse> {
        self.0
            .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())
            .map(|r| r)
            .map_err(Into::into)
    }
}
