use std::{collections::BTreeMap, path::PathBuf};

pub mod provider;

#[derive(Debug, Default)]
pub struct Project {
    pub name: Name,
    pub projects: BTreeMap<Name, Project>,
    pub rules: BTreeMap<Name, Rule>,
    pub path: PathBuf,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Name(pub String);

impl<T: ToString> From<T> for Name {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: Name,
    pub cmds: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub prefix: Option<char>,
    pub program: String,
    pub args: Vec<String>,
}
