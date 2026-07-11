use std::process::Command;

use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

fn proxy_run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let domain = m.get_one::<String>("domain").unwrap();
    let proxy_pass = m.get_one::<String>("proxy-pass").unwrap();

    let nginx_config = include_str!("../stubs/proxy-nginx.conf")
        .replace("{{VALEX_DOMAIN}}", domain)
        .replace("{{VALEX_PROXY_PASS}}", proxy_pass);

    let nginx_file_path = app.nginx_files_path.join(format!("{domain}.conf"));

    if nginx_file_path.exists() {
        bail!("{domain} already has an nginx config at {}", nginx_file_path.display());
    }

    println!("→ Writing proxy config to {}", nginx_file_path.display());
    std::fs::write(&nginx_file_path, nginx_config)?;

    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if status.success() {
        println!("✓ Proxy set up for {domain} → {proxy_pass}");
    } else {
        println!("Failed to reload nginx. Reload manually for changes to take effect.");
    }

    Ok(())
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "proxy",
        build: || {
            clap::Command::new("proxy")
                .about("Set up an nginx reverse proxy for a domain")
                .arg(clap::arg!(<domain> "Domain to serve (e.g. myapp.test)"))
                .arg(
                    clap::Arg::new("proxy-pass")
                        .required(true)
                        .help("Upstream URL (e.g. http://localhost:3000)"),
                )
        },
        run: proxy_run,
    }
}
