use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::core::AppContext;
use crate::drivers;

pub(crate) mod completions;
pub(crate) mod restart;
pub(crate) mod setup;

pub fn dispatch(matches: &ArgMatches, app: &AppContext) -> Result<()> {
    match matches.subcommand() {
        Some(("serve", sub)) => {
            let (name, driver_m) = sub
                .subcommand()
                .ok_or_else(|| anyhow::anyhow!("serve requires a driver"))?;
            for driver in drivers::drivers() {
                for cmd in driver.commands() {
                    if cmd.command.get_name() == name {
                        return (cmd.handler)(driver_m, app);
                    }
                }
            }
            bail!("unknown driver: {name}")
        }
        Some(("completions", sub)) => completions::run(sub, app),
        Some(("setup", sub)) => setup::run(sub, app),
        Some(("restart", _)) => restart::run(app),
        Some((name, _)) => bail!("unknown command: {name}"),
        None => {
            crate::build_cli().print_help()?;
            println!();
            Ok(())
        }
    }
}
