use color_eyre::eyre::eyre;
use repkg_common::{provider::PackageProvider, Command};

use crate::{
    exec::cmd_provider::CmdProviderT,
    exec_order_resolver::{Resolver, ResolverT},
    parser, Project,
};

pub struct Executor<'a, P: PackageProvider, C: CmdProviderT<()>> {
    context: &'a Project,
    project_provider: Option<&'a P>,
    cmd_provider: &'a C,
}

impl<'a, P: PackageProvider, C: CmdProviderT<()>> Executor<'a, P, C> {
    pub fn new(context: &'a Project, project_provider: Option<&'a P>, cmd_provider: &'a C) -> Self {
        Self {
            context,
            project_provider,
            cmd_provider,
        }
    }
}

impl<'a, P: PackageProvider, C: CmdProviderT<()>> super::ExecutorT<'a> for Executor<'a, P, C> {
    fn run_command(&self, command: &Command, project: &'a Project) -> color_eyre::Result<()> {
        let prev_path = std::env::current_dir()?;
        std::env::set_current_dir(&project.path.canonicalize()?)?;
        let res = match command.prefix {
            Some('#') => {
                let project = if command.program == "self".to_string() {
                    project
                } else if let Some(project_provider) = self.project_provider {
                    project_provider
                        .get_latest_project(&command.program)
                        .map_err(|e| eyre!("This project does not exist: {}", e))?
                } else {
                    self.context
                        .projects
                        .get(&command.program.clone().into())
                        .ok_or(eyre!("This project does not exist"))?
                };

                for rule_name in &command.args {
                    let initial = project.rules.get(&rule_name.into()).ok_or(eyre!(
                        "Attempted to call rule '{}' for project '{}' but it does not have it",
                        &rule_name,
                        &project.name.0
                    ))?;
                    let exec_order = Resolver::get_tasks(initial, project);

                    self.execute(&exec_order, project)?
                }

                Ok(())
            }
            Some('$') => {
                let args: &Vec<&str> = &(&command.args).into_iter().map(|x| x.as_str()).collect();
                self.cmd_provider
                    .cmd_inner(&command.program, args.as_slice())
            }
            Some('!') => {
                let family = os_info::get().family();

                if command.program
                    != match family {
                        os_info::Family::BSD => "bsd",
                        os_info::Family::Linux => "linux",
                        os_info::Family::MacOS => "macos",
                        os_info::Family::WindowsNT => "windows",
                        os_info::Family::SunOS => "sunos",
                        os_info::Family::Unknown => "unknown",
                        _ => todo!(),
                    }
                {
                    return Ok(());
                }

                let to_parse = command.args.join(" ");
                let command = parser::command().parse(to_parse.as_bytes())?;

                self.run_command(&command, project)
            }
            Some(p) => Err(eyre!("Unsupported prefix '{}'", p)),
            None => {
                let args: &Vec<&str> = &(&command.args).into_iter().map(|x| x.as_str()).collect();
                self.cmd_provider
                    .cmd_external(&command.program, args.as_slice())

                // let status = process::Command::new(&command.program)
                //     .args(&command.args)
                //     .status()?;

                // if status.success() {
                //     Ok(())
                // } else {
                //     Err(eyre!(
                //         "Command '{} {}' failed with exit code '{}'",
                //         command.program,
                //         command.args.join(" "),
                //         status.code().unwrap()
                //     ))
                // }
            }
        };
        std::env::set_current_dir(&prev_path)?;
        res
    }
}
