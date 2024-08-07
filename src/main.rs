use anyhow::Result;
use dolphin::network::{get_bssid, get_mac, tick_update, Network};
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::info;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let format = fmt::format();
    tracing_subscriber::fmt().event_format(format).init();

    let mdns = ServiceDaemon::new().expect("failed to create daemon");
    let service_type = "_dolphin._tcp.local.";

    let ip = local_ip()?.to_string();
    let port = 5201;
    let properties = [("mac", get_mac()?)];

    let service = ServiceInfo::new(
        service_type,
        &get_mac()?,
        &format!("{ip}.local."),
        &ip,
        port,
        &properties[..],
    )?;

    // register the network service
    mdns.register(service).expect("Failed to register service");

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;

    // sets up the laptop by registering it with server and giving incremental updates
    // PERF: don't really need the laptop to load balance because it is very unlikely that a bunch
    // laptops will all turn on at the same time
    let network = Network::new();
    network.register().await?;
    tokio::spawn(tick_update(network));

    loop {
        let (socket, peer) = listener.accept().await?;
        info!("new request from {}", peer);

        tokio::spawn(process(socket));
    }
}

/// Connects to client and writes laptop's location back
async fn process(mut stream: TcpStream) -> Result<()> {
    let buf = get_bssid().await?.to_string();
    stream.write_all(buf.as_bytes()).await?;

    Ok(())
}
