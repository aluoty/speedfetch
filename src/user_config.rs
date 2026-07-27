use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct UserConfig {
    pub logo: LogoConfig,
    pub key: KeyConfig,
    pub color: ColorConfig,
    pub display: DisplayConfig,
    pub bar: BarConfig,
    pub percent: PercentConfig,
    pub size: SizeConfig,
    pub temp: TempConfig,
    pub layout: LayoutConfig,
    #[serde(default)]
    pub show: Vec<String>,
    #[serde(default)]
    pub separator: String,
    #[serde(default)]
    pub distro: std::collections::HashMap<String, DistroOverride>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LogoConfig {
    #[serde(default = "default_logo_mode")]
    pub mode: LogoMode,
    #[serde(default)]
    pub padding: LogoPadding,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LogoPadding {
    #[serde(default)]
    pub top: usize,
    #[serde(default)]
    pub left: usize,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum LogoMode {
    Small,
    Regular,
    None,
}

impl Default for LogoMode {
    fn default() -> Self {
        LogoMode::Regular
    }
}

fn default_logo_mode() -> LogoMode {
    LogoMode::Regular
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct KeyConfig {
    pub width: Option<usize>,
    #[serde(default = "default_key_padding")]
    pub padding_left: usize,
    #[serde(default = "default_key_type")]
    #[serde(rename = "type")]
    pub key_type: KeyType,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum KeyType {
    String,
    Icon,
    Both,
    None,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::String
    }
}

fn default_key_padding() -> usize {
    0
}

fn default_key_type() -> KeyType {
    KeyType::String
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ColorConfig {
    pub keys: Option<String>,
    pub values: Option<String>,
    pub separator: Option<String>,
    #[serde(default)]
    pub bright: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
    pub errors: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BarConfig {
    pub width: usize,
    #[serde(default = "default_bar_char")]
    pub char_elapsed: char,
    #[serde(default = "default_bar_char_total")]
    pub char_total: char,
    pub border_left: Option<String>,
    pub border_right: Option<String>,
}

fn default_bar_char() -> char {
    '█'
}

fn default_bar_char_total() -> char {
    '░'
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct PercentConfig {
    #[serde(default = "default_percent_type")]
    #[serde(rename = "type")]
    pub percent_type: PercentType,
    #[serde(default = "default_ndigits")]
    pub ndigits: usize,
    pub color_green: Option<String>,
    pub color_yellow: Option<String>,
    pub color_red: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum PercentType {
    Number,
    Bar,
    Both,
    Colored,
}

impl Default for PercentType {
    fn default() -> Self {
        PercentType::Number
    }
}

fn default_percent_type() -> PercentType {
    PercentType::Number
}

fn default_ndigits() -> usize {
    1
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SizeConfig {
    #[serde(default = "default_binary_prefix")]
    pub binary_prefix: BinaryPrefix,
    #[serde(default)]
    pub ndigits: usize,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum BinaryPrefix {
    Iec,
    Si,
    Jedec,
}

impl Default for BinaryPrefix {
    fn default() -> Self {
        BinaryPrefix::Iec
    }
}

fn default_binary_prefix() -> BinaryPrefix {
    BinaryPrefix::Iec
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct TempConfig {
    #[serde(default = "default_temp_unit")]
    pub unit: TempUnit,
    #[serde(default)]
    pub ndigits: usize,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum TempUnit {
    C,
    F,
    K,
}

impl Default for TempUnit {
    fn default() -> Self {
        TempUnit::C
    }
}

fn default_temp_unit() -> TempUnit {
    TempUnit::C
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LayoutConfig {
    pub boxes: bool,
    pub title: bool,
    pub separator_line: bool,
    pub padding: LayoutPadding,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LayoutPadding {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            boxes: false,
            title: false,
            separator_line: false,
            padding: LayoutPadding::default(),
        }
    }
}

impl Default for LayoutPadding {
    fn default() -> Self {
        Self {
            top: 0,
            bottom: 0,
            left: 1,
            right: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DistroOverride {
    #[serde(default)]
    pub logo: Option<Vec<String>>,
    #[serde(default)]
    pub small_logo: Option<Vec<String>>,
    pub theme: Option<DistroThemeOverride>,
}

#[derive(Debug, Deserialize)]
pub struct DistroThemeOverride {
    pub label: Option<String>,
    pub value: Option<String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            logo: LogoConfig::default(),
            key: KeyConfig::default(),
            color: ColorConfig::default(),
            display: DisplayConfig::default(),
            bar: BarConfig::default(),
            percent: PercentConfig::default(),
            size: SizeConfig::default(),
            temp: TempConfig::default(),
            layout: LayoutConfig::default(),
            show: Vec::new(),
            separator: " ".to_string(),
            distro: std::collections::HashMap::new(),
        }
    }
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            mode: LogoMode::Regular,
            padding: LogoPadding::default(),
        }
    }
}

impl Default for LogoPadding {
    fn default() -> Self {
        Self { top: 0, left: 0 }
    }
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            width: None,
            padding_left: 0,
            key_type: KeyType::String,
        }
    }
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            width: 20,
            char_elapsed: '█',
            char_total: '░',
            border_left: None,
            border_right: None,
        }
    }
}

impl Default for PercentConfig {
    fn default() -> Self {
        Self {
            percent_type: PercentType::Number,
            ndigits: 1,
            color_green: None,
            color_yellow: None,
            color_red: None,
        }
    }
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            binary_prefix: BinaryPrefix::Iec,
            ndigits: 1,
        }
    }
}

impl Default for TempConfig {
    fn default() -> Self {
        Self {
            unit: TempUnit::C,
            ndigits: 1,
        }
    }
}

impl UserConfig {
    pub fn load(path: Option<&std::path::Path>) -> Self {
        let path = path.map(|p| p.to_path_buf()).unwrap_or_else(config_path);
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("speedfetch: warning: failed to parse {}: {e}", path.display());
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs().join("speedfetch").join("config.toml")
}

fn dirs() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return PathBuf::from(xdg);
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".config");
    }
    PathBuf::from(".config")
}
