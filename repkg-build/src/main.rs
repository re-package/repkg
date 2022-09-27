use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec::{Executor, ExecutorT},
    exec_order_resolver::{Resolver, ResolverT},
    parser::parser,
};
use repkg_common::provider::NonePackageProvider;

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
    }) {
        Command::Run { command, dry_run } => {
            let content = read_to_string("PACKAGE.repkg")?;

            let program = parser().parse(content.as_bytes())?;

            for project in cli.projects.as_ref().unwrap_or(&vec!["root".to_string()]) {
                let project = if project == &"root".to_string() {
                    &program
                } else {
                    program
                        .projects
                        .get(&project.into())
                        .ok_or(eyre!("project '{}' does not exist", project))?
                };

                let to_exec = project
                    .rules
                    .get(&command.clone().into())
                    .ok_or(eyre!("No rules found matching '{}'", &command))?;

                let to_exec = Resolver::get_tasks(&to_exec, &project);

                let executor = Executor::new(&project);

                if !dry_run && !cli.dry_run {
                    executor.execute(&to_exec, &project, None::<&NonePackageProvider>)?;
                }
            }
        }
        Command::Build { dry_run } => {
            cli.command = Some(Command::Run {
                command: "build".to_string(),
                dry_run: *dry_run,
            });
            run(cli)?;
        }
        Command::Test { dry_run } => {
            cli.command = Some(Command::Run {
                command: "test".to_string(),
                dry_run: *dry_run,
            });
            run(cli)?;
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
}

#[derive(Subcommand)]
enum Command {
    Run {
        command: String,
        #[clap(short, long)]
        dry_run: bool,
    },
    Build {
        #[clap(short, long)]
        dry_run: bool,
    },
    Test {
        #[clap(short, long)]
        dry_run: bool,
    },
}
