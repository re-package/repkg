use std::{
    path::{Path, PathBuf},
    str::pattern::Pattern,
};

use color_eyre::{eyre::eyre, Result};

pub fn relocate_file(
    path: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf> {
    let parent_dir = from
        .as_ref()
        .to_str()
        .ok_or(eyre!("Failed to convert to a string"))?;
    let new_path: PathBuf;

    if parent_dir.is_contained_in(
        path.as_ref()
            .canonicalize()?
            .to_str()
            .ok_or(eyre!("Failed to convert to a string"))?,
    ) {
        new_path = to
            .as_ref()
            .join(path.as_ref().canonicalize()?.strip_prefix(parent_dir)?)
            .into();
    } else {
        new_path = to.as_ref().join(path.as_ref()).into();
    }

    Ok(new_path)
}
