// The Swift Programming Language
// https://docs.swift.org/swift-book
// 
// Swift Argument Parser
// https://swiftpackageindex.com/apple/swift-argument-parser/documentation

import ArgumentParser
import Foundation
import Network
import Starscream

class TCPClient {
    let connection: NWConnection
    let queue = DispatchQueue(label: "TCP Client Queue")
    
    init(host: String, port: UInt16) {
        let nwEndpoint = NWEndpoint.hostPort(host: NWEndpoint.Host(host), port: NWEndpoint.Port(rawValue: port)!)
        connection = NWConnection(to: nwEndpoint, using: .tcp)
        startConnection()
    }
    
    private func startConnection() {
        connection.stateUpdateHandler = { (newState) in
            switch newState {
            case .ready:
                print("Connected to the server.")
                self.receiveData()
            case .waiting(let error):
                print("Connection is waiting: \(error)")
            case .failed(let error):
                print("Connection failed: \(error)")
            default:
                break
            }
        }
        connection.start(queue: queue)
    }
    
    func sendData(data: Data) {
        connection.send(content: data, completion: .contentProcessed { error in
            if let error = error {
                print("Failed to send data: \(error)")
            } else {
                print("Data was sent successfully.")
            }
        })
    }
    
    private func receiveData() {
        connection.receive(minimumIncompleteLength: 1, maximumLength: 65536) { (data, _, isComplete, error) in
            if let data = data, !data.isEmpty {
                print("Received data: \(data)")
                // Process received data here
            }
            if isComplete {
                print("Connection closed by server.")
            } else if let error = error {
                print("Receive error: \(error)")
            } else {
                self.receiveData() // Continue to receive more data
            }
        }
    }
    
    func stopConnection() {
        connection.cancel()
        print("Connection stopped.")
    }
}

@main
struct Monitor: ParsableCommand {
    @Argument(help: "the tcp port to send wifi events to")
    var port: String
    
    func run() throws {
        let monitor = NWPathMonitor.init(requiredInterfaceType: .wifi)
        let q = DispatchQueue.global(qos: .utility)
        
        monitor.pathUpdateHandler = { path in
            let tcpClient = TCPClient(host: "localhost", port: UInt16(port)!)
        }
        
        monitor.start(queue: q)
        
        while (true) {
            Thread.sleep(forTimeInterval: Double.greatestFiniteMagnitude)
        }
    }
}
