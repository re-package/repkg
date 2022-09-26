use color_eyre::Result;
use repkg_common::{provider::PackageProvider, Command, Rule};

use crate::Project;

mod executor;
pub use executor::*;

pub trait ExecutorT<'a> {
    fn execute(
        &self,
        rules: &Vec<&Rule>,
        has_rules: &'a Project,
        project_provider: Option<&impl PackageProvider>,
    ) -> Result<()> {
        for rule in rules {
            for command in &rule.cmds {
                self.run_command(command, has_rules, project_provider)?;
            }
        }

        Ok(())
    }

    fn run_command(
        &self,
        command: &Command,
        project: &'a Project,
        project_provider: Option<&impl PackageProvider>,
    ) -> Result<()>;
}
