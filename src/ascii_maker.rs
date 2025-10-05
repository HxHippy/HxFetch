use rascii_art::{render_to, RenderOptions};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct AsciiMaker;

impl AsciiMaker {
    pub fn run() {
        println!("🎨 hxfetch ASCII Art Maker");
        println!("=========================");
        println!("Convert images to ASCII art for use in hxfetch logos!");
        println!();

        loop {
            println!("Choose an option:");
            println!("1. Convert image to ASCII art (25x18, terminal-friendly)");
            println!("2. Convert image to colored ASCII art (25x18, with background options)");
            println!("3. Manual ASCII editor");
            println!("4. Exit");
            println!();

            print!("Enter your choice (1-4): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                match input.trim() {
                    "1" => Self::image_to_ascii(false),
                    "2" => Self::image_to_ascii(true),
                    "3" => Self::manual_editor(),
                    "4" => break,
                    _ => println!("Invalid choice. Please enter 1-4."),
                }
            }
        }
    }

    fn image_to_ascii(colored: bool) {
        println!();
        if colored {
            println!("🌈 Image to Colored ASCII Art");
            println!("-----------------------------");
        } else {
            println!("🖼️  Image to ASCII Art");
            println!("----------------------");
        }

        print!("Enter path to image file: ");
        io::stdout().flush().unwrap();

        let mut path = String::new();
        if io::stdin().read_line(&mut path).is_ok() {
            let path = path.trim();

            if path.is_empty() {
                println!("❌ No path provided");
                return;
            }

            if !Path::new(path).exists() {
                println!("❌ File not found: {}", path);
                return;
            }

            // Validate it's an image file
            let path_lower = path.to_lowercase();
            if !path_lower.ends_with(".png")
                && !path_lower.ends_with(".jpg")
                && !path_lower.ends_with(".jpeg")
                && !path_lower.ends_with(".bmp")
                && !path_lower.ends_with(".gif")
                && !path_lower.ends_with(".svg")
            {
                println!("⚠️  Warning: File doesn't have a common image extension");
                println!("   Supported formats: PNG, JPEG, BMP, GIF, SVG");
                print!("Continue anyway? (y/n): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                if io::stdin().read_line(&mut confirm).is_ok()
                    && confirm.trim().to_lowercase() != "y"
                {
                    return;
                }
            }

            // Use standardized dimensions for consistency across all logos
            let width = 25; // Standard width to match existing logos
            let height = 18; // Standard height to match existing logos

            println!(
                "📏 Using standardized dimensions: {}x{} characters",
                width, height
            );
            println!("   (This ensures consistency with existing hxfetch logos)");

            println!();
            println!("Converting image...");

            // Limit dimensions to prevent excessive output
            let max_width = 80;
            let max_height = 40;
            let limited_width = width.min(max_width);
            let limited_height = height.min(max_height);

            // Create render options
            let options = RenderOptions::new()
                .width(limited_width as u32)
                .height(limited_height as u32)
                .colored(colored);

            let mut buffer = Vec::new();
            match render_to(path, &mut buffer, options) {
                Ok(_) => {
                    let ascii_art = String::from_utf8_lossy(&buffer).to_string();

                    // Limit output lines to prevent buffer overflow
                    let lines: Vec<&str> = ascii_art.lines().collect();
                    let limited_lines = if lines.len() > max_height {
                        println!(
                            "⚠️  Output limited to {} lines (original had {})",
                            max_height,
                            lines.len()
                        );
                        &lines[..max_height]
                    } else {
                        &lines
                    };

                    // Check total character count to prevent excessive output
                    let total_chars: usize = limited_lines.iter().map(|line| line.len()).sum();
                    if total_chars > 10000 {
                        println!(
                            "⚠️  Output too large ({} characters), showing preview only",
                            total_chars
                        );
                        println!();
                        println!("Preview (first 20 lines):");
                        println!("=========================");
                        for line in limited_lines.iter().take(20) {
                            println!("{}", line);
                        }
                        println!("=========================");
                        println!("Use smaller dimensions for full output.");
                    } else {
                        println!();
                        println!("Generated ASCII Art:");
                        println!("===================");
                        for line in limited_lines {
                            println!("{}", line);
                        }
                        println!("===================");
                    }

                    // Convert limited lines to Vec<String> for saving
                    let save_lines: Vec<String> =
                        limited_lines.iter().map(|s| s.to_string()).collect();

                    let filename = Path::new(path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("custom");

                    // Sanitize filename for valid Rust identifier
                    let sanitized_name = filename
                        .replace("-", "_")
                        .replace(".", "_")
                        .replace(" ", "_")
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '_')
                        .collect::<String>()
                        .to_lowercase();

                    Self::save_ascii_art(&save_lines, &format!("{}_logo", sanitized_name), colored);
                }
                Err(e) => {
                    println!("❌ Error converting image: {}", e);
                    println!("Make sure the file is a valid image format (PNG, JPEG, etc.)");
                }
            }
        }
    }

    fn manual_editor() {
        println!();
        println!("✏️  Manual ASCII Editor");
        println!("----------------------");
        println!("📏 Recommended size: 25 characters wide, 18 lines tall");
        println!("   (This ensures consistency with other hxfetch logos)");
        println!();
        println!("Enter your ASCII art line by line.");
        println!("Type 'DONE' on a new line when finished.");
        println!("Type 'PREVIEW' to see your current art.");
        println!("Type 'CLEAR' to start over.");
        println!();

        let mut lines = Vec::new();

        loop {
            print!("Line {}: ", lines.len() + 1);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim_end(); // Keep leading spaces but remove trailing newline

                match input {
                    "DONE" => {
                        if lines.is_empty() {
                            println!("No lines entered. Please add some ASCII art first.");
                            continue;
                        }
                        break;
                    }
                    "PREVIEW" => {
                        println!();
                        println!("Current ASCII Art:");
                        println!("------------------");
                        for (i, line) in lines.iter().enumerate() {
                            println!("{:2}: {}", i + 1, line);
                        }
                        println!("------------------");
                        println!();
                        continue;
                    }
                    "CLEAR" => {
                        lines.clear();
                        println!("Cleared all lines. Starting over...");
                        println!();
                        continue;
                    }
                    _ => {
                        lines.push(input.to_string());
                    }
                }
            }
        }

        if !lines.is_empty() {
            println!();
            println!("Final ASCII Art:");
            println!("================");
            for line in &lines {
                println!("{}", line);
            }
            println!("================");

            print!("Enter a name for this logo: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            if io::stdin().read_line(&mut name).is_ok() {
                let name = name.trim();
                if !name.is_empty() {
                    Self::save_ascii_art(
                        &lines,
                        &format!("{}_logo", name.to_lowercase().replace(" ", "_")),
                        false,
                    );
                }
            }
        }
    }

    fn save_ascii_art(lines: &[String], function_name: &str, colored: bool) {
        println!();
        print!("Save this ASCII art as a Rust function? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() && input.trim().to_lowercase() == "y" {
            let filename = format!("{}.rs", function_name);

            let mut content = String::new();
            content.push_str(&format!("    fn {}() -> Vec<String> {{\n", function_name));
            content.push_str("        vec![\n");

            for line in lines {
                // Escape quotes and backslashes in the line
                let escaped_line = line.replace("\\", "\\\\").replace("\"", "\\\"");
                content.push_str(&format!("            \"{}\".to_string(),\n", escaped_line));
            }

            content.push_str("        ]\n");
            content.push_str("    }\n");

            match fs::write(&filename, &content) {
                Ok(_) => {
                    println!("✅ ASCII art saved to '{}'", filename);
                    println!();
                    if colored {
                        println!("⚠️  Note: This ASCII art contains color codes.");
                        println!("   It will look best in terminals that support ANSI colors.");
                        println!();

                        // Offer to create a background-free version
                        print!("🎨 Create a version without black background for better terminal theme compatibility? (y/n): ");
                        io::stdout().flush().unwrap();
                        let mut bg_input = String::new();
                        if io::stdin().read_line(&mut bg_input).is_ok()
                            && bg_input.trim().to_lowercase() == "y"
                        {
                            let no_bg_lines = Self::strip_background_colors(lines);
                            let no_bg_filename = format!("{}_no_bg.rs", function_name);

                            let mut no_bg_content = String::new();
                            no_bg_content.push_str(&format!(
                                "    fn {}_no_bg() -> Vec<String> {{\n",
                                function_name
                            ));
                            no_bg_content.push_str("        vec![\n");

                            for line in &no_bg_lines {
                                let escaped_line = line.replace("\\", "\\\\").replace("\"", "\\\"");
                                no_bg_content.push_str(&format!(
                                    "            \"{}\".to_string(),\n",
                                    escaped_line
                                ));
                            }

                            no_bg_content.push_str("        ]\n");
                            no_bg_content.push_str("    }\n");

                            if fs::write(&no_bg_filename, &no_bg_content).is_ok() {
                                println!(
                                    "✅ Background-free version saved to '{}'",
                                    no_bg_filename
                                );
                                println!("🌈 This version will adapt to your terminal theme!");
                            }
                        }
                    }
                    println!();
                    println!("📋 To use this in hxfetch:");
                    println!(
                        "Apply it automatically, or manually save to: ~/.config/hxfetch/logos.toml"
                    );
                    println!("Format:");
                    println!("  [logos.your_os_name]");
                    println!("  function_name = \"{}\"", function_name);
                    println!("  colored = {}", colored);
                    println!("  lines = [ ... ]");
                    println!();

                    // Also save as plain text
                    let txt_filename = format!("{}.txt", function_name);
                    let plain_content = lines.join("\n");
                    if fs::write(&txt_filename, &plain_content).is_ok() {
                        println!("📄 Also saved as plain text: '{}'", txt_filename);
                    }

                    // Ask if user wants to apply it to hxfetch
                    Self::offer_apply_to_hxfetch(lines, function_name, colored);
                }
                Err(e) => {
                    println!("❌ Error saving file: {}", e);
                }
            }
        } else {
            println!("ASCII art not saved.");
        }

        println!();
    }

    fn strip_background_colors(lines: &[String]) -> Vec<String> {
        lines
            .iter()
            .map(|line| {
                // Remove background color codes (like [38;2;0;0;0m for black background)
                // Keep foreground colors but remove background colors
                let mut result = String::new();
                let mut chars = line.chars().peekable();

                while let Some(ch) = chars.next() {
                    if ch == '\x1b' && chars.peek() == Some(&'[') {
                        // Found ANSI escape sequence
                        let mut escape_seq = String::new();
                        escape_seq.push(ch);
                        escape_seq.push(chars.next().unwrap()); // consume '['

                        // Read the rest of the escape sequence
                        for escape_char in chars.by_ref() {
                            escape_seq.push(escape_char);
                            if escape_char.is_ascii_alphabetic() || escape_char == 'm' {
                                break;
                            }
                        }

                        // Check if this is a background color (black specifically for transparency)
                        if escape_seq.contains("38;2;0;0;0") {
                            // Skip black background - this makes it transparent to terminal theme
                            continue;
                        } else {
                            // Keep other colors
                            result.push_str(&escape_seq);
                        }
                    } else {
                        result.push(ch);
                    }
                }
                result
            })
            .collect()
    }

    fn offer_apply_to_hxfetch(lines: &[String], function_name: &str, colored: bool) {
        println!();
        print!("🚀 Apply this logo to hxfetch automatically? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() && input.trim().to_lowercase() == "y" {
            print!("Enter the OS/distro name this logo should match: ");
            io::stdout().flush().unwrap();
            println!("💡 Tip: Use the exact name that appears in your 'OS:' field");
            println!("   Examples: 'My Custom Linux', 'debian', 'ubuntu', 'Arch Linux'");
            print!("OS name: ");
            io::stdout().flush().unwrap();

            let mut os_name = String::new();
            if io::stdin().read_line(&mut os_name).is_ok() {
                let os_name = os_name.trim();
                if !os_name.is_empty() {
                    // Clean the OS name for function naming but keep original for matching
                    let clean_name = os_name
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '_')
                        .collect::<String>();

                    match Self::apply_to_ascii_rs(lines, function_name, &clean_name) {
                        Ok(_) => {
                            println!("✅ Successfully saved logo configuration!");
                            println!(
                                "📝 Logo saved to: {}",
                                crate::logo_config::LogoConfig::get_config_path().display()
                            );
                            println!(
                                "🚀 Your logo will be used automatically when running hxfetch!"
                            );
                            if colored {
                                println!(
                                    "🌈 Your logo will display in color on supported terminals"
                                );
                            }
                        }
                        Err(e) => {
                            println!("❌ Failed to apply logo: {}", e);
                            println!("💡 You can still manually copy the function to src/ascii.rs");
                        }
                    }
                } else {
                    println!("❌ No OS name provided");
                }
            }
        }
    }

    fn apply_to_ascii_rs(
        lines: &[String],
        function_name: &str,
        os_name: &str,
    ) -> Result<(), String> {
        // Use TOML configuration instead of modifying source code
        use crate::logo_config::LogoConfig;

        let mut config = LogoConfig::load();

        // Determine if this is a colored logo (check for ANSI escape sequences)
        let colored = lines.iter().any(|line| line.contains("\x1b["));

        // Add the logo to configuration
        config.add_logo(os_name, function_name, lines.to_vec(), colored);

        // Save the configuration
        config
            .save()
            .map_err(|e| format!("Failed to save logo configuration: {}", e))?;

        Ok(())
    }
}
