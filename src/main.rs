mod ascii;
mod ascii_maker;
mod config;
mod display;
mod logo;
mod logo_config;
mod redactor;
mod security;
mod sysinfo;

use crate::config::Config;
use crate::display::Display;
use crate::sysinfo::SystemInfo;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rfetch")
        .version("0.1.0")
        .author("Your Name")
        .about("A fast system information tool written in Rust")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Use custom config file"),
        )
        .arg(
            Arg::new("no-config")
                .long("no-config")
                .help("Don't use config file, use defaults")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("generate-config")
                .long("generate-config")
                .help("Generate default config file")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("screenshot")
                .short('s')
                .long("screenshot")
                .help("Screenshot mode - redact sensitive information")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ascii-maker")
                .long("ascii-maker")
                .help("Launch the ASCII art maker tool")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("ascii-maker") {
        crate::ascii_maker::AsciiMaker::run();
        return;
    }

    if matches.get_flag("generate-config") {
        let config = Config::default();
        match config.save() {
            Ok(_) => println!("Default config file generated successfully!"),
            Err(e) => eprintln!("Error generating config file: {}", e),
        }
        return;
    }

    let config = if matches.get_flag("no-config") {
        Config::default()
    } else {
        Config::load()
    };

    // Enable screenshot mode if flag is set
    let screenshot_mode = matches.get_flag("screenshot");

    let system_info = SystemInfo::new();
    let display = Display::new(config);

    display.show(&system_info, screenshot_mode);
}
