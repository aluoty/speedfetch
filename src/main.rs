use std::io::{IsTerminal, Write, BufWriter};
use std::path::PathBuf;

use unicode_width::UnicodeWidthStr;

use clap::Parser;
use serde::Serialize;

mod config;
mod distro;
mod distro_styles;
mod info;
mod loader;
mod theme;
mod user_config;
mod utils;

use config::{Config, DistroConfig};
use theme::Theme;
use user_config::{LogoMode, PercentType};

#[derive(Parser)]
#[command(name = "speedfetch", version, about = "A pretty system info fetcher")]
struct Args {
    /// Distro to display (overrides auto-detection)
    #[arg(short, long)]
    distro: Option<String>,

    /// List available distro presets
    #[arg(long)]
    list: bool,

    /// Output format (json, toml)
    #[arg(long = "type", value_name = "FORMAT")]
    output_format: Option<String>,

    /// Save output to file
    #[arg(long, value_name = "FILE")]
    save: Option<String>,

    /// Logo mode: small, regular, none
    #[arg(long, value_name = "TYPE")]
    logo: Option<LogoMode>,

    /// Hide logo, show info only (alias for --logo none)
    #[arg(long, short = 'b')]
    no_logo: bool,

    /// When to colorize output [auto, always, never]
    #[arg(long)]
    color: Option<String>,

    /// Show only specific fields (comma-separated: os,kernel,cpu,memory,...)
    #[arg(long, value_name = "FIELDS")]
    show: Option<String>,

    /// Path to custom config file
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn resolve_inheritance(config: &Config, entry: DistroConfig) -> DistroConfig {
    if entry.inherits.is_empty() {
        return entry;
    }

    let child_theme = entry.theme;
    let child_logo = entry.logo;
    let parent_key = entry.inherits.clone();

    let mut resolved = config
        .distro
        .get(&parent_key)
        .map(|p| resolve_inheritance(config, p.clone()))
        .unwrap_or_else(|| DistroConfig {
            inherits: String::new(),
            logo: child_logo.clone(),
            small_logo: Vec::new(),
            theme: child_theme.clone(),
        });

    if !child_logo.is_empty() {
        resolved.logo = child_logo;
    }
    resolved.theme = child_theme;
    resolved.inherits = String::new();
    resolved
}

#[derive(Serialize)]
struct SystemInfo {
    os: String,
    hostname: String,
    kernel: String,
    arch: String,
    init: String,
    packages: String,
    user_host: String,
    shell: String,
    terminal: String,
    terminal_size: String,
    de: String,
    wm: String,
    wm_theme: String,
    uptime: String,
    datetime: String,
    locale: String,
    cpu: String,
    cpu_usage: String,
    gpu: String,
    memory: String,
    swap: String,
    battery: String,
    drive: String,
    disk: String,
    processes: String,
    local_ip: String,
    public_ip: String,
    resolution: String,
    font: String,
    bios: String,
    board: String,
    wifi: String,
}

fn collect_info() -> SystemInfo {
    SystemInfo {
        os: info::os(),
        hostname: info::hostname(),
        kernel: info::kernel(),
        arch: info::arch(),
        init: info::init_system(),
        packages: info::packages(),
        user_host: info::user_host(),
        shell: info::shell(),
        terminal: info::terminal(),
        terminal_size: info::terminal_size(),
        de: info::de(),
        wm: info::wm(),
        wm_theme: info::wm_theme(),
        uptime: info::uptime(),
        datetime: info::datetime(),
        locale: info::locale(),
        cpu: info::cpu(),
        cpu_usage: info::cpu_usage(),
        gpu: info::gpu(),
        memory: info::memory(),
        swap: info::swap(),
        battery: info::battery(),
        drive: info::drive(),
        disk: info::disk(),
        processes: info::processes(),
        local_ip: info::local_ip(),
        public_ip: info::public_ip(),
        resolution: info::resolution(),
        font: info::font(),
        bios: info::bios(),
        board: info::board(),
        wifi: info::wifi(),
    }
}

struct RowDef {
    label: &'static str,
    key: &'static str,
}

fn all_rows() -> Vec<RowDef> {
    vec![
        RowDef { label: "OS:", key: "os" },
        RowDef { label: "Host:", key: "host" },
        RowDef { label: "Kernel:", key: "kernel" },
        RowDef { label: "Uptime:", key: "uptime" },
        RowDef { label: "Packages:", key: "pkgs" },
        RowDef { label: "Shell:", key: "shell" },
        RowDef { label: "Resolution:", key: "res" },
        RowDef { label: "DE:", key: "de" },
        RowDef { label: "WM:", key: "wm" },
        RowDef { label: "Terminal:", key: "term" },
        RowDef { label: "CPU:", key: "cpu" },
        RowDef { label: "GPU:", key: "gpu" },
        RowDef { label: "Memory:", key: "memory" },
        RowDef { label: "Swap:", key: "swap" },
        RowDef { label: "Disk:", key: "disk" },
        RowDef { label: "Drive:", key: "drive" },
        RowDef { label: "Battery:", key: "battery" },
        RowDef { label: "Font:", key: "font" },
        RowDef { label: "Init:", key: "init" },
        RowDef { label: "Local IP:", key: "ip" },
        RowDef { label: "Date/Time:", key: "datetime" },
        RowDef { label: "Procs:", key: "procs" },
        RowDef { label: "Arch:", key: "arch" },
    ]
}

fn resolve_value<'a>(info: &'a SystemInfo, key: &str) -> Option<&'a str> {
    match key {
        "os" => Some(&info.os),
        "host" => Some(&info.hostname),
        "kernel" => Some(&info.kernel),
        "uptime" => Some(&info.uptime),
        "pkgs" => Some(&info.packages),
        "shell" => Some(&info.shell),
        "res" => Some(&info.resolution),
        "de" => Some(&info.de),
        "wm" => Some(&info.wm),
        "wm-theme" => Some(&info.wm_theme),
        "term" => Some(&info.terminal),
        "term-size" => Some(&info.terminal_size),
        "cpu" => Some(&info.cpu),
        "cpu-usage" => Some(&info.cpu_usage),
        "gpu" => Some(&info.gpu),
        "memory" => Some(&info.memory),
        "swap" => Some(&info.swap),
        "disk" => Some(&info.disk),
        "drive" => Some(&info.drive),
        "battery" => Some(&info.battery),
        "font" => Some(&info.font),
        "init" => Some(&info.init),
        "user" => Some(&info.user_host),
        "ip" => Some(&info.local_ip),
        "public-ip" => Some(&info.public_ip),
        "wifi" => Some(&info.wifi),
        "datetime" => Some(&info.datetime),
        "locale" => Some(&info.locale),
        "procs" => Some(&info.processes),
        "arch" => Some(&info.arch),
        "bios" => Some(&info.bios),
        "board" => Some(&info.board),
        _ => None,
    }
}

