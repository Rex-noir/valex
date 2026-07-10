use std::process::Command;

use anyhow::{Ok, Result, bail};

use crate::{core::AppContext, core::CommandManager, util};

#[derive(Debug)]
pub struct Dns;

impl Dns {
    pub fn setup(_app: &AppContext) -> Result<()> {
        let cm = CommandManager::init();

        // check systemd version

        let version = Command::new("sudo")
            .arg("systemctl")
            .args(["--version"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| Self::parse_systemd_version(&s));

        if let Some(v) = version {
            println!("Systemd version detected : {}", v);
            if v < 258 {
                eprintln!("Not supported systemd version less than 258.")
            } else {
                println!("Setting up systemd resolved dns delegate config ...");
                Self::setup_dns_delegate_config()?;

                println!("Disable systemd resolved DNSStubListener");
                Self::disable_systemd_resolved_dns_stub_listener()?;

                if !cm.is_installed("dnsmasq")? {
                    println!("Installing dnsmasq ...");
                    cm.install_package("dnsmasq")?;
                }

                println!("Setting up dnsmasq configuration ...");
                Self::setup_dnsmasq_configuration()?;

                println!("Restarting dnsmasq");
                Self::restart_dnsmasq()?;

                println!("Enable dnsmasq systemd service ...");
                Self::enable_dnsmasq_systemd_service()?;

                println!("Restarting systemd-resolved");
                Self::restart_systemd_resolved()?;
            }
        } else {
            bail!("Systemd version can't be determined! Aborting!!")
        }

        Ok(())
    }

    fn parse_systemd_version(stdout: &str) -> Option<u32> {
        let first_line = stdout.lines().next()?;

        let version_str = first_line.split_whitespace().nth(1)?;

        version_str.parse::<u32>().ok()
    }

    fn setup_dns_delegate_config() -> Result<()> {
        let dir = "/etc/systemd/dns-delegate.d";
        util::sudo_create_dir_all(dir)?;

        let path = "/etc/systemd/dns-delegate.d/valet-rust.dns-delegate";
        let content = "[Delegate]\nDNS=127.0.0.1\nDomains=~test\nDNSSECMode=no\n";
        util::sudo_write(path, content)?;

        println!("DNS delegate config written to {}", path);
        Ok(())
    }

    fn restart_systemd_resolved() -> Result<()> {
        if !Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "systemd-resolved"])
            .status()?
            .success()
        {
            bail!("Error restarting systemd-resolved");
        }

        Ok(())
    }

    fn setup_dnsmasq_configuration() -> Result<()> {
        let main_dnsmasq_conf_path = "/etc/dnsmasq.conf";
        let stub_dnsmasq_conf = include_str!("../stubs/dnsmasq.conf");

        util::sudo_write(main_dnsmasq_conf_path, stub_dnsmasq_conf)?;

        let dir = "/etc/dnsmasq.d";
        util::sudo_create_dir_all(dir)?;

        let path = "/etc/dnsmasq.d/valet-rust.conf";
        let config =
            "listen-address=127.0.0.1\nbind-interfaces\nno-resolv\naddress=/.test/127.0.0.1\n";
        util::sudo_write(path, config)?;

        println!("dnsmasq config written to {}", path);
        Ok(())
    }

    fn restart_dnsmasq() -> Result<()> {
        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "dnsmasq"])
            .status()?;

        if !status.success() {
            anyhow::bail!("failed to restart dnsmasq");
        }

        Ok(())
    }

    fn disable_systemd_resolved_dns_stub_listener() -> Result<()> {
        let dir = "/etc/systemd/resolved.conf.d";
        util::sudo_create_dir_all(dir)?;

        let path = "/etc/systemd/resolved.conf.d/no-stub-listener.conf";
        let content = "[Resolve]\nDNSStubListener=no\n";
        util::sudo_write(path, content)?;

        println!("DNSStubListener disabled via drop-in at {}", path);
        Ok(())
    }

    fn enable_dnsmasq_systemd_service() -> Result<()> {
        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["enable", "--now", "dnsmasq"])
            .status()?;

        if !status.success() {
            anyhow::bail!("failed to enable and start dnsmasq");
        }

        Ok(())
    }
}
