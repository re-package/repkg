use miette::Diagnostic;
use thiserror::Error;

#[cfg(feature = "client")]
mod client;
mod handshake;
#[cfg(feature = "server")]
mod server;

#[cfg(feature = "client")]
pub use client::*;
#[cfg(feature = "server")]
pub use server::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("An error occurred processing request")]
    #[diagnostic(code(repkg::protocols::request_error))]
    RequestError(#[source] reqwest::Error),
    #[error("An error occurred parsing url")]
    #[diagnostic(code(repkg::protocols::url_parse_error))]
    ParseError(#[source] url::ParseError),
    #[error("Failed to parse server response")]
    #[diagnostic(code(repkg::protocols::invalid_response))]
    InvalidResponse(#[source] reqwest::Error),
    #[error("Failed to parse server response")]
    #[diagnostic(code(repkg::protocols::invalid_response))]
    InvalidVerResponse(#[source] semver::Error),
}

const PROTOCOL_VERSION: &'static str = "0.1.0";
