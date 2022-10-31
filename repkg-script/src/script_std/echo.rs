use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use crate::exec::tree_walker::{DataType, ParseOutput};
use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Expected 1 argument, found {}", .0)]
    InvalidNumOfArguments(usize),
}

pub fn command(_ctx: &ParseOutput, args: &Vec<DataType>) -> Result<DataType> {
    if args.len() != 1 {
        bail!(InvalidNumOfArguments(args.len()))
    }

    let arg = args.get(0).unwrap();
    println!("{}", arg);

    Ok(DataType::Number(0))
}
