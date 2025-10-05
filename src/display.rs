use crate::config::Config;
use crate::logo::LogoDisplay;
use crate::redactor::DataRedactor;
use crate::sysinfo::SystemInfo;
use colored::*;

pub struct Display {
    config: Config,
}

impl Display {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn show(&self, system_info: &SystemInfo, screenshot_mode: bool) {
        // Redact sensitive information if in screenshot mode
        let display_info = if screenshot_mode {
            DataRedactor::redact_system_info(system_info, &self.config)
        } else {
            system_info.clone()
        };

        let logo = LogoDisplay::show_logo(&display_info.os, &self.config);
        let info_lines = self.format_system_info(&display_info, screenshot_mode);

        // Find the maximum width of the ASCII art (excluding ANSI escape sequences)
        let max_logo_width = logo
            .iter()
            .map(|line| Self::visual_width(line))
            .max()
            .unwrap_or(0);

        // Add extra spacing (gap) after the logo
        let gap = 8; // Adjust this number to make the gap wider/narrower
        let total_width = max_logo_width + gap;

        let max_lines = std::cmp::max(logo.len(), info_lines.len());
        let empty_string = String::new();

        for i in 0..max_lines {
            let logo_line = logo.get(i).unwrap_or(&empty_string);
            let info_line = info_lines.get(i).unwrap_or(&empty_string);

            // Calculate how much padding we need based on visual width
            let logo_visual_width = Self::visual_width(logo_line);
            let needed_padding = total_width.saturating_sub(logo_visual_width);

            println!(
                "{}{} {}",
                logo_line.color(self.config.logo_color.to_colored_string()),
                " ".repeat(needed_padding),
                info_line
            );
        }
    }

    fn format_system_info(&self, info: &SystemInfo, screenshot_mode: bool) -> Vec<String> {
        let mut lines = Vec::new();

        // Title with hostname
        lines.push(format!(
            "{}@{}",
            info.hostname
                .color(self.config.accent_color.to_colored_string())
                .bold(),
            info.hostname
                .color(self.config.accent_color.to_colored_string())
                .bold()
        ));

        // Separator
        lines.push(
            "-".repeat(info.hostname.len() * 2 + 1)
                .color(self.config.separator_color.to_colored_string())
                .to_string(),
        );

        // System information
        if self.config.show_os {
            lines.push(self.format_info_line("OS", &info.os));
        }

        if self.config.show_kernel {
            lines.push(self.format_info_line("Kernel", &info.kernel));
        }

        if self.config.show_uptime {
            lines.push(self.format_info_line("Uptime", &info.uptime));
        }

        if self.config.show_cpu {
            lines.push(self.format_info_line("CPU", &info.cpu));
        }

        if self.config.show_memory {
            lines.push(self.format_info_line("Memory", &info.memory));
        }

        if self.config.show_storage {
            lines.push(self.format_info_line("Storage", &info.storage));
        }

        if self.config.show_shell {
            lines.push(self.format_info_line("Shell", &info.shell));
        }

        if self.config.show_desktop && info.desktop != "Unknown" {
            lines.push(self.format_info_line("DE", &info.desktop));
        }

        if self.config.show_terminal {
            lines.push(self.format_info_line("Terminal", &info.terminal));
        }

        if self.config.show_gpu && info.gpu != "Unknown" {
            lines.push(self.format_info_line("GPU", &info.gpu));
        }

        if self.config.show_temperature && info.temperature != "Unknown" {
            lines.push(self.format_info_line("Temperature", &info.temperature));
        }

        if self.config.show_network && info.network != "Unknown" {
            lines.push(self.format_info_line("Network", &info.network));
        }

        if self.config.show_packages && info.packages != "Unknown" {
            lines.push(self.format_info_line("Packages", &info.packages));
        }

        // Security information
        if self.config.show_security {
            lines.push(String::new()); // Spacing
            lines.push(
                "Security:"
                    .color(self.config.accent_color.to_colored_string())
                    .bold()
                    .to_string(),
            );

            if !screenshot_mode {
                // Only show detailed security info when NOT in screenshot mode
                if self.config.show_firewall && info.security.firewall_status != "None detected" {
                    lines.push(self.format_info_line("Firewall", &info.security.firewall_status));
                }

                if self.config.show_selinux && info.security.selinux_status != "Not available" {
                    lines.push(self.format_info_line("SELinux", &info.security.selinux_status));
                }

                if self.config.show_ssh {
                    lines.push(self.format_info_line("SSH", &info.security.ssh_status));
                }

                if self.config.show_fail2ban && info.security.fail2ban_status != "Not installed" {
                    lines.push(self.format_info_line("Fail2Ban", &info.security.fail2ban_status));
                }

                if self.config.show_secure_boot && info.security.secure_boot != "Unknown" {
                    lines.push(self.format_info_line("Secure Boot", &info.security.secure_boot));
                }
            }

            // These are safe to show in screenshot mode (defensive security is good to share)
            if self.config.show_apparmor && info.security.apparmor_status != "Not available" {
                lines.push(self.format_info_line("AppArmor", &info.security.apparmor_status));
            }

            if self.config.show_updates {
                lines.push(self.format_info_line("Updates", &info.security.package_updates));
            }

            if self.config.show_encryption && info.security.disk_encryption != "None detected" {
                lines.push(self.format_info_line("Encryption", &info.security.disk_encryption));
            }

            // In screenshot mode, show a general security summary instead of details
            if screenshot_mode {
                lines.push(
                    "Additional security details hidden in screenshot mode"
                        .color(ColorChoice::Yellow.to_colored_string())
                        .italic()
                        .to_string(),
                );
            }
        }

        // Add some spacing
        lines.push(String::new());

        // Screenshot mode notice
        if screenshot_mode {
            lines.push(
                DataRedactor::get_redaction_notice()
                    .color(ColorChoice::Yellow.to_colored_string())
                    .bold()
                    .to_string(),
            );
            lines.push(String::new());
        }

        // Color palette
        if self.config.show_colors {
            lines.push(self.format_color_palette());
        }

        lines
    }

    fn format_info_line(&self, label: &str, value: &str) -> String {
        format!(
            "{}: {}",
            label
                .color(self.config.label_color.to_colored_string())
                .bold(),
            value.color(self.config.value_color.to_colored_string())
        )
    }

    fn format_color_palette(&self) -> String {
        let colors = [
            "███".black(),
            "███".red(),
            "███".green(),
            "███".yellow(),
            "███".blue(),
            "███".magenta(),
            "███".cyan(),
            "███".white(),
        ];

        colors
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    // Calculate visual width of a string, excluding ANSI escape sequences
    fn visual_width(text: &str) -> usize {
        let mut result = String::new();
        let mut chars = text.chars();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Skip ANSI escape sequence
                if chars.next() == Some('[') {
                    // Continue until we find the end of the escape sequence
                    for escape_char in chars.by_ref() {
                        if escape_char.is_ascii_alphabetic() || escape_char == 'm' {
                            break;
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result.chars().count()
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum ColorChoice {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
}

impl ColorChoice {
    pub fn to_colored_string(self) -> Color {
        match self {
            ColorChoice::Red => Color::Red,
            ColorChoice::Green => Color::Green,
            ColorChoice::Blue => Color::Blue,
            ColorChoice::Yellow => Color::Yellow,
            ColorChoice::Magenta => Color::Magenta,
            ColorChoice::Cyan => Color::Cyan,
            ColorChoice::White => Color::White,
            ColorChoice::Black => Color::Black,
        }
    }
}
