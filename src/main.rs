use std::env::args;

use clargo::{Mode, Res, Settings};

fn main() -> Res<()> {
    let args: Vec<String> = args().collect();
    let mut settings = Settings::new(args)?;

    match settings.mode {
        Mode::Help => settings.help_menu(),
        Mode::Init => settings.init()?,
        Mode::Check => settings.check()?,
        Mode::Build => settings.build()?,
        Mode::Clean => settings.clean()?,
        Mode::Run => settings.run()?,
    }

    Ok(())
}
