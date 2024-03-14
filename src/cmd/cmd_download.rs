use crate::config::DownloadConfig;
use crate::parser::parse_url;
use crate::parser::Repo;

fn api_releases(repo: &Repo) -> String {
    format!("https://api.github.com/repos/{}/releases", repo)
}

pub fn download(url: &str, config: DownloadConfig) {
    let Some(repo) = parse_url(url) else {
        eprintln!("Invalid url: {}", url);
        return;
    };

    let api_url = api_releases(&repo);
    dbg!(&api_url);

    dbg!(&config);
}
