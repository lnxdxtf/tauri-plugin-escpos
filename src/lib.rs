use std::sync::Arc;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

// Debug Feature
#[cfg(feature = "debug")]
#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
#[cfg(feature = "debug")]
extern crate android_logger;
#[cfg(feature = "debug")]
mod debug;

mod permission;

#[cfg(mobile)]
#[cfg(target_os = "android")]
mod java;

mod commands;
mod error;
mod models;
#[cfg(desktop)]
use desktop::Escpos;
pub use error::{Error, Result};
#[cfg(mobile)]
use mobile::Escpos;
use tokio::sync::Mutex;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the escpos APIs.
pub trait EscposExt<R: Runtime> {
    fn escpos(&self) -> &Escpos<R>;
}

impl<R: Runtime, T: Manager<R>> crate::EscposExt<R> for T {
    fn escpos(&self) -> &Escpos<R> {
        self.state::<Escpos<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[cfg(feature = "debug")]
    debug::debug_init();

    Builder::new("escpos")
        .invoke_handler(tauri::generate_handler![
            commands::request_permissions,
            commands::permissions_ok,
            commands::start,
            commands::check_store_state,
            commands::start_scan,
            commands::connect,
            commands::disconnect
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let escpos = mobile::init(app, api)?;
            #[cfg(desktop)]
            let escpos = desktop::init(app, api)?;

            app.manage(escpos);
            // manage state so it is accessible by the commands
            app.manage(Arc::new(Mutex::new(PrinterStore::default())));
            // To get the printer store state, you can use the following code:
            // let printer_store = app.state::<PrinterStore>();

            #[cfg(feature = "debug")]
            {
                #[cfg(mobile)]
                log::debug!("escpos mobile plugin initialized");
                #[cfg(desktop)]
                log::debug!("escpos desktop plugin initialized");
            }

            Ok(())
        })
        .build()
}
