use anyhow::Result;
use reqwest::Client;
use tokio::process::Command;
use tokio::time;

use crate::error::NetworkError;

use std::env;
use std::time::Duration;

pub struct Network {
    /// The last measured BSSID of laptop
    bssid: String,
    mac: String,
    client: Client,
}

/// Updates the location of laptop every X seconds
pub async fn tick_update() -> Result<()> {
    let mut network = Network::new();
    let mut interval = time::interval(Duration::from_secs(300));

    loop {
        interval.tick().await;
        network.refresh_location().await?;
    }
}

/// Returns the mac address of the current laptop
pub fn get_mac() -> Result<String> {
    let mac = mac_address::get_mac_address()?.unwrap();
    let mac = format!("{:x?}", mac.bytes())
        .chars()
        .map(|x| if x == ',' { ':' } else { x })
        .filter(|x| x.is_alphanumeric() || *x == ':')
        .collect::<String>();

    Ok(mac)
}

/// Results the BSSID of the connected access point
pub async fn get_bssid() -> Result<String, NetworkError> {
    let output = String::from_utf8(
        Command::new("sudo")
            .args(["wdutil", "info"])
            .output()
            .await?
            .stdout,
    )?;

    let Some(pos) = output.find("BSSID") else {
        return Err(NetworkError::WDUtilChanged);
    };

    let bssid = String::from_utf8(
        output
            .bytes()
            .skip(pos)
            .skip_while(|x| *x != b':')
            .skip(2)
            .take_while(|x| !x.is_ascii_whitespace())
            .collect::<Vec<_>>(),
    )?;

    if bssid == "None" {
        return Err(NetworkError::NoConnection);
    } else if bssid.len() != 17 {
        return Err(NetworkError::MalformedBssid(bssid));
    }

    Ok(bssid)
}

impl Network {
    pub fn new() -> Self {
        Self {
            bssid: String::new(),
            mac: get_mac().expect("failed to get mac address"),
            client: Client::new(),
        }
    }

    /// Ascertains whether the bssid has changed since last tick
    pub async fn bssid_changed(&mut self) -> Result<bool, NetworkError> {
        let curr = get_bssid().await?;

        if curr != self.bssid {
            self.bssid = curr;
            return Ok(true);
        }
        Ok(false)
    }

    /// Sends new location to server, if it has changed
    pub async fn refresh_location(&mut self) -> Result<()> {
        match self.bssid_changed().await {
            Ok(true) => {
                let client = self.client.clone();
                let server = env::var("SERVER")?;

                let message = format!("{}\n{}", self.mac, self.bssid);

                client
                    .post(format!("{}/location", server))
                    .body(message)
                    .send()
                    .await?;

                Ok(())
            }
            Err(NetworkError::NoConnection) | Ok(false) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}
