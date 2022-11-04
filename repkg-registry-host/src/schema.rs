use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistryConfig {
    pub db: RegistryConfigDB,
    pub server: RegistryConfigServer,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistryConfigDB {
    pub connection: String,
    #[serde(default = "repkg")]
    pub db: String,
    #[serde(default = "reg")]
    pub ns: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistryConfigServer {
    pub bind: [u8; 4],
    pub port: u16,
}

// Default functions
fn repkg() -> String {
    String::from("repkg")
}
fn reg() -> String {
    String::from("reg")
}
