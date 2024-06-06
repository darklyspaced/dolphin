use anyhow::Result;
use reqwest::Client;
use tokio::process::Command;
use tokio::time;

use crate::error::NetworkError;

use std::env;
use std::time::Duration;

struct Network {
    /// The last measured BSSID of laptop
    bssid: String,
    mac: String,
    client: Client,
}

pub async fn tick_update() -> Result<()> {
    let mut network = Network::new();
    let mut interval = time::interval(Duration::from_secs(300));

    loop {
        interval.tick().await;
        network.refresh_location().await?;
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            bssid: String::new(),
            mac: mac_address::get_mac_address().unwrap().unwrap().to_string(),
            client: Client::new(),
        }
    }

    /// Gets the BSSID of the currently connected router and stores it and returns true if it is
    /// different.
    pub async fn get_bssid(&mut self) -> Result<bool, NetworkError> {
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

        if bssid != self.bssid {
            self.bssid = bssid;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn refresh_location(&mut self) -> Result<()> {
        match self.get_bssid().await {
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
