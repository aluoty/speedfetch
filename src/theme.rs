use crate::config::ThemeConfig;
use crate::distro_styles;

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
pub struct Theme {
    pub label_color: String,
    pub value_color: String,
    pub reset: String,
}

pub struct ThemeRegistry {
    themes: std::collections::HashMap<String, Theme>,
}

impl ThemeRegistry {
    pub fn from(config: &crate::config::Config) -> Self {
        let mut themes = std::collections::HashMap::new();

        for (name, distro) in &config.distro {
            themes.insert(name.clone(), Theme::from(&distro.theme));
        }

        Self { themes }
    }

    pub fn get(&self, distro: &str) -> Theme {
        self.themes
            .get(distro)
            .cloned()
            .or_else(|| self.themes.get("unknown").cloned())
            .unwrap_or_else(Theme::plain)
    }
}

impl Theme {
    pub fn from(cfg: &ThemeConfig) -> Self {
        Self {
            label_color: map(&cfg.label),
            value_color: map(&cfg.value),
            reset: "\x1b[0m".to_string(),
        }
    }

    fn plain() -> Self {
        Self {
            label_color: String::new(),
            value_color: String::new(),
            reset: "\x1b[0m".to_string(),
        }
    }

    pub fn render_logo(&self, lines: &[String], distro: &str) -> Vec<String> {
        let style = distro_styles::distro_style(distro);
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| self.gradient_text(line, style, i))
            .collect()
    }

    fn gradient_text(&self, text: &str, style: distro_styles::DistroStyle, line_index: usize) -> String {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        if len == 0 {
            return String::new();
        }

        let len_f = len as f32;
        let line_shift = line_index as f32 * 0.06;
        let mut out = String::with_capacity(len * 18);

        let start = rgb_tuple(style.start);
        let mid = rgb_tuple(style.mid);
        let end = rgb_tuple(style.end);

        for (i, ch) in chars.iter().enumerate() {
            let t = i as f32 / len_f + line_shift;
            let color = sample_gradient(start, mid, end, t);
            write_ansi(&mut out, color);
            out.push(*ch);
        }

        out.push_str(&self.reset);
        out
    }

    pub fn label(&self, text: &str) -> String {
        colorize(&self.label_color, text, &self.reset)
    }

    pub fn value(&self, text: &str) -> String {
        colorize(&self.value_color, text, &self.reset)
    }
}

fn sample_gradient(start: Rgb, mid: Rgb, end: Rgb, t: f32) -> Rgb {
    let t = smoothstep(t);
    if t < 0.5 {
        mix(start, mid, t * 2.0)
    } else {
        mix(mid, end, (t - 0.5) * 2.0)
    }
}

fn rgb_tuple(t: (u8, u8, u8)) -> Rgb {
    Rgb {
        r: t.0,
        g: t.1,
        b: t.2,
    }
}

fn write_ansi(out: &mut String, color: Rgb) {
    use std::fmt::Write;
    let _ = write!(out, "\x1b[38;2;{};{};{}m", color.r, color.g, color.b);
}

fn smoothstep(t: f32) -> f32 {
    let t = fract(t);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    Rgb {
        r: lerp(a.r, b.r, t),
        g: lerp(a.g, b.g, t),
        b: lerp(a.b, b.b, t),
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn fract(t: f32) -> f32 {
    t - t.floor()
}

fn colorize(color: &str, text: &str, reset: &str) -> String {
    if color.is_empty() {
        text.to_string()
    } else {
        format!("{color}{text}{reset}")
    }
}

fn map(s: &str) -> String {
    match s {
        "blue" => "\x1b[34m",
        "cyan" => "\x1b[96m",
        "orange" => "\x1b[38;5;208m",
        "purple" => "\x1b[35m",
        "red" => "\x1b[31m",
        "magenta" => "\x1b[35m",
        "white" => "\x1b[37m",
        "gray" => "\x1b[90m",
        "dim" => "\x1b[90m",
        "none" => "",
        _ => "",
    }
    .to_string()
}
