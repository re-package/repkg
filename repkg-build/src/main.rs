use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

use repkg_build::parser::parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Run {
        command: "build".to_string(),
    }) {
        Command::Run { command } => {
            let content = read_to_string("PACKAGE.repkg")?;

            let program = parser().parse(content.as_bytes())?;

            dbg!(&program);

            let _to_exec = program
                .rules
                .get(&command.into())
                .ok_or(eyre!("No rules found"))?;
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
