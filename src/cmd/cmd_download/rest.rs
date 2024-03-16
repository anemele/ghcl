// See https://api.github.com/repos/{owner}/{repo}/releases

use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub label: String,
    pub size: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Debug)]
pub struct AssetItem {
    pub release: String,
    pub asset: String,
}

impl Display for AssetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.release, self.asset)
    }
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
}

impl Release {
    pub fn get_asset_item_list(&self) -> Vec<AssetItem> {
        let mut ret = vec![];

        for asset in &self.assets {
            ret.push(AssetItem {
                release: self.name.to_owned(),
                asset: asset.name.to_owned(),
            })
        }

        ret
    }
}

pub type Releases = Vec<Release>;
