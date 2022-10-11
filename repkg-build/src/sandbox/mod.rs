use std::{path::Path, process};

use color_eyre::Result;

use crate::exec::CommandT;

pub mod dry_run;
pub mod sandbox;

pub trait CmdProvider<'a, F: FileSystem>: IntoFileSystem<'a, F> {
    fn executable(&self, name: &str) -> color_eyre::Result<process::Command>;
    fn command(&mut self, program: &str, args: &[&str]) -> color_eyre::Result<()>;
    fn reg_cmd(&mut self, program: &str, cmd: impl CommandT<F> + 'static)
    where
        Self: Sized;
    fn new() -> Self;
}

pub trait FileSystem {
    fn exists(&self, path: &impl AsRef<Path>) -> bool;
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64>;
    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, contents: C) -> Result<()>;
}

impl<'a, F: FileSystem> IntoFileSystem<'a, F> for F {
    fn into_fs(&self) -> &F {
        self
    }

    fn into_fs_mut(&mut self) -> &mut F {
        self
    }
}

pub trait IntoFileSystem<'a, F: FileSystem> {
    fn into_fs(&self) -> &F;
    fn into_fs_mut(&mut self) -> &mut F;
}
