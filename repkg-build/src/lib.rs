pub mod schema;

use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Path must be a directory")]
    #[diagnostic(code(repkg_script::cli::path_must_be_dir))]
    PathMustBeDir,
    #[error("The specified directory does not contain a package file")]
    #[help("Try creating a repkg.toml file in that directory")]
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
}

pub fn run(cli: Cli) -> Result<()> {
    if !cli.path.is_dir() {
        bail!(PathMustBeDir)
    }

    let package_file = cli.path.join("repkg.toml");
    let file_contents = fs::read_to_string(package_file).map_err(|_| PackageFileDoesntExist)?;
    let package: schema::Package = toml::from_str(&file_contents).map_err(InvalidPackageFile)?;

    dbg!(package);

    Ok(())
}
