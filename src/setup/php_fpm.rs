use std::process::Command;

use anyhow::{Ok, Result, bail};

use crate::{core::AppContext, util};

pub struct PHPFpm {}

impl PHPFpm {
    pub(crate) fn setup(app: &AppContext) -> Result<()> {
        let fpm_config = include_str!("../stubs/valex-fpm.conf")
            .replace("{{VALEX_USER}}", &app.username)
            .replace("{{VALEX_USERGROUP}}", &app.groupname)
            .replace("{{VALEX_FPM_SOCKET_OWNER}}", &app.username)
            .replace("{{VALEX_FPM_SOCKET_GROUP}}", &app.groupname);

        for installation in app.config.php.values() {
            let config =
                fpm_config.replace("{{VALEX_FPM_SOCKET_PATH}}", &installation.fpm_socket_path);

            util::sudo_write(&installation.fpm_config_path, &config)?;
        }

        // Restart all fpm services
        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "php*-fpm.service"])
            .status()?;

        if !status.success() {
            bail!("Failed to restart fpm services");
        }

        Ok(())
    }
}
