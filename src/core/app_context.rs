use crate::core::Configuration;
use anyhow::Result;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub mod user_provider;

pub use user_provider::SystemUserProvider;
pub use user_provider::UserProvider;

pub struct AppContext {
    pub app_dir: PathBuf,
    pub config_file: PathBuf,
    pub config: Configuration,
    pub username: String,
    pub groupname: String,
    pub home_dir: PathBuf,
    pub state_dir: PathBuf,
    pub nginx_files_path: PathBuf,
    pub ssl_path: PathBuf,
    pub uid: u32,
    pub gid: u32,
}

impl AppContext {
    pub fn build(provider: &dyn UserProvider) -> Result<Self> {
        let username = provider.username()?;
        let groupname = provider.groupname()?;
        let home_dir = provider.home_dir(&username)?;
        let uid = provider.uid(&username)?;
        let gid = provider.gid(&username)?;
        let app_dir = home_dir.join(".config").join("valex");
        let state_dir = home_dir.join(".local").join("state").join("valex");
        let config_path = app_dir.join("config.json5");

        create_dir_all(&app_dir)?;
        create_dir_all(&state_dir)?;

        let nginx_files_path = app_dir.join("nginx");
        create_dir_all(&nginx_files_path)?;

        let ssl_path = app_dir.join("ssl");
        create_dir_all(&ssl_path)?;

        let config = Configuration::load_or_default(&config_path)?;

        Ok(AppContext {
            app_dir,
            config_file: config_path,
            config,
            username,
            groupname,
            nginx_files_path,
            ssl_path,
            home_dir,
            state_dir,
            uid,
            gid,
        })
    }
}
