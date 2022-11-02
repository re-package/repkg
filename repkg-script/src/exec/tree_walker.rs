use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    fs,
    path::Path,
    rc::Rc,
    str::Utf8Error,
};

use miette::{bail, miette, Diagnostic, Result};
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
    #[error("Variable doesnt exist: {}", .0)]
    #[diagnostic(code(repkg_script::var_doesnt_exist))]
    VarDoesntExist(String),
}

pub struct TreeWalker {
    tree: Tree,
    source: String,
}

impl TreeWalker {
    pub fn parse(path: impl AsRef<Path>) -> Result<Self> {
        let mut parser = Parser::new();
        let source = fs::read_to_string(path).map_err(IoError)?;
        parser
            .set_language(tree_sitter_repkg::language())
            .map_err(LanguageError)?;
        let tree = parser.parse(&source, None).unwrap(); // TODO: better error handling.
        Ok(Self { tree, source })
    }

    pub fn parse_text(source: &'static str) -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_repkg::language())
            .map_err(LanguageError)?;
        let tree = parser.parse(&source, None).unwrap(); // TODO: better error handling.
        Ok(Self {
            tree,
            source: source.to_string(),
        })
    }

    pub fn walk(&self) -> Result<ParseOutput> {
        let root_node = self.tree.root_node();

        let mut context = ParseOutput::default();
        self.walk_node_with_context(root_node, &mut context)?;

        Ok(context)
    }

    fn walk_node(&self, node: Node) -> Result<ParseOutput> {
        let mut context = ParseOutput::default();
        self.walk_node_with_context(node, &mut context)?;

        Ok(context)
    }

