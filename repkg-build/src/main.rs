use std::fs::{read_to_string, OpenOptions};

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec::{Executor, ExecutorT},
    package::Packager,
    parser::{self, project},
    sandbox::{dry_run::DryRunSandbox, SandboxT},
    task_order,
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
            let _dry_run = cli.dry_run || *dry_run;
            let _no_sandbox = cli.no_sandbox || *no_sandbox;

            let content =
                read_to_string(".repkg").map_err(|_| eyre!("No RePkg package file found"))?;

            let mut program = parser::parser().parse(content.as_bytes())??;

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

                let to_exec = command.into();
                let to_exec = task_order::calc_task_order(&[&to_exec], &project)?;

                let sandbox = DryRunSandbox::new();
                let executor = Executor::new(&sandbox);
                executor.execute(&to_exec, &project)?;

                // if !no_sandbox {
                //     let sandbox = Sandbox::new();
                //     let executor = Executor::new(&sandbox);
                //     executor.execute(&to_exec, project)?;
                // } else {
                //     let executor = Executor::new(&sandbox);
                //     executor.execute(&to_exec, &project)?;
                // };
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

            let mut program = project().parse(content.as_bytes())??;

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

                let mut sandbox = DryRunSandbox::new();
                if no_sandbox {
                    // let mut sandbox = None::<SystemCmdProvider>;
                    let packager = Packager::new(&project, &mut sandbox, "output/")?;
                    packager.package()?;
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open("output.tar.gz")?;
                    packager.compress(&mut file)?;
                } else {
                    // let mut sandbox = None::<SandboxCmdProvider>;
                    let packager = Packager::new(&project, &mut sandbox, "output/")?;
                    packager.package()?;
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
