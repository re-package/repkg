use std::{collections::BTreeMap, rc::Rc};

use miette::{bail, Diagnostic, NamedSource, Result, SourceSpan};
use repkg_core::protocols;
use thiserror::Error;

use super::{
    ast::{Error::*, Sourced},
    Command, Import, Value, ValueType,
};
use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Expected object!")]
    ExpectedObject,
    #[error("Expected function definition")]
    #[diagnostic(code(repkg_build::script::expected_func_def))]
    ExpectedFuncDef(#[label("here")] SourceSpan),
    #[error("Variable {:?} doesn't exist", .0)]
    #[diagnostic(code(repkg_build::script::var_doesnt_exist))]
    VarDoesntExist(Vec<String>),
}

pub struct VM {
    ffi: BTreeMap<String, Value>,
}

pub type Object = BTreeMap<String, Value>;

impl VM {
    pub fn init() -> Self {
        Self {
            ffi: BTreeMap::new(),
        }
    }

    pub fn with_ffi(&mut self, ffi: Value) -> Result<&mut Self> {
        match ffi.val {
            ValueType::Object(mut vars) => {
                self.ffi.append(&mut vars);
            }
            _ => {
                bail!(ExpectedObject)
            }
        }
        Ok(self)
    }

    pub async fn build(&self, object: &Value) -> Result<()> {
        match &object.val {
            ValueType::Object(vars) => {
                let imports = if let Some(imports) = vars.get("imports") {
                    self.imports(imports)?
                } else {
                    vec![]
                };

                for import in &imports {
                    // TODO: better errors
                    protocols::client::handshake::handshake(import.url.parse().unwrap()).await?;
                }

                if let Some(build) = vars.get("build") {
                    match &build.val {
                        ValueType::Function(cmds, _ret) => {
                            for cmd in cmds {
                                self.run_command(cmd, &imports, &vars, &vec![])?;
                            }
                        }
                        _ => {
                            bail!(Sourced::new(
                                ExpectedFuncDef(build.location.clone().into()),
                                NamedSource::new(
                                    build.name.as_ref(),
                                    build.source.as_ref().clone()
                                )
                            ))
                        }
                    }
                };
            }
            _ => {
                bail!(ExpectedObject)
            }
        }

        Ok(())
    }

    pub fn run(&self, object: &Value, args: Vec<String>) -> Result<Value> {
        Ok(match &object.val {
            ValueType::Object(vars) => {
                let imports = if let Some(imports) = vars.get("imports") {
                    self.imports(imports)?
                } else {
                    vec![]
                };

                if let Some(run) = vars.get("run") {
                    self.run_func(&vars, run, &args, &imports)?
                } else {
                    Value::new(
                        0..0,
                        ValueType::Number(0),
                        Rc::new("".to_string()),
                        Rc::new("".to_string()),
                    )
                }
            }
            _ => {
                dbg!(&object.val);
                bail!(ExpectedObject)
            }
        })
    }

    pub fn imports(&self, object: &Value) -> Result<Vec<Import>> {
        let mut imports = vec![];

        match &object.val {
            ValueType::Object(vars) => {
                for (name, path) in vars {
                    // TODO: enforce correct urls
                    let url = match &path.val {
                        ValueType::String(string) => string.clone(),
                        _ => {
                            bail!(Sourced::new(
                                ExpectedString(object.location.clone().into()),
                                NamedSource::new(path.name.as_ref(), path.source.as_ref().clone()),
                            ))
                        }
                    };

                    imports.push(Import {
                        name: name.clone(),
                        url,
                    })
                }
            }
            _ => {
                bail!(ExpectedObject)
            }
        }

        Ok(imports)
    }

    fn run_func(
        &self,
        obj: &Object,
        func: &Value,
        args: &Vec<String>,
        imports: &Vec<Import>,
    ) -> Result<Value> {
        Ok(match &func.val {
            ValueType::Function(cmds, ret) => {
                for cmd in cmds {
                    self.run_command(cmd, &imports, &obj, &args)?;
                }
                ret.as_ref().clone()
            }
            _ => {
                bail!(Sourced::new(
                    ExpectedFuncDef(func.location.clone().into()),
                    NamedSource::new(func.name.as_ref(), func.source.as_ref().clone())
                ))
            }
        })
    }

    fn run_command(
        &self,
        cmd: &Command,
        imports: &Vec<Import>,
        obj: &BTreeMap<String, Value>,
        args: &Vec<String>,
    ) -> Result<()> {
        let path = self.handle_path(&cmd.path, imports, obj)?;

        if cmd.args.len() == 0 {
            // Add as dependency
            todo!()
        } else {
            let args = args
                .iter()
                .map(|x| if x == "$" { args.join(" ") } else { x.clone() })
                .collect();
            let vm = VM::init();
            let result = vm.run_func(obj, &path, &args, imports)?;
            dbg!(result);
        }

        Ok(())
    }

    fn handle_path(
        &self,
        path: &Vec<String>,
        imports: &Vec<Import>,
        obj: &BTreeMap<String, Value>,
    ) -> Result<Value> {
        if path.len() == 0 {
            todo!()
        } else if path.len() == 1 {
            let part = &path[0];
            let val = if let Some(val) = obj.get(part) {
                val.clone()
            } else {
                let mut out = None;
                for Import { name, url: _url } in imports {
                    if name == part {
                        out = Some(Value::new(
                            0..0,
                            ValueType::Number(0),
                            Rc::new("d".to_string()),
                            Rc::new("d".to_string()),
                        ));
                    }
                }
                out.ok_or_else(|| VarDoesntExist(path.to_vec()))?.clone()
            };
            Ok(val)
        } else {
            let first_part = &path[0];
            let first_part = self.handle_path(&vec![first_part.clone()], imports, obj)?;
            let rest = &path[1..];
            self.path_inner(Vec::from(rest), &first_part)
        }
    }

    fn path_inner(&self, path: Vec<String>, obj: &Value) -> Result<Value> {
        let mut cur = obj;
        for segment in &path {
            match &cur.val {
                ValueType::Object(vars) => {
                    cur = vars
                        .get(segment)
                        .ok_or_else(|| VarDoesntExist(path.clone()))?;
                }
                _ => {
                    bail!(ExpectedObject)
                }
            }
        }
        Ok(cur.clone())
    }
}
