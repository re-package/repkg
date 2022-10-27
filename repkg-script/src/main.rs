use std::{
    env,
    fs::{read_to_string, OpenOptions},
};

use clap::{Parser, Subcommand};
use miette::{Diagnostic, IntoDiagnostic, Result};

use repkg_common::{registry::Registry, repository::Repository};
use repkg_script::{
    exec::{ctx_executor::ContextExecutor, tree_walker::TreeWalker, Executor},
    package::Packager,
    parser::project,
    parser_new::parser,
    task_order,
};
use thiserror::Error;

fn main() -> Result<()> {
    // parser_new::parser::Parser::new(&fs::read_to_string(".repkg").into_diagnostic()?).parse()?;

    let walker = TreeWalker::parse(".repkg")?;
    let context = walker.walk()?;

    ContextExecutor::execute(&context, &Registry {})?;

    // let mut cli = Cli::parse();

    // run(&mut cli)?;

    Ok(())
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("No RePKG package file found")]
    #[diagnostic(code(repkg_build::no_package_file))]
    NoPackageFile,
    #[error("Project '{}' does not exist", .0)]
    #[diagnostic(code(repkg_build::project_doesnt_exist))]
    ProjectDoesntExist(String),
}

fn _run(cli: &mut Cli) -> Result<()> {
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

            let content = read_to_string(".repkg").map_err(|_| Error::NoPackageFile)?;

            let mut program = parser::Parser::new(content.as_str()).parse()?;

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
                        .get_mut(project)
                        .ok_or_else(|| Error::ProjectDoesntExist(project.to_string()))?
                };

                dbg!(&project);

                let to_exec = [command];
                let to_exec = task_order::calc_task_order(&to_exec, project)?;

                // TODO: change repkg-repo to something more useful
                let repository =
                    Repository::new(env::current_dir().into_diagnostic()?.join("repkg-repo"))?;

                let executor = Executor::new(&repository);
                executor.execute(&to_exec, project)?;
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
            _run(cli)?;
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
            _run(cli)?;
        }
        Command::Package {
            dry_run,
            no_sandbox,
        } => {
            let _dry_run = cli.dry_run || *dry_run;
            let _no_sandbox = cli.no_sandbox || *no_sandbox;

            let content = read_to_string(".repkg").into_diagnostic()?;

            let mut program = project().parse(content.as_bytes()).into_diagnostic()??;

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
                        .get_mut(project)
                        .ok_or_else(|| Error::ProjectDoesntExist(project.to_string()))?
                };

                // TODO: change repkg-repo to something more useful
                let repository =
                    Repository::new(env::current_dir().into_diagnostic()?.join("repkg-repo"))?;

                let packager = Packager::new(project, "output/", repository)?;
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open("output.tar.gz")
                    .into_diagnostic()?;
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
