use super::rest::{AssetItem, Releases};
use crate::parser::Repo;
use std::fs;
use std::path::{Path, PathBuf};

fn api_releases(repo: &Repo) -> String {
    format!("https://api.github.com/repos/{}/releases", repo)
}

// fn api_asset(repo: Repo, release: Release) -> String {
//     format!("https://github.com/{}/releases/download/{}")
// }

pub fn get_releases(repo: &Repo) -> Option<Releases> {
    let url = api_releases(repo);
    let Ok(res) = tinyget::get(url).with_header("User-Agent", "gh2").send() else {
        return None;
    };
    let Ok(releases) = serde_json::from_slice(res.as_bytes()) else {
        return None;
    };

    Some(releases)
}

pub fn download_asset<P>(repo: &Repo, asset: &AssetItem, dst: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let url = format!("https://github.com/{}/releases/download/{}", repo, asset);
    let Ok(res) = tinyget::get(url).with_header("User-Agent", "gh2").send() else {
        return None;
    };

    let dst = dst.as_ref().join(&asset.asset);
    if fs::write(&dst, res.as_bytes()).is_ok() {
        Some(dst)
    } else {
        None
    }
}
