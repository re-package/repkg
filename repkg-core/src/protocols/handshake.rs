use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HandshakeResponse {
    pub(super) version: String,
}
