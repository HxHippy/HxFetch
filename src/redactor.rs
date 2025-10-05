use crate::config::Config;
use crate::sysinfo::SystemInfo;
use regex::Regex;

pub struct DataRedactor;

impl DataRedactor {
    pub fn redact_system_info(info: &SystemInfo, config: &Config) -> SystemInfo {
        SystemInfo {
            os: info.os.clone(),         // OS info is generally safe
            kernel: info.kernel.clone(), // Kernel version is safe
            hostname: Self::redact_hostname(&info.hostname),
            uptime: info.uptime.clone(), // Uptime is safe
            cpu: Self::redact_cpu(&info.cpu, config.redact_cpu_model),
            memory: Self::redact_memory(&info.memory),
            storage: Self::redact_storage(&info.storage),
            shell: info.shell.clone(),       // Shell is safe
            desktop: info.desktop.clone(),   // DE is safe
            terminal: info.terminal.clone(), // Terminal is safe
            security: info.security.clone(), // Security info is generally safe for sharing
            gpu: Self::redact_gpu(&info.gpu),
            temperature: info.temperature.clone(), // Temperature is safe
            network: Self::redact_network(&info.network),
            packages: info.packages.clone(), // Package count is safe
        }
    }

    fn redact_hostname(hostname: &str) -> String {
        if hostname.len() <= 2 {
            return "*".repeat(hostname.len());
        }

        // Keep first and last character, redact middle
        let chars: Vec<char> = hostname.chars().collect();
        let first = chars[0];
        let last = chars[chars.len() - 1];
        let middle_length = chars.len() - 2;

        format!("{}{}{}", first, "*".repeat(middle_length), last)
    }

    fn redact_cpu(cpu: &str, redact_model: bool) -> String {
        // Extract and preserve core count
        let core_count = if let Ok(re) = Regex::new(r"\((\d+) cores?\)") {
            if let Some(captures) = re.captures(cpu) {
                format!("({} cores)", &captures[1])
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let mut redacted = cpu.to_string();

        // Only redact if the config option is enabled
        if redact_model {
            // Remove potentially identifying details but keep general info
            let cpu_patterns = [
                (r"\bi[3579]-\d{4,}[A-Z]*\b", "iX-XXXX"), // Intel model numbers like i9-12900K, i7-3600X
                (r"\bRyzen\s+[3579]\s+\d{4}[A-Z]*\b", "Ryzen X XXXX"), // AMD Ryzen like Ryzen 7 5800X
                (r"@\s*\d+\.\d+\s*GHz", "@ X.X GHz"), // Redact specific frequencies
                (r"\([^)]*cores?\)", ""), // Remove original core count (we'll add it back)
            ];

            for (pattern, replacement) in &cpu_patterns {
                if let Ok(re) = Regex::new(pattern) {
                    redacted = re.replace_all(&redacted, *replacement).to_string();
                }
            }
        } else {
            // Just remove the core count so we can add it back consistently
            if let Ok(re) = Regex::new(r"\([^)]*cores?\)") {
                redacted = re.replace_all(&redacted, "").to_string();
            }
        }

        // Clean up extra spaces and add back core count
        redacted = redacted.split_whitespace().collect::<Vec<&str>>().join(" ");

        if !core_count.is_empty() {
            format!("{} {}", redacted.trim(), core_count)
        } else {
            redacted
        }
    }

    fn redact_memory(memory: &str) -> String {
        // Redact specific memory amounts but keep general size category
        if let Ok(re) = Regex::new(r"(\d+\.\d+)\s*GB\s*/\s*(\d+\.\d+)\s*GB") {
            if let Some(captures) = re.captures(memory) {
                let used: f64 = captures[1].parse().unwrap_or(0.0);
                let total: f64 = captures[2].parse().unwrap_or(0.0);

                // Categorize memory size ranges
                let total_category = match total {
                    t if t < 4.0 => "< 4",
                    t if t < 8.0 => "4-8",
                    t if t < 16.0 => "8-16",
                    t if t < 32.0 => "16-32",
                    t if t < 64.0 => "32-64",
                    t if t < 128.0 => "64-128",
                    _ => "> 128",
                };

                let usage_percent = (used / total * 100.0).round() as u32;
                return format!("{}% of {} GB", usage_percent, total_category);
            }
        }

        memory.to_string()
    }

    fn redact_storage(storage: &str) -> String {
        // Similar to memory, categorize storage sizes
        if let Ok(re) = Regex::new(r"(\d+\.\d+)\s*GB\s*/\s*(\d+\.\d+)\s*GB") {
            if let Some(captures) = re.captures(storage) {
                let used: f64 = captures[1].parse().unwrap_or(0.0);
                let total: f64 = captures[2].parse().unwrap_or(0.0);

                let total_category = match total {
                    t if t < 128.0 => "< 128",
                    t if t < 256.0 => "128-256",
                    t if t < 512.0 => "256-512",
                    t if t < 1024.0 => "512GB-1TB",
                    t if t < 2048.0 => "1-2TB",
                    t if t < 4096.0 => "2-4TB",
                    _ => "> 4TB",
                };

                let usage_percent = (used / total * 100.0).round() as u32;
                return format!("{}% of {}", usage_percent, total_category);
            }
        }

        storage.to_string()
    }

    fn redact_gpu(gpu: &str) -> String {
        if gpu == "Unknown" {
            return gpu.to_string();
        }

        // Keep general GPU family but remove specific model numbers
        let gpu_patterns = [
            (r"\b\d{4}\b", "XXXX"),     // Model numbers like GTX 1060, RTX 3080
            (r"\b\d{3}\b", "XXX"),      // Shorter model numbers
            (r"\brev\s+\w+", "rev XX"), // Revision information
            (r"\([^)]*\)", ""),         // Remove parenthetical information
        ];

        let mut redacted = gpu.to_string();
        for (pattern, replacement) in &gpu_patterns {
            if let Ok(re) = Regex::new(pattern) {
                redacted = re.replace_all(&redacted, *replacement).to_string();
            }
        }

        // Clean up extra spaces
        redacted = redacted.split_whitespace().collect::<Vec<&str>>().join(" ");
        redacted
    }

    fn redact_network(network: &str) -> String {
        if network == "Unknown" {
            return network.to_string();
        }

        // Redact IP addresses but keep interface names
        if let Ok(re) = Regex::new(r"(\d+\.\d+\.\d+\.\d+)\s*\(([^)]+)\)") {
            if let Some(captures) = re.captures(network) {
                let interface = &captures[2];
                return format!("XXX.XXX.XXX.XXX ({})", interface);
            }
        }

        // If it's just an interface name, keep it
        if !network.contains('.') {
            return network.to_string();
        }

        // Fallback: redact any IP-like patterns
        if let Ok(re) = Regex::new(r"\d+\.\d+\.\d+\.\d+") {
            return re.replace_all(network, "XXX.XXX.XXX.XXX").to_string();
        }

        network.to_string()
    }

    pub fn get_redaction_notice() -> String {
        "🔒 Screenshot mode: Sensitive information redacted".to_string()
    }
}
