use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SecurityInfo {
    pub firewall_status: String,
    pub selinux_status: String,
    pub apparmor_status: String,
    pub package_updates: String,
    pub ssh_status: String,
    pub fail2ban_status: String,
    pub disk_encryption: String,
    pub secure_boot: String,
}

impl SecurityInfo {
    pub fn new() -> Self {
        Self {
            firewall_status: Self::get_firewall_status(),
            selinux_status: Self::get_selinux_status(),
            apparmor_status: Self::get_apparmor_status(),
            package_updates: Self::get_package_updates(),
            ssh_status: Self::get_ssh_status(),
            fail2ban_status: Self::get_fail2ban_status(),
            disk_encryption: Self::get_disk_encryption_status(),
            secure_boot: Self::get_secure_boot_status(),
        }
    }

    fn get_firewall_status() -> String {
        // Check ufw first (Ubuntu/Debian default)
        if let Ok(output) = Command::new("ufw").arg("status").output() {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                if status.contains("Status: active") {
                    return "UFW: Active".to_string();
                } else if status.contains("Status: inactive") {
                    return "UFW: Inactive".to_string();
                }
            }
        }

        // Check iptables
        if let Ok(output) = Command::new("iptables").args(["-L", "-n"]).output() {
            if output.status.success() {
                let rules = String::from_utf8_lossy(&output.stdout);
                let rule_count = rules
                    .lines()
                    .filter(|line| {
                        !line.contains("Chain")
                            && !line.contains("target")
                            && !line.trim().is_empty()
                    })
                    .count();

                if rule_count > 0 {
                    return format!("iptables: {} rules", rule_count);
                }
            }
        }

