use super::req::download_asset;
use super::req::get_releases;
use crate::config::DownloadConfig;
use crate::parser::parse_url;

pub fn download(url: &str, config: DownloadConfig) {
    // dbg!(&config);

    let Some(repo) = parse_url(url) else {
        eprintln!("Invalid url: {}", url);
        return;
    };
    let Some(releases) = get_releases(&repo) else {
        return;
    };

    // dbg!(&releases);
    // for release in releases {
    //     let assets = release.get_asset_item_list();
    //     dbg!(&assets);
    // }
    let asset = &releases[0].get_asset_item_list()[0];
    if let Some(pth) = download_asset(&repo, asset, &config.destiny.unwrap_or_default()) {
        println!("Downloaded {}", pth.display())
    }
}
