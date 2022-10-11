#![feature(pattern)]

use std::{collections::BTreeMap, path::PathBuf};

pub mod fs_util;
pub mod repository;

#[derive(Debug, Default)]
pub struct Project {
    pub name: Name,
    pub projects: BTreeMap<Name, Project>,
    pub rules: BTreeMap<Name, Rule>,
    pub in_: PathBuf,
    // pub at_: Option<PathBuf>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Name(pub String);

impl<T: ToString> From<T> for Name {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

// impl ToString for Name {
//     fn to_string(&self) -> String {
//         self.0.clone()
//     }
// }

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: Name,
    pub cmds: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub prefix: Option<char>,
    /// separated by '.' ie. `rust.rustup` is vec!["rust", "rustup"]
    pub programs: Vec<String>,
    pub args: Vec<String>,
}
