pub mod incremental;

use super::Res;
use sha2::{Digest, Sha256};
use std::{
    fs::{read_dir, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn read_file(path: &Path) -> Res<Vec<u8>> {
    let mut handle = File::open(path)?;
    let mut buf = Vec::new();

    handle.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn write_file(path: &Path, data: &[u8]) -> Res<()> {
    let mut handle = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    handle.write_all(data)?;

    Ok(())
}

pub fn hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn list_files(dir: &Path) -> Res<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.append(&mut list_files(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
