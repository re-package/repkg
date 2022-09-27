use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec::{
        sandbox::SandBoxCmdProvider, system_cmd_provider::SystemCmdProvider, Executor, ExecutorT,
    },
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
        no_sandbox: false,
    }) {
        Command::Run {
            command,
            dry_run,
            no_sandbox,
        } => {
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

                if !dry_run && !cli.dry_run {
                    if !no_sandbox && !cli.no_sandbox {
                        let sandbox = SandBoxCmdProvider::new();
                        let executor = Executor::new(&NonePackageProvider, &sandbox);
                        executor.execute(&to_exec, &project)?;
                    } else {
                        let sandbox = SystemCmdProvider::new();
                        let executor = Executor::new(&NonePackageProvider, &sandbox);
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
}
