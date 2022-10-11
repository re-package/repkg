use std::{
    cell::RefCell,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use flate2::{write::GzEncoder, Compression};
use repkg_common::{repository::Repository, Project};

use color_eyre::{eyre::eyre, Result};

use crate::{
    exec::{CommandT, Executor},
    sandbox::{CmdProvider, FileSystem},
    task_order,
};

// The job of the packager is to run a project,
// then bundle all the ouput files into an archive
// with the package metadata aswell
pub struct Packager<'a, 'b, S: CmdProvider<'a, F>, F: FileSystem> {
    project: &'b Project,
    out_folder: Option<PathBuf>,
    sandbox: Rc<RefCell<S>>,
    _fs: Option<&'a F>,
    repository: Repository,
}

impl<'a, 'b, S: CmdProvider<'a, F>, F: FileSystem> Packager<'a, 'b, S, F> {
    pub fn new(
        project: &'b Project,
        sandbox: Rc<RefCell<S>>,
        path: impl AsRef<Path>,
        repository: Repository,
    ) -> Result<Self> {
        if !path.as_ref().exists() {
            fs::create_dir_all(&path)?;
        }
        sandbox.borrow_mut().reg_cmd(
            "output",
            OutputCommand {
                out_folder: path.as_ref().to_path_buf(),
            },
        );
        Ok(Self {
            project,
            out_folder: None,
            sandbox,
            _fs: None,
            repository,
        })
    }

    /// execute the project and gather outputs into a folder
    pub fn package(&self) -> Result<&Self> {
        let package_rule = "package".into();
        let to_exec = task_order::calc_task_order(&[&package_rule], self.project)?;

        let new_sandbox = self.sandbox.clone();
        let executor = Executor::new(new_sandbox, &self.repository);
        executor.execute(&to_exec, self.project)?;

        Ok(self)
    }

    pub fn compress<O: Write>(&self, mut buf: O) -> Result<()> {
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

impl<F: FileSystem> CommandT<F> for OutputCommand {
    fn call(&self, fs: &mut F, args: &[&str]) -> Result<()> {
        if args.len() < 1 {
            Err(eyre!("No args!"))
        } else if args.len() == 1 {
            let from = PathBuf::from(args[0]);
            let to = self.out_folder.join(from.file_name().unwrap());

            if from.is_file() {
                fs::copy(&from, &to)?;
                fs.copy(&from, &to)?;
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
