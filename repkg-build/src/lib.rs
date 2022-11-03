pub mod schema;

use std::{fs, path::PathBuf, process::Command};

use clap::{Parser, Subcommand};
use miette::{bail, miette, Diagnostic, Result};
use repkg_core::artifacts::generate;
use thiserror::Error;

use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Path must be a directory")]
    #[diagnostic(code(repkg_script::cli::path_must_be_dir))]
    PathMustBeDir,
    #[error("The specified directory does not contain a package file")]
    #[diagnostic(help("Try creating a repkg.toml file in that directory"))]
    #[diagnostic(code(repkg_script::cli::package_file_doesnt_exist))]
    PackageFileDoesntExist,
    #[error("Invalid package file")]
    #[diagnostic(code(repkg_script::invalid_package_file))]
    InvalidPackageFile(#[from] toml::de::Error),
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, default_value = ".")]
    pub path: PathBuf,
    #[clap(subcommand)]
    pub subcommand: Option<CliCommand>,
}

#[derive(Subcommand)]
pub enum CliCommand {
    #[cfg(feature = "recar")]
    Recar {
        #[clap(subcommand)]
        subcommand: CliRecarCommand,
        file: PathBuf,
    },
}

#[derive(Subcommand)]
#[cfg(feature = "recar")]
pub enum CliRecarCommand {
    Hash {
        #[clap(short, long, default_value = "x")]
        format: char,
    },
    /// Check that the hash in the file's name matches it's true hash
    /// If not, fail with an error message
    Check,
}

pub fn run(cli: Cli) -> Result<()> {
    if cli.subcommand.is_some() {
        #[cfg(feature = "recar")]
        match cli.subcommand.unwrap() {
            #[cfg(feature = "recar")]
            CliCommand::Recar { subcommand, file } => {
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
                    }
                }
            }
        }
        Ok(())
    } else {
        if !cli.path.is_dir() {
            bail!(PathMustBeDir)
        }

        let package_file = cli.path.join("repkg.toml");
        let file_contents = fs::read_to_string(package_file).map_err(|_| PackageFileDoesntExist)?;
        let package: schema::Package =
            toml::from_str(&file_contents).map_err(InvalidPackageFile)?;

        for _dep in package.build.dependencies {
            // TODO: Get dependency
        }

        let mut command = Command::new(
            package
                .build
                .command
                .get(0)
                .ok_or_else(|| miette!("build.command must have at least one argument"))?,
        );
        command.args(&package.build.command[1..]);
        let status = command
            .status()
            .map_err(|_| miette!("Failed to spawn command"))?;

        if !status.success() {
            bail!(miette!("Build command failed!"))
        }

        let tmp_dir = cli.path.join("repkg-tmp");
        if tmp_dir.exists() {
            fs::remove_dir_all(&tmp_dir).map_err(|_| miette!("Failed to remove tmp directory"))?;
        }
        fs::create_dir(&tmp_dir)
            .map_err(|_| miette!("Failed to create tmp directory at '{}'", tmp_dir.display()))?;
        for file in package.build.files {
            for file in glob::glob(
                file.to_str()
                    .ok_or_else(|| miette!("Failed to convert path to string"))?,
            )
            .map_err(|e| miette!("Failed to parse glob path: {}", e))?
            {
                let file = file.map_err(|e| miette!("Glob error: {}", e))?;
                if file.to_str().unwrap().contains("..") {
                    bail!(miette!("Paths may not contain '..'"))
                }
                let path = cli.path.join(&file);
                if !path.exists() {
                    bail!(miette!("Specified file does not exist: {}", path.display()))
                }
                let parent = file.parent();
                if let Some(parent) = parent {
                    let parent = tmp_dir.join(parent); // TODO: better error
                    fs::create_dir_all(&parent).map_err(|e| {
                        miette!("Failed to create directory {}: {}", parent.display(), e)
                    })?;
                }

                if file.is_file() {
                    fs::copy(&path, tmp_dir.join(&file))
                        .map_err(|e| miette!("Failed to copy file '{}': {}", file.display(), e,))?;
                } else if file.is_dir() {
                    copy_dir::copy_dir(&path, tmp_dir.join(&file))
                        .map_err(|e| miette!("Failed to copy dir '{}': {}", file.display(), e))?;
                }
            }
        }

        let hash = generate::make_artifact(&tmp_dir, "repkg-tmp.recar")?;
        let file_name = format!("{:x}.recar", hash);
        fs::rename("repkg-tmp.recar", &file_name)
            .map_err(|e| miette!("Failed to rename repkg-tmp.recar to {}: {}", &file_name, e))?;

        Ok(())
    }
}
