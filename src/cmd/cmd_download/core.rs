use crate::cmd::cmd_download::req::get_releases;
use crate::config::DownloadConfig;
use crate::parser::parse_url;

pub fn download(url: &str, config: DownloadConfig) {
    dbg!(&config);

    let Some(repo) = parse_url(url) else {
        eprintln!("Invalid url: {}", url);
        return;
    };
    let Some(releases) = get_releases(repo) else {
        return;
    };

    dbg!(&releases);
}
