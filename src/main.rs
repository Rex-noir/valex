use anyhow::Result;
use valex::{
    build_cli, commands,
    core::{AppContext, SystemUserProvider},
};

fn main() -> Result<()> {
    let app = AppContext::build(&SystemUserProvider::new())?;
    let matches = build_cli().get_matches();

    commands::dispatch(&matches, &app)
}
