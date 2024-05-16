use anyhow::Result;
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

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
                    println!("address is: {:?}", info.get_addresses_v4());
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

    let ip = local_ip()?.to_string();
    let properties = [("running", "true")];

    let service = ServiceInfo::new(
        service_type,
        "oh my fwicking gyatt",
        &format!("{ip}.local."),
        ip,
        5200,
        &properties[..],
    )
    .unwrap();

    mdns.register(service)
        .expect("Failed to register our service");
    std::thread::sleep(std::time::Duration::from_secs(1000));
    mdns.shutdown().unwrap();

    Ok(())
}
