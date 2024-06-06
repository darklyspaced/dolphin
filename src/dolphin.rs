use anyhow::Result;
use tokio::process::Command;

pub async fn get_bssid() -> Result<String> {
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

    Ok(bssid)
}
