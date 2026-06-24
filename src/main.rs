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
    #[arg(long)]
    no_logo: bool,
}

struct Panel {
    title: String,
    items: Vec<(String, String)>,
}

impl Panel {
    fn new(title: String, items: Vec<(String, String)>) -> Self {
        Self { title, items }
    }

    fn compute_width(&self, label_width: usize) -> usize {
        let inner = self
            .items
            .iter()
            .map(|(l, v)| {
                let label = utils::strip_ansi(l).width();
                let pad = label_width.saturating_sub(label);
                label + pad + 1 + utils::strip_ansi(v).width()
            })
            .max()
            .unwrap_or(0);

        let title_width = self.title.width();
        std::cmp::max(inner, title_width) + 4
    }

    fn label_width(&self) -> usize {
        self.items
            .iter()
            .map(|(l, _)| utils::strip_ansi(l).width())
            .max()
            .unwrap_or(0)
    }

    fn render_at(&self, width: usize, label_width: usize) -> Vec<String> {
        let inner = width.saturating_sub(4);

        let mut lines = Vec::new();
        lines.push(format!("┌{}┐", "─".repeat(width - 2)));
        lines.push(format!("│ {:<inner$} │", self.title, inner = inner));
        lines.push(format!("├{}┤", "─".repeat(width - 2)));

        for (label, value) in &self.items {
            let visible_label = utils::strip_ansi(label).width();
            let pad = label_width.saturating_sub(visible_label);
            let item = format!("{}{} {}", label, " ".repeat(pad), value);
            let visible = utils::strip_ansi(&item).width();
            let padding = inner.saturating_sub(visible);
            lines.push(format!("│ {} │", item + &" ".repeat(padding)));
        }

        lines.push(format!("└{}┘", "─".repeat(width - 2)));
        lines
    }
}

fn render_panels(panels: &[Panel]) -> Vec<String> {
    let label_width = panels.iter().map(Panel::label_width).max().unwrap_or(0);
    let width = panels
        .iter()
        .map(|p| p.compute_width(label_width))
        .max()
        .unwrap_or(0);

    let mut output = Vec::new();
    for (i, panel) in panels.iter().enumerate() {
        if i > 0 {
            output.push(String::new());
        }
        output.extend(panel.render_at(width, label_width));
    }
    output
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
    de_wm: String,
    uptime: String,
    locale: String,
    cpu: String,
    gpu: String,
    memory: String,
    disk: String,
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
        de_wm: info::de_wm(),
        uptime: info::uptime(),
        locale: info::locale(),
        cpu: info::cpu(),
        gpu: info::gpu(),
        memory: info::memory(),
        disk: info::disk(),
        resolution: info::resolution(),
        font: info::font(),
    }
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

    let row = |label: &str, value: &str| {
        (theme.label(label), theme.value(value))
    };

    let system_panel = Panel::new(
        "System".into(),
        vec![
            row("OS:", &info.os),
            row("Host:", &info.hostname),
            row("Kernel:", &info.kernel),
            row("Arch:", &info.arch),
            row("Init:", &info.init),
            row("Pkgs:", &info.packages),
        ],
    );

    let session_panel = Panel::new(
        "Session".into(),
        vec![
            row("User:", &info.user_host),
            row("Shell:", &info.shell),
            row("Term:", &info.terminal),
            row("DE/WM:", &info.de_wm),
            row("Uptime:", &info.uptime),
            row("Locale:", &info.locale),
        ],
    );

    let hardware_panel = Panel::new(
        "Hardware".into(),
        vec![
            row("CPU:", &info.cpu),
            row("GPU:", &info.gpu),
            row("Memory:", &info.memory),
            row("Disk:", &info.disk),
        ],
    );

    let display_panel = Panel::new(
        "Display".into(),
        vec![
            row("Res:", &info.resolution),
            row("Font:", &info.font),
        ],
    );

    let info_col = join_columns(
        render_panels(&[system_panel, session_panel]),
        render_panels(&[hardware_panel, display_panel]),
        3,
    );

    join_columns(logo_lines, info_col, 3)
}

fn compose_no_logo(theme: &Theme, info: &SystemInfo) -> Vec<String> {
    let row = |label: &str, value: &str| {
        format!("{} {}", theme.label(label), theme.value(value))
    };

    vec![
        row("OS:", &info.os),
        row("Host:", &info.hostname),
        row("Kernel:", &info.kernel),
        row("Arch:", &info.arch),
        row("Init:", &info.init),
        row("Pkgs:", &info.packages),
        row("User:", &info.user_host),
        row("Shell:", &info.shell),
        row("Term:", &info.terminal),
        row("DE/WM:", &info.de_wm),
        row("Uptime:", &info.uptime),
        row("Locale:", &info.locale),
        row("CPU:", &info.cpu),
        row("GPU:", &info.gpu),
        row("Memory:", &info.memory),
        row("Disk:", &info.disk),
        row("Res:", &info.resolution),
        row("Font:", &info.font),
    ]
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
            // resolve theme key the same way compose resolves the logo key
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

    match &args.save {
        Some(path) => std::fs::write(path, &output).unwrap_or_else(|e| {
            eprintln!("error: failed to write to '{path}': {e}");
            std::process::exit(1);
        }),
        None => print!("{output}"),
    }
}
