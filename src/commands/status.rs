use std::process::Command;

use anyhow::Result;
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

fn status_run(_m: &ArgMatches, ctx: &AppContext) -> Result<()> {
    println!("Service status:");

    check("nginx");
    check("dnsmasq");

    for (version, installation) in &ctx.config.php {
        let name = &installation.fpm_service_name;
        if !name.is_empty() {
            check_fpm(version, name);
        }
    }

    Ok(())
}

fn check(service: &str) {
    let output = Command::new("systemctl")
        .args(["is-active", service])
        .output();

    match output {
        Ok(out) => {
            let state = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("  {service:12} {state}");
        }
        Err(e) => {
            eprintln!("  {service:12} ERROR: {e}");
        }
    }
}

fn check_fpm(version: &str, name: &str) {
    let label = format!("php{version}");
    let output = Command::new("systemctl")
        .args(["is-active", name])
        .output();

    match output {
        Ok(out) => {
            let state = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("  {label:12} {state}");
        }
        Err(e) => {
            eprintln!("  {label:12} ERROR: {e}");
        }
    }
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "status",
        build: || clap::Command::new("status").about("Show status of nginx, dnsmasq, and PHP-FPM services"),
        run: status_run,
    }
}
