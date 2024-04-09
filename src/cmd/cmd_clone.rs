use std::path::Path;
use std::process::Command;

use crate::config::CloneConfig;
use crate::parser::parse_url;

pub fn clone(url: &str, config: CloneConfig) {
    let Some(repo) = parse_url(url) else {
        eprintln!("Invalid url: {}", url);
        return;
    };

    let repo_url = config.mirror_url.unwrap_or_default() + &repo.to_string() + ".git";

    let local_dir = if config.no_owner.unwrap_or(false) {
        repo.repo
    } else {
        repo.to_string()
    };
    let local_dst = Path::new(&config.destiny.unwrap_or_default()).join(local_dir);

    let mut cmd = &mut Command::new("git");
    cmd = cmd.arg("clone").arg(repo_url).arg(local_dst);
    if let Some(git_config) = config.git_config {
        cmd = cmd.args(git_config);
    }

    let _ = cmd.status();
}
