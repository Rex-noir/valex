use std::{env, fs, path::PathBuf, process::Command};

use anyhow::{Context, Result};
use clap::ArgMatches;
use slug::slugify;

use crate::{
    core::AppContext,
    drivers::{Driver, DriverCommand},
};

static NAME: &str = "Laravel";

pub struct Laravel;

fn laravel_handler(m: &ArgMatches, app: &AppContext) -> Result<()> {
    let path: PathBuf = {
        let p = m.get_one::<String>("path").map(String::as_str).unwrap_or(".");
        if p == "." {
            env::current_dir()?
        } else {
            PathBuf::from(p)
        }
    };

    let php_version = m.get_one::<String>("php-version").context("PHP version is required")?;

    let mut domain = m
        .get_one::<String>("domain")
        .cloned()
        .unwrap_or_else(|| {
            let name = path.file_name().unwrap().to_string_lossy();
            slugify(name.as_ref())
        });

    if !domain.ends_with(".test") {
        domain.push_str(".test");
    }

    let php_installation = app.config.php.get(php_version).context(format!(
        "Failed to get php installation config for php {php_version}",
    ))?;

    println!("✓ Using domain: {domain}");
    println!("✓ Using PHP-FPM: {php_version}");

    let public_path = path.join("public");

    let nginx_config = include_str!("../stubs/php-fpm-nginx.conf")
        .replace("{{VALEX_DOMAIN}}", &domain)
        .replace("{{VALEX_ROOT}}", &public_path.to_string_lossy())
        .replace("{{DRIVER}}", NAME)
        .replace("{{VALEX_PHP_FPM_SOCKET}}", &php_installation.fpm_socket_path);

    let nginx_file_path = app.nginx_files_path.join(format!("{domain}.conf"));

    fs::write(&nginx_file_path, nginx_config)?;

    let status = Command::new("sudo")
        .arg("systemctl")
        .args(["restart", "nginx"])
        .status()?;

    if status.success() {
        println!("✓ Nginx created successfully");
    } else {
        println!("Failed to reload nginx, please reload it manually.");
    }

    Ok(())
}

impl Driver for Laravel {
    fn name(&self) -> &'static str {
        NAME
    }

    fn commands(&self) -> Vec<DriverCommand> {
        vec![DriverCommand {
            command: clap::Command::new("laravel")
                .about("Serve a Laravel project")
                .arg(clap::arg!([path] "Project directory").default_value("."))
                .arg(clap::arg!([domain] "Custom domain"))
                .arg(clap::arg!(--"php-version" <VERSION> "PHP version").required(true)),
            handler: laravel_handler,
        }]
    }
}
