use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_netwait);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Netwait<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("nl.bgma.netwait", "NetwaitPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_netwait)?;
  Ok(Netwait(handle))
}

/// Access to the netwait APIs.
pub struct Netwait<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Netwait<R> {
  pub fn wait_for_network(&self) -> crate::Result<NetworkStatusResponse> {
    self
      .0
      .run_mobile_plugin("waitForNetwork", ())
      .map_err(Into::into)
  }

  pub fn check_network_status(&self) -> crate::Result<NetworkStatusResponse> {
    self
      .0
      .run_mobile_plugin("checkNetworkStatus", ())
      .map_err(Into::into)
  }
}
