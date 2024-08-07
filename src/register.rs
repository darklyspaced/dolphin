use std::env;

use anyhow::Result;

/// Registers the laptop at login
pub async fn register() -> Result<()> {
    let server = env::var("SERVER");

    reqwest::get(format!("https://www.rust-lang.org")).await?;
    Ok(())
}
