use crate::display::ColorChoice;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub show_os: bool,
    pub show_kernel: bool,
    pub show_uptime: bool,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_storage: bool,
    pub show_shell: bool,
    pub show_desktop: bool,
    pub show_terminal: bool,
    pub show_colors: bool,

    // Security options
    pub show_security: bool,
    pub show_firewall: bool,
    pub show_selinux: bool,
    pub show_apparmor: bool,
    pub show_updates: bool,
    pub show_ssh: bool,
    pub show_fail2ban: bool,
    pub show_encryption: bool,
    pub show_secure_boot: bool,

    // Advanced metrics
    pub show_gpu: bool,
    pub show_temperature: bool,
    pub show_network: bool,
    pub show_packages: bool,

    // Privacy settings
    pub redact_cpu_model: bool,

    // Logo settings
    pub custom_logo_path: Option<String>,
    pub logo_width: Option<u32>,
    pub logo_height: Option<u32>,

    pub logo_color: ColorChoice,
    pub accent_color: ColorChoice,
    pub label_color: ColorChoice,
    pub value_color: ColorChoice,
    pub separator_color: ColorChoice,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_os: true,
            show_kernel: true,
            show_uptime: true,
            show_cpu: true,
            show_memory: true,
            show_storage: true,
            show_shell: true,
            show_desktop: true,
            show_terminal: true,
            show_colors: true,

            // Security defaults
            show_security: true,
            show_firewall: true,
            show_selinux: true,
            show_apparmor: true,
            show_updates: true,
            show_ssh: true,
            show_fail2ban: true,
            show_encryption: true,
            show_secure_boot: true,

            // Advanced metrics defaults
            show_gpu: true,
            show_temperature: true,
            show_network: true,
            show_packages: true,

            // Privacy defaults
            redact_cpu_model: false,

            // Logo defaults
            custom_logo_path: None,
            logo_width: Some(30),
            logo_height: Some(20),

            logo_color: ColorChoice::Cyan,
            accent_color: ColorChoice::Blue,
            label_color: ColorChoice::Cyan,
            value_color: ColorChoice::White,
            separator_color: ColorChoice::Blue,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }

        // Return default config and create config file
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(config_path, contents)?;

        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.push("rfetch");
        config_dir.push("config.toml");
        config_dir
    }
}
