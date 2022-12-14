#[cfg(unix)]
use std::os::unix;
#[cfg(windows)]
use std::os::windows;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use miette::{bail, Diagnostic, Result, SourceSpan};
use tar::Archive;
use thiserror::Error;

use Error::*;

use crate::artifacts::generate::hash;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Path must be a file: {}", .0.display())]
    #[diagnostic(code(repkg::install::path_must_be_file))]
    PathMustBeFile(PathBuf),
    #[error("Failed to open file: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_open_file))]
    FailedToOpenFile(#[source] std::io::Error, PathBuf),
    #[error("Failed to unpack archive: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_unpack))]
    FailedToUnpack(#[source] std::io::Error, PathBuf),
    #[error("Artifact already installed!")]
    #[diagnostic(code(repkg::install::already_installed))]
    AlreadyInstalled(#[source_code] String, #[label("hash")] SourceSpan),
    #[error("Failed to create dir: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_create_dir))]
    FailedToCreateDir(#[source] std::io::Error, PathBuf),
    #[error("Failed to delete dir: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_delete_dir))]
    FailedToDeleteDir(#[source] std::io::Error, PathBuf),
    #[error("Failed to read dir: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_read_dir))]
    FailedToReadDir(#[source] std::io::Error, PathBuf),
    #[error("Failed to obtain file metadata: {}", .1.display())]
    #[diagnostic(code(repkg::install::failed_to_obtain_file_metadata))]
    FailedToObtainFileMetadata(#[source] std::io::Error, PathBuf),
    #[error("Failed to create symlink from {} to {}", .1.display(), .2.display())]
    #[diagnostic(code(repkg::install::failed_to_copy_file))]
    FailedToCreateSymlink(#[source] std::io::Error, PathBuf, PathBuf),
}

pub struct InstallFlags {
    pub force: bool,
}

pub struct Installer {
    target: PathBuf,
}

impl Installer {
    pub fn new(target: impl AsRef<Path>) -> Self {
        let target = target.as_ref().to_path_buf();

        Self { target }
    }

    pub fn install(&self, artifact: impl AsRef<Path>, flags: &InstallFlags) -> Result<()> {
        let path = artifact.as_ref();
        if path.is_file() {
            let dst = self.extract(path, &flags)?;
            self.link(&dst, "bin")?;
            self.link(&dst, "lib")?;
            self.link(&dst, "include")?;
        }

        Ok(())
    }

    pub fn extract(&self, path: impl AsRef<Path>, flags: &InstallFlags) -> Result<PathBuf> {
        let path = path.as_ref();
        let cache = self.target.join("cache");
        if !path.exists() || !path.is_file() {
            bail!(PathMustBeFile(path.to_path_buf()))
        }

        let hash = format!("{:x}", hash(path)?);
        let dst = cache.join(&hash);
        if dst.exists() {
            if !flags.force {
                let hash_len = hash.len();
                bail!(AlreadyInstalled(hash, (0, hash_len).into()))
            } else {
                fs::remove_dir_all(&dst).map_err(|e| FailedToDeleteDir(e, dst.clone()))?;
            }
        }
        fs::create_dir_all(&dst).map_err(|e| FailedToCreateDir(e, dst.clone()))?;
        let file = File::open(path).map_err(|e| FailedToOpenFile(e, path.to_path_buf()))?;
        let mut archive = Archive::new(file);
        archive
            .unpack(&dst)
            .map_err(|e| FailedToUnpack(e, path.to_path_buf()))?;

        Ok(dst)
    }

    fn link(&self, artifact: impl AsRef<Path>, dir_name: impl AsRef<Path>) -> Result<()> {
        let artifact = artifact.as_ref();
        let dst = artifact.join(&dir_name);
        if dst.exists() {
            let dir = self.target.join(&dir_name);
            if !dir.exists() {
                fs::create_dir_all(&dir).map_err(|e| FailedToCreateDir(e, dir.clone()))?;
            }
            symlink_all_to(dst, dir)?;
        }
        Ok(())
    }
}

fn symlink_all_to(from: PathBuf, to: PathBuf) -> Result<()> {
    for entry in from.read_dir().map_err(|e| FailedToReadDir(e, from))? {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name();
                let new_path = to.join(file_name);
                #[cfg(windows)]
                let metadata = entry
                    .metadata()
                    .map_err(|e| FailedToObtainFileMetadata(e, entry.path()))?;
                #[cfg(windows)]
                if metadata.is_file() {
                    windows::fs::symlink_file(entry.path(), &new_path)
                } else if metadata.is_dir() {
                    windows::fs::symlink_dir(entry.path(), &new_path)
                } else {
                    panic!()
                }
                .map_err(|e| FailedToCreateSymlink(e, entry.path(), new_path))?;
                #[cfg(unix)]
                unix::fs::symlink(entry.path(), &new_path)
                    .map_err(|e| FailedToCreateSymlink(e, entry.path(), new_path))?;
            }
            Err(_) => {}
        }
    }

    Ok(())
}
