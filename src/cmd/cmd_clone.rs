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

    let local_dir = if config.no_owner.unwrap_or(false) {
        repo.repo
    } else {
        repo.to_string()
    };

    let local_dst = match config.destiny {
        Some(d) => Path::new(&d).join(local_dir),
        None => Path::new(&local_dir).to_path_buf(),
    };

    let mut cmd = &mut Command::new("git");
    cmd = cmd.arg("clone").arg(repo_url).arg(local_dst);

    if let Some(git_config) = config.git_config {
        cmd = cmd.args(git_config);
    }

    let _ = cmd.status();
}
