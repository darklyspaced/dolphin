// The Swift Programming Language
// https://docs.swift.org/swift-book
// 
// Swift Argument Parser
// https://swiftpackageindex.com/apple/swift-argument-parser/documentation

import ArgumentParser
import Foundation
import Network
import Starscream

@main
struct Monitor: ParsableCommand {
    @Argument(help: "the tcp port to send wifi events to")
    var port: String
    
    func run() throws {
        let monitor = NWPathMonitor.init(requiredInterfaceType: .wifi)
        let q = DispatchQueue.global(qos: .utility)
        
        monitor.pathUpdateHandler = { path in
            let request = URLRequest(url: URL(string: "http://localhost:\(port)")!)
            let socket = WebSocket(request: request)
            socket.connect() // don't need to recieve any data so no delegation
            socket.write(string: "changed")
        }
        
        monitor.start(queue: q)
        
        while (true) {
            Thread.sleep(forTimeInterval: Double.greatestFiniteMagnitude)
        }
    }
}
