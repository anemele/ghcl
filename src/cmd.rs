use std::path::Path;
use std::process::Command;

use crate::config::Config;
use crate::parser::parse_url;

pub fn clone(url: &str, config: Config) -> anyhow::Result<()> {
    let Some(repo) = parse_url(url) else {
        anyhow::bail!("Invalid url: {}", url);
    };

    let repo_url = config.mirror_url + &repo.to_string() + ".git";

    let local_dir = if config.no_owner {
        repo.repo
    } else {
        repo.to_string()
    };
    let local_dst = Path::new(&config.destiny).join(local_dir);

    let mut cmd = &mut Command::new("git");
    cmd = cmd.arg("clone").arg(repo_url).arg(local_dst);
    if !config.git_config.is_empty() {
        cmd = cmd.args(config.git_config);
    }

    let code = cmd.status()?;
    if !code.success() {
        anyhow::bail!("Failed to clone repository: {}", url);
    }

    Ok(())
}
