use std::{collections::BTreeMap, rc::Rc, str::FromStr};

use miette::{bail, miette, Diagnostic, NamedSource, Result, SourceSpan};
use thiserror::Error;
use tree_sitter::{Node, Tree};

use super::{Command, Value, ValueType};

use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("UTF8 Error")]
    #[diagnostic(code(repkg_build::script::utf8_error))]
    Utf8Error(#[source] std::str::Utf8Error),
    #[error("Expected expr")]
    #[diagnostic(code(repkg_build::script::expected_expr))]
    ExpectedExpr(#[label("Expected expr")] SourceSpan),
    #[error("Expected number")]
    #[diagnostic(code(repkg_build::script::expected_number))]
    ExpectedNumber(#[label("Expected number")] SourceSpan),
    #[error("Expected string")]
    #[diagnostic(code(repkg_build::script::expected_string))]
    #[diagnostic(help("Try wrapping this in quotes \"\""))]
    ExpectedString(#[label("Expected string")] SourceSpan),
    #[error("Expected path")]
    #[diagnostic(code(repkg_build::script::expected_path))]
    ExpectedPath(#[label("Expected path")] SourceSpan),
    #[error("Invalid number")]
    #[diagnostic(code(repkg_build::script::invalid_number))]
    InvalidNumber(#[label("here")] SourceSpan, #[source] <i32 as FromStr>::Err),
    #[error("Invalid path")]
    #[diagnostic(code(repkg_build::script::invalid_path))]
    InvalidPath(#[label("here")] SourceSpan),
}

#[derive(Error, Diagnostic, Debug)]
#[error("")]
pub(crate) struct Sourced<E: Diagnostic> {
    #[related]
    errs: Vec<E>,
    #[source_code]
    source_code: NamedSource,
}

impl<E: Diagnostic> Sourced<E> {
    pub fn new(err: E, source_code: NamedSource) -> Self {
        Self {
            errs: vec![err],
            source_code,
        }
    }
}

pub struct ASTBuilder {
    source: Rc<String>,
    name: Rc<String>,
}

impl ASTBuilder {
    pub fn new(source: Rc<String>, name: Rc<String>) -> Self {
        Self { source, name }
    }

    pub fn construct(&self, tree: Tree) -> Result<Value> {
        let root_node = tree.root_node();
        let cursor = &mut root_node.walk();
        cursor.goto_first_child();
        let child = cursor.node();
        match child.kind() {
            "object" => self.object(child),
            _ => todo!(),
        }
    }

    fn object(&self, node: Node) -> Result<Value> {
        let cursor = &mut node.walk();
        let mut obj = BTreeMap::new();
        for child in node.children_by_field_name("child", cursor) {
            self.definition(child, &mut obj)?;
        }
        Ok(Value::new(
            node.byte_range(),
            super::ValueType::Object(obj),
            self.source.clone(),
            self.name.clone(),
        ))
    }

    fn definition(&self, node: Node, obj: &mut BTreeMap<String, Value>) -> Result<()> {
        let id = node
            .child_by_field_name("id")
            .ok_or_else(|| miette!("Expected an id! {:?}", node))?;
        let id = self.node_to_string(&id)?;
        let val = node.child_by_field_name("expr").ok_or_else(|| {
            Sourced::new(
                ExpectedExpr(node.end_byte().into()),
                NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
            )
        })?;

        let val = match val.kind() {
            "object" => self.object(val)?,
            "number" => self.number(val)?,
            "string" => self.string(val)?,
            "func_def" => self.func_def(val)?,
            _ => {
                bail!(miette!("Invalid expr: {:?}", val.kind()))
            }
        };

        obj.insert(id, val);
        Ok(())
    }

    fn func_def(&self, node: Node) -> Result<Value> {
        let cursor = &mut node.walk();
        let mut commands = vec![];

        for cmd in node.children_by_field_name("commands", cursor) {
            match cmd.kind() {
                "command" => commands.push(self.command(cmd)?),
                _ => {
                    bail!(miette!("Expected command, found {:?}", cmd.kind()))
                }
            }
        }

        let ret = node.child_by_field_name("return");
        let ret = if let Some(ret) = ret {
            match ret.kind() {
                "number" => self.number(ret)?,
                "string" => self.string(ret)?,
                _ => {
                    todo!()
                }
            }
        } else {
            Value::unit()
        };

        Ok(Value::new(
            node.byte_range(),
            ValueType::Function(commands, Rc::new(ret)),
            self.source.clone(),
            self.name.clone(),
        ))
    }

    fn command(&self, node: Node) -> Result<Command> {
        let path = node.child_by_field_name("path").ok_or_else(|| {
            Sourced::new(
                ExpectedPath(node.start_byte().into()),
                NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
            )
        })?;
        let cursor = &mut node.walk();
        let path = self.path(&path)?;
        let args = node
            .children_by_field_name("args", cursor)
            .map(|node| self.node_to_string(&node));
        let mut new_args = vec![];
        for arg in args {
            new_args.push(arg?);
        }

        Ok(Command {
            path,
            args: new_args,
        })
    }

    fn path(&self, node: &Node) -> Result<Vec<String>> {
        Ok(match node.kind() {
            "identifier" => {
                let id = self.node_to_string(node)?;
                vec![id]
            }
            "nested_identifier" => {
                let path = node.child_by_field_name("path").unwrap();
                let mut path = self.path(&path)?;
                let name = node.child_by_field_name("name").unwrap();
                let name = self.node_to_string(&name)?;

                path.push(name);
                path
            }
            _ => {
                bail!(Sourced::new(
                    InvalidPath(node.byte_range().into()),
                    NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
                ))
            }
        })
    }

    fn string(&self, node: Node) -> Result<Value> {
        Ok(match node.kind() {
            "string" => {
                let cursor = &mut node.walk();
                let children = node.children(cursor);
                let mut string = String::new();
                for child in children {
                    let str_frag = self.node_to_string(&child)?;
                    string.push_str(&str_frag);
                }
                dbg!(&string);
                Value::new(
                    node.byte_range(),
                    ValueType::String(string),
                    self.source.clone(),
                    self.name.clone(),
                )
            }
            _ => {
                bail!(Sourced::new(
                    ExpectedString(node.byte_range().into()),
                    NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
                ))
            }
        })
    }

    fn number(&self, node: Node) -> Result<Value> {
        Ok(match node.kind() {
            "number" => {
                let num_str = self.node_to_string(&node)?;
                let num = num_str.parse().map_err(|e| {
                    Sourced::new(
                        InvalidNumber(node.byte_range().into(), e),
                        NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
                    )
                })?;
                Value::new(
                    node.byte_range(),
                    ValueType::Number(num),
                    self.source.clone(),
                    self.name.clone(),
                )
            }
            _ => {
                bail!(Sourced::new(
                    ExpectedNumber(node.byte_range().into()),
                    NamedSource::new(self.name.as_ref().clone(), self.source.as_ref().clone()),
                ))
            }
        })
    }

    fn node_to_string(&self, node: &Node) -> Result<String> {
        Ok(node
            .utf8_text(self.source.as_bytes())
            .map_err(Utf8Error)?
            .to_string())
    }
}
