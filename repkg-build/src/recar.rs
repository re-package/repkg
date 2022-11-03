use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process,
};

use clap::Subcommand;
use dialoguer::Confirm;
use miette::{bail, miette, Result};
use repkg_core::artifacts::generate;

#[derive(Subcommand)]
pub enum CliRecarCommand {
    Hash {
        #[clap(short, long, default_value = "x")]
        format: char,
    },
    /// Check that the hash in the file's name matches it's true hash
    /// If not, fail with an error message
    Check,
    Rename {
        #[clap(short, long)]
        yes: bool,
    },
    Describe,
}

pub fn run(subcommand: CliRecarCommand, file: PathBuf) -> Result<()> {
    if !file.exists() || file.is_dir() {
        bail!(miette!("Path must be a file"))
    }

    match subcommand {
        CliRecarCommand::Hash { format } => {
            let hash = generate::hash(file)?;
            match format {
                'x' => println!("{:x}", hash),
                'X' => println!("{:X}", hash),
                _ => println!("{}", hash),
            }
        }
        CliRecarCommand::Check => {
            check(&file)?;
        }
        CliRecarCommand::Rename { yes } => {
            let new_file_name = format!("{:x}.recar", generate::hash(&file)?);
            let file_name = file.file_name().unwrap().to_str().unwrap();
            if file_name == new_file_name.as_str() {
                return Ok(());
            }
            if !yes {
                if !Confirm::new()
                    .with_prompt(format!("Rename {} to {}?", file_name, new_file_name))
                    .interact()
                    .map_err(|e| miette!("An error occurred when retrieving user input: {}", e))?
                {
                    process::exit(-1);
                }
            }
            let new_file = if file.parent().is_some() {
                file.parent().unwrap().join(new_file_name)
            } else {
                PathBuf::from(new_file_name)
            };
            fs::rename(&file, &new_file)
                .map_err(|e| miette!("An error occurred renaming this file: {}", e))?;
        }
        CliRecarCommand::Describe => {
            let hash = generate::hash(&file)?;
            let size = File::open(&file).unwrap().metadata().unwrap().len();
            let healthy = check(&file).is_ok();
            println!("RECAR Archive:");
            println!("path: {}", file.display());
            println!("hash: {:x}", hash);
            println!("size: {}", pretty_bytes::converter::convert(size as f64));
            println!("healthy: {}", healthy)
        }
    }
    Ok(())
}

fn check(file: impl AsRef<Path>) -> Result<()> {
    let file = file.as_ref();
    let file_name = PathBuf::from(file.file_name().unwrap()).with_extension("");
    let hash = file_name.to_str().unwrap();
    let true_hash = format!("{:x}", generate::hash(&file)?);
    let true_hash = true_hash.as_str();
    if hash != true_hash {
        bail!(miette!(
            "Hashes don't match!\nclaimed hash: {}\ntrue hash: {}",
            hash,
            true_hash,
        ))
    }
    Ok(())
}
