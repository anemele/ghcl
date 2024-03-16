use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::parser::Repo;

use super::rest::{AssetItem, Releases};
use reqwest::blocking::Client;

fn api_releases(repo: &Repo) -> String {
    format!("https://api.github.com/repos/{}/releases", repo)
}

// fn api_asset(repo: Repo, release: Release) -> String {
//     format!("https://github.com/{}/releases/download/{}")
// }

pub fn get_releases(repo: &Repo) -> Option<Releases> {
    let url = api_releases(repo);
    let Ok(res) = Client::new().get(url).header("User-Agent", "gh2").send() else {
        return None;
    };
    let Ok(releases) = res.json::<Releases>() else {
        return None;
    };

    Some(releases)
}

pub fn download_asset<P>(repo: &Repo, asset: &AssetItem, dst: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let url = format!("https://github.com/{}/releases/download/{}", repo, asset);
    let Ok(res) = Client::new().get(url).header("User-Agent", "gh2").send() else {
        return None;
    };
    let Ok(bytes) = res.bytes() else {
        return None;
    };

    let dst = dst.as_ref().join(&asset.asset);
    if fs::write(&dst, bytes).is_ok() {
        Some(dst)
    } else {
        None
    }
}
