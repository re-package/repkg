pub mod schema;

use std::{fs, path::PathBuf, process::Command};

use clap::Parser;
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
        if file.to_str().unwrap().contains("..") {
            bail!(miette!("Paths may not contain '..'"))
        }
        let path = cli.path.join(&file);
        if !path.exists() {
            bail!(miette!("Specified file does not exist: {}", path.display()))
        }
        fs::copy(&path, tmp_dir.join(&file))
            .map_err(|e| miette!("Failed to copy file '{}': {}", file.display(), e))?;
    }

    let hash = generate::make_artifact(&tmp_dir, "repkg-tmp.recar")?;
    let file_name = format!("{:x}.recar", hash);
    fs::copy("repkg-tmp.recar", &file_name)
        .map_err(|e| miette!("Failed to rename repkg-tmp.recar to {}: {}", &file_name, e))?;

    Ok(())
}