fn make_bar(value: f64, max: f64, cfg: &user_config::BarConfig) -> String {
    let width = cfg.width.max(1);
    let ratio = (value / max).clamp(0.0, 1.0);
    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let mut bar = String::with_capacity(width + 4);
    if let Some(ref left) = cfg.border_left {
        bar.push_str(left);
    }
    bar.extend(std::iter::repeat(cfg.char_elapsed).take(filled));
    bar.extend(std::iter::repeat(cfg.char_total).take(empty));
    if let Some(ref right) = cfg.border_right {
        bar.push_str(right);
    }
    bar
}

fn color_for_percent(value: f64, cfg: &user_config::PercentConfig) -> Option<String> {
    let green = cfg.color_green.as_deref().unwrap_or("green");
    let yellow = cfg.color_yellow.as_deref().unwrap_or("yellow");
    let red = cfg.color_red.as_deref().unwrap_or("red");

    if value < 50.0 {
        Some(map_color(green))
    } else if value < 80.0 {
        Some(map_color(yellow))
    } else {
        Some(map_color(red))
    }
}

fn map_color(s: &str) -> String {
    match s {
        "black" => "\x1b[30m".to_string(),
        "red" => "\x1b[31m".to_string(),
        "green" => "\x1b[32m".to_string(),
        "yellow" => "\x1b[33m".to_string(),
        "blue" => "\x1b[34m".to_string(),
        "purple" | "magenta" => "\x1b[35m".to_string(),
        "cyan" => "\x1b[36m".to_string(),
        "white" => "\x1b[37m".to_string(),
        "gray" | "grey" | "dim" => "\x1b[90m".to_string(),
        "bright_red" => "\x1b[91m".to_string(),
        "bright_green" => "\x1b[92m".to_string(),
        "bright_yellow" => "\x1b[93m".to_string(),
        "bright_blue" => "\x1b[94m".to_string(),
        "bright_magenta" => "\x1b[95m".to_string(),
        "bright_cyan" => "\x1b[96m".to_string(),
        "bright_white" => "\x1b[97m".to_string(),
        "orange" => "\x1b[38;5;208m".to_string(),
        "pink" => "\x1b[38;5;205m".to_string(),
        "lavender" => "\x1b[38;5;141m".to_string(),
        "teal" => "\x1b[38;5;30m".to_string(),
        "lime" => "\x1b[38;5;112m".to_string(),
        "gold" => "\x1b[38;5;220m".to_string(),
        "brown" => "\x1b[38;5;130m".to_string(),
        _ => String::new(),
    }
}

