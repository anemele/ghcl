use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub clone: Option<CloneConfig>,
    pub download: Option<DownloadConfig>,
}

#[derive(Deserialize, Debug)]
pub struct CloneConfig {
    pub mirror_url: Option<String>,
    pub destiny: Option<String>,
    pub no_owner: Option<bool>,
    pub git_config: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct DownloadConfig {
    pub mirror_urls: Option<Vec<String>>,
    pub destiny: Option<String>,
}
