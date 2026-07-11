pub mod commands;
pub mod core;
pub mod drivers;
pub mod setup;
pub mod util;

/// Build the complete CLI with all core and driver commands.
pub fn build_cli() -> clap::Command {
    let serve = {
        let mut cmd = clap::Command::new("serve")
            .about("Serve a project with a specific driver")
            .subcommand_required(true);
        for driver in drivers::drivers() {
            for dc in driver.commands() {
                cmd = cmd.subcommand(dc.command);
            }
        }
        cmd
    };

    clap::Command::new("valex")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Local PHP development server")
        .subcommand(serve)
        .subcommand(commands::completions::command())
        .subcommand(commands::setup::command())
        .subcommand(commands::restart::command())
}
