use clap::Parser;
use miette::Result;
use repkg::Cli;

fn main() -> Result<()> {
    repkg::run(Cli::parse())
}
