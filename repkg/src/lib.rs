use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::Result;
use repkg_core::install::Installer;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Install {
        #[clap(short, long)]
        file: PathBuf,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        Command::Install { file } => {
            #[cfg(unix)]
            let dir = "/repkg";
            #[cfg(windows)]
            let dir = "C:\\repkg";
            #[cfg(all(not(windows), not(unix)))]
            bail!(miette!("Unsupported platform!"));

            let installer = Installer::new(&dir);
            installer.install(file)?;
        }
    }

    Ok(())
}
