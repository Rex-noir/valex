use std::process::Command;

use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

fn unserve_run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let domain = m.get_one::<String>("domain").unwrap();
    let nginx_file = app.nginx_files_path.join(format!("{domain}.conf"));

    if !nginx_file.exists() {
        bail!("no served config found for {domain}");
    }

    println!("→ Removing nginx config for {domain}");
    std::fs::remove_file(&nginx_file)?;

    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if status.success() {
        println!("✓ Removed {domain} and reloaded nginx");
    } else {
        println!("Config removed. Failed to reload nginx — reload manually.");
    }

    Ok(())
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "unserve",
        build: || {
            clap::Command::new("unserve")
                .about("Remove a served project's nginx config")
                .arg(clap::arg!(<domain> "Domain to remove (e.g. myapp.test)"))
        },
        run: unserve_run,
    }
}