        // Check firewalld
        if let Ok(output) = Command::new("firewall-cmd").arg("--state").output() {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                if status.trim() == "running" {
                    return "firewalld: Active".to_string();
                }
            }
        }

        "None detected".to_string()
    }

    fn get_selinux_status() -> String {
        if let Ok(contents) = fs::read_to_string("/sys/fs/selinux/enforce") {
            match contents.trim() {
                "1" => return "Enforcing".to_string(),
                "0" => return "Permissive".to_string(),
                _ => {}
            }
        }

        if let Ok(output) = Command::new("getenforce").output() {
            if output.status.success() {
                return String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }

        "Not available".to_string()
    }

    fn get_apparmor_status() -> String {
        if let Ok(contents) = fs::read_to_string("/sys/module/apparmor/parameters/enabled") {
            if contents.trim() == "Y" {
                // Check how many profiles are loaded
                if let Ok(profiles) = fs::read_to_string("/sys/kernel/security/apparmor/profiles") {
                    let profile_count = profiles.lines().count();
                    return format!("Active ({} profiles)", profile_count);
                }
                return "Active".to_string();
            }
        }

        if let Ok(output) = Command::new("aa-status").output() {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = status.lines().find(|l| l.contains("profiles are loaded")) {
                    return format!("Active ({})", line.trim());
                }
            }
        }

        "Not available".to_string()
    }

    fn get_package_updates() -> String {
        // Check for apt-based systems
        if let Ok(output) = Command::new("apt").args(["list", "--upgradable"]).output() {
            if output.status.success() {
                let upgradeable = String::from_utf8_lossy(&output.stdout);
                let count = upgradeable
                    .lines()
                    .filter(|line| line.contains("upgradable") && !line.starts_with("Listing"))
                    .count();

                if count > 0 {
                    return format!("{} available", count);
                } else {
                    return "Up to date".to_string();
                }
            }
        }

        // Check for yum/dnf-based systems
        if let Ok(output) = Command::new("dnf")
            .args(["check-update", "--quiet"])
            .output()
        {
            match output.status.code() {
                Some(0) => return "Up to date".to_string(),
                Some(100) => {
                    let updates = String::from_utf8_lossy(&output.stdout);
                    let count = updates
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .count();
                    return format!("{} available", count);
                }
                _ => {}
            }
        }

        "Unknown".to_string()
    }

    fn get_ssh_status() -> String {
        // Check for SSH daemon on different systems
        let ssh_services = ["sshd", "ssh", "openssh"];

        for service in &ssh_services {
            if let Ok(output) = Command::new("systemctl")
                .args(["is-active", service])
                .output()
            {
                let status = String::from_utf8_lossy(&output.stdout);
                match status.trim() {
                    "active" => return "Running".to_string(),
                    "inactive" => return "Stopped".to_string(),
                    "failed" => return "Failed".to_string(),
                    _ => continue,
                }
            }
        }

        // Check if SSH is listening on port 22
        if let Ok(output) = Command::new("ss").args(["-tlnp"]).output() {
            let netstat_output = String::from_utf8_lossy(&output.stdout);
            if netstat_output.contains(":22 ") {
                return "Running (port 22)".to_string();
            }
        }

        // Alternative check with netstat
        if let Ok(output) = Command::new("netstat").args(["-tlnp"]).output() {
            let netstat_output = String::from_utf8_lossy(&output.stdout);
            if netstat_output.contains(":22 ") {
                return "Running (port 22)".to_string();
            }
        }

        "Stopped".to_string()
    }

    fn get_fail2ban_status() -> String {
        if let Ok(output) = Command::new("systemctl")
            .args(["is-active", "fail2ban"])
            .output()
        {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                match status.trim() {
                    "active" => {
                        // Get jail count if possible
                        if let Ok(jail_output) =
                            Command::new("fail2ban-client").arg("status").output()
                        {
                            if jail_output.status.success() {
                                let jail_status = String::from_utf8_lossy(&jail_output.stdout);
                                if let Some(line) =
                                    jail_status.lines().find(|l| l.contains("Jail list:"))
                                {
                                    let jails = line.split(':').nth(1).unwrap_or("").trim();
                                    let jail_count =
                                        jails.split(',').filter(|j| !j.trim().is_empty()).count();
                                    return format!("Active ({} jails)", jail_count);
                                }
                            }
                        }
                        return "Active".to_string();
                    }
                    "inactive" => return "Inactive".to_string(),
                    _ => {}
                }
            }
        }

        "Not installed".to_string()
    }

    fn get_disk_encryption_status() -> String {
        // Check for LUKS encryption
        if let Ok(output) = Command::new("lsblk").args(["-f"]).output() {
            if output.status.success() {
                let lsblk_output = String::from_utf8_lossy(&output.stdout);
                if lsblk_output.contains("crypto_LUKS") {
                    let luks_count = lsblk_output
                        .lines()
                        .filter(|line| line.contains("crypto_LUKS"))
                        .count();
                    return format!("LUKS ({} encrypted)", luks_count);
                }
            }
        }

        // Check for encrypted filesystems in /proc/mounts
        if let Ok(contents) = fs::read_to_string("/proc/mounts") {
            if contents.contains("dm-crypt") {
                return "dm-crypt detected".to_string();
            }
        }

        "None detected".to_string()
    }

    fn get_secure_boot_status() -> String {
        if let Ok(contents) = fs::read_to_string(
            "/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c",
        ) {
            if contents.len() >= 5 {
                let secure_boot_enabled = contents.as_bytes()[4] == 1;
                if secure_boot_enabled {
                    return "Enabled".to_string();
                } else {
                    return "Disabled".to_string();
                }
            }
        }

        // Alternative check using mokutil
        if let Ok(output) = Command::new("mokutil").arg("--sb-state").output() {
            if output.status.success() {
                let status = String::from_utf8_lossy(&output.stdout);
                if status.contains("SecureBoot enabled") {
                    return "Enabled".to_string();
                } else if status.contains("SecureBoot disabled") {
                    return "Disabled".to_string();
                }
            }
        }

        "Unknown".to_string()
    }
}
