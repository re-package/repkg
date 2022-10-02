use std::{
    fs::{self, read_to_string, OpenOptions},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec::{
        cmd_provider::CmdProviderT, sandbox::SandboxCmdProvider,
        system_cmd_provider::SystemCmdProvider, Executor, ExecutorT,
    },
    exec_order_resolver::{Resolver, ResolverT},
    package::Packager,
    parser::{self, project},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut cli = Cli::parse();

    run(&mut cli)?;

    Ok(())
}

fn run(cli: &mut Cli) -> Result<()> {
    match cli.command.as_ref().unwrap_or(&Command::Run {
        command: "build".to_string(),
        dry_run: false,
        no_sandbox: false,
    }) {
        Command::Run {
            command,
            dry_run,
            no_sandbox,
        } => {
            let content =
                read_to_string(".repkg").map_err(|_| eyre!("No RePkg package file found"))?;

            let mut program = parser::parser().parse(content.as_bytes())?;

            for project in cli
                .projects
                .as_mut()
                .unwrap_or(&mut vec!["root".to_string()])
            {
                let project = if project == &"root".to_string() {
                    &mut program
                } else {
                    program
                        .projects
                        .get_mut(&project.into())
                        .ok_or(eyre!("project '{}' does not exist", project))?
                };

                for (_, project) in &mut project.projects {
                    if let Some(at) = &project.at_ {
                        let content = fs::read_to_string(at)
                            .map_err(|_| eyre!("File '{}' does not exist", at.display()))?;
                        let mut new_project = parser::project().parse(content.as_bytes())?;
                        project.projects.append(&mut new_project.projects);
                        project.rules.append(&mut new_project.rules);
                        project.in_ = at.canonicalize()?.parent().unwrap().to_path_buf();
                    } else if project.in_ != PathBuf::from(".")
                        && project.in_.join(".repkg").exists()
                    {
                        let at = project.in_.join(".repkg");
                        let content = fs::read_to_string(&at)
                            .map_err(|_| eyre!("File '{}' does not exist", at.display()))?;
                        let mut new_project = parser::project().parse(content.as_bytes())?;
                        project.projects.append(&mut new_project.projects);
                        project.rules.append(&mut new_project.rules);
                    }
                }

                if let Some(at) = &project.at_ {
                    let content = fs::read_to_string(at)?;
                    let mut new_project = parser::project().parse(content.as_bytes())?;
                    project.projects.append(&mut new_project.projects);
                    project.rules.append(&mut new_project.rules);
                    project.in_ = at.canonicalize()?.parent().unwrap().to_path_buf();
                }

                let to_exec = project
                    .rules
                    .get(&command.clone().into())
                    .ok_or(eyre!("No rules found matching '{}'", &command))?;

                let to_exec = Resolver::get_tasks(&to_exec, &project);

                // TODO: properly dry run the script
                if !dry_run && !cli.dry_run {
                    if !no_sandbox && !cli.no_sandbox {
                        let sandbox = SandboxCmdProvider::new();
                        let executor = Executor::new(&sandbox);
                        executor.execute(&to_exec, project)?;
                    } else {
                        let sandbox = SystemCmdProvider::new();
                        let executor = Executor::new(&sandbox);
                        executor.execute(&to_exec, &project)?;
                    };
                }
            }
        }
        Command::Build {
            dry_run,
            no_sandbox,
        } => {
            cli.command = Some(Command::Run {
                command: "build".to_string(),
                dry_run: *dry_run,
                no_sandbox: *no_sandbox,
            });
            run(cli)?;
        }
        Command::Test {
            dry_run,
            no_sandbox,
        } => {
            cli.command = Some(Command::Run {
                command: "test".to_string(),
                dry_run: *dry_run,
                no_sandbox: *no_sandbox,
            });
            run(cli)?;
        }
        Command::Package {
            dry_run,
            no_sandbox,
        } => {
            let _dry_run = cli.dry_run || *dry_run;
            let no_sandbox = cli.no_sandbox || *no_sandbox;

            let content = read_to_string(".repkg")?;

            let mut program = project().parse(content.as_bytes())?;

            for project in cli
                .projects
                .as_mut()
                .unwrap_or(&mut vec!["root".to_string()])
            {
                let project = if project == &"root".to_string() {
                    &mut program
                } else {
                    program
                        .projects
                        .get_mut(&project.into())
                        .ok_or(eyre!("project '{}' does not exist", project))?
                };

                for (_, project) in &mut project.projects {
                    if let Some(at) = &project.at_ {
                        let content = fs::read_to_string(at)
                            .map_err(|_| eyre!("File '{}' does not exist", at.display()))?;
                        let mut new_project = parser::project().parse(content.as_bytes())?;
                        project.projects.append(&mut new_project.projects);
                        project.rules.append(&mut new_project.rules);
                        project.in_ = at.canonicalize()?.parent().unwrap().to_path_buf();
                    } else if project.in_ != PathBuf::from(".")
                        && project.in_.join(".repkg").exists()
                    {
                        let at = project.in_.join(".repkg");
                        let content = fs::read_to_string(&at)
                            .map_err(|_| eyre!("File '{}' does not exist", at.display()))?;
                        let mut new_project = parser::project().parse(content.as_bytes())?;
                        project.projects.append(&mut new_project.projects);
                        project.rules.append(&mut new_project.rules);
                        project.in_ = at.canonicalize()?.parent().unwrap().to_path_buf();
                    }
                }

                if let Some(at) = &project.at_ {
                    let content = fs::read_to_string(at)?;
                    let mut new_project = parser::project().parse(content.as_bytes())?;
                    project.projects.append(&mut new_project.projects);
                    project.rules.append(&mut new_project.rules);
                    project.in_ = at.canonicalize()?.parent().unwrap().to_path_buf();
                }

                if no_sandbox {
                    let mut sandbox = None::<SystemCmdProvider>;
                    let mut packager = Packager::new(&project, &mut sandbox);
                    packager.package_to(&"output/")?;
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open("output.tar.gz")?;
                    packager.compress(&mut file)?;
                } else {
                    let mut sandbox = None::<SandboxCmdProvider>;
                    let mut packager = Packager::new(&project, &mut sandbox);
                    packager.package_to(&"output/")?;
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open("output.tar.gz")?;
                    packager.compress(&mut file)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
    #[clap(short, long = "project")]
    projects: Option<Vec<String>>,
    /// Parse the script, but don't do anything.
    #[clap(short, long)]
    dry_run: bool,
    #[clap(short, long)]
    no_sandbox: bool,
}

#[derive(Subcommand)]
enum Command {
    Run {
        command: String,
        #[clap(short, long)]
        dry_run: bool,
        #[clap(short, long)]
        no_sandbox: bool,
    },
    Build {
        #[clap(short, long)]
        dry_run: bool,
        #[clap(short, long)]
        no_sandbox: bool,
    },
    Test {
        #[clap(short, long)]
        dry_run: bool,
        #[clap(short, long)]
        no_sandbox: bool,
    },
    Package {
        #[clap(short, long)]
        dry_run: bool,
        #[clap(short, long)]
        no_sandbox: bool,
    },
}
