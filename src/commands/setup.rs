use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::core::AppContext;
use crate::setup::{Dns, Nginx, PHPFpm};

pub fn command() -> clap::Command {
    clap::Command::new("setup")
        .about("Run setup tasks")
        .subcommand(clap::Command::new("Dns").about("Set up DNS resolution"))
        .subcommand(clap::Command::new("Nginx").about("Set up nginx"))
        .subcommand(clap::Command::new("PHPFpm").about("Set up PHP-FPM pool configs"))
}

pub fn run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    match m.subcommand() {
        Some(("Dns", _)) => Dns::setup(app),
        Some(("Nginx", _)) => Nginx::setup(app),
        Some(("PHPFpm", _)) => PHPFpm::setup(app),
        Some((name, _)) => bail!("unknown setup task: {name}"),
        None => {
            Dns::setup(app)?;
            Nginx::setup(app)?;
            PHPFpm::setup(app)?;
            Ok(())
        }
    }
}
