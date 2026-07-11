use std::process::Command;

use anyhow::{Result, bail};
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

fn proxy_run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let domain = m.get_one::<String>("domain").unwrap();
    let proxy_pass = m.get_one::<String>("proxy-pass").unwrap();
    let https = m.get_flag("https");

    let nginx_file_path = app.nginx_files_path.join(format!("{domain}.conf"));

    if nginx_file_path.exists() {
        bail!("{domain} already has an nginx config at {}", nginx_file_path.display());
    }

    let nginx_config = if https {
        let cert = app.ssl_path.join(format!("{domain}.pem"));
        let key = app.ssl_path.join(format!("{domain}-key.pem"));

        if !cert.exists() || !key.exists() {
            bail!("Certificate not found for {domain}. Run `valex secure {domain}` first.");
        }

        include_str!("../stubs/proxy-nginx-ssl.conf")
            .replace("{{VALEX_DOMAIN}}", domain)
            .replace("{{VALEX_SSL_CERT}}", &cert.to_string_lossy())
            .replace("{{VALEX_SSL_KEY}}", &key.to_string_lossy())
            .replace("{{VALEX_PROXY_PASS}}", proxy_pass)
    } else {
        include_str!("../stubs/proxy-nginx.conf")
            .replace("{{VALEX_DOMAIN}}", domain)
            .replace("{{VALEX_PROXY_PASS}}", proxy_pass)
    };

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
                .arg(clap::arg!(--https "Enable HTTPS (run `secure` first to generate certs)"))
        },
        run: proxy_run,
    }
}
