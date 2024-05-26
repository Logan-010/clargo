mod modes;
pub mod utils;

use serde::Deserialize;
use std::{error::Error, fmt::Display, path::Path};
use utils::read_file;

pub type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum Errors {
    MissingArg,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Errors {}

pub enum Mode {
    Help,
    Init,
    Check,
    Build,
    Clean,
    Run,
}

pub struct Settings {
    pub mode: Mode,
    pub args: Vec<String>,
    pub config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub cflags: Vec<String>,
    pub libs: Vec<String>,
    pub release: bool,
    pub incremental: bool,
    pub cc: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::new(),
            cflags: Vec::new(),
            libs: Vec::new(),
            release: false,
            incremental: true,
            cc: String::from("gcc"),
        }
    }
}

impl Config {
    pub fn update_from_config(&mut self) -> Res<()> {
        let config_file = String::from_utf8(read_file(Path::new("Clargo.toml"))?)?;
        let data: Config = toml::from_str(&config_file)?;

        *self = data;

        Ok(())
    }
}

impl Settings {
    pub fn new(mut args: Vec<String>) -> Res<Self> {
        Ok(Self {
            mode: {
                if args.len() >= 2 {
                    match &args[1] as &str {
                        "help" => Mode::Help,
                        "init" => Mode::Init,
                        "check" => Mode::Check,
                        "build" => Mode::Build,
                        "clean" => Mode::Clean,
                        "run" => Mode::Run,
                        _ => {
                            return Err(format!(
                    "Incorrect subcommand \'{}\', options are (help, init, check, run, build, clean)",
                    args[1]
                )
                            .into())
                        }
                    }
                } else {
                    Mode::Help
                }
            },
            args: args.drain(1..).collect(),
            config: Config::default(),
        })
    }

    pub fn help_menu(&self) {
        modes::help::help();
    }

    pub fn init(&self) -> Res<()> {
        modes::init::init(&self.args, &self.config)?;

        Ok(())
    }

    pub fn check(&mut self) -> Res<()> {
        self.config.update_from_config()?;

        if self.args.contains(&String::from("release")) {
            self.config.release = true;
        }

        modes::check::check(&self.config)?;

        Ok(())
    }

    pub fn build(&mut self) -> Res<()> {
        self.check()?;

        modes::build::build(&self.config)?;

        Ok(())
    }

    pub fn clean(&self) -> Res<()> {
        modes::clean::clean()?;

        Ok(())
    }

    pub fn run(&mut self) -> Res<()> {
        self.build()?;

        modes::run::run(&self.config)?;

        Ok(())
    }
}
