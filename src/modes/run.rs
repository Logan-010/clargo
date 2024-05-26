use crate::{Config, Res};
use std::process::Command;

pub fn run(config: &Config) -> Res<()> {
    println!(
        "{}",
        String::from_utf8(
            Command::new(format!(
                "build/{}/{}",
                if config.release { "release" } else { "debug" },
                config.name
            ))
            .spawn()?
            .wait_with_output()?
            .stdout
        )?
    );

    Ok(())
}
