use std::{
    path::{Path, PathBuf, StripPrefixError},
    str::pattern::Pattern,
};

use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to convert to a string")]
    #[diagnostic(code(repkg_common::fs_util::StringConversionError))]
    StringConversionError,
    #[error("strip prefix error")]
    #[diagnostic(code(std::path::StripPrefixError))]
    StripPrefixError(#[from] StripPrefixError),
}

pub fn relocate_file(
    path: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf> {
    let parent_dir = from.as_ref().to_str().ok_or(Error::StringConversionError)?;
    let new_path: PathBuf;

    if parent_dir.is_contained_in(
        path.as_ref()
            .canonicalize()
            .map_err(crate::io_error)?
            .to_str()
            .ok_or(Error::StringConversionError)?,
    ) {
        new_path = to
            .as_ref()
            .join(
                path.as_ref()
                    .canonicalize()
                    .map_err(crate::io_error)?
                    .strip_prefix(parent_dir)
                    .map_err(|e| Error::StripPrefixError(e))?,
            )
            .into();
    } else {
        new_path = to.as_ref().join(path.as_ref()).into();
    }

    Ok(new_path)
}
