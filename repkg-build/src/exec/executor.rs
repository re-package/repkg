use std::{
    cell::RefCell,
    process::{self},
    rc::Rc,
};

use color_eyre::{eyre::eyre, Result};

use repkg_common::{repository::Repository, Command, Rule};

use crate::{
    parser,
    sandbox::{CmdProvider, FileSystem},
    task_order, Project,
};

pub struct Executor<'a, 'b, F: FileSystem, S: CmdProvider<'a, F>> {
    sandbox: Rc<RefCell<S>>,
    repository: &'b Repository,
    // Get rid of compiler errors
    _fs: Option<&'a F>,
}

impl<'a, 'b, S: CmdProvider<'a, F>, F: FileSystem> Executor<'a, 'b, F, S> {
    pub fn new(sandbox: Rc<RefCell<S>>, repository: &'b Repository) -> Self {
        Self {
            sandbox,
            _fs: None,
            repository,
        }
    }
}

impl<'a, 'b, S: CmdProvider<'a, F>, F: FileSystem> Executor<'a, 'b, F, S> {
    fn run_command(&self, command: &Command, project: &Project) -> color_eyre::Result<()> {
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
                self.sandbox
                    .borrow_mut()
                    .command(&command.programs[0], args.as_slice())
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
                // self.sandbox
                //     .borrow()
                //     .executable(&command.programs[0])?
                //     .args(args)
                //     .spawn()?;

                if let Some(path) = self
                    .repository
                    .file(format!("{}.exe", command.programs.join("")))?
                {
                    // TODO: enforce cross-compilation targets
                    let status = process::Command::new(path).args(args).status()?;
                    if !status.success() {
                        return Err(eyre!(
                            "process '{}' failed: {}",
                            command.programs.join(""),
                            status
                        ));
                    }
                } else if let Some(path) = self.repository.file(command.programs.join(""))? {
                    let status = process::Command::new(path).args(args).status()?;
                    if !status.success() {
                        return Err(eyre!(
                            "process '{}' failed: {}",
                            command.programs.join(""),
                            status
                        ));
                    }
                } else {
                    return Err(eyre!(
                        "The command '{}' does not exist",
                        command.programs.join("")
                    ));
                }

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

    pub fn execute(&self, rules: &Vec<&Rule>, project: &Project) -> Result<()> {
        for rule in rules {
            for command in &rule.cmds {
                self.run_command(command, project)?;
            }
        }

        Ok(())
    }
}
