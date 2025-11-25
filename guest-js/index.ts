import { invoke } from '@tauri-apps/api/core'

export interface NetworkStatusResponse {
  isConnected?: boolean
}

export async function waitForNetwork (): Promise<NetworkStatusResponse> {
  return await invoke('plugin:netwait|wait_for_network');
}

export async function checkNetworkStatus (): Promise<NetworkStatusResponse> {
  return await invoke('plugin:netwait|check_network_status');
}
