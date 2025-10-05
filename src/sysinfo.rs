use crate::security::SecurityInfo;
use std::fs;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub uptime: String,
    pub cpu: String,
    pub memory: String,
    pub storage: String,
    pub shell: String,
    pub desktop: String,
    pub terminal: String,
    pub security: SecurityInfo,
    pub gpu: String,
    pub temperature: String,
    pub network: String,
    pub packages: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: Self::get_os_info(),
            kernel: Self::get_kernel_version(),
            hostname: Self::get_hostname(),
            uptime: Self::get_uptime(&sys),
            cpu: Self::get_cpu_info(&sys),
            memory: Self::get_memory_info(&sys),
            storage: Self::get_storage_info(&sys),
            shell: Self::get_shell(),
            desktop: Self::get_desktop_environment(),
            terminal: Self::get_terminal(),
            security: SecurityInfo::new(),
            gpu: Self::get_gpu_info(),
            temperature: Self::get_temperature(),
            network: Self::get_network_info(),
            packages: Self::get_package_count(),
        }
    }

    fn get_os_info() -> String {
        // Check for OS environment variable override first
        if let Ok(os_override) = std::env::var("OS") {
            return os_override;
        }
        
        match std::env::consts::OS {
            "linux" => Self::get_linux_os_info(),
            "macos" => Self::get_macos_os_info(),
            _ => std::env::consts::OS.to_string(),
        }
    }

    fn get_linux_os_info() -> String {
        if let Ok(contents) = fs::read_to_string("/etc/os-release") {
            for line in contents.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line
                        .split('=')
                        .nth(1)
                        .unwrap_or("Unknown")
                        .trim_matches('"')
                        .to_string();
                }
            }
        }
        "Linux".to_string()
    }

    fn get_macos_os_info() -> String {
        use std::process::Command;

        if let Ok(output) = Command::new("sw_vers").arg("-productName").output() {
            if let Ok(product) = String::from_utf8(output.stdout) {
                if let Ok(version_output) = Command::new("sw_vers").arg("-productVersion").output()
                {
                    if let Ok(version) = String::from_utf8(version_output.stdout) {
                        return format!("{} {}", product.trim(), version.trim());
                    }
                }
                return product.trim().to_string();
            }
        }
        "macOS".to_string()
    }

    fn get_kernel_version() -> String {
        use std::process::Command;

        match std::env::consts::OS {
            "linux" => {
                if let Ok(contents) = fs::read_to_string("/proc/version") {
                    if let Some(version) = contents.split_whitespace().nth(2) {
                        return version.to_string();
                    }
                }
                "Unknown".to_string()
            }
            "macos" => {
                if let Ok(output) = Command::new("uname").arg("-r").output() {
                    if let Ok(version) = String::from_utf8(output.stdout) {
                        return format!("Darwin {}", version.trim());
                    }
                }
                "Unknown".to_string()
            }
            _ => "Unknown".to_string(),
        }
    }

    fn get_hostname() -> String {
        if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
            hostname.trim().to_string()
        } else {
            "Unknown".to_string()
        }
    }

    fn get_uptime(_sys: &System) -> String {
        let uptime_seconds = System::uptime();
        let days = uptime_seconds / 86400;
        let hours = (uptime_seconds % 86400) / 3600;
        let minutes = (uptime_seconds % 3600) / 60;

        if days > 0 {
            format!("{} days, {} hours, {} minutes", days, hours, minutes)
        } else if hours > 0 {
            format!("{} hours, {} minutes", hours, minutes)
        } else {
            format!("{} minutes", minutes)
        }
    }

    fn get_cpu_info(sys: &System) -> String {
        if let Some(cpu) = sys.cpus().first() {
            let cpu_name = cpu.brand().trim();
            let cpu_count = sys.cpus().len();
            format!("{} ({} cores)", cpu_name, cpu_count)
        } else {
            "Unknown".to_string()
        }
    }

    fn get_memory_info(sys: &System) -> String {
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = used_memory as f64 / 1024.0 / 1024.0 / 1024.0;

        format!("{:.1} GB / {:.1} GB", used_gb, total_gb)
    }

    fn get_storage_info(_sys: &System) -> String {
        use sysinfo::Disks;
        let disks = Disks::new_with_refreshed_list();

        let mut total_space = 0u64;
        let mut total_used = 0u64;
        let mut disk_count = 0;

        for disk in disks.iter() {
            // Skip pseudo filesystems and temporary mounts
            let mount_point = disk.mount_point().to_str().unwrap_or("");
            let fs_type = disk.file_system().to_str().unwrap_or("");

            // Skip common pseudo/temporary filesystems
            if mount_point.starts_with("/proc")
                || mount_point.starts_with("/sys")
                || mount_point.starts_with("/dev")
                || mount_point.starts_with("/run")
                || mount_point.starts_with("/tmp")
                || fs_type == "tmpfs"
                || fs_type == "devtmpfs"
                || fs_type == "sysfs"
                || fs_type == "proc"
                || disk.total_space() == 0
            {
                continue;
            }

            total_space += disk.total_space();
            total_used += disk.total_space() - disk.available_space();
            disk_count += 1;
        }

        if disk_count > 0 {
            let total_gb = total_space as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_gb = total_used as f64 / 1024.0 / 1024.0 / 1024.0;

            if disk_count == 1 {
                format!("{:.1} GB / {:.1} GB", used_gb, total_gb)
            } else {
                format!(
                    "{:.1} GB / {:.1} GB ({} drives)",
                    used_gb, total_gb, disk_count
                )
            }
        } else {
            "Unknown".to_string()
        }
    }

    fn get_shell() -> String {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "Unknown".to_string())
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string()
    }

    fn get_desktop_environment() -> String {
        let de_vars = [
            "XDG_CURRENT_DESKTOP",
            "DESKTOP_SESSION",
            "GNOME_DESKTOP_SESSION_ID",
        ];

        for var in &de_vars {
            if let Ok(value) = std::env::var(var) {
                return value;
            }
        }

        "Unknown".to_string()
    }

    fn get_terminal() -> String {
        std::env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
    }

    fn get_gpu_info() -> String {
        use std::process::Command;

        // Try NVIDIA first (most specific)
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=name", "--format=csv,noheader,nounits"])
            .output()
        {
            if output.status.success() {
                let nvidia_output = String::from_utf8_lossy(&output.stdout);
                if let Some(gpu_name) = nvidia_output.lines().next() {
                    if !gpu_name.trim().is_empty() {
                        return format!("NVIDIA {}", gpu_name.trim());
                    }
                }
            }
        }

        // Try AMD GPU detection
        if let Ok(output) = Command::new("rocm-smi")
            .args(["--showproductname"])
            .output()
        {
            if output.status.success() {
                let amd_output = String::from_utf8_lossy(&output.stdout);
                for line in amd_output.lines() {
                    if line.contains("Card series:") || line.contains("Card model:") {
                        if let Some(model) = line.split(":").nth(1) {
                            return format!("AMD {}", model.trim());
                        }
                    }
                }
            }
        }

        // Try lspci for comprehensive GPU detection
        if let Ok(output) = Command::new("lspci").output() {
            let lspci_output = String::from_utf8_lossy(&output.stdout);
            for line in lspci_output.lines() {
                let lower_line = line.to_lowercase();
                if lower_line.contains("vga")
                    || lower_line.contains("3d")
                    || lower_line.contains("display")
                {
                    if let Some(gpu_part) = line.split(": ").nth(1) {
                        let gpu_info = gpu_part.to_string();

                        // Enhanced parsing for better GPU identification
                        if lower_line.contains("nvidia")
                            || gpu_info.to_lowercase().contains("geforce")
                            || gpu_info.to_lowercase().contains("quadro")
                            || gpu_info.to_lowercase().contains("tesla")
                        {
                            return Self::parse_nvidia_gpu(&gpu_info);
                        } else if lower_line.contains("amd")
                            || lower_line.contains("radeon")
                            || gpu_info.to_lowercase().contains("radeon")
                        {
                            return Self::parse_amd_gpu(&gpu_info);
                        } else if lower_line.contains("intel") {
                            return Self::parse_intel_gpu(&gpu_info);
                        }

                        return gpu_info;
                    }
                }
            }
        }

        // Try glxinfo for OpenGL renderer info
        if let Ok(output) = Command::new("glxinfo").args(["-B"]).output() {
            let glx_output = String::from_utf8_lossy(&output.stdout);
            for line in glx_output.lines() {
                if line.trim().starts_with("OpenGL renderer string:") {
                    if let Some(renderer) = line.split(": ").nth(1) {
                        let renderer_info = renderer.trim();

                        // Parse renderer string for better identification
                        if renderer_info.to_lowercase().contains("nvidia")
                            || renderer_info.to_lowercase().contains("geforce")
                        {
                            return Self::parse_nvidia_gpu(renderer_info);
                        } else if renderer_info.to_lowercase().contains("amd")
                            || renderer_info.to_lowercase().contains("radeon")
                        {
                            return Self::parse_amd_gpu(renderer_info);
                        }

                        return renderer_info.to_string();
                    }
                }
            }
        }

        // macOS GPU detection
        if std::env::consts::OS == "macos" {
            if let Ok(output) = Command::new("system_profiler")
                .args(["SPDisplaysDataType"])
                .output()
            {
                let gpu_output = String::from_utf8_lossy(&output.stdout);
                for line in gpu_output.lines() {
                    if line.trim().starts_with("Chipset Model:") {
                        if let Some(model) = line.split(":").nth(1) {
                            return model.trim().to_string();
                        }
                    }
                }
            }
        }

        "Unknown".to_string()
    }

    fn parse_nvidia_gpu(gpu_info: &str) -> String {
        // Clean up NVIDIA GPU names
        let mut name = gpu_info.to_string();

        // Remove common prefixes/suffixes
        name = name.replace("NVIDIA Corporation", "NVIDIA");
        name = name.replace("[", "").replace("]", "");

        // Clean up extra whitespace
        name = name.split_whitespace().collect::<Vec<&str>>().join(" ");

        if !name.starts_with("NVIDIA")
            && (name.contains("GeForce")
                || name.contains("GTX")
                || name.contains("RTX")
                || name.contains("Quadro")
                || name.contains("Tesla"))
        {
            format!("NVIDIA {}", name)
        } else {
            name
        }
    }

    fn parse_amd_gpu(gpu_info: &str) -> String {
        // Clean up AMD GPU names
        let mut name = gpu_info.to_string();

        // Remove common prefixes
        name = name.replace("Advanced Micro Devices", "AMD");
        name = name.replace("ATI Technologies Inc", "AMD");
        name = name.replace("[AMD/ATI]", "AMD");
        name = name.replace("[", "").replace("]", "");

        // Clean up extra whitespace
        name = name.split_whitespace().collect::<Vec<&str>>().join(" ");

        if !name.starts_with("AMD") && name.contains("Radeon") {
            format!("AMD {}", name)
        } else {
            name
        }
    }

    fn parse_intel_gpu(gpu_info: &str) -> String {
        // Clean up Intel GPU names
        let mut name = gpu_info.to_string();

        // Remove common prefixes
        name = name.replace("Intel Corporation", "Intel");
        name = name.replace("[", "").replace("]", "");

        // Clean up extra whitespace
        name = name.split_whitespace().collect::<Vec<&str>>().join(" ");

        if !name.starts_with("Intel") {
            format!("Intel {}", name)
        } else {
            name
        }
    }

    fn get_temperature() -> String {
        use std::process::Command;

        // Try different temperature sources
        let temp_paths = [
            "/sys/class/thermal/thermal_zone0/temp",
            "/sys/class/thermal/thermal_zone1/temp",
        ];

        for path in &temp_paths {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(temp_millicelsius) = contents.trim().parse::<i32>() {
                    let temp_celsius = temp_millicelsius / 1000;
                    if temp_celsius > 0 && temp_celsius < 150 {
                        // Reasonable range
                        return format!("{}°C", temp_celsius);
                    }
                }
            }
        }

        // Try sensors command
        if let Ok(output) = Command::new("sensors").output() {
            let sensors_output = String::from_utf8_lossy(&output.stdout);
            for line in sensors_output.lines() {
                if line.contains("Core 0") || line.contains("temp1") {
                    if let Some(temp_part) = line.split("+").nth(1) {
                        if let Some(temp) = temp_part.split("°C").next() {
                            return format!("{}°C", temp.trim());
                        }
                    }
                }
            }
        }

        "Unknown".to_string()
    }

    fn get_network_info() -> String {
        use std::process::Command;

        // Get network interface info
        if let Ok(output) = Command::new("ip")
            .args(["route", "show", "default"])
            .output()
        {
            let route_output = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = route_output.lines().next() {
                if let Some(dev_part) = line.split("dev ").nth(1) {
                    if let Some(interface) = dev_part.split_whitespace().next() {
                        // Get IP address for this interface
                        if let Ok(ip_output) = Command::new("ip")
                            .args(["addr", "show", interface])
                            .output()
                        {
                            let ip_info = String::from_utf8_lossy(&ip_output.stdout);
                            for ip_line in ip_info.lines() {
                                if ip_line.trim().starts_with("inet ")
                                    && !ip_line.contains("127.0.0.1")
                                {
                                    if let Some(ip_part) = ip_line.trim().split("inet ").nth(1) {
                                        if let Some(ip) = ip_part.split("/").next() {
                                            return format!("{} ({})", ip.trim(), interface);
                                        }
                                    }
                                }
                            }
                        }
                        return interface.to_string();
                    }
                }
            }
        }

        "Unknown".to_string()
    }

    fn get_package_count() -> String {
        use std::process::Command;

        // Try different package managers
        let package_managers = [
            ("dpkg", vec!["-l"]),
            ("rpm", vec!["-qa"]),
            ("pacman", vec!["-Q"]),
            ("brew", vec!["list"]),
            ("pkg", vec!["info"]),
            ("xbps-query", vec!["-l"]),
        ];

        for (cmd, args) in &package_managers {
            if let Ok(output) = Command::new(cmd).args(args).output() {
                if output.status.success() {
                    let package_output = String::from_utf8_lossy(&output.stdout);
                    let count = package_output
                        .lines()
                        .filter(|line| !line.trim().is_empty() && !line.starts_with("Listing"))
                        .count();

                    if count > 0 {
                        return format!("{} ({})", count, cmd);
                    }
                }
            }
        }

        // Try flatpak
        if let Ok(output) = Command::new("flatpak").args(["list"]).output() {
            if output.status.success() {
                let flatpak_output = String::from_utf8_lossy(&output.stdout);
                let flatpak_count = flatpak_output
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count();

                if flatpak_count > 0 {
                    return format!("{} (flatpak)", flatpak_count);
                }
            }
        }

        "Unknown".to_string()
    }
}
