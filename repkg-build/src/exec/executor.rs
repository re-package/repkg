use color_eyre::eyre::eyre;
use repkg_common::Command;

use crate::{
    parser,
    sandbox::{FileSystem, SandboxT},
    task_order, Project,
};

pub struct Executor<'a, F: FileSystem, S: SandboxT<'a, F>> {
    sandbox: &'a S,
    // Get rid of compiler errors
    _fs: Option<F>,
}

impl<'a, S: SandboxT<'a, F>, F: FileSystem> Executor<'a, F, S> {
    pub fn new(sandbox: &'a S) -> Self {
        Self { sandbox, _fs: None }
    }
}

impl<'a, S: SandboxT<'a, F>, F: FileSystem> super::ExecutorT<'a> for Executor<'a, F, S> {
    fn run_command(&self, command: &Command, project: &'a Project) -> color_eyre::Result<()> {
        let prev_path = std::env::current_dir()?;
        std::env::set_current_dir(&project.in_.canonicalize()?)?;
        let res = match command.prefix {
            Some('#') => {
                let mut project: &Project = project;

                for project_name in &command.programs {
                    // Already on self
                    if project_name == "self" {
                    } else {
                        project = project
                            .projects
                            .get(&project_name.into())
                            .ok_or(eyre!("The requested project does not exist"))?;
                    }
                }

                for rule_name in &command.args {
                    let initial = rule_name.into();
                    let exec_order = task_order::calc_task_order(&[&initial], project)?;

                    self.execute(&exec_order, project)?
                }

                Ok(())
            }
            Some('$') => {
                let args: &Vec<&str> = &(&command.args).into_iter().map(|x| x.as_str()).collect();
                self.sandbox.command(&command.programs[0], args.as_slice())
            }
            Some('!') => {
                let family = os_info::get().family();

                if command.programs[0]
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
                // self.sandbox
                //     .cmd_external(&command.programs[0], args.as_slice())
                self.sandbox
                    .executable(&command.programs[0])?
                    .args(args)
                    .spawn()?;
                Ok(())

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
