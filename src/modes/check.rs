use crate::{
    utils::{incremental::update_hashes, list_files, read_file},
    Config, Res,
};
use std::{fs::remove_file, path::Path, process::Command};

pub fn check(config: &Config) -> Res<()> {
    if !Path::new("Clargo.toml").exists() {
        return Err("Not in Clargo project directory!".into());
    }

    let files = list_files(Path::new("src"))?;

    if config.incremental {
        update_hashes()?;

        for file in &files {
            let filename = file.file_name().unwrap().to_str().unwrap();
            let file_data = read_file(file)?;
            let hash_data =
                read_file(&Path::new("build/incremental").join(format!("{}.hash", filename)))?;

            if file_data != hash_data {
                let mut command = Command::new(config.cc.first().expect("No CC in config file!"));

                for cc_ext in &config.cc[1..] {
                    command.arg(cc_ext);
                }

                command.arg(file.to_str().unwrap());

                for arg in &config.cflags {
                    command.arg(arg);
                }

                command.arg("-Llib");

                for lib in &config.libs {
                    command.arg(format!("-l{}", lib));
                }

                if config.release {
                    command
                        .arg("-o")
                        .arg(format!("build/release/objects/{}.o", filename));
                    command.arg("-O3");

                    command.arg("-Iinclude");
                } else {
                    command
                        .arg("-o")
                        .arg(format!("build/debug/objects/{}.o", filename));
                    command.arg("-O0");

                    command.arg("-Iinclude");
                }

                command.arg("-c");

                println!(
                    "{}",
                    String::from_utf8(command.spawn()?.wait_with_output()?.stdout)?
                );
            }
        }
    } else {
        for file in &files {
            let mut command = Command::new(config.cc.first().expect("No CC in config file!"));

            for cc_ext in &config.cc[1..] {
                command.arg(cc_ext);
            }

            command.arg(file.to_str().unwrap());

            for arg in &config.cflags {
                command.arg(arg);
            }

            command.arg("-Llib");

            for lib in &config.libs {
                command.arg(format!("-l{}", lib));
            }

            if config.release {
                command.arg("-o").arg(format!(
                    "build/release/objects/{}.o",
                    file.file_name().unwrap().to_str().unwrap()
                ));
                command.arg("-O3");
            } else {
                command.arg("-o").arg(format!(
                    "build/debug/objects/{}.o",
                    file.file_name().unwrap().to_str().unwrap()
                ));
                command.arg("-O0");
            }

            command.arg("-c");

            println!(
                "{}",
                String::from_utf8(command.spawn()?.wait_with_output()?.stdout)?
            );
        }
    }

    if config.release {
        for object in list_files(Path::new("build/release/objects"))? {
            let filename = object.file_stem().unwrap().to_str().unwrap();

            let mut expired = true;
            for file in &files {
                if file.file_name().unwrap().to_str().unwrap() == filename {
                    expired = false;
                }
            }

            if expired {
                remove_file(object)?;
            }
        }
    } else {
        for object in list_files(Path::new("build/debug/objects"))? {
            let filename = object.file_stem().unwrap().to_str().unwrap();

            let mut expired = true;
            for file in &files {
                if file.file_name().unwrap().to_str().unwrap() == filename {
                    expired = false;
                }
            }

            if expired {
                remove_file(object)?;
            }
        }
    }

    Ok(())
}
