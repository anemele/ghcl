use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
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

impl Default for CloneConfig {
    fn default() -> Self {
        Self {
            mirror_url: Some(String::from("https://github.com/")),
            destiny: Some(String::from(".")),
            no_owner: Default::default(),
            git_config: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DownloadConfig {
    pub mirror_urls: Option<Vec<String>>,
    pub destiny: Option<String>,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            mirror_urls: Default::default(),
            destiny: Some(String::from(".")),
        }
    }
}
