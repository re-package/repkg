use crate::protocols::{handshake::HandshakeResponse, PROTOCOL_VERSION};

pub fn response() -> HandshakeResponse {
    HandshakeResponse {
        version: PROTOCOL_VERSION.to_string(),
    }
}
