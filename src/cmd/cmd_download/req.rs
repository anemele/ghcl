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

pub fn get_releases(repo: &Repo) -> anyhow::Result<Releases> {
    let url = api_releases(repo);
    let res = tinyget::get(url).with_header("User-Agent", "gh2").send()?;
    let releases = serde_json::from_slice(res.as_bytes())?;
    Ok(releases)
}

pub fn download_asset<P>(repo: &Repo, asset: &AssetItem, dst: P) -> anyhow::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let url = format!("https://github.com/{}/releases/download/{}", repo, asset);
    let res = tinyget::get(url).with_header("User-Agent", "gh2").send()?;

    let dst = dst.as_ref().join(&asset.asset);
    fs::write(&dst, res.as_bytes())?;

    Ok(dst)
}
