use std::{
    collections::HashMap,
    process::{self, ExitStatus},
};

use miette::{bail, Diagnostic, Result};

use repkg_common::{repository::Repository, Command, Task};
use thiserror::Error;

use crate::{parser, task_order, Project};

use super::{commands::commands, CommandT};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("The requested project does not exist: {}", .0)]
    #[diagnostic(code(repkg_build::executor::ProjectDoesntExist))]
    ProjectDoesntExist(String),
    #[error("The command '{}' does not exist", .0)]
    #[diagnostic(code(repkg_build::executor::CommandDoesntExist))]
    CommandDoesntExist(String),
    #[error("Unsupported prefix '{}'", .0)]
    #[diagnostic(code(repkg_build::executor::UnsupportedPrefix))]
    UnsupportedPrefix(char),
    #[error("Process '{}' failed: {}", .0, .1)]
    #[diagnostic(code(repkg_build::executor::ProcessFailed))]
    ProcessFailed(String, ExitStatus),
}

pub struct Executor<'a> {
    repository: &'a Repository,
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl<'a> Executor<'a> {
    pub fn new(repository: &'a Repository) -> Self {
        Self {
            repository,
            commands: commands(),
        }
    }

    pub fn reg_cmd(&mut self, program: &str, cmd: impl CommandT + 'static) -> &mut Self {
        self.commands.insert(program.to_string(), Box::new(cmd));
        self
    }
}

impl<'a> Executor<'a> {
    fn run_command(&self, command: &Command, project: &Project) -> Result<()> {
        let prev_path = std::env::current_dir().map_err(crate::io_error)?;
        std::env::set_current_dir(&project.in_.canonicalize().map_err(crate::io_error)?)
            .map_err(crate::io_error)?;
        match command.prefix {
            Some('#') => {
                let mut project: &Project = project;

                for project_name in &command.programs {
                    // Already on self
                    if project_name == "self" {
                    } else {
                        project = project
                            .projects
                            .get(project_name)
                            .ok_or_else(|| Error::ProjectDoesntExist(project_name.clone()))?;
                    }
                }

                for task_name in &command.args {
                    let tasks = &[task_name];
                    let exec_order = task_order::calc_task_order(tasks, project)?;

                    self.execute(&exec_order, project)?
                }
            }
            Some('$') => {
                let args: &Vec<&str> = &(command.args).iter().map(|x| x.as_str()).collect();
                // self.sandbox
                //     .borrow_mut()
                //     .command(&command.programs[0], args.as_slice())
                let cmd = self
                    .commands
                    .get(&command.programs[0])
                    .ok_or_else(|| Error::CommandDoesntExist(command.programs[0].clone()))?;
                cmd.call(args.as_slice())?;
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
                let command = parser::command()
                    .parse(to_parse.as_bytes())
                    .map_err(crate::parser::Error::ParseError)?;

                self.run_command(&command, project)?;
            }
            Some(p) => bail!(Error::UnsupportedPrefix(p)),
            None => {
                let args: &Vec<&str> = &(command.args).iter().map(|x| x.as_str()).collect();

                if let Some(path) = self
                    .repository
                    .file(format!("{}.exe", command.programs.join("")))?
                {
                    // TODO: enforce cross-compilation targets
                    let status = process::Command::new(path)
                        .args(args)
                        .status()
                        .map_err(crate::io_error)?;
                    if !status.success() {
                        bail!(Error::ProcessFailed(command.programs.join(""), status));
                    }
                } else if let Some(path) = self.repository.file(command.programs.join(""))? {
                    let status = process::Command::new(path)
                        .args(args)
                        .status()
                        .map_err(crate::io_error)?;
                    if !status.success() {
                        bail!(Error::ProcessFailed(command.programs.join(""), status))
                    }
                } else {
                    bail!(Error::CommandDoesntExist(command.programs.join("")));
                }
            }
        };
        std::env::set_current_dir(&prev_path).map_err(crate::io_error)?;
        Ok(())
    }

    pub fn execute(&self, rules: &Vec<&Task>, project: &Project) -> Result<()> {
        for rule in rules {
            for command in &rule.cmds {
                self.run_command(command, project)?;
            }
        }

        Ok(())
    }
}
