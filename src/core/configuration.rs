use std::{
    collections::HashMap,
    fs::{exists, read_to_string, write},
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PhpInstallation {
    pub fpm_config_path: String,
    pub fpm_socket_path: String,
    pub fpm_service_name: String,
    pub fpm_binary_path: Option<String>,
    pub cli_binary_path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub php: HashMap<String, PhpInstallation>,
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.php == other.php
    }
}

impl Configuration {
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if exists(path)? {
            let text = read_to_string(path)?;
            Ok(serde_json5::from_str(&text)?)
        } else {
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let text = serde_json5::to_string(self)?;
        write(path, &text)?;
        Ok(())
    }
}
