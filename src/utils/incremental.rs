use super::{hash_data, list_files, read_file, write_file};
use crate::Res;
use std::fs::remove_file;
use std::path::Path;

pub fn update_hashes() -> Res<()> {
    let files = list_files(Path::new("src"))?;
    let hashes = list_files(Path::new("build/incremental"))?;
    for file in &files {
        let filename = file.file_name().unwrap().to_str().unwrap();
        let mut hash_exists = false;
        for hash in &hashes {
            if format!("{}.hash", filename) == *hash.file_name().unwrap().to_str().unwrap() {
                hash_exists = true;
            }
        }

        if !hash_exists {
            write_file(
                Path::new(&format!("build/incremental/{}.hash", filename)),
                &hash_data(&read_file(file)?),
            )?;
        }
    }

    for hash in hashes {
        let mut expired = true;
        let filename = hash.file_stem().unwrap().to_str().unwrap();
        for file in &files {
            if file.file_name().unwrap().to_str().unwrap() == filename {
                expired = false;
            }
        }

        if expired {
            remove_file(hash)?;
        }
    }
    Ok(())
}
