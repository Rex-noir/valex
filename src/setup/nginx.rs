use std::process::Command;

use anyhow::{Result, bail};

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
        let tmp_dir = nginx_state.join("tmp");

        util::sudo_create_dir_all(&tmp_dir.to_string_lossy())?;
        util::sudo_chown(&tmp_dir.to_string_lossy(), Some(app.uid), Some(app.gid))?;

        Self::write_nginx_config(app)?;
        Self::restart_nginx()?;

        Ok(())
    }

    fn load_nginx_config(app: &AppContext) -> Result<String> {
        let nginx_path = app.nginx_files_path.join("*.conf").display().to_string();
        let state_nginx = app.state_dir.to_string_lossy().to_string();

        Ok(include_str!("../stubs/nginx.conf")
            .replace("{{VALEX_USER}}", &app.username)
            .replace("{{VALEX_STATE_DIR}}", &state_nginx)
            .replace("{{VALEX_NGINX_CONFIGS_PATH}}", &nginx_path))
    }

    fn write_nginx_config(app: &AppContext) -> Result<()> {
        let config = Self::load_nginx_config(app)?;

        util::sudo_write("/etc/nginx/nginx.conf", &config)?;

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
}
