use clap::Parser;
use miette::Result;

use repkg_registry_host::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    repkg_registry_host::run(cli).await
}
