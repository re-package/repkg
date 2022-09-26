use color_eyre::Result;

use crate::{rule::Rule, Command, Project};

mod executor;
pub use executor::*;

pub trait ExecutorT<'a> {
    fn execute(&self, rules: &Vec<&Rule>, has_rules: &'a Project) -> Result<()> {
        for rule in rules {
            for command in &rule.cmds {
                self.run_command(command, has_rules)?;
            }
        }

        Ok(())
    }

    fn run_command(&self, command: &Command, project: &'a Project) -> Result<()>;
}
