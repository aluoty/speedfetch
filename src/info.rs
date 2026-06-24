use std::collections::HashSet;
use std::env;
use std::fs;
use std::process::Command;

pub fn os() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|data| {
            data.lines().find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|v| v.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "Unknown OS".to_string())
}

pub fn kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|k| format!("Linux {}", k.trim()))
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("HOST"))
        .unwrap_or_else(|_| {
            Command::new("hostname")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "localhost".to_string())
        })
}

pub fn username() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string())
}

pub fn user_host() -> String {
    format!("{}@{}", username(), hostname())
}

pub fn shell() -> String {
    env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(str::to_string))
        .unwrap_or_else(|| "sh".to_string())
}

pub fn terminal() -> String {
    if let Ok(t) = env::var("TERM_PROGRAM") {
        if let Ok(v) = env::var("TERM_PROGRAM_VERSION") {
            return format!("{t} {v}");
        }
        return t;
    }
    if env::var("ALACRITTY_WINDOW_ID").is_ok() {
        return "Alacritty".to_string();
    }
    if env::var("KITTY_PID").is_ok() {
        return "Kitty".to_string();
    }
    if env::var("WEZTERM_EXECUTABLE_DIR").is_ok() {
        return "WezTerm".to_string();
    }
    if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return "Ghostty".to_string();
    }
    if let Ok(t) = env::var("TERMINAL_EMULATOR") {
        return t.replace("()", "").trim().to_string();
    }
    env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
}

pub fn de() -> String {
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        if !de.is_empty() {
            return de;
        }
    }
    if let Ok(s) = env::var("DESKTOP_SESSION") {
        if !s.is_empty() {
            return s;
        }
    }
    if let Ok(s) = env::var("XDG_SESSION_DESKTOP") {
        if !s.is_empty() {
            return s;
        }
    }
    "N/A".to_string()
}

pub fn wm() -> String {
    let de = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    // Known DE→WM mappings
    if de == "GNOME" || de.starts_with("GNOME:") || de.starts_with("gnome") {
        return "Mutter".to_string();
    }
    if de.contains("KDE") || de.contains("Plasma") || de.contains("kde") || de.contains("plasma") {
        return "KWin".to_string();
    }
    if de.contains("XFCE") || de.contains("xfce") || de.contains("Xfce") {
        return "xfwm4".to_string();
    }
    if de.contains("MATE") || de.contains("mate") || de.contains("Mate") {
        return "Marco".to_string();
    }
    if de.contains("Budgie") || de.contains("budgie") {
        return "BudgieWM".to_string();
    }
    if de.contains("Cinnamon") || de.contains("cinnamon") {
        return "Muffin".to_string();
    }

    // Env-var-detectable WMs
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return "Hyprland".to_string();
    }
    if env::var("SWAYSOCK").is_ok() {
        return "Sway".to_string();
    }
    if env::var("I3SOCK").is_ok() || env::var("I3_CONFIG_DIR").is_ok() {
        return "i3".to_string();
    }

    // Scan process list for known WM binaries
    if let Ok(out) = Command::new("ps").args(["-e", "-o", "comm="]).output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            for name in &[
                "mutter", "kwin_x11", "kwin_wayland", "openbox", "fluxbox",
                "bspwm", "dwm", "qtile", "awesome", "xmonad", "xfwm4",
                "i3", "sway", "hyprland", "budgie-wm", "marco", "muffin",
                "berry", "herbstluftwm", "spectrwm", "leftwm",
            ] {
                if s.lines().any(|l| l.trim() == *name) {
                    return name.to_string();
                }
            }
        }
    }

    // Protocol fallback
    if env::var("WAYLAND_DISPLAY").is_ok() {
        return "Wayland".to_string();
    }
    "N/A".to_string()
}

pub fn locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C.UTF-8".to_string())
}

pub fn font() -> String {
    if let Ok(out) = Command::new("fc-match")
        .args(["-f", "%{family} %{size}pt", "monospace"])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() && s != "monospace" {
            return s;
        }
    }
    if let Ok(f) = env::var("KITTY_FONT_FAMILY") {
        return f;
    }
    if let Ok(f) = env::var("ALACRITTY_FONT_FAMILY") {
        return f;
    }
    "Default".to_string()
}

pub fn resolution() -> String {
    if let Ok(out) = Command::new("xrandr")
        .args(["--current"])
        .output()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if line.contains('*') {
                if let Some(res) = line.split_whitespace().nth(0) {
                    return res.to_string();
                }
            }
        }
    }
    if let Ok(out) = Command::new("wlr-randr").output() {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if line.contains("current") && line.contains('x') {
                return line.split_whitespace().nth(1).unwrap_or("").to_string();
            }
        }
    }
    "N/A".to_string()
}

