use clap::Parser;
use miette::Result;

use repkg_script::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    repkg_script::run(cli)
}
