use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use Error::*;

use crate::exec::tree_exec::{Context, DataType};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Expected 1 argument, found {}", .0)]
    InvalidNumOfArguments(usize),
}

pub fn command<'a>(_ctx: &Context, args: &Vec<DataType>) -> Result<DataType<'a>> {
    if args.len() != 1 {
        dbg!(args);
        bail!(InvalidNumOfArguments(args.len()))
    }

    let arg = args.get(0).unwrap();
    println!("{}", arg);

    Ok(DataType::Number(0))
}