pub fn cpu() -> String {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok();
    if let Some(ref info) = cpuinfo {
        for prefix in &["model name\t: ", "model name: ", "Processor\t: ", "Processor : ", "Hardware\t: ", "Hardware : "] {
            if let Some(name) = info.lines().find_map(|line| {
                line.strip_prefix(prefix).map(str::trim).map(str::to_string)
            }) {
                if !name.is_empty() {
                    return name;
                }
            }
        }
    }
    if let Ok(out) = Command::new("lscpu").output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            for line in s.lines() {
                if let Some(name) = line.strip_prefix("Model name:") {
                    let name = name.trim();
                    if !name.is_empty() {
                        return name.to_string();
                    }
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

fn pci_vendor_name(vendor: &str) -> &'static str {
    match vendor {
        "0x8086" => "Intel",
        "0x1002" | "0x1022" => "AMD",
        "0x10de" => "NVIDIA",
        "0x1ae0" => "Google",
        _ => "Unknown",
    }
}

fn pci_device_name(vendor: &str, device: &str) -> Option<&'static str> {
    Some(match (vendor, device) {
        // Intel
        ("0x8086", "0x8a5a") => "Iris Plus Graphics G4 (Ice Lake)",
        ("0x8086", "0x8a51") => "Iris Plus Graphics G7 (Ice Lake)",
        ("0x8086", "0x9b41") => "UHD Graphics (Comet Lake)",
        ("0x8086", "0x9bc4") => "UHD Graphics (Comet Lake)",
        ("0x8086", "0x3e9b") => "UHD Graphics 630 (Coffee Lake)",
        ("0x8086", "0x3ea0") => "UHD Graphics 630 (Coffee Lake)",
        ("0x8086", "0x3e92") => "HD Graphics 630 (Kaby Lake)",
        ("0x8086", "0x591b") => "HD Graphics 630 (Kaby Lake)",
        ("0x8086", "0x5917") => "HD Graphics 620 (Kaby Lake)",
        ("0x8086", "0x5916") => "HD Graphics 620 (Kaby Lake)",
        ("0x8086", "0x3185") => "UHD Graphics 605 (Gemini Lake)",
        ("0x8086", "0x22b1") => "HD Graphics 615 (Braswell)",
        ("0x8086", "0x0f31") => "HD Graphics (Bay Trail)",
        ("0x8086", "0x2a42") => "Integrated Graphics (Mobile 4)",
        ("0x8086", "0x46a6") => "Iris Xe Graphics (Alder Lake)",
        ("0x8086", "0x46a8") => "Iris Xe Graphics (Alder Lake)",
        ("0x8086", "0xa7a0") => "Iris Xe Graphics (Raptor Lake)",
        ("0x8086", "0xa7a1") => "UHD Graphics (Raptor Lake)",
        ("0x8086", "0x7d55") => "Iris Xe Graphics (Meteor Lake)",
        // AMD
        ("0x1002", _) | ("0x1022", _) => "AMD Radeon",
        // NVIDIA
        ("0x10de", "0x1f03") => "GeForce RTX 3070",
        ("0x10de", "0x1f04") => "GeForce RTX 3070 Ti",
        ("0x10de", "0x1e84") => "GeForce RTX 2080",
        ("0x10de", "0x1eb1") => "GeForce RTX 2060",
        ("0x10de", "0x1f82") => "GeForce RTX 3060",
        ("0x10de", "0x2484") => "GeForce RTX 4060",
        ("0x10de", "0x2684") => "GeForce RTX 5060",
        _ => return None,
    })
}

fn gpu_from_sysfs() -> Option<String> {
    let pci_root = "/sys/bus/pci/devices";
    for entry in std::fs::read_dir(pci_root).ok()? {
        let entry = entry.ok()?;
        let class_path = entry.path().join("class");
        let class = std::fs::read_to_string(class_path).ok()?;
        let class = class.trim();
        if class != "0x030000" && class != "0x030200" {
            continue;
        }
        let vendor = std::fs::read_to_string(entry.path().join("vendor")).ok()?;
        let device = std::fs::read_to_string(entry.path().join("device")).ok()?;
        let vendor = vendor.trim();
        let device = device.trim();
        let driver_path = entry.path().join("driver");
        let driver = std::fs::read_link(&driver_path)
            .ok()
            .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()));

        if let Some(name) = pci_device_name(vendor, device) {
            return Some(name.to_string());
        }

        let vendor_name = pci_vendor_name(vendor);
        return Some(match driver {
            Some(d) => format!("{vendor_name} GPU ({d})"),
            None => format!("{vendor_name} GPU"),
        });
    }
    None
}

