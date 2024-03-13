use std::path::Path;
use std::process::Command;

use crate::parse_url;
use crate::SSH_URL;

pub fn clone<P>(url: &str, dst: P)
where
    P: AsRef<Path>,
{
    let Some(repo) = parse_url(url) else {
        return;
    };

    let ssh_url = SSH_URL.to_owned() + &repo.to_string() + ".git";
    let local_dst = dst.as_ref().join(repo.repo);

    let _ = Command::new("git")
        .arg("clone")
        .arg(ssh_url)
        .arg(local_dst)
        .arg("--depth=1")
        .status();
}
