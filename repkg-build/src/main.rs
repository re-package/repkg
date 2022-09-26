use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec::{Executor, ExecutorT},
    exec_order_resolver::{Resolver, Resolver1},
    parser::parser,
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
    }) {
        Command::Run { command } => {
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

                let to_exec = Resolver1::get_tasks(&to_exec, &project);

                let executor = Executor::new(&project);
                executor.execute(&to_exec, &project)?;
            }
        }
        Command::Build => {
            cli.command = Some(Command::Run {
                command: "build".to_string(),
            });
            run(cli)?;
        }
        Command::Test => {
            cli.command = Some(Command::Run {
                command: "test".to_string(),
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
}

#[derive(Subcommand)]
enum Command {
    Run { command: String },
    Build,
    Test,
}
