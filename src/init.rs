use super::{Config, Errors, Res};
use colored::Colorize;
use std::{
    fs::{create_dir, create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

pub fn init(args: &[String], config: &Config) -> Res<()> {
    let name = Path::new(args.get(1).ok_or(Errors::MissingArg)?);

    create_dir(name)?;
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(name.join("Clargo.toml"))?
        .write_all(
            format!(
                "name = \"{}\"\ncflags = {:?}\nlibs = {:?}\nrelease = {}\nincremental = {}",
                name.to_str().unwrap(),
                config.cflags,
                config.libs,
                config.release,
                config.incremental
            )
            .as_bytes(),
        )?;
    create_dir(name.join("src"))?;
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(name.join("src/main.c"))?
        .write_all(
            String::from(
                r#"
#include <stdio.h>

int main() {
printf("Hello, world!\n");

return 0;
}
"#,
            )
            .as_bytes(),
        )?;
    create_dir(name.join("include"))?;
    create_dir(name.join("lib"))?;
    create_dir_all(name.join("build/incremental"))?;
    create_dir_all(name.join("build/debug/objects"))?;
    create_dir_all(name.join("build/release/objects"))?;
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(name.join(".gitignore"))?
        .write_all(
            String::from("build/incremental/*\nbuild/release/objects/*\nbuild/debug/objects*")
                .as_bytes(),
        )?;

    println!(
        "{} -\n\t./{}",
        "Project created!".green().bold(),
        name.to_str().unwrap().cyan().italic()
    );

    Ok(())
}
