use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub distro: HashMap<String, DistroConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DistroConfig {
    #[serde(default)]
    pub inherits: String,
    #[serde(default)]
    pub logo: Vec<String>,
    #[serde(default)]
    pub small_logo: Vec<String>,
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}
