use std::{path::Path, process::Command};

use super::{utils::list_files, Config, Res};

pub fn build(config: &Config) -> Res<()> {
    if config.release {
        let mut command = Command::new("gcc");
        for object in list_files(Path::new("build/release/object"))? {
            command.arg(object.to_str().unwrap());
        }

        command.arg("-O3");

        command.arg("-Iinclude");

        #[cfg(target_os = "windows")]
        command.arg(format!("-o build/release/{}.exe", config.name));

        #[cfg(not(target_os = "windows"))]
        command.arg(format!("-o build/release/{}", config.name));

        for arg in &config.cflags {
            command.arg(arg);
        }

        command.arg("-Llib");

        for lib in &config.libs {
            command.arg(format!("-l{}", lib));
        }

        println!(
            "{}",
            String::from_utf8(command.spawn()?.wait_with_output()?.stdout)?
        );
    } else {
        let mut command = Command::new("gcc");
        for object in list_files(Path::new("build/debug/object"))? {
            command.arg(object.to_str().unwrap());
        }
        
        command.arg("-O0");

        command.arg("-Iinclude");

        #[cfg(target_os = "windows")]
        command.arg(format!("-o build/debug/{}.exe", config.name));

        #[cfg(not(target_os = "windows"))]
        command.arg(format!("-o build/debug/{}", config.name));

        for arg in &config.cflags {
            command.arg(arg);
        }

        command.arg("-Llib");

        for lib in &config.libs {
            command.arg(format!("-l{}", lib));
        }

        println!(
            "{}",
            String::from_utf8(command.spawn()?.wait_with_output()?.stdout)?
        );
    }

    Ok(())
}
