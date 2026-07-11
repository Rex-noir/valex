use std::process::Command;

use anyhow::{Result, bail};

use crate::core::AppContext;

pub fn command() -> clap::Command {
    clap::Command::new("restart").about("Restart nginx, dnsmasq, and PHP-FPM services")
}

pub fn run(ctx: &AppContext) -> Result<()> {
    println!("Restarting services:");

    restart("nginx")?;
    restart("dnsmasq")?;

    for (version, installation) in &ctx.config.php {
        let name = &installation.fpm_service_name;
        if !name.is_empty() {
            restart_php(version, name);
        }
    }

    Ok(())
}

fn restart(service: &str) -> Result<()> {
    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", service])
        .status()?;

    if !status.success() {
        bail!("{service}: restart failed");
    }

    println!("  {service}: OK");
    Ok(())
}

fn restart_php(version: &str, name: &str) {
    let label = format!("php{version}");
    let result = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", name])
        .output();

    match result {
        Ok(out) if out.status.success() => println!("  {label}: OK"),
        Ok(out) => {
            println!("  {label}: FAILED");
            eprintln!("  {}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => {
            println!("  {label}: ERROR");
            eprintln!("{e}");
        }
    }
}
