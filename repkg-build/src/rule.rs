use crate::{Name, Value};

#[derive(Debug)]
pub struct Rule {
    pub name: Name,
    // Calculate the values in order
    pub values: Vec<Value>,
}
