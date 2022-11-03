use std::{
    fs::{self, File},
    hash::Hasher,
    io::{prelude::*, BufReader},
    path::Path,
};

use flate2::{write::*, Compression};
use miette::Result;
use seahash::SeaHasher;

use crate::Error::*;

pub fn make_artifact(path: impl AsRef<Path>, output_file: impl AsRef<Path>) -> Result<u64> {
    let path = path.as_ref();
    let output_file = output_file.as_ref();

    if path.is_file() {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let file = File::open(path).map_err(IoError)?;
        let mut buf_reader = BufReader::new(file);

        let mut buf: [u8; 1024] = [0; 1024];
        while buf_reader.has_data_left().map_err(IoError)? {
            buf_reader.read(&mut buf).map_err(IoError)?;
            encoder.write(&buf).map_err(IoError)?;
        }

        fs::write(output_file, encoder.finish().map_err(IoError)?).map_err(IoError)?;
        Ok(hash(output_file)?)
    } else if path.is_dir() {
        let mut builder = tar::Builder::new(Vec::new());
        builder.append_dir_all("", path).map_err(IoError)?;
        let data = builder.into_inner().map_err(IoError)?;
        fs::write(output_file, data).map_err(IoError)?;

        Ok(hash(output_file)?)
    } else {
        todo!()
    }
}

pub fn hash(path: impl AsRef<Path>) -> Result<u64> {
    // let mut hasher = metrohash::MetroHash64::new();
    let mut hasher = SeaHasher::new();
    let file = File::open(path).map_err(IoError)?;
    let mut buf_reader = BufReader::new(file);

    while buf_reader.has_data_left().map_err(IoError)? {
        let mut buf: [u8; 1024] = [0; 1024];
        for i in 0..buf_reader.read(&mut buf).map_err(IoError)? {
            hasher.write_u8(buf[i]);
        }
    }

    Ok(hasher.finish())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn hash() {
        super::hash("Cargo.toml").unwrap();
    }

    #[test]
    fn make_artifact() {
        let file = "src/";
        let hash = super::make_artifact(file, "src.gz").unwrap();

        let new_hash = super::hash("src.gz").unwrap();

        assert_eq!(hash, new_hash);

        fs::remove_file("src.gz").unwrap();
    }
}
