use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPackageRequest {
    pub name: String,
    pub version: Version,
    pub branch: String,
    pub namespace: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPackageResponse {}
