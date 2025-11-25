use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Netwait;
#[cfg(mobile)]
use mobile::Netwait;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the netwait APIs.
pub trait NetwaitExt<R: Runtime> {
  fn netwait(&self) -> &Netwait<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NetwaitExt<R> for T {
  fn netwait(&self) -> &Netwait<R> {
    self.state::<Netwait<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("netwait")
    .invoke_handler(tauri::generate_handler![commands::wait_for_network, commands::check_network_status])
    .setup(|app, api| {
      #[cfg(mobile)]
      let netwait = mobile::init(app, api)?;
      #[cfg(desktop)]
      let netwait = desktop::init(app, api)?;
      app.manage(netwait);
      Ok(())
    })
    .build()
}
