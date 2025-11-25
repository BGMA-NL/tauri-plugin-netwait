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

    @objc public func waitForNetwork(_ invoke: Invoke) async throws {
    let isConnected = await waitForNetworkInternal()

    invoke.resolve(["is_connected": isConnected])
  }

  @objc public func checkNetworkStatus(_ invoke: Invoke) throws {
    let monitor = NWPathMonitor()
    let semaphore = DispatchSemaphore(value: 0)
    var isConnected = false
    
      monitor.pathUpdateHandler = { path in
          isConnected = (path.status == .satisfied)
          semaphore.signal()
          monitor.cancel()
      }
      
      let queue = DispatchQueue(label: "NetworkMonitorQueue")
      monitor.start(queue: queue)
      
      _ = semaphore.wait(timeout: .now() + 1)
      
      invoke.resolve(["is_connected": isConnected])
  }
}

@_cdecl("init_plugin_netwait")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
