use std::{path::Path, process};

use crate::exec::CommandT;

pub mod dry_run;

pub trait SandboxT<'a, F: FileSystem>: IntoFileSystem<'a, F> {
    fn executable(&self, name: &str) -> color_eyre::Result<process::Command>;
    fn command(&self, program: &str, args: &[&str]) -> color_eyre::Result<()>;
    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT + 'static);
    fn new() -> Self;
}

pub trait FileSystem {
    fn exists(&self, path: &impl AsRef<Path>) -> bool;
}

impl<'a, F: FileSystem> IntoFileSystem<'a, F> for F {
    fn into_fs(&'a self) -> &'a F {
        self
    }

    fn into_fs_mut(&'a mut self) -> &'a mut F {
        self
    }
}

pub trait IntoFileSystem<'a, F: FileSystem> {
    fn into_fs(&'a self) -> &'a F;
    fn into_fs_mut(&'a mut self) -> &'a mut F;
}
