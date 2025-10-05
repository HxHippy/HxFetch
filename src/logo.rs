use crate::ascii::AsciiArt;
use crate::config::Config as AppConfig;

pub struct LogoDisplay;

impl LogoDisplay {
    pub fn show_logo(os_name: &str, _config: &AppConfig) -> Vec<String> {
        // For now, always use ASCII art until we can properly position images
        // TODO: Implement proper image positioning alongside system info

        // Custom image logos could be implemented here in the future
        // For now, ASCII art provides excellent visual appeal and perfect layout compatibility

        // Always use ASCII art for now
        AsciiArt::get_logo(os_name)
    }
}
