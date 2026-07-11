use std::io::Write;
use std::process::Command;

use anyhow::{Result, bail};
use clap::ArgMatches;
use which::which;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

const SUDOERS_PATH: &str = "/etc/sudoers.d/valex";

fn elevate_run(_m: &ArgMatches, app: &AppContext) -> Result<()> {
    let systemctl = which("systemctl")
        .map_err(|_| anyhow::anyhow!("systemctl not found in PATH"))?;
    let systemctl = systemctl.to_string_lossy();

    let user = &app.username;
    let lines: Vec<String> = vec![
        format!("{user} ALL=(root) NOPASSWD: {systemctl} restart nginx"),
        format!("{user} ALL=(root) NOPASSWD: {systemctl} restart dnsmasq"),
        format!("{user} ALL=(root) NOPASSWD: {systemctl} restart php*-fpm"),
        format!("{user} ALL=(root) NOPASSWD: {systemctl} is-active *"),
    ];

    let content = lines.join("\n") + "\n";

    // Write via sudo tee so the file gets root ownership
    let mut child = Command::new("sudo")
        .args(["tee", SUDOERS_PATH])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn sudo tee: {e}"))?;

    child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdin for sudo tee"))?
        .write_all(content.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write sudoers content: {e}"))?;

    let output = child
        .wait_with_output()
        .map_err(|e| anyhow::anyhow!("Failed waiting on sudo tee: {e}"))?;

    if !output.status.success() {
        bail!(
            "Failed to write sudoers file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Set correct permissions: sudoers files must be 0440
    let chmod_status = Command::new("sudo")
        .args(["chmod", "0440", SUDOERS_PATH])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to chmod sudoers file: {e}"))?;

    if !chmod_status.success() {
        bail!("Failed to set permissions on {SUDOERS_PATH}");
    }

    println!("✓ Passwordless sudo configured for systemctl commands");
    println!("  You may need to log out and back in for changes to take effect.");

    Ok(())
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "elevate",
        build: || {
            clap::Command::new("elevate")
                .about("Configure passwordless sudo for systemctl commands used by valex")
        },
        run: elevate_run,
    }
}
