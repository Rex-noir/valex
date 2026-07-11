pub mod commands;
pub mod core;
pub mod drivers;
pub mod setup;
pub mod util;

/// Build the complete CLI with all core and driver commands.
pub fn build_cli() -> clap::Command {
    let mut cmd = clap::Command::new("valex")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Local PHP development server")
        .subcommand(commands::serve_command());

    for desc in commands::core_commands() {
        cmd = cmd.subcommand((desc.build)());
    }
    cmd
}
