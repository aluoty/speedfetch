use crate::config::Config;

pub fn load_config() -> Config {
    toml::from_str(include_str!("config.toml")).expect("invalid config.toml")
}
