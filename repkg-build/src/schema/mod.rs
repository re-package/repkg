use std::{collections::BTreeMap, path::PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub build: BuildInfo,
    pub app: Option<AppInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfo {}

#[derive(Deserialize, Serialize, Debug)]
pub struct BuildInfo {
    pub command: Vec<String>,
    pub files: Vec<PathBuf>,
    pub dependencies: Dependencies,
}

pub type Dependencies = BTreeMap<String, Dependency>;

#[derive(Deserialize, Serialize, Debug)]
pub enum Dependency {
    Short(Version),
    Full { version: Version },
}