    fn walk_node_with_context(&self, node: Node, context: &mut ParseOutput) -> Result<()> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.handle_node(child, context)?;
        }

        Ok(())
    }

    fn handle_node(&self, node: Node, context: &mut ParseOutput) -> Result<()> {
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
                self.handle_command(node, context)?;
            }
            "comment" => {}
            "\n" => {}
            "" => {}
            a => {
                dbg!(a);
                bail!(UnknownNodeType(a))
            }
        }
        Ok(())
    }

    fn handle_namespace(&self, node: Node, context: &mut ParseOutput) -> Result<()> {
        let id = self.get_id(node)?;

        let inner = node
            .child_by_field_name("body")
            .ok_or(MissingField("body"))?;
        let mut new_context = self.walk_node(inner)?;
        new_context.parent = Some(context as *mut ParseOutput);
        context.set(id, DataType::Child(new_context));

        Ok(())
    }

    fn handle_import_node(&self, _node: Node, _context: &mut ParseOutput) -> Result<()> {
        println!("Import Expression!");

        todo!()
    }

    fn handle_variable_def(&self, node: Node, context: &mut ParseOutput) -> Result<()> {
        let id = self.get_id(node)?;
        let val = self.get_primitive(
            node.child_by_field_name("val").ok_or(MissingField("val"))?,
            context,
        )?;
        context.set(id, val);

        Ok(())
    }

    fn handle_command(&self, node: Node, context: &mut ParseOutput) -> Result<()> {
        let cmd = self.get_command(node, context)?;
        context.to_execute.push(cmd);

        Ok(())
    }

    fn get_command(&self, node: Node, context: &mut ParseOutput) -> Result<Command> {
        let program = node
            .child_by_field_name("program")
            .ok_or(MissingField("program"))?;
        let program = self.handle_path(program, context)?;

        let mut args: Vec<DataType> = vec![];
        for arg in node.children_by_field_name("args", &mut node.walk()) {
            let data: DataType = match arg.kind() {
                "text" => DataType::String(self.node_to_string(arg)?),
                "primitive" => {
                    let mut cursor = arg.walk();
                    cursor.goto_first_child();
                    let child = cursor.node();
                    self.get_primitive(child, context)?
                }
                "variable" => self.get_var(arg, context)?,
                "command" => DataType::WaitCalc(self.get_command(arg, context)?),
                "nested_identifier" | "identifier" => {
                    DataType::WaitCalc((self.handle_path(arg, context)?, vec![]))
                }
                a => {
                    dbg!(&a);
                    bail!(UnknownNodeType(a))
                }
            };
            args.push(data);
        }

        Ok((program, args))
    }

    fn handle_path(&self, node: Node, context: &mut ParseOutput) -> Result<Vec<String>> {
        match node.kind() {
            "nested_identifier" => {
                let mut first_part = self.handle_path(
                    node.child_by_field_name("path")
                        .ok_or(MissingField("path"))?,
                    context,
                )?;
                let mut second_part = self.handle_path(
                    node.child_by_field_name("name")
                        .ok_or(MissingField("name"))?,
                    context,
                )?;

                first_part.append(&mut second_part);
                Ok(first_part)
            }
            "identifier" => Ok(vec![self.node_to_string(node)?]),
            a => {
                bail!(UnknownNodeType(a),)
            }
        }
    }

    fn node_to_string(&self, node: Node) -> Result<String> {
        Ok(node
            .utf8_text(self.source.as_bytes())
            .map_err(Utf8Error)
            .map(ToString::to_string)?)
    }

    fn get_id(&self, node: Node) -> Result<String> {
        let id = node.child_by_field_name("id").ok_or(MissingField("id"))?;
        Ok(id
            .utf8_text(self.source.as_bytes())
            .map_err(Utf8Error)
            .map(ToString::to_string)?)
    }

    fn get_var(&self, node: Node, context: &mut ParseOutput) -> Result<DataType> {
        let var = node.child_by_field_name("var").ok_or(MissingField("var"))?;

        let var = match var.kind() {
            "number" => {
                let num = self.get_primitive(var, context)?;
                match num {
                    DataType::Number(a) => DataType::Wait(format!("arg{}", a)),
                    _ => {
                        bail!(miette!("Number must be a number!"))
                    }
                }
            }
            "percent_sign" => DataType::Wait("args".to_string()),
            "text" => {
                let name = self.node_to_string(var)?;
                context.get(&name).ok_or(VarDoesntExist(name))?.clone()
            }
            a => {
                bail!(UnknownNodeType(a))
            }
        };

        Ok(var)
    }

    fn get_primitive(&self, node: Node, context: &mut ParseOutput) -> Result<DataType> {
        match node.kind() {
            "array" => {
                let mut cursor = node.walk();
                let mut val = Vec::new();
                for child in node.children(&mut cursor) {
                    val.push(self.get_primitive(child, context)?);
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
            "variable" => Ok(self.get_var(node, context)?),
            a => {
                dbg!("Here");
                bail!(UnknownNodeType(a))
            }
        }
    }
}

#[derive(Clone)]
pub enum DataType {
    Array(Vec<DataType>),
    String(String),
    Number(usize),
    Child(ParseOutput),
    // Dont calculate variable until runtime.
    Wait(String),
    // Calculate command at request time
    WaitCalc(Command),
    Custom(Rc<Box<dyn 'static + Fn(&ParseOutput, &Vec<DataType>) -> Result<DataType>>>),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Array(array) => {
                let mut string = format!("[{}", array[0]);
                for i in &array[1..] {
                    string.push_str(format!(", {}", i).as_str());
                }

                write!(f, "{}]", string)
            }
            DataType::String(s) => write!(f, "{s}"),
            DataType::Number(n) => write!(f, "{n}"),
            DataType::Child(obj) => {
                write!(
                    f,
                    "{{\n{}}}",
                    obj.vars
                        .iter()
                        .map(|(name, val)| { format!("{}: {}\n", name, val) })
                        .collect::<String>()
                )
            }
            DataType::Wait(_) => todo!(),
            DataType::WaitCalc(_) => todo!(),
            DataType::Custom(_) => todo!(),
        }
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Child(arg0) => f.debug_tuple("Child").field(arg0).finish(),
            Self::Wait(arg0) => f.debug_tuple("Wait").field(arg0).finish(),
            Self::WaitCalc(arg0) => f.debug_tuple("WaitCalc").field(arg0).finish(),
            Self::Custom(_) => f.debug_tuple("Custom").finish(),
        }
    }
}

type Command = (Vec<String>, Vec<DataType>);

#[derive(Default, Debug, Clone)]
pub struct ParseOutput {
    pub(super) vars: BTreeMap<String, DataType>,
    pub(super) to_execute: Vec<Command>,
    pub(super) parent: Option<*mut ParseOutput>,
}

impl ParseOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, string: &String) -> Option<&DataType> {
        self.vars.get(string)
    }

    pub fn set(&mut self, name: impl ToString, val: DataType) {
        self.vars.insert(name.to_string(), val);
    }
}
