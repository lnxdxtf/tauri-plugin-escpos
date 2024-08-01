use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

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
        self.0.run_mobile_plugin::<PermissionResponse>(
            "requestPermissions",
            RequestPermissions {
                bluetooth: true,
                bluetooth_scan: true,
                bluetooth_connect: true,
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

// For JAVA - JNI INIT Android
#[cfg(target_os = "android")]
pub mod android {
    use jni::objects::GlobalRef;
    use jni::{JNIEnv, JavaVM};
    use once_cell::sync::OnceCell;

    pub static JAVAVM: OnceCell<JavaVM> = OnceCell::new();

    #[no_mangle]
    pub extern "C" fn JNI_OnLoad(
        vm: jni::JavaVM,
        _res: *const std::os::raw::c_void,
    ) -> jni::sys::jint {
        log::info!("JNI_OnLoad init");

        let env = vm.get_env().unwrap();

        if let Err(err) = jni_utils::init(&env) {
            log::error!("Error initializing JNI utils: {:?}", err);
            return jni::sys::JNI_ERR;
        }

        if let Err(err) = eco_print::escpos::btleplug::platform::init(&env) {
            log::error!("Error initializing eco_print | btleplug-android : {:?}", err);
            return jni::sys::JNI_ERR;
        }

        if let Err(err) = JAVAVM.set(vm) {
            log::error!("Error setting JavaVM");
            return jni::sys::JNI_ERR;
        }

        jni::JNIVersion::V6.into()
    }
}
