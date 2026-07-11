use std::process::Command;

use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

fn unproxy_run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let domain = m.get_one::<String>("domain").unwrap();
    let nginx_file = app.nginx_files_path.join(format!("{domain}.conf"));

    if !nginx_file.exists() {
        bail!("no proxy config found for {domain}");
    }

    println!("→ Removing proxy config for {domain}");
    std::fs::remove_file(&nginx_file)?;

    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if status.success() {
        println!("✓ Removed proxy for {domain} and reloaded nginx");
    } else {
        println!("Config removed. Failed to reload nginx — reload manually.");
    }

    Ok(())
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "unproxy",
        build: || {
            clap::Command::new("unproxy")
                .about("Remove a proxy's nginx config")
                .arg(clap::arg!(<domain> "Domain to remove (e.g. myapp.test)"))
        },
        run: unproxy_run,
    }
}
