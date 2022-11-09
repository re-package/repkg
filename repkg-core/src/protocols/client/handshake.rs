use miette::{bail, Diagnostic, Result};
use reqwest::Url;
use semver::Version;
use thiserror::Error;

use crate::protocols::{handshake::HandshakeResponse, PROTOCOL_VERSION};

use super::super::Error::*;
use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("The server has an incompatible version, cannot connect")]
    #[diagnostic(code(repkg::protocols::handshake::incompatible_version))]
    IncompatibleVersion,
}

/// Handshake protocol, to ensure both client and server are running compatible versions
/// of repkg, and potentially for encryption (future).
pub async fn handshake(url: Url) -> Result<()> {
    url.join("api/handshake").map_err(ParseError)?;
    let res = reqwest::get(url).await.map_err(RequestError)?;
    let hs_res = res
        .json::<HandshakeResponse>()
        .await
        .map_err(InvalidResponse)?;
    let local_ver = Version::parse(PROTOCOL_VERSION).unwrap();
    let remote_ver = Version::parse(&hs_res.version).map_err(InvalidVerResponse)?;
    if local_ver.major != remote_ver.major {
        bail!(IncompatibleVersion)
    }

    Ok(())
}
