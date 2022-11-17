#![feature(pattern)]
#![feature(buf_read_has_data_left)]

use std::{collections::BTreeMap, path::PathBuf};

use directories::ProjectDirs;
use miette::{miette, Diagnostic, Result};
use thiserror::Error;

pub mod artifacts;
pub mod fs_util;
pub mod install;
#[cfg(feature = "protocols")]
pub mod protocols;
pub mod registry;
pub mod repository;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),
}

fn io_error(e: std::io::Error) -> Error {
    Error::IoError(e)
}

#[derive(Debug, Default)]
pub struct Project {
    pub name: String,
    pub projects: BTreeMap<String, Project>,
    pub rules: BTreeMap<String, Task>,
    pub in_: PathBuf,
    // pub at_: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub cmds: Vec<Command>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Command {
    pub prefix: Option<char>,
    /// separated by '.' ie. `rust.rustup` is vec!["rust", "rustup"]
    pub programs: Vec<String>,
    pub args: Vec<String>,
}

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "repkg").ok_or_else(|| miette!("Failed to retrieve home dir"))
}
