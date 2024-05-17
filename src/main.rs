use tokio::net::{TcpListener, TcpStream};

use anyhow::Result;
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let mdns = ServiceDaemon::new().expect("failed to create daemon");
    let service_type = "_dolphin._tcp.local.";
    let receiver = mdns.browse(service_type).expect("failed to browse");

    tokio::spawn(async move {
        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("new service resolved at {:?}", info);
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

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
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            info!("new connection detected!");
            process(socket).await
        });
    }
}

async fn process(stream: TcpStream) {
    loop {
        println!("yaya");
    }
}