fn format_value_with_options(
    raw: &str,
    key: &str,
    theme: &Theme,
    cfg: &user_config::UserConfig,
) -> (String, String) {
    let reset = "\x1b[0m";

    let value_color = cfg.color.values.as_ref()
        .map(|c| map_color(c))
        .unwrap_or_else(|| theme.value_color.clone());

    let display_val = match key {
        "memory" => {
            if let Some(pct) = extract_pct(raw) {
                let bar = make_bar(pct, 100.0, &cfg.bar);
                if cfg.percent.percent_type == PercentType::Bar {
                    format!("{bar}")
                } else if cfg.percent.percent_type == PercentType::Both {
                    format!("{bar} {raw}")
                } else {
                    raw.to_string()
                }
            } else {
                raw.to_string()
            }
        }
        "swap" | "battery" | "cpu-usage" => {
            if let Some(pct) = extract_pct(raw) {
                match cfg.percent.percent_type {
                    PercentType::Bar => make_bar(pct, 100.0, &cfg.bar),
                    PercentType::Both => {
                        let bar = make_bar(pct, 100.0, &cfg.bar);
                        format!("{bar} {raw}")
                    }
                    PercentType::Colored => {
                        if let Some(color) = color_for_percent(pct, &cfg.percent) {
                            format!("{color}{raw}{reset}")
                        } else {
                            raw.to_string()
                        }
                    }
                    PercentType::Number => raw.to_string(),
                }
            } else {
                raw.to_string()
            }
        }
        _ => raw.to_string(),
    };

    (display_val, value_color)
}

fn extract_pct(s: &str) -> Option<f64> {
    let start = s.rfind('(').unwrap_or(0);
    let end = s.rfind(')')?;
    let inner = &s[start + 1..end];
    let num_str = inner.trim_end_matches('%');
    num_str.parse::<f64>().ok()
}

fn info_rows(
    theme: &Theme,
    info: &SystemInfo,
    show: &[&str],
    cfg: &user_config::UserConfig,
) -> Vec<(String, String)> {
    let rows = all_rows();
    let reset = "\x1b[0m";

    let result: Vec<(String, String)> = rows
        .into_iter()
        .filter_map(|row| {
            let val = resolve_value(info, row.key)?;
            if val == "N/A" {
                return None;
            }
            if !show.is_empty() && !show.iter().any(|s| s.eq_ignore_ascii_case(row.key)) {
                return None;
            }

            let display_val = raw_value(info, row.key);

            let key_str = match cfg.key.key_type {
                user_config::KeyType::None => String::new(),
                _ => {
                    let label_color = cfg.color.keys.as_ref()
                        .map(|c| map_color(c))
                        .unwrap_or_else(|| theme.label_color.clone());

                    let label_text = match cfg.key.key_type {
                        user_config::KeyType::Icon => {
                            short_key(row.key).to_string()
                        }
                        _ => row.label.to_string(),
                    };

                    if label_color.is_empty() {
                        label_text
                    } else {
                        format!("{label_color}{label_text}{reset}")
                    }
                }
            };

            let (formatted_val, _val_color) = format_value_with_options(display_val, row.key, theme, cfg);

            Some((key_str, formatted_val))
        })
        .collect();

    result
}

fn raw_value<'a>(info: &'a SystemInfo, key: &str) -> &'a str {
    resolve_value(info, key).unwrap_or("N/A")
}

fn short_key(key: &str) -> &'static str {
    match key {
        "os" => " \u{f17c} ",
        "host" => " \u{f330} ",
        "kernel" => " \u{e73f} ",
        "uptime" => " \u{f017} ",
        "pkgs" => " \u{f187} ",
        "shell" => " \u{e795} ",
        "res" => " \u{f03e} ",
        "de" => " \u{e710} ",
        "wm" => " \u{f2d2} ",
        "term" => " \u{e795} ",
        "term-size" => " \u{f03e} ",
        "cpu" => " \u{f2db} ",
        "cpu-usage" => " \u{f2db} ",
        "gpu" => " \u{e99b} ",
        "memory" => " \u{f0e4} ",
        "swap" => " \u{f0e4} ",
        "disk" => " \u{f0a0} ",
        "drive" => " \u{f0a0} ",
        "battery" => " \u{f242} ",
        "font" => " \u{f031} ",
        "init" => " \u{e703} ",
        "user" => " \u{f007} ",
        "ip" => " \u{f0ac} ",
        "public-ip" => " \u{f0ac} ",
        "wifi" => " \u{f1eb} ",
        "datetime" => " \u{f017} ",
        "locale" => " \u{f0e6} ",
        "procs" => " \u{f544} ",
        "arch" => " \u{f120} ",
        "bios" => " \u{f2db} ",
        "board" => " \u{f2db} ",
        _ => " \u{f059} ",
    }
}

