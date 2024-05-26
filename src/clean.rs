use super::{utils::list_files, Res};
use std::{fs::remove_file, path::Path};

pub fn clean() -> Res<()> {
    for file in list_files(Path::new("build"))? {
        remove_file(file)?;
    }

    println!("Build directory cleaned!");

    Ok(())
}
