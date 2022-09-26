use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::{
    exec_order_resolver::{Resolver, Resolver1},
    parser::parser,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    run(cli)?;

    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command.unwrap_or(Command::Run {
        command: "build".to_string(),
    }) {
        Command::Run { command } => {
            let content = read_to_string("PACKAGE.repkg")?;

            let program = parser().parse(content.as_bytes())?;

            let to_exec = program
                .rules
                .get(&command.clone().into())
                .ok_or(eyre!("No rules found matching '{}'", &command))?;

            let to_exec = Resolver1::get_tasks(&to_exec, &program);

            dbg!(to_exec);
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Run { command: String },
}
