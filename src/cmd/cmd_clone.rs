use std::path::Path;
use std::process::Command;

use crate::config::CloneConfig;
use crate::parse_url;
use crate::SSH_URL;

pub fn clone(url: &str, config: CloneConfig) {
    let Some(repo) = parse_url(url) else {
        eprintln!("Invalid url: {}", url);
        return;
    };

    let mirror_url = match config.mirror_url {
        Some(u) => u,
        None => SSH_URL.to_owned(),
    };
    let repo_url = mirror_url + &repo.to_string() + ".git";

    let no_owner = match config.no_owner {
        Some(n) => n,
        None => false,
    };

    let local_dir = if no_owner {
        repo.repo
    } else {
        repo.to_string()
    };

    let local_dst = match config.destiny {
        Some(d) => Path::new(&d).join(local_dir),
        None => Path::new(&local_dir).to_path_buf(),
    };

    let _ = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(local_dst)
        .arg("--depth=1")
        .status();
}
