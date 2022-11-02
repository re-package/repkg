pub mod echo;
pub mod todo;

use std::rc::Rc;

use crate::exec::tree_walker::{DataType, ParseOutput};

pub fn make() -> ParseOutput {
    let mut output = ParseOutput::default();

    output.set("todo", DataType::Custom(Rc::new(Box::new(todo::command))));
    // output.set("echo", DataType::Custom(Rc::new(Box::new(echo::command))));

    output
}
