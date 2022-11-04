#![feature(adt_const_params)]
#![allow(incomplete_features)]
pub mod schema;

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, Subcommand};
use miette::{bail, miette, Diagnostic, Result};
use repkg_core::artifacts::generate;
use schema::CopyTarget;
use thiserror::Error;

use Error::*;

#[cfg(feature = "recar")]
pub mod recar;

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
    #[error("An error occurred copying a directory")]
    #[diagnostic(code(repkg_script::copy_dir_error))]
    CopyDirError(#[related] Vec<Error>),
    #[error(transparent)]
    #[diagnostic(code(repkg_script::io_error))]
    IoError(#[from] io::Error),
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
        subcommand: recar::CliRecarCommand,
        file: PathBuf,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    if cli.subcommand.is_some() {
        #[cfg(feature = "recar")]
        match cli.subcommand.unwrap() {
            #[cfg(feature = "recar")]
            CliCommand::Recar { subcommand, file } => {
                recar::run(subcommand, file)?;
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

        if let Some(bin) = package.bin {
            for cp in bin {
                run_copy_target(&tmp_dir, cp)?
            }
        }
        if let Some(include) = package.include {
            for cp in include {
                run_copy_target(&tmp_dir, cp)?
            }
        }
        if let Some(lib) = package.lib {
            for cp in lib {
                run_copy_target(&tmp_dir, cp)?
            }
        }

        // for file in package.build.files {
        //     for file in glob::glob(
        //         file.to_str()
        //             .ok_or_else(|| miette!("Failed to convert path to string"))?,
        //     )
        //     .map_err(|e| miette!("Failed to parse glob path: {}", e))?
        //     {
        //         let file = file.map_err(|e| miette!("Glob error: {}", e))?;
        //         if file.to_str().unwrap().contains("..") {
        //             bail!(miette!("Paths may not contain '..'"))
        //         }
        //         let path = cli.path.join(&file);
        //         if !path.exists() {
        //             bail!(miette!("Specified file does not exist: {}", path.display()))
        //         }
        //         let parent = file.parent();
        //         if let Some(parent) = parent {
        //             let parent = tmp_dir.join(parent); // TODO: better error
        //             fs::create_dir_all(&parent).map_err(|e| {
        //                 miette!("Failed to create directory {}: {}", parent.display(), e)
        //             })?;
        //         }

        //         if file.is_file() {
        //             fs::copy(&path, tmp_dir.join(&file))
        //                 .map_err(|e| miette!("Failed to copy file '{}': {}", file.display(), e,))?;
        //         } else if file.is_dir() {
        //             copy_dir::copy_dir(&path, tmp_dir.join(&file))
        //                 .map_err(|e| miette!("Failed to copy dir '{}': {}", file.display(), e))?;
        //         }
        //     }
        // }

        let hash = generate::make_artifact(&tmp_dir, "repkg-tmp.recar")?;
        let file_name = format!("{:x}.recar", hash);
        fs::rename("repkg-tmp.recar", &file_name)
            .map_err(|e| miette!("Failed to rename repkg-tmp.recar to {}: {}", &file_name, e))?;

        Ok(())
    }
}

fn run_copy_target<const PREFIX: &'static str>(
    output: impl AsRef<Path>,
    cp: CopyTarget<PREFIX>,
) -> Result<()> {
    let mut output = output.as_ref().to_path_buf().join(PREFIX);
    if let Some(to) = cp.to {
        output = output.join(to);
    }
    if !output.exists() {
        fs::create_dir_all(&output).map_err(IoError)?;
    }
    if cp.path.to_str().unwrap().contains("*") {
        for path in
            glob::glob(cp.path.to_str().unwrap()).map_err(|e| miette!("Glob error: {}", e))?
        {
            let path = path.map_err(|e| miette!("Glob error: {}", e))?;
            dbg!(&path);
            let output = output.join(path.file_name().unwrap());
            if path.is_file() {
                fs::copy(&path, &output).map_err(IoError)?;
            }
        }
    } else {
        let output = output.join(cp.path.file_name().unwrap());
        if cp.path.is_file() {
            fs::copy(&cp.path, &output).map_err(IoError)?;
        } else if cp.path.is_dir() {
            let errs = copy_dir::copy_dir(&cp.path, &output).map_err(IoError)?;
            if !errs.is_empty() {
                let mut errors = vec![];
                for err in errs {
                    errors.push(IoError(err));
                }
                bail!(CopyDirError(errors))
            }
        }
    }
    Ok(())
}
