use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use repkg_common::{repository::Repository, Project};

use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use crate::{exec::Executor, io_error, task_order};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("No package output destination")]
    #[diagnostic(code(repkg_build::package::NoOutputDest))]
    NoOutputDest,
    #[error("Must be packaged first")]
    NotPackaged,
}

// The job of the packager is to run a project,
// then bundle all the ouput files into an archive
// with the package metadata aswell
pub struct Packager<'a> {
    project: &'a Project,
    out_folder: Option<PathBuf>,
    repository: Repository,
}

impl<'a> Packager<'a> {
    pub fn new(
        project: &'a Project,
        path: impl AsRef<Path>,
        repository: Repository,
    ) -> Result<Self> {
        if !path.as_ref().exists() {
            fs::create_dir_all(&path).map_err(crate::io_error)?;
        }
        Ok(Self {
            project,
            out_folder: None,
            repository,
        })
    }

    /// execute the project and gather outputs into a folder
    pub fn package(&self) -> Result<&Self> {
        let to_exec = task_order::calc_task_order(&["package"], self.project)?;

        let mut executor = Executor::new(&self.repository);
        executor.reg_cmd(
            "output",
            output::OutputCommand::new(
                self.out_folder.as_ref().ok_or(Error::NoOutputDest)?.clone(),
            ),
        );
        executor.execute(&to_exec, self.project)?;

        Ok(self)
    }

    pub fn compress<O: Write>(&self, mut buf: O) -> Result<()> {
        if self.out_folder.is_none() {
            bail!(Error::NotPackaged)
        } else if let Some(out_folder) = &self.out_folder {
            let mut archive = tar::Builder::new(&mut buf);

            for entry in out_folder.read_dir().map_err(crate::io_error)? {
                let entry = entry.map_err(crate::io_error)?.path();

                if entry.is_dir() {
                    archive
                        .append_dir_all(entry.file_name().unwrap(), &entry)
                        .map_err(crate::io_error)?;
                } else if entry.is_file() {
                    archive
                        .append_file(
                            entry.file_name().unwrap(),
                            &mut File::open(&entry).map_err(crate::io_error)?,
                        )
                        .map_err(crate::io_error)?;
                }
            }

            archive.finish().map_err(io_error)?;
            drop(archive);

            // Compress with GZip
            GzEncoder::new(&mut buf, Compression::best())
                .finish()
                .map_err(io_error)?;

            Ok(())
        } else {
            panic!()
        }
    }
}

mod output {
    use std::{fs, path::PathBuf};

    use miette::{bail, Diagnostic, Result};
    use thiserror::Error;

    use crate::{exec::CommandT, io_error};

    #[derive(Error, Diagnostic, Debug)]
    pub enum Error {
        #[error("No args!")]
        NoArgs,
    }

    pub struct OutputCommand {
        out_folder: PathBuf,
    }

    impl OutputCommand {
        pub(crate) fn new(out_folder: PathBuf) -> Self {
            Self { out_folder }
        }
    }

    impl CommandT for OutputCommand {
        fn call(&self, args: &[&str]) -> Result<()> {
            if args.is_empty() {
                bail!(Error::NoArgs)
            } else if args.len() == 1 {
                let from = PathBuf::from(args[0]);
                let to = self.out_folder.join(from.file_name().unwrap());

                if from.is_file() {
                    fs::copy(&from, &to).map_err(io_error)?;
                } else if from.is_dir() {
                    copy_dir::copy_dir(&from, &to).map_err(io_error)?;
                }

                Ok(())
            } else if args.len() == 2 {
                let from = PathBuf::from(args[0]);
                let to = self.out_folder.join(args[1]);

                if from.is_file() {
                    fs::copy(&from, &to).map_err(io_error)?;
                } else if from.is_dir() {
                    copy_dir::copy_dir(&from, &to).map_err(io_error)?;
                }

                Ok(())
            } else {
                Ok(())
            }
        }
    }
}
