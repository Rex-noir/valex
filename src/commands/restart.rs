use std::process::Command;

use anyhow::{Result, bail};

use crate::core::AppContext;

pub fn run(app_context: &AppContext) -> Result<()> {
    println!("Restarting nginx");
    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if !status.success() {
        bail!("Failed to restart nginx");
    } else {
        println!("Nginx restart successful");
    }

    println!("Restarting dnsmasq");
    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "dnsmasq"])
        .status()?;
    if !status.success() {
        bail!("Failed to restart dnsmasq");
    } else {
        println!("Done");
    }

    let configuration = &app_context.config;

    for (version, installation) in &configuration.php {
        println!("Restarting fpm version : {}", version);
        let fpm_service_name = &installation.fpm_service_name;
        if !fpm_service_name.is_empty() {
            let status = Command::new("sudo")
                .arg("systemctl")
                .args(["restart", fpm_service_name])
                .status()?;

            if !status.success() {
                eprintln!("Failed to restart fpm service. Version : {}", version);
            } else {
                println!("Done")
            }
        }
    }

    Ok(())
}
