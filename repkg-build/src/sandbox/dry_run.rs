use std::{collections::HashMap, path::Path, process};

use crate::exec::{commands, CommandT};

use super::{CmdProvider, FileSystem, IntoFileSystem};

use color_eyre::{eyre::eyre, Result};

pub struct DryRunSandbox {
    fs: DryRunFS,
    commands: HashMap<String, Box<dyn CommandT<DryRunFS>>>,
}

impl<'a> CmdProvider<'a, DryRunFS> for DryRunSandbox {
    fn new() -> Self {
        Self {
            fs: DryRunFS {
                root: File::Directory(Directory {
                    files: Default::default(),
                }),
            },
            commands: commands::commands(),
        }
    }

    fn executable(&self, name: &str) -> Result<process::Command> {
        #[cfg(not(windows))]
        let executable = name;
        #[cfg(windows)]
        let executable = format!("{}.exe", name);

        let fs = &self.fs;
        if fs.exists(&executable) {
            Err(eyre!("Cannot execute in a dry run"))
        } else {
            Err(eyre!("This command does not exist"))
        }
    }

    fn command(&mut self, program: &str, args: &[&str]) -> color_eyre::Result<()> {
        let fs = &mut self.fs;
        self.commands
            .get(&program.to_string())
            .ok_or(eyre!("The command '{}' does not exist", program))?
            .call(fs, args)
    }

    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT<DryRunFS> + 'static) {
        self.commands.insert(program.to_string(), Box::new(cmd));
    }
}

impl<'a> Default for DryRunSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoFileSystem<'a, DryRunFS> for DryRunSandbox {
    fn into_fs(&self) -> &DryRunFS {
        &self.fs
    }

    fn into_fs_mut(&mut self) -> &mut DryRunFS {
        &mut self.fs
    }
}

pub struct DryRunFS {
    root: File,
}

impl FileSystem for DryRunFS {
    fn exists(&self, path: &impl AsRef<std::path::Path>) -> bool {
        let path = path.as_ref().to_path_buf();
        let res = self.traverse_path(path);
        if let Ok(_) = res {
            true
        } else {
            false
        }
    }

    fn copy<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
        &mut self,
        _from: P,
        _to: Q,
    ) -> Result<u64> {
        todo!()
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, _contents: C) -> Result<()> {
        let path = path.as_ref();
        let dir: &mut Directory = if let Some(parent) = path.parent() {
            self.traverse_path_mut(parent)?
        } else {
            &mut self.root
        }
        .dir_mut()?;

        dir.files.insert(
            path.file_name()
                .ok_or(eyre!("No file name"))?
                .to_str()
                .ok_or(eyre!("Failed to convert string"))?
                .to_string(),
            File::File,
        );

        Ok(())
    }
}

impl DryRunFS {
    fn traverse_path(&self, path: impl AsRef<Path>) -> Result<&File> {
        let path = path.as_ref();

        let segments = path.components();

        let mut cur_dir = &self.root;
        for segment in segments {
            let file = cur_dir
                .dir()?
                .files
                .get(
                    segment
                        .as_os_str()
                        .to_str()
                        .ok_or(eyre!("Failed to convert string"))?,
                )
                .ok_or(eyre!("The requested file does not exist"))?;
            match file {
                File::Directory(_) => cur_dir = file,
                File::File => return Ok(file),
            }
        }

        Ok(cur_dir)
    }

    fn traverse_path_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut File> {
        let path = path.as_ref();

        let segments = path.components();

        let mut cur_dir = &mut self.root;
        for segment in segments {
            let file = cur_dir
                .dir_mut()?
                .files
                .get_mut(
                    segment
                        .as_os_str()
                        .to_str()
                        .ok_or(eyre!("Failed to convert string"))?,
                )
                .ok_or(eyre!("The requested file does not exist"))?;
            match file {
                File::Directory(_) => cur_dir = file,
                File::File => return Ok(file),
            }
        }

        Ok(cur_dir)
    }
}

struct Directory {
    files: HashMap<String, File>,
}

enum File {
    Directory(Directory),
    File,
}

impl File {
    pub fn dir(&self) -> Result<&Directory> {
        match self {
            File::Directory(dir) => Ok(dir),
            File::File => Err(eyre!("Not a directory")),
        }
    }

    pub fn dir_mut(&mut self) -> Result<&mut Directory> {
        match self {
            File::Directory(dir) => Ok(dir),
            File::File => Err(eyre!("Not a directory")),
        }
    }
}
