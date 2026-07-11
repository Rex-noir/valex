use std::io;

use anyhow::{Result, bail};
use clap::ArgMatches;
use clap_complete::Shell;

use crate::core::AppContext;

pub fn command() -> clap::Command {
    clap::Command::new("completions")
        .about("Generate shell completion scripts")
        .arg(
            clap::arg!(<shell> "Shell to generate completions for")
                .value_parser(["bash", "elvish", "fish", "powershell", "zsh"]),
        )
}

pub fn run(m: &ArgMatches, _app: &AppContext) -> Result<()> {
    let shell = m.get_one::<String>("shell").unwrap();
    let mut cmd = crate::build_cli();
    match shell.as_str() {
        "bash" => clap_complete::generate(Shell::Bash, &mut cmd, "valex", &mut io::stdout()),
        "elvish" => clap_complete::generate(Shell::Elvish, &mut cmd, "valex", &mut io::stdout()),
        "fish" => clap_complete::generate(Shell::Fish, &mut cmd, "valex", &mut io::stdout()),
        "powershell" => clap_complete::generate(Shell::PowerShell, &mut cmd, "valex", &mut io::stdout()),
        "zsh" => clap_complete::generate(Shell::Zsh, &mut cmd, "valex", &mut io::stdout()),
        _ => bail!("unsupported shell: {shell}"),
    }
    Ok(())
}
