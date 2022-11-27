use clap::{Parser, Subcommand};

use crate::cargo_toml::CargoToml;
use crate::version::Version;

#[derive(Parser)]
#[command(author, version, about, long_about = Some("Cargo plugin to tag crate version"))]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

impl Cli {
    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        if let Some(cmd) = cli.command {
            cmd.exec();
            return Ok(());
        }

        Ok(())
    }
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// Print current package version
    Current,
    /// Bumps crate's minor version
    Minor,
    /// Bumps crate's major version
    Major,
    /// Bumps crate's patch version
    Patch,
}

impl Command {
    pub fn exec(&self) {
        match *self {
            Command::Current => {
                let cargo_toml = CargoToml::open().unwrap();

                println!("{}", cargo_toml.package.version);
            }
            Command::Minor | Command::Major | Command::Patch => {
                let cargo_toml = CargoToml::open().unwrap();
                let mut version = Version::from(&cargo_toml.package.version);

                match self {
                    Command::Major => version.bump_major(),
                    Command::Minor => version.bump_minor(),
                    Command::Patch => version.bump_patch(),
                    _ => unreachable!(),
                };

                cargo_toml
                    .write_version(version.into())
                    .expect("Failed to write version");

                cargo_toml
                    .run_cargo_check()
                    .expect("Failed to run `cargo check`");
            }
        }
    }
}
