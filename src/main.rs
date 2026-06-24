use std::io::IsTerminal;
use unicode_width::UnicodeWidthStr;

use clap::Parser;
use serde::Serialize;

mod config;
mod distro;
mod distro_styles;
mod info;
mod loader;
mod theme;
mod utils;

use config::{Config, DistroConfig};
use theme::Theme;

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

    /// Hide logo, show info only (bare output)
    #[arg(long, short = 'b')]
    no_logo: bool,

    /// When to colorize output [auto, always, never]
    #[arg(long, default_value = "auto")]
    color: String,
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
    de: String,
    wm: String,
    uptime: String,
    locale: String,
    cpu: String,
    gpu: String,
    memory: String,
    swap: String,
    drive: String,
    disk: String,
    processes: String,
    local_ip: String,
    resolution: String,
    font: String,
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
        de: info::de(),
        wm: info::wm(),
        uptime: info::uptime(),
        locale: info::locale(),
        cpu: info::cpu(),
        gpu: info::gpu(),
        memory: info::memory(),
        swap: info::swap(),
        drive: info::drive(),
        disk: info::disk(),
        processes: info::processes(),
        local_ip: info::local_ip(),
        resolution: info::resolution(),
        font: info::font(),
    }
}

fn info_rows<'a>(theme: &'a Theme, info: &'a SystemInfo) -> Vec<(String, String)> {
    let raw: Vec<(&str, &str)> = vec![
        ("OS:", &info.os),
        ("Host:", &info.hostname),
        ("Kernel:", &info.kernel),
        ("Arch:", &info.arch),
        ("Uptime:", &info.uptime),
        ("Init:", &info.init),
        ("Pkgs:", &info.packages),
        ("User:", &info.user_host),
        ("Shell:", &info.shell),
        ("Term:", &info.terminal),
        ("DE:", &info.de),
        ("WM:", &info.wm),
        ("Locale:", &info.locale),
        ("CPU:", &info.cpu),
        ("GPU:", &info.gpu),
        ("Memory:", &info.memory),
        ("Swap:", &info.swap),
        ("Drive:", &info.drive),
        ("Disk:", &info.disk),
        ("Procs:", &info.processes),
        ("IP:", &info.local_ip),
        ("Res:", &info.resolution),
        ("Font:", &info.font),
    ];

    raw.into_iter()
        .filter(|(_, v)| *v != "N/A")
        .map(|(l, v)| (theme.label(l), theme.value(v)))
        .collect()
}

fn join_columns(mut left: Vec<String>, mut right: Vec<String>, gap: usize) -> Vec<String> {
    let height = std::cmp::max(left.len(), right.len());
    let left_width = left
        .iter()
        .map(|l| utils::strip_ansi(l).width())
        .max()
        .unwrap_or(0);

    left.resize(height, String::new());
    right.resize(height, String::new());

    let spacer = " ".repeat(gap);
    let mut output = Vec::with_capacity(height);

    for i in 0..height {
        let visible = utils::strip_ansi(&left[i]).width();
        let padding = left_width.saturating_sub(visible);
        output.push(format!("{}{}{}", left[i], " ".repeat(padding), spacer));
        output.last_mut().unwrap().push_str(&right[i]);
    }

    output
}

fn compose(config: &Config, distro: &str, theme: &Theme, info: &SystemInfo) -> Vec<String> {
    let logo_key = if config.distro.contains_key(distro) {
        distro.to_string()
    } else {
        distro_styles::logo_family(distro).to_string()
    };

    let entry = config
        .distro
        .get(&logo_key)
        .or_else(|| config.distro.get("unknown"))
        .expect("unknown distro must exist in config")
        .clone();

    let entry = resolve_inheritance(config, entry);
    let logo_lines = theme.render_logo(&entry.logo, distro);

    let rows = info_rows(theme, info);
    let label_width = rows.iter().map(|(l, _)| utils::strip_ansi(l).width()).max().unwrap_or(0);

    let info_lines: Vec<String> = rows.iter().map(|(label, value)| {
        let visible = utils::strip_ansi(label).width();
        let pad = label_width - visible;
        format!("{}{} {}", label, " ".repeat(pad), value)
    }).collect();

    join_columns(logo_lines, info_lines, 3)
}

fn compose_no_logo(theme: &Theme, info: &SystemInfo) -> Vec<String> {
    let rows = info_rows(theme, info);
    let label_width = rows.iter().map(|(l, _)| utils::strip_ansi(l).width()).max().unwrap_or(0);

    rows.iter().map(|(label, value)| {
        let visible = utils::strip_ansi(label).width();
        let pad = label_width - visible;
        format!("{}{} {}", label, " ".repeat(pad), value)
    }).collect()
}

fn main() {
    let args = Args::parse();
    let config = loader::load_config();

    if args.list {
        let mut keys: Vec<&String> = config.distro.keys().collect();
        keys.sort();
        for k in keys {
            if k != "unknown" {
                println!("{k}");
            }
        }
        return;
    }

    let distro = args.distro.unwrap_or_else(distro::distro);
    let info = collect_info();
    let no_logo = args.no_logo;

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
            let mut out = String::new();
            if no_logo {
                for line in compose_no_logo(&theme, &info) {
                    out.push_str(&line);
                    out.push('\n');
                }
            } else {
                for line in compose(&config, &distro, &theme, &info) {
                    out.push_str(&line);
                    out.push('\n');
                }
            }
            out
        }
    };

    let output = match args.color.as_str() {
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
        None => print!("{output}"),
    }
}
