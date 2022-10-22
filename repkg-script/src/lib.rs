use miette::Diagnostic;
use repkg_common::{Project, Task};
use thiserror::Error;

pub mod exec;
pub mod package;
pub mod parser;
pub mod parser_new;
pub mod task_order;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),
}

fn io_error(e: std::io::Error) -> Error {
    Error::IoError(e)
}

#[derive(Debug)]
pub struct Import {
    pub url: String,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub enum ASTNode {
    Project(Project),
    Import(Import),
    Rule(Task),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    FunctionCall(FunctionCall),
    String(String),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub func_name: String,
    pub arguments: Vec<Value>,
}
