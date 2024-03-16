use crate::parser::Repo;

use super::rest::Releases;
use reqwest::blocking::Client;

fn api_releases(repo: Repo) -> String {
    format!("https://api.github.com/repos/{}/releases", repo)
}

pub fn get_releases(repo: Repo) -> Option<Releases> {
    let url = api_releases(repo);
    let Ok(res) = Client::new().get(url).header("User-Agent", "gh2").send() else {
        return None;
    };
    let Ok(releases) = res.json::<Releases>() else {
        return None;
    };

    Some(releases)
}
