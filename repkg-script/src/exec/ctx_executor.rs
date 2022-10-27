use miette::{bail, miette, Diagnostic, Result};
use repkg_common::registry::Registry;
use thiserror::Error;

use super::tree_walker::{DataType, ParseOutput, TreeWalker};

use Error::*;

#[derive(Diagnostic, Error, Debug)]
pub enum Error {
    #[error("This path does not exist: {:?}", .0)]
    PathDoesntExist(Vec<String>),
}

pub struct ContextExecutor;

impl ContextExecutor {
    pub fn execute(parse_output: &ParseOutput, registry: &Registry) -> Result<()> {
        // TODO: proper implementation of standard library
        let mut standard_lib = TreeWalker::parse_text(
            "
todo {
    echo 'TODO!'
}
        ",
        )?
        .walk()?;

        standard_lib.set("echo", DataType::Child(ParseOutput::default()));

        for (command_path, _args) in &parse_output.to_execute {
            let command_path =
                ContextExecutor::follow_path(command_path, parse_output, registry, &standard_lib)?;
            match command_path {
                DataType::Child(parse_output) => {
                    ContextExecutor::execute(parse_output, registry)?;
                }
                a => {
                    dbg!(a);
                }
            }
        }

        todo!()
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
