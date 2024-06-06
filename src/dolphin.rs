use anyhow::Result;
use tokio::net::TcpListener;
use tokio::process::Command;

struct Network {
    /// The last measured BSSID of laptop
    bssid: String,
    /// Signfies whether there is a disparity between server's BSSID and laptops's current one
    disparity: bool,
}

pub async fn smart_update() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let network = Network::new();

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("failed to accept connection");
            tokio::spawn(network);
        }
    });

    Command::new("monitor").arg(format!("{}", port));

    Ok(())
}

impl Network {
    pub fn new() -> Self {
        Self {
            bssid: String::new(),
            disparity: false,
        }
    }
    /// Gets the BSSID of the currently connected router and stores it if it is different
    pub async fn get_bssid(&mut self) -> Result<()> {
        let output = String::from_utf8(
            Command::new("sudo")
                .args(["wdutil", "info"])
                .output()
                .await?
                .stdout,
        )?;

        let Some(pos) = output.find("BSSID") else {
            anyhow::bail!("wdutil doesn't display BSSID anymore...")
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
            anyhow::bail!("returned BSSID is malformed, recieved {}", bssid)
        }

        if bssid != self.bssid {
            self.bssid = bssid;
            self.disparity = true;
        }

        Ok(())
    }
}
