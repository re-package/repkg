use clap::Parser;
use miette::Result;

use repkg_build::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    repkg_build::run(cli)
}
