use std::{collections::BTreeMap, path::PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub build: BuildInfo,
    pub app: Option<AppInfo>,
    pub bin: Option<Vec<CopyTarget<"bin/">>>,
    pub include: Option<Vec<CopyTarget<"include/">>>,
    pub lib: Option<Vec<CopyTarget<"lib/">>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CopyTarget<const PREFIX: &'static str> {
    pub path: PathBuf,
    pub to: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfo {}

#[derive(Deserialize, Serialize, Debug)]
pub struct BuildInfo {
    pub command: Vec<String>,
    pub dependencies: Dependencies,
}

pub type Dependencies = BTreeMap<String, Dependency>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    Short(Version),
    Full { version: Version },
}
