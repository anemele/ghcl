use super::req::download_asset;
use super::req::get_releases;
use crate::config::DownloadConfig;
use crate::parser::parse_url;

pub fn download(url: &str, config: DownloadConfig) -> anyhow::Result<()> {
    // dbg!(&config);

    let Some(repo) = parse_url(url) else {
        anyhow::bail!("Invalid url: {}", url);
    };
    let releases = get_releases(&repo)?;

    // dbg!(&releases);
    // for release in releases {
    //     let assets = release.get_asset_item_list();
    //     dbg!(&assets);
    // }
    let asset = &releases[0].get_asset_item_list()[0];
    let pth = download_asset(&repo, asset, &config.destiny.unwrap_or_default())?;
    println!("Downloaded {}", pth.display());

    Ok(())
}
