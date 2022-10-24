use std::{collections::BTreeMap, fs, path::Path, str::Utf8Error};

use miette::{bail, Diagnostic, Result};
use thiserror::Error;
use tree_sitter::{LanguageError, Node, Parser, Tree};

use self::Error::*;
use crate::Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(repkg_script::language_error))]
    LanguageError(#[from] LanguageError),
    #[error(transparent)]
    #[diagnostic(code(repkg_script::from_utf8_error))]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid number, expected usize, found: {}", .0)]
    #[diagnostic(code(repkg_script::invalid_number))]
    InvalidNumber(String),
    #[error("Unknown node type: {}", .0)]
    #[diagnostic(code(repkg_script::unknown_node_type))]
    UnknownNodeType(&'static str),
    #[error("Missing {}", .0)]
    #[diagnostic(code(repkg_script::missing_field))]
    MissingField(&'static str),
}

pub struct TreeSitterExecutor {
    tree: Tree,
    source: String,
}

impl TreeSitterExecutor {
    pub fn parse(path: impl AsRef<Path>) -> Result<Self> {
        let mut parser = Parser::new();
        let source = fs::read_to_string(path).map_err(IoError)?;
        parser
            .set_language(tree_sitter_repkg::language())
            .map_err(LanguageError)?;
        let tree = parser.parse(&source, None).unwrap(); // TODO: better error handling.
        Ok(Self { tree, source })
    }

    pub fn walk(&self) -> Result<Context> {
        let root_node = self.tree.root_node();

        let mut context = Context::default();
        self.walk_node_with_context(root_node, &mut context)?;

        Ok(context)
    }

    fn walk_node(&self, node: Node) -> Result<Context> {
        let mut context = Context::default();
        self.walk_node_with_context(node, &mut context)?;

        Ok(context)
    }

    fn walk_node_with_context(&self, node: Node, context: &mut Context) -> Result<()> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.handle_node(child, context)?;
        }

        Ok(())
    }

    fn handle_node(&self, node: Node, context: &mut Context) -> Result<()> {
        match node.kind() {
            "import_expr" => {
                self.handle_import_node(node, context)?;
            }
            "variable_def" => {
                self.handle_variable_def(node, context)?;
            }
            "namespace" => {
                self.handle_namespace(node, context)?;
            }
            "command" => {
                todo!()
            }
            "comment" => {}
            a => bail!(UnknownNodeType(a)),
        }
        Ok(())
    }

    fn handle_namespace(&self, node: Node, context: &mut Context) -> Result<()> {
        let id = self.get_id(node)?;

        let inner = node
            .child_by_field_name("body")
            .ok_or(MissingField("body"))?;
        let new_context = self.walk_node(inner)?;
        context.set(id, DataType::Context(new_context));

        Ok(())
    }

    fn handle_import_node(&self, _node: Node, _context: &mut Context) -> Result<()> {
        println!("Import Expression!");

        todo!()
    }

    fn handle_variable_def(&self, node: Node, context: &mut Context) -> Result<()> {
        let id = self.get_id(node)?;
        let val =
            self.get_primitive(node.child_by_field_name("val").ok_or(MissingField("val"))?)?;
        context.set(id, val);

        Ok(())
    }

    fn get_id(&self, node: Node) -> Result<String> {
        let id = node.child_by_field_name("id").ok_or(MissingField("id"))?;
        Ok(id
            .utf8_text(self.source.as_bytes())
            .map_err(Utf8Error)
            .map(ToString::to_string)?)
    }

    fn get_primitive(&self, node: Node) -> Result<DataType> {
        match node.kind() {
            "array" => {
                let mut cursor = node.walk();
                let mut val = Vec::new();
                for child in node.children(&mut cursor) {
                    val.push(self.get_primitive(child)?);
                }
                Ok(DataType::Array(val))
            }
            "string" => {
                let mut cursor = node.walk();
                cursor.goto_first_child();
                let string = cursor.node();
                let string = string
                    .utf8_text(self.source.as_bytes())
                    .map_err(Utf8Error)?;
                let string = if string.starts_with('\'') {
                    string
                        .strip_prefix('\'')
                        .unwrap()
                        .strip_suffix('\'')
                        .unwrap()
                } else if string.starts_with('\"') {
                    string.strip_prefix('"').unwrap().strip_suffix('"').unwrap()
                } else {
                    // TODO: better error handling
                    panic!()
                };
                Ok(DataType::String(string.to_string()))
            }
            "number" => {
                let string = node.utf8_text(self.source.as_bytes()).map_err(Utf8Error)?;
                let num: usize = string
                    .parse()
                    .map_err(|_| InvalidNumber(string.to_string()))?;

                Ok(DataType::Number(num))
            }
            a => {
                bail!(UnknownNodeType(a))
            }
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Array(Vec<DataType>),
    String(String),
    Number(usize),
    Context(Context),
}

#[derive(Default, Debug)]
pub struct Context {
    vars: BTreeMap<String, DataType>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, string: impl AsRef<String>) -> Option<&DataType> {
        self.vars.get(string.as_ref())
    }

    pub fn set(&mut self, name: impl ToString, val: DataType) {
        self.vars.insert(name.to_string(), val);
    }
}
