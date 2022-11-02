use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
    str::Utf8Error,
};

use miette::{bail, miette, Diagnostic, Result, SourceSpan};
use thiserror::Error;
use tree_sitter::{Node, Tree};

use Error::*;

use crate::script_std;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Expected one of {}", format_vec(.expected))]
    Expected {
        #[source_code]
        source_code: String,
        #[label("here")]
        span: SourceSpan,
        expected: Vec<String>,
    },
    #[error(transparent)]
    #[diagnostic(code(repkg_script::from_utf8_error))]
    Utf8Error(#[from] Utf8Error),
    #[error("Variable doesnt exist: {}", .name)]
    #[diagnostic(code(repkg_script::var_doesnt_exist))]
    VarDoesntExist {
        #[source_code]
        source_code: String,
        #[label("here")]
        span: SourceSpan,
        name: String,
    },
    #[error("Invalid number, expected usize")]
    #[diagnostic(code(repkg_script::invalid_number))]
    InvalidNumber {
        #[source_code]
        source_code: String,
        #[label("here")]
        span: SourceSpan,
    },
}

fn format_vec<T: Display>(vec: &Vec<T>) -> String {
    if vec.len() == 0 {
        String::new()
    } else {
        let first_part = format!("{}", vec[0]);
        let mut second_part = String::new();

        for part in &vec[1..] {
            second_part.push_str(format!(", {}", part).as_str())
        }

        format!("{}{}", first_part, second_part)
    }
}

#[derive(Clone)]
pub enum DataType<'a> {
    Array(Vec<DataType<'a>>),
    String(String),
    Number(usize),
    Node(Node<'a>),
    Object {
        ctx: Context<'a>,
        name: String,
        body_node: Node<'a>,
    },
    Custom(Rc<Box<dyn 'static + Fn(&Context, &Vec<DataType>) -> Result<DataType<'a>>>>),
}

impl<'a> Display for DataType<'a> {
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
            DataType::Node(_) => {
                write!(f, "object")
            }
            DataType::Custom(_) => todo!(),
            DataType::Object {
                ctx,
                name,
                body_node: _,
            } => write!(f, "{}: {{{:?}}}", name, ctx),
        }
    }
}

impl<'a> Debug for DataType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Node(arg0) => f.debug_tuple("Node").field(arg0).finish(),
            Self::Object {
                ctx,
                name,
                body_node: _,
            } => f
                .debug_struct("Object")
                .field("ctx", ctx)
                .field("name", name)
                .finish(),
            Self::Custom(_) => f.debug_tuple("Custom").finish(),
        }
    }
}

pub type Context<'a> = HashMap<String, DataType<'a>>;

pub struct Executor {
    source_code: String,
    tree: Tree,
}

impl Executor {
    pub fn new(tree: Tree, source_code: String) -> Self {
        Self { source_code, tree }
    }

    pub fn execute_tree(&self) -> Result<()> {
        let mut cursor = self.tree.walk();
        let root_node = self.tree.root_node();
        let mut context = Context::new();

        let mut inject = Context::new();
        inject.insert(
            "echo".to_string(),
            DataType::Custom(Rc::new(Box::new(script_std::echo::command))),
        );

        for child in root_node.children(&mut cursor) {
            match child.kind() {
                "comment" => {}
                "namespace" => {
                    let obj = self.object(child, &mut context)?;
                    match &obj {
                        DataType::Object {
                            ctx: _,
                            name,
                            body_node: _,
                        } => {
                            context.insert(name.clone(), obj);
                        }
                        _ => todo!(),
                    }
                }
                "command" => {
                    self.command(child, &mut context, &inject, &vec![])?;
                }
                "" => {}
                "\n" => {}
                a => {
                    dbg!(a);

                    let start = child.byte_range();
                    bail!(Expected {
                        source_code: self.source_code.clone(),
                        span: start.into(),
                        expected: vec!["namespace".to_string(), "command".to_string()]
                    })
                }
            }
        }

        Ok(())
    }

    fn object<'a>(&self, node: Node<'a>, _context: &mut Context<'a>) -> Result<DataType<'a>> {
        let id = self.get_id(&node)?;
        let new_context = Context::new();

        let body = node.child_by_field_name("body").ok_or(Expected {
            source_code: self.source_code.clone(),
            span: node.byte_range().into(),
            expected: vec!["body".to_string()],
        })?;

        Ok(DataType::Object {
            ctx: new_context,
            name: id,
            body_node: body,
        })
    }

