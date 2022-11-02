#![feature(anonymous_lifetime_in_impl_trait)]

use miette::Diagnostic;
use thiserror::Error;

pub mod exec;
pub mod script_std;
pub mod task_order;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),
}
