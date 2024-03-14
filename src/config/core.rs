use std::fs;

use super::segs::Config;
use crate::consts::get_rc_path;
use toml;

pub fn read_config() -> Option<Config> {
    let rc_path = get_rc_path();
    // dbg!(&rc_path);

    if !rc_path.exists() {
        // dbg!("Not found config file");
        return None;
    }

    let Ok(s) = fs::read_to_string(&rc_path) else {
        // eprintln!("Failed to read config: {}", rc_path.display());
        return None;
    };
    // dbg!(&s);

    let Ok(config) = toml::from_str::<Config>(&s) else {
        // eprintln!("Failed to load config: {}", rc_path.display());
        return None;
    };
    // dbg!(&config);

    Some(config)
}
