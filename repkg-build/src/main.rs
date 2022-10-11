use std::{
    cell::RefCell,
    env,
    fs::{read_to_string, OpenOptions},
    rc::Rc,
};

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    cmd_provider::{dry_run::DryRunCMD, CmdProvider},
    exec::Executor,
    package::Packager,
    parser::{self, project},
    task_order,
};
use repkg_common::repository::Repository;

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

                // TODO: change repkg-repo to something more useful
                let repository = Repository::new(env::current_dir()?.join("repkg-repo"))?;

                let sandbox = Rc::new(RefCell::new(DryRunCMD::new()));
                let executor = Executor::new(sandbox, &repository);
                executor.execute(&to_exec, &project)?;
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
            let _no_sandbox = cli.no_sandbox || *no_sandbox;

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

                // TODO: change repkg-repo to something more useful
                let repository = Repository::new(env::current_dir()?.join("repkg-repo"))?;

                let sandbox = Rc::new(RefCell::new(DryRunCMD::new()));
                let packager = Packager::new(project, sandbox, "output/", repository)?;
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open("output.tar.gz")?;
                packager.package()?.compress(&mut file)?;
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
