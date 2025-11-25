use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::NetwaitExt;

#[command]
pub(crate) async fn wait_for_network<R: Runtime>(app: AppHandle<R>) -> Result<NetworkStatusResponse> {
    app.netwait().wait_for_network()
}

#[command]
pub(crate) async fn check_network_status<R: Runtime>(
    app: AppHandle<R>,
) -> Result<NetworkStatusResponse> {
    app.netwait().check_network_status()
}