fn join_columns(left: &[String], right: &[String], gap: usize) -> Vec<String> {
    let height = std::cmp::max(left.len(), right.len());
    let left_width = left
        .iter()
        .map(|l| utils::strip_ansi(l).width())
        .max()
        .unwrap_or(0);

    let spacer = " ".repeat(gap);
    let mut output = Vec::with_capacity(height);

    for i in 0..height {
        let l = left.get(i).map(|s| s.as_str()).unwrap_or("");
        let r = right.get(i).map(|s| s.as_str()).unwrap_or("");
        let visible = utils::strip_ansi(l).width();
        let padding = left_width.saturating_sub(visible);
        let mut line = String::with_capacity(left_width + gap + 64);
        line.push_str(l);
        line.extend(std::iter::repeat(' ').take(padding));
        line.push_str(&spacer);
        line.push_str(r);
        output.push(line);
    }

    output
}

fn get_logo_lines(config: &Config, distro: &str, mode: LogoMode) -> Vec<String> {
    if mode == LogoMode::None {
        return Vec::new();
    }

    let logo_key = if config.distro.contains_key(distro) {
        distro.to_string()
    } else {
        distro_styles::logo_family(distro).to_string()
    };

    let entry = config
        .distro
        .get(&logo_key)
        .or_else(|| config.distro.get("unknown"))
        .expect("unknown distro must exist in config");

    let entry = resolve_inheritance(config, entry.clone());

    match mode {
        LogoMode::Small => {
            if !entry.small_logo.is_empty() {
                entry.small_logo
            } else {
                let total = entry.logo.len();
                let half = (total / 2).max(1);
                entry.logo[total - half..].to_vec()
            }
        }
        LogoMode::Regular => entry.logo,
        LogoMode::None => unreachable!(),
    }
}

fn compose(
    config: &Config,
    distro: &str,
    theme: &Theme,
    info: &SystemInfo,
    show: &[&str],
    logo_mode: LogoMode,
    cfg: &user_config::UserConfig,
) -> Vec<String> {
    let logo_lines = get_logo_lines(config, distro, logo_mode);
    let rows = info_rows(theme, info, show, cfg);

    let key_width = cfg.key.width.unwrap_or_else(|| {
        rows.iter().map(|(l, _)| utils::strip_ansi(l).width()).max().unwrap_or(0)
    });

    let sep = &cfg.separator;
    let key_pad = cfg.key.padding_left;

    let info_lines: Vec<String> = rows.iter().map(|(label, value)| {
        let visible = utils::strip_ansi(label).width();
        let pad = key_width.saturating_sub(visible);
        let mut line = String::with_capacity(key_pad + key_width + sep.len() + value.len() + 16);
        line.extend(std::iter::repeat(' ').take(key_pad));
        line.push_str(label);
        line.extend(std::iter::repeat(' ').take(pad));
        line.push_str(sep);
        line.push_str(value);
        line
    }).collect();

    let mut info_lines = info_lines;

    if cfg.layout.boxes {
        info_lines = wrap_box(
            &info_lines,
            if cfg.layout.title {
                Some(info.user_host.as_str())
            } else {
                None
            },
            cfg.layout.separator_line,
            &cfg.layout.padding,
        );
    }

    if logo_lines.is_empty() {
        let mut padded: Vec<String> = Vec::with_capacity(info_lines.len() + cfg.logo.padding.top);
        for _ in 0..cfg.logo.padding.top {
            padded.push(String::new());
        }
        padded.extend(info_lines);
        padded
    } else {
        let rendered = theme.render_logo(&logo_lines, distro);
        let mut logo_padded: Vec<String> = Vec::with_capacity(rendered.len() + cfg.logo.padding.top);
        for _ in 0..cfg.logo.padding.top {
            logo_padded.push(String::new());
        }
        logo_padded.extend(rendered);
        join_columns(&logo_padded, &info_lines, 3)
    }
}

