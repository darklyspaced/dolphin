use anyhow::Result;
use dolphin::network::{get_bssid, get_mac, tick_update};
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

    mdns.register(service).expect("Failed to register service");

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;
    tokio::spawn(tick_update());

    loop {
        let (socket, peer) = listener.accept().await?;
        info!("new request from {}", peer);

        tokio::spawn(process(socket));
    }
}

/// Handles the connection to the network service
async fn process(stream: TcpStream) -> Result<()> {
    let peer = stream.peer_addr()?;

    let mut stream = TcpStream::connect(peer).await?;
    let buf = get_bssid().await?.to_string();

    stream.write_all(buf.as_bytes()).await?;

    Ok(())
}