    fn exec_obj<'a>(
        &self,
        object: &mut DataType<'a>,
        args: &Vec<DataType<'a>>,
        inject: &Context<'a>,
    ) -> Result<DataType<'a>> {
        if let DataType::Object {
            ctx,
            name: _,
            body_node,
        } = object
        {
            self.body(*body_node, ctx, inject, args)
        } else {
            todo!()
        }
    }

    fn body<'a>(
        &self,
        node: Node,
        context: &mut Context<'a>,
        inject: &Context<'a>,
        args: &Vec<DataType<'a>>,
    ) -> Result<DataType<'a>> {
        let mut cursor = node.walk();

        let mut value = None;
        for child in node.children(&mut cursor) {
            match child.kind() {
                "command" => {
                    value = Some(self.command(child, context, inject, &args)?);
                }
                _ => {}
            }
        }

        Ok(value.unwrap_or(DataType::Number(0)))
    }

    fn command<'a>(
        &self,
        node: Node,
        context: &mut Context<'a>,
        inject: &Context<'a>,
        parent_args: &Vec<DataType<'a>>,
    ) -> Result<DataType<'a>> {
        let program = node.child_by_field_name("program").ok_or(Expected {
            source_code: self.source_code.clone(),
            span: node.start_byte().into(),
            expected: vec![
                "identifier".to_string(),
                "variable".to_string(),
                "path".to_string(),
            ],
        })?;
        let mut cursor = node.walk();
        let arg_nodes = node.children_by_field_name("args", &mut cursor);
        let mut args = vec![];
        for arg in arg_nodes {
            args.push(match arg.kind() {
                "text" => DataType::String(self.node_to_string(arg)?),
                "variable" => self.get_var(arg, context, parent_args)?,
                "nested_identifier" => self.handle_path(arg, context, inject, parent_args)?,
                "primitive" => {
                    let mut cursor = arg.walk();
                    cursor.goto_first_child();
                    self.get_primitive(cursor.node(), context)?
                }
                a => {
                    dbg!(a);
                    todo!()
                }
            })
        }

        let mut program = self.handle_path(program, context, inject, &args)?;

        Ok(match program {
            DataType::Custom(custom_command) => custom_command(context, &args)?,
            DataType::Object {
                ctx: _,
                name: _,
                body_node: _,
            } => self.exec_obj(&mut program, &args, inject)?,
            a => a,
        })
    }

    fn get_var<'a>(
        &self,
        node: Node,
        context: &mut Context<'a>,
        args: &Vec<DataType<'a>>,
    ) -> Result<DataType<'a>> {
        let var = node.child_by_field_name("var").unwrap();

        let var = match var.kind() {
            "number" => {
                dbg!("number");
                let num = self.get_primitive(var, context)?;
                match num {
                    DataType::Number(a) => args
                        .get(a)
                        .ok_or_else(|| {
                            dbg!(a);
                            VarDoesntExist {
                                source_code: self.source_code.clone(),
                                span: var.byte_range().into(),
                                name: format!("%%{}", a),
                            }
                        })?
                        .clone(),
                    _ => {
                        bail!(miette!("Number must be a number!"))
                    }
                }
            }
            "percent_sign" => DataType::Array(args.clone()),
            "text" => {
                let name = self.node_to_string(var)?;
                context
                    .get(&name)
                    .ok_or(VarDoesntExist {
                        source_code: self.source_code.clone(),
                        span: var.byte_range().into(),
                        name,
                    })?
                    .clone()
            }
            _ => {
                dbg!("var-fail");
                bail!(Expected {
                    source_code: self.source_code.clone(),
                    span: var.byte_range().into(),
                    expected: vec!["number".to_string(), "%".to_string(), "text".to_string()]
                })
            }
        };

        Ok(var)
    }

    fn get_primitive<'a>(&self, node: Node, context: &mut Context<'a>) -> Result<DataType<'a>> {
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
                    .utf8_text(self.source_code.as_bytes())
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
                let string = self.node_to_string(node)?;
                let num: usize = string.parse().map_err(|_| InvalidNumber {
                    source_code: self.source_code.clone(),
                    span: node.byte_range().into(),
                })?;

                Ok(DataType::Number(num))
            }
            a => {
                dbg!(a);
                bail!(Expected {
                    source_code: self.source_code.clone(),
                    span: node.byte_range().into(),
                    expected: vec![
                        "number".to_string(),
                        "string".to_string(),
                        "array".to_string()
                    ]
                })
            }
        }
    }

    fn handle_path<'a>(
        &self,
        node: Node,
        context: &mut Context<'a>,
        inject: &Context<'a>,
        args: &Vec<DataType<'a>>,
    ) -> Result<DataType<'a>> {
        let mut out = match node.kind() {
            "identifier" => {
                let name = self.node_to_string(node)?;
                if let Some(out) = context.get(&name) {
                    out
                } else {
                    inject.get(&name).ok_or(VarDoesntExist {
                        source_code: self.source_code.clone(),
                        span: node.byte_range().into(),
                        name,
                    })?
                }
                .clone()
            }
            "nested_identifier" => {
                let path_node = node.child_by_field_name("path").unwrap();
                let name = node.child_by_field_name("name").unwrap();
                let mut path = self.handle_path(path_node, context, inject, args)?;
                dbg!(&path);
                match &path {
                    DataType::Object {
                        ctx: _,
                        name: _,
                        body_node: _,
                    } => {
                        // Parse object
                        self.exec_obj(&mut path, args, inject)?;
                        match path {
                            DataType::Object {
                                ctx,
                                name: _,
                                body_node: _,
                            } => {
                                dbg!(&ctx);
                            }
                            _ => panic!(),
                        }

                        todo!()
                    }
                    _ => {
                        bail!(VarDoesntExist {
                            source_code: self.source_code.clone(),
                            span: node.byte_range().into(),
                            name: self.node_to_string(name)?,
                        })
                    }
                }
            }
            "variable" => {
                dbg!("variable");
                self.get_var(node, context, args)?
            }
            a => {
                dbg!(a);
                todo!()
            }
        };

        match &out {
            DataType::Object {
                ctx: _,
                name: _,
                body_node: _,
            } => {
                self.exec_obj(&mut out, args, inject)?;
            }
            _ => {}
        };

        Ok(out)
    }

    fn node_to_string(&self, node: Node) -> Result<String> {
        Ok(node
            .utf8_text(self.source_code.as_bytes())
            .map_err(Utf8Error)
            .map(ToString::to_string)?)
    }

    fn get_id(&self, node: &Node) -> Result<String> {
        let id = node.child_by_field_name("id").ok_or(Expected {
            source_code: self.source_code.clone(),
            span: node.start_byte().into(),
            expected: vec!["identifier".to_string()],
        })?;
        Ok(id
            .utf8_text(self.source_code.as_bytes())
            .map_err(Utf8Error)
            .map(ToString::to_string)?)
    }
}
