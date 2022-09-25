use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use repkg_build::parser::parser;

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Run {
        command: "build".to_string(),
    }) {
        Command::Run { command } => {
            let content = read_to_string("PACKAGE.repkg").unwrap();

            let program = parser().parse(content.as_bytes()).unwrap();

            dbg!(&program);

            let to_exec = program.rules.get(&command.into()).unwrap();

            dbg!(&to_exec);
        }
    }
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
