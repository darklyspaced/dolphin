use anyhow::Result;
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, info_span, Instrument};
use tracing_subscriber::fmt;

use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let format = fmt::format().pretty();
    tracing_subscriber::fmt().event_format(format).init();

    let mdns = ServiceDaemon::new().expect("failed to create daemon");
    let service_type = "_dolphin._tcp.local.";
    let receiver = mdns.browse(service_type).expect("failed to browse");

    let browse_span = info_span!("browse");

    tokio::spawn(
        async move {
            while let Ok(event) = receiver.recv_async().await {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        info!("new service resolved: {:?}", info);
                    }
                    other_event => {
                        info!("received other event: {:?}", &other_event);
                    }
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

async fn process(_stream: TcpStream) {
    std::thread::sleep(Duration::from_secs(100));
}
