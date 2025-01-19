use std::fs;

use crate::consts::get_rc_path;
use serde::Deserialize;

fn default_mirror_url() -> String {
    "https://github.com/".to_string()
}

fn default_destiny() -> String {
    ".".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_mirror_url")]
    pub mirror_url: String,
    #[serde(default = "default_destiny")]
    pub destiny: String,
    #[serde(default)]
    pub no_owner: bool,
    #[serde(default)]
    pub git_config: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mirror_url: default_mirror_url(),
            destiny: default_destiny(),
            no_owner: Default::default(),
            git_config: Default::default(),
        }
    }
}

pub fn read_config() -> Config {
    let Ok(rc_path) = get_rc_path() else {
        return Config::default();
    };
    // dbg!(&rc_path);

    if !rc_path.exists() {
        // dbg!("Not found config file");
        return Config::default();
    }

    let Ok(s) = fs::read_to_string(&rc_path) else {
        // eprintln!("Failed to read config: {}", rc_path.display());
        return Config::default();
    };
    // dbg!(&s);

    let Ok(config) = toml::from_str::<Config>(&s) else {
        // eprintln!("Failed to load config: {}", rc_path.display());
        return Config::default();
    };
    // dbg!(&config);

    config
}
