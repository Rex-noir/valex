use std::{fs, path::Path, process::Command};

use anyhow::{Context, Ok, Result, bail};

use crate::{
    core::{AppContext, CommandManager},
    util,
};

pub struct Nginx;

impl Nginx {
    pub(crate) fn setup(app: &AppContext) -> Result<()> {
        println!("Setting up nginx");

        let cm = CommandManager::init();
        cm.install_package("nginx")?;

        let nginx_state = app.state_dir.join("nginx");
        fs::create_dir_all(nginx_state.join("tmp"))?;

        Self::write_nginx_config(app)?;
        Self::restart_nginx()?;

        Ok(())
    }

    fn load_nginx_config(app: &AppContext) -> Result<String> {

        let nginx_path = app.nginx_files_path.join("*.conf").display().to_string();
        let state_nginx = app.state_dir.join("nginx").to_string_lossy().to_string();

        Ok(include_str!("../stubs/nginx.conf")
            .replace("{{VALEX_USER}}", &app.username)
            .replace("{{VALEX_STATE_DIR}}", &state_nginx)
            .replace("{{VALEX_NGINX_CONFIGS_PATH}}", &nginx_path))
    }

    fn write_nginx_config(app: &AppContext) -> Result<()> {
        let config = Self::load_nginx_config(app)?;

        util::sudo_write("/etc/nginx/nginx.conf", &config)?;

        Self::configure_selinux()?;

        Ok(())
    }

    fn restart_nginx() -> Result<()> {
        let status = Command::new("sudo")
            .arg("systemctl")
            .arg("restart")
            .arg("nginx")
            .status()?;

        if !status.success() {
            bail!("Nginx service restart failed");
        }

        Ok(())
    }

    fn configure_selinux() -> Result<()> {
        let selinux_exists = Path::new("/sys/fs/selinux").exists();

        if selinux_exists {
            let status = Command::new("sudo")
                .args(["setsebool", "-P", "httpd_read_user_content", "1"])
                .status()
                .context("failed to execute setsebool")?;

            anyhow::ensure!(status.success(), "failed to enable httpd_read_user_content");
        }

        Ok(())
    }
}
