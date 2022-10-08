use std::{collections::HashMap, path::PathBuf, process};

use crate::exec::CommandT;

use super::{FileSystem, IntoFileSystem, SandboxT};

use color_eyre::{eyre::eyre, Result};

pub struct DryRunSandbox {
    fs: DryRunFS,
    commands: HashMap<String, Box<dyn CommandT>>,
}

impl<'a> SandboxT<'a, DryRunFS> for DryRunSandbox {
    fn new() -> Self {
        Self {
            fs: DryRunFS {
                root: Directory {
                    files: Default::default(),
                },
            },
            commands: Default::default(),
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

    fn command(&self, program: &str, args: &[&str]) -> color_eyre::Result<()> {
        self.commands
            .get(&program.to_string())
            .ok_or(eyre!("The command '{}' does not exist", program))?
            .call(args)
    }

    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT + 'static) {
        self.commands.insert(program.to_string(), Box::new(cmd));
    }
}

impl<'a> Default for DryRunSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoFileSystem<'a, DryRunFS> for DryRunSandbox {
    fn into_fs(&'a self) -> &'a DryRunFS {
        &self.fs
    }

    fn into_fs_mut(&'a mut self) -> &'a mut DryRunFS {
        &mut self.fs
    }
}

pub struct DryRunFS {
    root: Directory,
}

impl FileSystem for DryRunFS {
    fn exists(&self, path: &impl AsRef<std::path::Path>) -> bool {
        let path = path.as_ref().to_path_buf();
        let res = self.root.files.get(&path);
        if let Some(_) = res {
            true
        } else {
            false
        }
    }
}

struct Directory {
    files: HashMap<PathBuf, File>,
}

struct File;
