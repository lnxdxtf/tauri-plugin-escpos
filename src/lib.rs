pub use error::{Error, Result};
pub use models::*;
use std::sync::Arc;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::sync::Mutex;

mod commands;
mod error;
mod models;
mod permission;

// Desktop =======================
#[cfg(desktop)]
mod desktop;
#[cfg(desktop)]
use desktop::Escpos;
// ===============================
// Mobile ========================
#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
use mobile::Escpos;
// ================================
// Debug Feature ==================
#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
extern crate android_logger;
mod debug;
// ================================

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
    debug::debug_init();

    Builder::new("escpos")
        .invoke_handler(tauri::generate_handler![
            commands::request_permissions,
            commands::permissions_status,
            commands::start,
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
            {
                #[cfg(mobile)]
                debug!("escpos mobile plugin initialized");
                #[cfg(desktop)]
                debug!("escpos desktop plugin initialized");
            }

            Ok(())
        })
        .build()
}
