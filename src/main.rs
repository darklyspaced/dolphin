use anyhow::Result;
use dolphin::dolphin::get_bssid;
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug_span, error, info, Instrument};
use tracing_subscriber::fmt;

use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    get_bssid().await;

    let format = fmt::format();
    tracing_subscriber::fmt().event_format(format).init();

    let mdns = ServiceDaemon::new().expect("failed to create daemon");
    let service_type = "_dolphin._tcp.local.";
    let receiver = mdns.browse(service_type).expect("failed to browse");

    let browse_span = debug_span!("browse");
    tokio::spawn(
        async move {
            while let Ok(event) = receiver.recv_async().await {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        info!("new service resolved: {:?}", info);
                        error!(
                            "can connect to the service at {:?} and port {}",
                            info.get_addresses(),
                            info.get_port()
                        )
                    }
                    _other_event => {}
                }
            }
        }
        .instrument(browse_span),
    );

    let ip = local_ip()?.to_string();
    let port = 5200;
    let properties = [("running", "true")];

    let service = ServiceInfo::new(
        service_type,
        "dolphin",
        &format!("{ip}.local."),
        &ip,
        port,
        &properties[..],
    )?;

    mdns.register(service)
        .expect("Failed to register our service");

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;

    loop {
        let (socket, peer) = listener.accept().await?;
        info!("new request from {}", peer);

        tokio::spawn(process(socket));
    }
}

/// Handles the connection to the network service
///
/// Expects the client to provide (ip, port) that it can connect to and send its current location
async fn process(_stream: TcpStream) {
    std::thread::sleep(Duration::from_secs(100));
}
