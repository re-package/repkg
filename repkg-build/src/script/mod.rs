use std::{collections::BTreeMap, ops::Range, rc::Rc};

pub mod ast;
pub mod vm;

#[derive(Debug, Clone)]
pub struct Value {
    location: Range<usize>,
    source: Rc<String>,
    name: Rc<String>,
    val: ValueType,
}

impl Value {
    pub fn new(
        source_span: Range<usize>,
        val: ValueType,
        source: Rc<String>,
        name: Rc<String>,
    ) -> Self {
        Self {
            location: source_span,
            val,
            source,
            name,
        }
    }

    pub fn unit() -> Self {
        Self::new(
            0..0,
            ValueType::Unit,
            Rc::new(String::new()),
            Rc::new("unit".to_string()),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Object(BTreeMap<String, Value>),
    Number(i32),
    String(String),
    Function(Vec<Command>, Rc<Value>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct Import {
    name: String,
    _url: String,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub(crate) path: Vec<String>,
    pub(crate) args: Vec<String>,
}
