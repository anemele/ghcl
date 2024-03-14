use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub clone: Option<CloneConfig>,
}

#[derive(Deserialize, Debug)]
pub struct CloneConfig {
    pub mirror_url: Option<String>,
    pub destiny: Option<String>,
    pub no_owner: Option<bool>,
    pub git_config: Option<Vec<String>>,
}
