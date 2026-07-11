use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::core::AppContext;
use crate::drivers;

pub(crate) mod completions;
pub(crate) mod elevate;
pub(crate) mod proxy;
pub(crate) mod restart;
pub(crate) mod secure;
pub(crate) mod setup;
pub(crate) mod status;
pub(crate) mod unproxy;
pub(crate) mod unserve;
pub(crate) mod unsecure;

pub struct CommandDescriptor {
    pub name: &'static str,
    pub build: fn() -> clap::Command,
    pub run: fn(&ArgMatches, &AppContext) -> Result<()>,
}

pub(crate) fn core_commands() -> Vec<CommandDescriptor> {
    vec![
        completions::descriptor(),
        elevate::descriptor(),
        proxy::descriptor(),
        secure::descriptor(),
        setup::descriptor(),
        status::descriptor(),
        unproxy::descriptor(),
        unserve::descriptor(),
        unsecure::descriptor(),
        restart::descriptor(),
    ]
}

fn dispatch_serve(sub: &ArgMatches, app: &AppContext) -> Result<()> {
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

pub fn dispatch(matches: &ArgMatches, app: &AppContext) -> Result<()> {
    match matches.subcommand() {
        Some(("serve", sub)) => dispatch_serve(sub, app),
        Some((name, sub)) => {
            for cmd in core_commands() {
                if cmd.name == name {
                    return (cmd.run)(sub, app);
                }
            }
            bail!("unknown command: {name}")
        }
        None => {
            crate::build_cli().print_help()?;
            println!();
            Ok(())
        }
    }
}

pub fn serve_command() -> clap::Command {
    let mut cmd = clap::Command::new("serve")
        .about("Serve a project with a specific driver")
        .subcommand_required(true);
    for driver in drivers::drivers() {
        for dc in driver.commands() {
            cmd = cmd.subcommand(dc.command);
        }
    }
    cmd
}
