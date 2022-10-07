use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use repkg_common::Project;

use color_eyre::{eyre::eyre, Result};

use crate::{
    exec::{cmd_provider::CmdProviderT, CommandT, Executor, ExecutorT},
    task_order,
};

// The job of the packager is to run a project,
// then bundle all the ouput files into an archive
// with the package metadata aswell
pub struct Packager<'a, C: CmdProviderT<()>> {
    project: &'a Project,
    out_folder: Option<PathBuf>,
    sandbox: &'a mut Option<C>,
}

impl<'a, C: CmdProviderT<()>> Packager<'a, C> {
    pub fn new(project: &'a Project, sandbox: &'a mut Option<C>) -> Self {
        Self {
            project,
            out_folder: None,
            sandbox,
        }
    }

    /// execute the project and gather outputs into a folder
    pub fn package_to(&mut self, path: &impl AsRef<Path>) -> Result<&mut Self> {
        self.out_folder = Some(path.as_ref().to_path_buf());
        if !path.as_ref().exists() {
            fs::create_dir_all(path)?;
        }

        if let Some(_) = self.sandbox {
            // Do nothing
        } else {
            // Create a new sandbox
            let _ = self.sandbox.insert(C::new());
            self.sandbox.as_mut().unwrap().serve(
                "output".to_string(),
                OutputCommand {
                    out_folder: path.as_ref().to_path_buf(),
                },
            );
        }

        let executor = Executor::new(self.sandbox.as_ref().unwrap());

        let package_rule = "package".into();
        // let to_exec = Resolver::get_tasks(package_rule, self.project);
        let to_exec = task_order::calc_task_order(&[&package_rule], self.project)?;

        executor.execute(&to_exec, self.project)?;

        Ok(self)
    }

    pub fn compress<O: Write>(&mut self, mut buf: O) -> Result<()> {
        if !self.out_folder.is_some() {
            Err(eyre!("Must be packaged first"))
        } else if let Some(out_folder) = &self.out_folder {
            let mut archive = tar::Builder::new(&mut buf);

            for entry in out_folder.read_dir()? {
                let entry = entry?.path();

                if entry.is_dir() {
                    archive.append_dir_all(&entry.file_name().unwrap(), &entry)?;
                } else if entry.is_file() {
                    archive.append_file(&entry.file_name().unwrap(), &mut File::open(&entry)?)?;
                }
            }

            archive.finish()?;
            drop(archive);

            // Compress with GZip
            GzEncoder::new(&mut buf, Compression::best()).finish()?;

            Ok(())
        } else {
            panic!()
        }
    }
}

pub struct OutputCommand {
    out_folder: PathBuf,
}

impl CommandT for OutputCommand {
    fn call(&self, args: &[&str]) -> Result<()> {
        if args.len() < 1 {
            Err(eyre!("No args!"))
        } else if args.len() == 1 {
            let from = PathBuf::from(args[0]);
            let to = self.out_folder.join(from.file_name().unwrap());

            if from.is_file() {
                fs::copy(&from, &to)?;
            } else if from.is_dir() {
                copy_dir::copy_dir(&from, &to)?;
            }

            Ok(())
        } else if args.len() == 2 {
            let from = PathBuf::from(args[0]);
            let to = self.out_folder.join(args[1]);

            if from.is_file() {
                fs::copy(&from, &to)?;
            } else if from.is_dir() {
                copy_dir::copy_dir(&from, &to)?;
            }

            Ok(())
        } else {
            Ok(())
        }
    }
}
