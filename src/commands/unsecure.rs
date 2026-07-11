use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::ArgMatches;

use crate::commands::CommandDescriptor;
use crate::core::AppContext;

enum Upstream {
    Proxy(String),
    PhpFpm { driver: String, root: PathBuf, socket: String },
}

fn detect_upstream(content: &str, domain: &str) -> Result<Upstream> {
    for line in content.lines() {
        let line = line.trim();
        if let Some(url) = line.strip_prefix("proxy_pass ") {
            let url = url.trim_end_matches(';').trim();
            return Ok(Upstream::Proxy(url.to_string()));
        }
        if let Some(sock) = line.strip_prefix("fastcgi_pass unix:") {
            let socket = sock.trim_end_matches(';').trim().to_string();
            let driver = content
                .lines()
                .find(|l| l.trim().starts_with("# ") && !l.trim().starts_with("# VALEX"))
                .and_then(|l| l.trim().strip_prefix("# "))
                .unwrap_or("unknown")
                .to_string();
            let root = content
                .lines()
                .find(|l| l.trim().starts_with("root "))
                .and_then(|l| {
                    let rest = l.trim().strip_prefix("root ")?.trim_end_matches(';').trim();
                    Some(PathBuf::from(rest))
                })
                .context("could not find root directive in nginx config")?;
            return Ok(Upstream::PhpFpm { driver, root, socket });
        }
    }
    bail!("could not determine config type for {domain}")
}

fn unsecure_run(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let domain = m.get_one::<String>("domain").unwrap();
    let nginx_file = app.nginx_files_path.join(format!("{domain}.conf"));

    if !nginx_file.exists() {
        bail!("no nginx config found for {domain}");
    }

    let cert = app.ssl_path.join(format!("{domain}.pem"));
    let key = app.ssl_path.join(format!("{domain}-key.pem"));

    if cert.exists() {
        fs::remove_file(&cert)?;
    }
    if key.exists() {
        fs::remove_file(&key)?;
    }

    let existing = fs::read_to_string(&nginx_file)
        .context("failed to read existing nginx config")?;

    let upstream = detect_upstream(&existing, domain)?;

    match upstream {
        Upstream::Proxy(url) => {
            let stub = include_str!("../stubs/proxy-nginx.conf")
                .replace("{{VALEX_DOMAIN}}", domain)
                .replace("{{VALEX_PROXY_PASS}}", &url);
            fs::write(&nginx_file, stub)?;
        }
        Upstream::PhpFpm { driver, root, socket } => {
            let stub = include_str!("../stubs/php-fpm-nginx.conf")
                .replace("{{VALEX_DOMAIN}}", domain)
                .replace("{{VALEX_ROOT}}", &root.to_string_lossy())
                .replace("{{DRIVER}}", &driver)
                .replace("{{VALEX_PHP_FPM_SOCKET}}", &socket);
            fs::write(&nginx_file, stub)?;
        }
    }

    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if status.success() {
        println!("✓ HTTPS disabled for {domain}");
    } else {
        println!("Failed to reload nginx. Reload manually.");
    }

    Ok(())
}

pub fn descriptor() -> CommandDescriptor {
    CommandDescriptor {
        name: "unsecure",
        build: || {
            clap::Command::new("unsecure")
                .about("Disable HTTPS for a domain")
                .arg(clap::arg!(<domain> "Domain to unsecure (e.g. myapp.test)"))
        },
        run: unsecure_run,
    }
}