pub fn gpu() -> String {
    if let Ok(out) = Command::new("lspci").output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            if let Some(line) = s.lines().find(|l| {
                l.contains("VGA") || l.contains("3D") || l.contains("Display") || l.contains("GPU")
            }) {
                if let Some((_, name)) = line.split_once(": ") {
                    let name = name.trim().to_string();
                    if !name.is_empty() {
                        return name;
                    }
                }
            }
        }
    }

    gpu_from_sysfs().unwrap_or_else(|| "Unknown GPU".to_string())
}

pub fn memory() -> String {
    let meminfo = match fs::read_to_string("/proc/meminfo") {
        Ok(m) => m,
        Err(_) => return "Unknown".to_string(),
    };

    let mut total_kb = 0.0_f64;
    let mut avail_kb = 0.0_f64;

    for line in meminfo.lines() {
        if let Some(v) = line.strip_prefix("MemTotal:") {
            total_kb = parse_kb(v);
        } else if let Some(v) = line.strip_prefix("MemAvailable:") {
            avail_kb = parse_kb(v);
        }
    }

    if total_kb <= 0.0 {
        return "Unknown".to_string();
    }

    let (total, avail, unit) = if total_kb > 1024.0 * 1024.0 {
        (total_kb / 1024.0 / 1024.0, avail_kb / 1024.0 / 1024.0, "GB")
    } else {
        (total_kb / 1024.0, avail_kb / 1024.0, "MB")
    };

    let used = ((total - avail) * 10.0).round() / 10.0;
    let total = (total * 10.0).round() / 10.0;
    let pct = ((used / total) * 1000.0).round() / 10.0;
    format!("{used}{unit} / {total}{unit} ({pct}%)")
}

fn parse_kb(s: &str) -> f64 {
    s.split("kB")
        .next()
        .unwrap_or(s)
        .trim()
        .parse()
        .unwrap_or(0.0)
}

