use std::env::temp_dir;

use anyhow::Ok;
use valex::core::UserProvider;

struct MockUserProvider;

impl MockUserProvider {
    fn new() -> Self {
        Self {}
    }
}

impl UserProvider for MockUserProvider {
    fn username(&self) -> anyhow::Result<String> {
        Ok("testuser".to_string())
    }

    fn groupname(&self) -> anyhow::Result<String> {
        Ok("testgroup".to_string())
    }

    fn home_dir(&self, username: &str) -> anyhow::Result<std::path::PathBuf> {
        Ok(temp_dir().join(username))
    }

    fn uid(&self, _: &str) -> anyhow::Result<u32> {
        Ok(1000)
    }

    fn gid(&self, _: &str) -> anyhow::Result<u32> {
        Ok(1000)
    }
}

#[cfg(test)]
#[test]
fn build_expected_app_context() {
    use std::env::temp_dir;
    use valex::core::AppContext;

    let provider = MockUserProvider::new();

    let app = AppContext::build(&provider).unwrap();

    let expected_home = temp_dir().join("testuser").join(".config");
    let expected_app_dir = expected_home.join("valex");
    let expected_config_file = expected_app_dir.join("config.json5");

    assert_eq!(app.username, "testuser");
    assert_eq!(app.groupname, "testgroup");
    assert_eq!(app.app_dir, expected_app_dir);
    assert_eq!(app.config_file, expected_config_file);

    assert_eq!(app.config, Default::default());
}
