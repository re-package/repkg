use miette::{bail, miette, Diagnostic, Result};
use repkg_common::registry::Registry;
use thiserror::Error;

use super::tree_walker::{DataType, ParseOutput};

#[derive(Diagnostic, Error, Debug)]
pub enum Error {}

pub struct ContextExecutor;

impl ContextExecutor {
    pub fn execute(parse_output: &ParseOutput, registry: &Registry) -> Result<()> {
        for (command_path, _args) in &parse_output.to_execute {
            let command_path = ContextExecutor::follow_path(command_path, parse_output, registry)?;
            dbg!(&command_path);
        }

        todo!()
    }

    fn follow_path<'a>(
        path: &Vec<String>,
        ctx: &'a ParseOutput,
        _registry: &Registry,
    ) -> Result<&'a DataType> {
        let (name, path) = path.split_last().ok_or(miette!("Path is empty"))?;

        let mut current = ctx;
        for section in path {
            current = match section.as_str() {
                "super" => {
                    let parent = current
                        .parent
                        .as_ref()
                        .ok_or(miette!("This path does not exist"))?;
                    parent
                }
                a => {
                    let value = current
                        .get(&a.to_string())
                        .ok_or(miette!("This path does not exist"))?;
                    match value {
                        DataType::Child(parse_output) => parse_output,
                        _ => bail!(miette!("This is an invalid path")),
                    }
                }
            }
        }

        current.get(name).ok_or(miette!("This path does not exist"))
    }
}