pub fn disk() -> String {
    fn nz(s: &str) -> String {
        s.replace('T', "TB")
            .replace('G', "GB")
            .replace('M', "MB")
            .replace('K', "KB")
    }

    let mut parts: Vec<String> = Vec::new();

    // Build set of real device-backed mount points
    let real_mounts: HashSet<String> = fs::read_to_string("/proc/mounts")
        .ok()
        .map(|s| {
            s.lines()
                .filter_map(|line| {
                    let p: Vec<&str> = line.split_whitespace().collect();
                    if p.len() >= 2 && p[0].starts_with("/dev/") {
                        Some(p[1].to_string())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    // Check root plus other mount points (up to 3 total entries)
    let mount_points = ["/", "/boot", "/home", "/nix", "/new", "/var", "/.snapshots"];
    for mount in &mount_points {
        let is_root = mount == &"/";
        if !is_root && !real_mounts.contains(*mount) {
            continue;
        }
        if parts.len() >= 3 {
            break;
        }
        if let Ok(out) = Command::new("df")
            .args(["-h", "--output=used,size,pcent", mount])
            .output()
        {
            if let Ok(s) = String::from_utf8(out.stdout) {
                if let Some(line) = s.lines().nth(1) {
                    let cols: Vec<&str> = line.split_whitespace().collect();
                    if cols.len() >= 3 && cols[0] != "0" {
                        let entry = if is_root {
                            format!("{} / {} ({})", nz(cols[0]), nz(cols[1]), cols[2])
                        } else {
                            let label = mount.trim_start_matches('/');
                            format!("{} / {} ({}, {})", nz(cols[0]), nz(cols[1]), cols[2], label)
                        };
                        parts.push(entry);
                    }
                }
            }
        }
    }

    if parts.is_empty() {
        "N/A".to_string()
    } else {
        parts.join("  ")
    }
}

pub fn drive() -> String {
    fn nz(s: &str) -> String {
        s.replace('T', "TB")
            .replace('G', "GB")
            .replace('M', "MB")
            .replace('K', "KB")
    }

    if let Ok(out) = Command::new("lsblk").args(["-d", "-n", "-o", "model,size"]).output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            if let Some(line) = s.lines().next() {
                let line = line.trim();
                if !line.is_empty() {
                    if let Some((model, size)) = line.rsplit_once(' ') {
                        let model = model.trim();
                        if !model.is_empty() && !model.contains("name") {
                            return format!("{model} ({})", nz(size));
                        }
                    }
                }
            }
        }
    }
    "N/A".to_string()
}

pub fn swap() -> String {
    let meminfo = match fs::read_to_string("/proc/meminfo") {
        Ok(m) => m,
        Err(_) => return "N/A".to_string(),
    };

    let mut total_kb = 0.0_f64;
    let mut free_kb = 0.0_f64;

    for line in meminfo.lines() {
        if let Some(v) = line.strip_prefix("SwapTotal:") {
            total_kb = parse_kb(v);
        } else if let Some(v) = line.strip_prefix("SwapFree:") {
            free_kb = parse_kb(v);
        }
    }

    if total_kb == 0.0 {
        return "N/A".to_string();
    }

    let used_kb = total_kb - free_kb;
    let used_gb = used_kb / 1_048_576.0;
    let total_gb = total_kb / 1_048_576.0;
    let pct = (used_kb / total_kb * 100.0).round();
    format!("{:.1}GB / {:.1}GB ({:.0}%)", used_gb, total_gb, pct)
}

pub fn processes() -> String {
    fs::read_dir("/proc")
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name();
                    let s = name.to_string_lossy();
                    s.chars().all(|c| c.is_ascii_digit())
                })
                .count()
        })
        .map(|n| n.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

pub fn local_ip() -> String {
    if let Ok(out) = Command::new("hostname").args(["-I"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            if let Some(ip) = s.split_whitespace().next() {
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }
    "N/A".to_string()
}

fn has_pkg_manager(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .ok()
        .is_some_and(|o| o.status.success())
}

pub fn packages() -> String {
    // Check which package managers are available first, then try them
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let is_nixos = os_release.contains("ID=nixos");
    let is_arch = os_release.contains("ID=arch") || fs::metadata("/var/lib/pacman").is_ok();
    let is_debian = fs::metadata("/var/lib/dpkg").is_ok();
    let is_fedora = fs::metadata("/var/lib/rpm").is_ok();
    let is_alpine = fs::metadata("/etc/apk").is_ok();

    if is_nixos {
        let mut nix_parts: Vec<String> = Vec::new();

        // Filter matching fastfetch's isValidNixPkg: skip -doc/-man/-info/-dev/-bin,
        // skip nixos-system-nixos-*, and only count paths with a version number.
        let count_nix_pkgs = |profile: &str| -> Option<usize> {
            let out = Command::new("nix-store")
                .args(["-qR", profile])
                .output().ok()?;
            if !out.status.success() { return None; }
            let output = String::from_utf8_lossy(&out.stdout);
            let mut count = 0;
            for line in output.lines() {
                if line.is_empty() { continue; }
                let line = line.trim();
                // Must exist as a directory
                if !fs::metadata(line).map(|m| m.is_dir()).unwrap_or(false) {
                    continue;
                }
                // Extract package name after the store hash
                let name = match line.rsplit_once('/') {
                    Some((_, rest)) => rest,
                    None => continue,
                };
                // Skip nixos-system-nixos-*
                if name.starts_with("nixos-system-nixos-") {
                    continue;
                }
                // Skip -doc, -man, -info, -dev, -bin
                if name.ends_with("-doc")
                    || name.ends_with("-man")
                    || name.ends_with("-info")
                    || name.ends_with("-dev")
                    || name.ends_with("-bin")
                {
                    continue;
                }
                // Must contain a version number: \d+\.\d
                let has_version = {
                    let b = name.as_bytes();
                    let mut state = 0u8; // 0=start, 1=in-digits, 2=dot, 3=matched
                    for &c in b {
                        match state {
                            0 => { if c.is_ascii_digit() { state = 1; } }
                            1 => {
                                if c == b'.' { state = 2; }
                                else if !c.is_ascii_digit() { state = 0; }
                            }
                            2 => {
                                if c.is_ascii_digit() { state = 3; break; }
                                else { state = 0; }
                            }
                            _ => {}
                        }
                    }
                    state == 3
                };
                if !has_version {
                    continue;
                }
                count += 1;
            }
            Some(count)
        };

        if let Some(n) = count_nix_pkgs("/run/current-system") {
            nix_parts.push(format!("{n} (nix-system)"));
        }

        let user = env::var("USER").unwrap_or_default();
        let home_manager_profile = format!("/etc/profiles/per-user/{user}");
        if fs::metadata(&home_manager_profile).is_ok() {
            if let Some(n) = count_nix_pkgs(&home_manager_profile) {
                nix_parts.push(format!("{n} (nix-user)"));
            }
        } else if let Ok(out) = Command::new("nix-env").args(["-q"]).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout).lines().count();
                nix_parts.push(format!("{n} (nix-user)"));
            }
        }

        // Resolve default profile through its symlink chain to a store path
        let default_profiles = [
            "/nix/var/nix/profiles/default",
            "/nix/var/nix/profiles/per-user/root/profile",
        ];
        for &default_path in &default_profiles {
            let mut current: Option<std::path::PathBuf> = Some(default_path.into());
            let mut found = false;
            while let Some(ref p) = current {
                if p.starts_with("/nix/store/") {
                    if let Some(n) = count_nix_pkgs(&p.to_string_lossy()) {
                        nix_parts.push(format!("{n} (nix-default)"));
                        found = true;
                    }
                    break;
                }
                match fs::read_link(p) {
                    Ok(target) => {
                        let next = if target.is_absolute() {
                            target
                        } else if let Some(parent) = p.parent() {
                            parent.join(&target)
                        } else {
                            break;
                        };
                        current = Some(next);
                    }
                    Err(_) => break,
                }
            }
            if found {
                break;
            }
        }

        if !nix_parts.is_empty() {
            return nix_parts.join("  ");
        }
    }
    if is_arch && has_pkg_manager("pacman") {
        if let Ok(out) = Command::new("pacman").args(["-Q"]).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout).lines().count();
                return format!("{n} (pacman)");
            }
        }
    }
    if is_debian && has_pkg_manager("dpkg") {
        if let Ok(out) = Command::new("dpkg").args(["--list"]).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .filter(|l| l.starts_with("ii"))
                    .count();
                return format!("{n} (dpkg)");
            }
        }
    }
    if is_fedora && has_pkg_manager("rpm") {
        if let Ok(out) = Command::new("rpm").args(["-qa"]).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout).lines().count();
                return format!("{n} (rpm)");
            }
        }
    }
    if is_alpine && has_pkg_manager("apk") {
        if let Ok(out) = Command::new("apk").args(["info"]).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout).lines().count();
                return format!("{n} (apk)");
            }
        }
    }
    // Generic fallback: try everything
    for (cmd, args, filter) in &[
        ("pacman", &["-Q"] as &[&str], None as Option<fn(&str) -> bool>),
        ("dpkg", &["--list"], Some(|l: &str| l.starts_with("ii"))),
        ("rpm", &["-qa"], None),
        ("apk", &["info"], None),
        ("nix-store", &["-qR", "/run/current-system"], None),
        ("flatpak", &["list"], None),
        ("snap", &["list"], None),
    ] {
        if !has_pkg_manager(cmd) {
            continue;
        }
        if let Ok(out) = Command::new(cmd).args(*args).output() {
            if out.status.success() {
                let n = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .filter(|l| filter.map_or(true, |f| f(l)))
                    .count();
                return format!("{n} ({})", cmd.split('/').next().unwrap_or(cmd));
            }
        }
    }
    "N/A".to_string()
}

