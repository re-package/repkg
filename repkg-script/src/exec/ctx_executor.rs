use std::fmt::Debug;

use miette::{bail, miette, Diagnostic, Result};
use repkg_common::registry::Registry;
use thiserror::Error;

use crate::exec::tree_walker;

use super::tree_walker::{DataType, ParseOutput};

use Error::*;

#[derive(Diagnostic, Error, Debug)]
pub enum Error {
    #[error("This path does not exist: {:?}", .0)]
    #[diagnostic(code(repkg_script::path_doesnt_exist))]
    PathDoesntExist(Vec<String>),
    #[error("Failed to convert a string to a number")]
    #[diagnostic(code(repkg_script::executor::number_conversion_error))]
    NumberConversionError,
}

pub struct ContextExecutor;

impl ContextExecutor {
    pub fn execute(
        parse_output: &ParseOutput,
        registry: &Registry,
        parent_args: Vec<DataType>,
        inject: &ParseOutput,
    ) -> Result<()> {
        // TODO: proper implementation of standard library
        //         let mut inject = TreeWalker::parse_text(
        //             "
        // repkg-version {
        //     echo '1.0.0'
        // }
        //             ",
        //         )?
        //         .walk()?;

        //         inject.set(
        //             "todo",
        //             DataType::Custom(Rc::new(Box::new(|_ctx, args| {
        //                 for arg in args {
        //                     println!("todo: {arg:?}");
        //                 }
        //                 DataType::Number(0)
        //             }))),
        //         );

        for (command_path, args) in &parse_output.to_execute {
            // let command_path =
            //     ContextExecutor::follow_path(&command_path, parse_output, registry, &standard_lib)?;

            // let args = Self::handle_args(args, &parent_args, parse_output)?;

            // match command_path {
            //     DataType::Child(parse_output) => {
            //         ContextExecutor::execute(parse_output, registry, args)?;
            //     }
            //     DataType::Custom(custom_command) => {
            //         custom_command(parse_output, &args);
            //     }
            //     a => {
            //         bail!(miette!("Invalid command: {:?}", a))
            //     }
            // }
            Self::handle_command(
                command_path,
                args,
                parse_output,
                registry,
                &inject,
                &parent_args,
            )?;
        }

        Ok(())
    }

    fn handle_command(
        command_path: &Vec<String>,
        args: &Vec<DataType>,
        ctx: &ParseOutput,
        registry: &Registry,
        inject: &ParseOutput,
        parent_args: &Vec<DataType>,
    ) -> Result<DataType> {
        let command_path = ContextExecutor::follow_path(&command_path, ctx, registry, inject)?;

        let args = Self::handle_args(args, parent_args, ctx, registry, inject)?;

        match command_path {
            DataType::Child(parse_output) => {
                ContextExecutor::execute(parse_output, registry, args, inject)?;
                Ok(DataType::Number(0))
            }
            DataType::Custom(custom_command) => custom_command(ctx, &args),
            a => {
                bail!(miette!("Invalid command: {:?}", a))
            }
        }
    }

    fn handle_args(
        args_in: &Vec<DataType>,
        global_args: &Vec<DataType>,
        ctx: &ParseOutput,
        registry: &Registry,
        inject: &ParseOutput,
    ) -> Result<Vec<DataType>> {
        let mut args = vec![];
        for arg in args_in {
            args.push(match arg {
                DataType::Wait(var_name) => {
                    if var_name.eq(&String::from("args")) {
                        DataType::Array(global_args.to_vec())
                    } else if var_name.starts_with("arg") {
                        let var_number: usize =
                            var_name[3..].parse().map_err(|_| NumberConversionError)?;
                        let out = global_args
                            .get(var_number)
                            .ok_or(tree_walker::Error::VarDoesntExist(var_name.to_string()))?
                            .clone();
                        out
                    } else {
                        ctx.get(var_name)
                            .ok_or(tree_walker::Error::VarDoesntExist(var_name.to_string()))?
                            .clone()
                    }
                }
                DataType::WaitCalc((command_path, args)) => {
                    Self::handle_command(command_path, args, ctx, registry, inject, global_args)?
                }
                _ => arg.clone(),
            })
        }
        Ok(args)
    }

    fn follow_path<'a>(
        first_path: &Vec<String>,
        ctx: &'a ParseOutput,
        _registry: &Registry,
        inject: &'a ParseOutput,
    ) -> Result<&'a DataType> {
        let (name, path) = first_path.split_last().ok_or(miette!("Path is empty"))?;

        let mut current = ctx;
        for (idx, section) in path.iter().enumerate() {
            current = match section.as_str() {
                "super" => {
                    let parent = unsafe {
                        current
                            .parent
                            .ok_or_else(|| miette!("This path does not exist"))?
                            .as_ref()
                    }
                    .ok_or(miette!("The pointer was null"))?;

                    parent
                }
                a => {
                    let a = a.to_string();
                    // Allow injected paths
                    let value = if idx > 0 {
                        current.get(&a).ok_or(PathDoesntExist(first_path.clone()))?
                    } else {
                        if let Some(a) = current.get(&a) {
                            a
                        } else {
                            dbg!(inject);
                            inject.get(&a).ok_or(PathDoesntExist(first_path.clone()))?
                        }
                    };
                    match value {
                        DataType::Child(parse_output) => &parse_output,
                        _ => bail!(miette!("This is an invalid path")),
                    }
                }
            }
        }

        let mut out = current.get(name);

        if let None = out {
            out = inject.get(name)
        }

        Ok(out.ok_or_else(|| PathDoesntExist(first_path.clone()))?)
    }
}
