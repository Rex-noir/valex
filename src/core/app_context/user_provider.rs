use std::{env, path::PathBuf};

use anyhow::Result;
use uzers::{get_current_groupname, get_current_username, get_user_by_name, os::unix::UserExt};

pub trait UserProvider {
    fn username(&self) -> Result<String>;
    fn groupname(&self) -> Result<String>;
    fn home_dir(&self, username: &str) -> Result<PathBuf>;
    fn uid(&self, username: &str) -> Result<u32>;
    fn gid(&self, username: &str) -> Result<u32>;
}

pub struct SystemUserProvider;

impl SystemUserProvider {
    pub fn new() -> Self {
        Self
    }
}

impl UserProvider for SystemUserProvider {
    fn username(&self) -> Result<String> {
        Ok(env::var("SUDO_USER").unwrap_or_else(|_| {
            get_current_username()
                .expect("failed to determine current user")
                .into_string()
                .expect("username is not valid UTF-8")
        }))
    }

    fn groupname(&self) -> Result<String> {
        Ok(get_current_groupname()
            .expect("Failed to get user groupname")
            .into_string()
            .expect("Groupname is not a valid groupname"))
    }

    fn home_dir(&self, username: &str) -> Result<PathBuf> {
        let user = get_user_by_name(username).expect("failed to look up user");
        Ok(user.home_dir().to_path_buf())
    }

    fn uid(&self, username: &str) -> Result<u32> {
        let user = get_user_by_name(username).expect("failed to look up user");
        Ok(user.uid())
    }

    fn gid(&self, username: &str) -> Result<u32> {
        let user = get_user_by_name(username).expect("failed to look up user");
        Ok(user.primary_group_id())
    }
}
