use color_eyre::eyre::eyre;
use repkg_common::{provider::PackageProvider, Command};
use std::process;

use crate::{
    exec_order_resolver::{Resolver, ResolverT},
    Project,
};

pub struct Executor<'a> {
    context: &'a Project,
}

impl<'a> Executor<'a> {
    pub fn new(context: &'a Project) -> Self {
        Self { context }
    }
}

impl<'a> super::ExecutorT<'a> for Executor<'a> {
    fn run_command(
        &self,
        command: &Command,
        project: &'a Project,
        project_provider: Option<&impl PackageProvider>,
    ) -> color_eyre::Result<()> {
        let prev_path = std::env::current_dir()?;
        std::env::set_current_dir(&project.path.canonicalize()?)?;
        let res = match command.prefix {
            Some('#') => {
                // TODO: add remote projects (ie. dependencies)
                let project = if command.program == "self".to_string() {
                    project
                } else if let Some(project_provider) = project_provider {
                    project_provider.get_latest_project(&command.program)?
                } else {
                    self.context
                        .projects
                        .get(&command.program.clone().into())
                        .ok_or(eyre!("Requested project does not exist"))?
                };

                for rule_name in &command.args {
                    let initial = project.rules.get(&rule_name.into()).ok_or(eyre!(
                        "Attempted to call rule '{}' for project '{}' but it does not have it",
                        &rule_name,
                        &project.name.0
                    ))?;
                    let exec_order = Resolver::get_tasks(initial, project);

                    self.execute(&exec_order, project, project_provider)?
                }

                Ok(())
            }
            // TODO
            Some('$') => Err(eyre!("'$' prefix not supported yet")),
            Some(_) => Err(eyre!(
                "Invalid prefix, the available prefixes are '#' and '$'"
            )),
            None => {
                let status = process::Command::new(&command.program)
                    .args(&command.args)
                    .status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(eyre!(
                        "Command '{} {}' failed with exit code '{}'",
                        command.program,
                        command.args.join(" "),
                        status.code().unwrap()
                    ))
                }
            }
        };
        std::env::set_current_dir(&prev_path)?;
        res
    }
}
