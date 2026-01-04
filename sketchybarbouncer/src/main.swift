import Cocoa

class WorkspaceObserver {
    let socketPath: URL
    
    init() {
        let cacheDir: URL
        if let xdgCache = ProcessInfo.processInfo.environment["XDG_CACHE_HOME"] {
            cacheDir = URL(fileURLWithPath: xdgCache)
        } else {
            let homeDir = FileManager.default.homeDirectoryForCurrentUser
            cacheDir = homeDir.appendingPathComponent(".cache")
        }
        socketPath = cacheDir.appendingPathComponent("sketchybar/helper.sock")
        
        let workspace = NSWorkspace.shared
        
        workspace.notificationCenter.addObserver(
            forName: NSWorkspace.didLaunchApplicationNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                self?.sendMessage("workspace-change")
            }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                self?.sendMessage("workspace-change")
            }
        }
        
        workspace.notificationCenter.addObserver(
            forName: NSWorkspace.didTerminateApplicationNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            self?.sendMessage("workspace-change")
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                self?.sendMessage("workspace-change")
            }
        }
    }
    
    func sendMessage(_ message: String) {
        let socket = socket(AF_UNIX, SOCK_STREAM, 0)
        guard socket >= 0 else { return }
        defer { close(socket) }
        
        var addr = sockaddr_un()
        addr.sun_family = sa_family_t(AF_UNIX)
        
        let path = socketPath.path
        guard path.count < MemoryLayout.size(ofValue: addr.sun_path) else { return }
        
        _ = withUnsafeMutablePointer(to: &addr.sun_path.0) { ptr in
            path.withCString { cString in
                strcpy(ptr, cString)
            }
        }
        
        let connectResult = withUnsafePointer(to: &addr) { ptr in
            ptr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                connect(socket, sockaddrPtr, socklen_t(MemoryLayout<sockaddr_un>.size))
            }
        }
        
        guard connectResult >= 0 else { return }
        
        let messageWithNewline = message + "\n"
        _ = messageWithNewline.withCString { cString in
            send(socket, cString, strlen(cString), 0)
        }
    }
}

// Keep the observer alive
let observer = WorkspaceObserver()

// Run the event loop
RunLoop.main.run()
