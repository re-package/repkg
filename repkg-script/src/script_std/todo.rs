use miette::Result;

use crate::exec::tree_walker::{DataType, ParseOutput};

pub fn command(_ctx: &ParseOutput, args: &Vec<DataType>) -> Result<DataType> {
    for arg in args {
        println!("todo: {arg:?}");
    }

    Ok(DataType::Number(0))
}
