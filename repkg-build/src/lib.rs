use std::{collections::BTreeMap, path::PathBuf};

use rule::Rule;

pub mod parser;
pub mod rule;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct Name(pub String);

impl<T: ToString> From<T> for Name {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

#[derive(Debug)]
pub struct Import {
    pub url: String,
    pub items: Vec<Name>,
}

#[derive(Debug)]
pub enum ASTNode {
    Project(Project),
    Import(Import),
    Rule(Rule),
}

#[derive(Debug)]
pub struct Project {
    pub name: Name,
    pub rules: Vec<Rule>,
    pub path: PathBuf,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    FunctionCall(FunctionCall),
    String(String),
}

#[derive(Debug)]
pub struct Function {
    pub name: Name,
    pub body: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub func_name: Name,
    pub arguments: Vec<Value>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: BTreeMap<Name, Function>,
    pub projects: BTreeMap<Name, Project>,
    pub imports: Vec<Import>,
    pub rules: BTreeMap<Name, Rule>,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}
