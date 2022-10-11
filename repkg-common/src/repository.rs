use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::Result;

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.exists() {
            fs::create_dir_all(&root)?;
        }

        Ok(Self { root })
    }

    /// Search the bin folder for a relative path
    pub fn file(&self, path: impl AsRef<Path>) -> Result<Option<PathBuf>> {
        let path = self.bin_folder()?.join(path);
        if !path.exists() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }
}

impl Repository {
    // Get the bin folder, and make sure it exists
    fn bin_folder(&self) -> Result<PathBuf> {
        let path = self.root.join("bin");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(path)
    }
}
