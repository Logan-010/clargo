use crate::{utils::list_files, Config, Res};
use std::{path::Path, process::Command};

pub fn build(config: &Config) -> Res<()> {
    let output_dir = if config.release {
        Path::new("build/release")
    } else {
        Path::new("build/debug")
    };

    let mut command = Command::new(config.cc.first().expect("No CC in config file!"));

    for cc_ext in &config.cc[1..] {
        command.arg(cc_ext);
    }

    let objects_path = if config.release {
        Path::new("build/release/objects")
    } else {
        Path::new("build/debug/objects")
    };

    for object in list_files(objects_path)? {
        command.arg(object.to_str().unwrap());
    }

    if config.release {
        command.arg("-O3");
    } else {
        command.arg("-O0");
    }

    command.arg("-Iinclude");

    let output_file = if cfg!(target_os = "windows") {
        format!("{}/{}.exe", output_dir.display(), config.name)
    } else {
        format!("{}/{}", output_dir.display(), config.name)
    };

    command.arg("-o");
    command.arg(output_file);

    for arg in &config.cflags {
        command.arg(arg);
    }

    command.arg("-Llib");

    for lib in &config.libs {
        command.arg(format!("-l{}", lib));
    }

    let output = command.output()?;

    println!("{}", String::from_utf8(output.stdout)?);

    Ok(())
}

