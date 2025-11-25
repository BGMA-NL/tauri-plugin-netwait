use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Netwait<R>> {
  Ok(Netwait(app.clone()))
}

/// Access to the netwait APIs.
pub struct Netwait<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Netwait<R> {
  pub fn wait_for_network(&self) -> crate::Result<NetworkStatusResponse> {
    Ok(NetworkStatusResponse { is_connected: Some(true) })
  }

  pub fn check_network_status(&self) -> crate::Result<NetworkStatusResponse> {
    let network_interfaces = NetworkInterface::show().unwrap();

    let is_connected = network_interfaces.iter().any(|iface| {
        iface.name != "lo" && !iface.addr.is_empty()
    });

    Ok(NetworkStatusResponse { is_connected: Some(is_connected) })
  }
}
