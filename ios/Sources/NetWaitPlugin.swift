import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {

@MainActor
func waitForNetworkInternal() async -> Bool {
    let monitor = NWPathMonitor()
    let queue = DispatchQueue(label: "NetworkMonitorQueue")
    
    return await withCheckedContinuation { continuation in
        monitor.pathUpdateHandler = { path in
            if path.status == .satisfied {
                continuation.resume(returning: true)
                monitor.cancel() 
            }
        }
        
        monitor.start(queue: queue)
    }
}

@objc public func waitForNetwork(_ invoke: Invoke) throws {
    let isConnected = await waitForNetworkInternal()

    invoke.resolve(["is_connected": isConnected])
  }

  @objc public func checkNetworkStatus(_ invoke: Invoke) throws {
    
    let isConnected = Reachability.isConnectedToNetwork()
    invoke.resolve(["is_connected": isConnected])
  }
}

@_cdecl("init_plugin_netwait")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
