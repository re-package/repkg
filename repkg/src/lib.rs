use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::Result;
use repkg_core::install::{InstallFlags, Installer};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Install {
        #[clap(long)]
        file: PathBuf,
        #[clap(short, long)]
        force: bool,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.subcommand {
        Command::Install { file, force } => {
            #[cfg(unix)]
            let dir = "/repkg";
            #[cfg(windows)]
            let dir = "C:\\repkg";
            #[cfg(all(not(windows), not(unix)))]
            bail!(miette!("Unsupported platform!"));

            let installer = Installer::new(&dir);
            let flags = InstallFlags { force };
            installer.install(file, &flags)?;
        }
    }

    Ok(())
}
