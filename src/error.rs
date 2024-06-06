use std::{io, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("not currently connected to wifi")]
    NoConnection,
    #[error("wdutil was changed and no longer displays BSSID")]
    WDUtilChanged,
    #[error("returned BSSID, from wdutil, was malformed in some way: `{0}`")]
    MalformedBssid(String),
    #[error("wdutil failed to run")]
    CommandFailed(#[from] io::Error),
    #[error("failed to convert from bytes to string")]
    BytesToStringFailed(#[from] FromUtf8Error),
}
