use std::{fs, process::Command};

use anyhow::{Context, Result};
use slug::slugify;

use crate::{
    core::AppContext,
    drivers::{Driver, ServeContext},
};

pub struct Laravel;

impl Driver for Laravel {
    fn name(&self) -> &'static str {
        "Laravel"
    }

    fn serves(&self, path: &std::path::Path) -> bool {
        path.join("artisan").exists() && path.join("public").join("index.php").exists()
    }

    fn serve(
        &self,
        ServeContext {
            php_version,
            domain,
            path,
            ..
        }: ServeContext,
        app: &AppContext,
    ) -> Result<()> {
        println!("→ Serving laravel project ...");

        let php_version = php_version.context("PHP version is required")?;

        let config = &app.config;

        let php_installation = config.php.get(&php_version).context(format!(
            "Failed to get php installation config for php {}",
            php_version
        ))?;

        let mut domain = domain.unwrap_or_else(|| {
            let name = path.file_name().unwrap().to_string_lossy();
            slugify(name.as_ref())
        });

        if !domain.ends_with(".test") {
            domain.push_str(".test");
        }

        println!("✓ Using domain: {}", domain);
        println!("✓ Using PHP-FPM: {}", php_version);

        let public_path = path.join("public");

        let nginx_config = include_str!("../stubs/php-fpm-nginx.conf")
            .replace("{{VALEX_DOMAIN}}", &domain)
            .replace("{{VALEX_ROOT}}", &public_path.to_string_lossy())
            .replace("{{DRIVER}}", Self::name(self))
            .replace(
                "{{VALEX_PHP_FPM_SOCKET}}",
                &php_installation.fpm_socket_path,
            );

        let nginx_file_path = app.nginx_files_path.join(format!("{domain}.conf"));

        println!("→ Writing nginx file to {}", nginx_file_path.display());

        fs::write(&nginx_file_path, nginx_config)?;

        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "nginx"])
            .status()?;

        if status.success() {
            println!("Nginx file created successfully!")
        } else {
            println!("Failed to reload nginx, please reload it manually.")
        }

        println!("✓ Nginx created successfully");

        Ok(())
    }
}
