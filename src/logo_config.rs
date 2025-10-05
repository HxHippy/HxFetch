use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoConfig {
    #[serde(default)]
    pub logos: HashMap<String, CustomLogo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLogo {
    pub function_name: String,
    pub lines: Vec<String>,
    #[serde(default)]
    pub colored: bool,
}

impl LogoConfig {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Warning: Failed to parse logo config: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to read logo config: {}", e);
                }
            }
        }

        // Return default config if file doesn't exist or failed to load
        Self {
            logos: HashMap::new(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    pub fn get_config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("rfetch").join("logos.toml")
        } else {
            // Fallback to current directory
            PathBuf::from("logos.toml")
        }
    }

    pub fn add_logo(
        &mut self,
        os_name: &str,
        function_name: &str,
        lines: Vec<String>,
        colored: bool,
    ) {
        let logo = CustomLogo {
            function_name: function_name.to_string(),
            lines,
            colored,
        };

        self.logos.insert(os_name.to_lowercase(), logo);
    }

    pub fn get_logo(&self, os_name: &str) -> Option<&CustomLogo> {
        // Try exact match first
        if let Some(logo) = self.logos.get(&os_name.to_lowercase()) {
            return Some(logo);
        }

        // Try partial matching - find any logo where the os_name contains the key
        let os_lower = os_name.to_lowercase();
        for (key, logo) in &self.logos {
            if os_lower.contains(key) {
                return Some(logo);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn list_logos(&self) -> Vec<&String> {
        self.logos.keys().collect()
    }

    #[allow(dead_code)]
    pub fn remove_logo(&mut self, os_name: &str) -> bool {
        self.logos.remove(&os_name.to_lowercase()).is_some()
    }
}
