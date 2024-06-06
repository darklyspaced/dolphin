use anyhow::Result;
use tokio::net::TcpListener;
use tokio::process::Command;

use crate::error::NetworkError;

struct Network {
    /// The last measured BSSID of laptop
    bssid: String,
}

pub async fn smart_update() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let mut network = Network::new();

    let _ = Command::new("monitor").arg(format!("{}", port)).spawn();

    loop {
        let _ = listener
            .accept()
            .await
            .expect("failed to accept connection");

        network.refresh_location().await?;
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            bssid: String::new(),
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
                // TODO send request to server with new location
                todo!()
            }
            Err(NetworkError::NoConnection) | Ok(false) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