fn wrap_box(
    lines: &[String],
    title: Option<&str>,
    separator_line: bool,
    padding: &user_config::LayoutPadding,
) -> Vec<String> {
    let visible_widths: Vec<usize> = lines.iter()
        .map(|l| utils::strip_ansi(l).width())
        .collect();

    let content_width = visible_widths.iter().copied().max().unwrap_or(0);

    let title_str = title.unwrap_or("");
    let title_display_width = unicode_width::UnicodeWidthStr::width(title_str);
    let title_line_width = if title_str.is_empty() { 0 } else { title_display_width + 1 }; // +1 for trailing space

    let inner_width = content_width.max(title_line_width);

    let left_pad = " ".repeat(padding.left);
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 4 + padding.top + padding.bottom);

    for _ in 0..padding.top {
        out.push(String::new());
    }

    // Top border: ╭─ title ──────────────────╮
    let top_line = {
        let after_title = inner_width.saturating_sub(title_line_width);
        if title_str.is_empty() {
            format!("╭{}╮", "─".repeat(inner_width + padding.left + padding.right))
        } else {
            format!(
                "╭─{} {}─{}╮",
                title_str,
                "",
                "─".repeat(after_title + padding.right - 1),
            )
        }
    };
    out.push(top_line);

    // Optional separator line after title
    if separator_line && !title_str.is_empty() {
        let sep_line = format!(
            "├{}┤",
            "─".repeat(inner_width + padding.left + padding.right),
        );
        out.push(sep_line);
    }

    // Content lines
    for (i, line) in lines.iter().enumerate() {
        let vis = visible_widths[i];
        let pad_right = inner_width.saturating_sub(vis) + padding.right;
        out.push(format!(
            "│{}{}{}│",
            left_pad,
            line,
            " ".repeat(pad_right),
        ));
    }

    // Bottom border
    let bot_line = format!(
        "╰{}╯",
        "─".repeat(inner_width + padding.left + padding.right),
    );
    out.push(bot_line);

    for _ in 0..padding.bottom {
        out.push(String::new());
    }

    out
}

fn main() {
    let args = Args::parse();
    let (config, user_cfg) = loader::load_config(args.config.as_deref());

    if args.list {
        let mut keys: Vec<&String> = config.distro.keys().collect();
        keys.sort();
        let mut out = BufWriter::new(std::io::stdout());
        for k in keys {
            if k != "unknown" {
                writeln!(out, "{k}").ok();
            }
        }
        return;
    }

    let distro = args.distro.unwrap_or_else(distro::distro);

    let logo_mode = if args.no_logo {
        LogoMode::None
    } else {
        args.logo.unwrap_or(user_cfg.logo.mode)
    };

    let color_mode = args.color.as_deref().unwrap_or("auto");

    let show: Vec<&str> = if let Some(ref s) = args.show {
        s.split(',').map(|s| s.trim()).collect()
    } else if !user_cfg.show.is_empty() {
        user_cfg.show.iter().map(|s| s.as_str()).collect()
    } else {
        Vec::new()
    };

    let info = collect_info();

    let output = match args.output_format.as_deref() {
        Some("json") => serde_json::to_string_pretty(&info).unwrap(),
        Some("toml") => toml::to_string(&info).unwrap(),
        Some(f) => {
            eprintln!("error: unknown output format '{f}' (use json or toml)");
            std::process::exit(1);
        }
        None => {
            let registry = theme::ThemeRegistry::from(&config);
            let theme_key = if config.distro.contains_key(&distro) {
                distro.as_str()
            } else {
                distro_styles::logo_family(&distro)
            };
            let theme = registry.get(theme_key);
            let lines = compose(&config, &distro, &theme, &info, &show, logo_mode, &user_cfg);
            let mut buf = String::with_capacity(lines.iter().map(|l| l.len() + 1).sum::<usize>());
            for line in lines {
                buf.push_str(&line);
                buf.push('\n');
            }
            buf
        }
    };

    let output = match color_mode {
        "never" => utils::strip_ansi(&output),
        "always" => output,
        _ if !std::io::stdout().is_terminal() => utils::strip_ansi(&output),
        _ => output,
    };

    match &args.save {
        Some(path) => std::fs::write(path, &output).unwrap_or_else(|e| {
            eprintln!("error: failed to write to '{path}': {e}");
            std::process::exit(1);
        }),
        None => {
            let mut out = BufWriter::new(std::io::stdout());
            out.write_all(output.as_bytes()).ok();
        }
    }
}
