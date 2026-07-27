use crate::config::{Config, DistroConfig, ThemeConfig};
use crate::user_config::UserConfig;

pub fn load_config(config_path: Option<&std::path::Path>) -> (Config, UserConfig) {
    let embedded: Config =
        toml::from_str(include_str!("config.toml")).expect("invalid embedded config.toml");

    let user = UserConfig::load(config_path);

    let config = apply_user_config(embedded, &user, config_path);

    (config, user)
}

fn apply_user_config(mut config: Config, _user: &UserConfig, config_path: Option<&std::path::Path>) -> Config {
    let path = config_path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(crate::user_config::config_path);

    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(parsed) = toml::from_str::<toml::Value>(&content) {
            if let Some(distro_table) = parsed.get("distro").and_then(|v| v.as_table()) {
                for (name, override_val) in distro_table {
                    let Ok(entry_cfg) = override_val.clone().try_into::<crate::user_config::DistroOverride>() else {
                        continue;
                    };

                    let entry = config.distro.entry(name.clone()).or_insert_with(|| DistroConfig {
                        inherits: String::new(),
                        logo: Vec::new(),
                        small_logo: Vec::new(),
                        theme: ThemeConfig {
                            label: "blue".to_string(),
                            value: "white".to_string(),
                        },
                    });

                    if let Some(logo) = entry_cfg.logo {
                        entry.logo = logo;
                    }
                    if let Some(small) = entry_cfg.small_logo {
                        entry.small_logo = small;
                    }
                    if let Some(theme) = entry_cfg.theme {
                        if let Some(label) = theme.label {
                            entry.theme.label = label;
                        }
                        if let Some(value) = theme.value {
                            entry.theme.value = value;
                        }
                    }
                }
            }
        }
    }

    config
}