pub fn uptime() -> String {
    let secs: f64 = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_string))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let mins = (secs / 60.0).floor() as u64;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{days}d {}h", hours % 24)
    } else if hours > 0 {
        format!("{hours}h {}m", mins % 60)
    } else {
        format!("{mins}m")
    }
}

pub fn arch() -> String {
    if let Ok(a) = env::var("HOSTTYPE") {
        if !a.is_empty() {
            return a;
        }
    }
    if let Ok(a) = env::var("ARCH") {
        if !a.is_empty() {
            return a;
        }
    }
    if let Ok(out) = Command::new("uname").arg("-m").output() {
        if let Ok(s) = String::from_utf8(out.stdout) {
            let s = s.trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }
    if let Ok(info) = fs::read_to_string("/proc/cpuinfo") {
        if let Some(a) = info.lines().find_map(|l| {
            l.strip_prefix("CPU architecture: ").map(str::trim).map(str::to_string)
        }) {
            return a;
        }
    }
    "unknown".to_string()
}

pub fn init_system() -> String {
    if fs::metadata("/run/systemd/system").is_ok() {
        return "systemd".to_string();
    }
    if fs::metadata("/sbin/openrc").is_ok() {
        return "OpenRC".to_string();
    }
    if fs::metadata("/sbin/runit").is_ok() {
        return "runit".to_string();
    }
    fs::read_to_string("/proc/1/comm")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "N/A".to_string())
}
