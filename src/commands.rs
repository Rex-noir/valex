use clap::Subcommand;

use anyhow::Result;

use crate::commands::setup::SetupArgs;
use crate::core::AppContext;

mod install;
mod restart;
mod serve;
mod setup;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve(serve::ServeArgs),
    Install,
    Setup(SetupArgs),
    Restart,
}

impl Commands {
    pub fn run(self, app: &AppContext) -> Result<()> {
        match self {
            Commands::Serve(args) => serve::run(args, app),
            Commands::Install => install::run(app),
            Commands::Setup(args) => setup::run(args, app),
            Commands::Restart => restart::run(app),
        }
    }
}
