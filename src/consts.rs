use std::path::PathBuf;

const RC_FILE: &str = ".gh2rc";
pub fn get_rc_path() -> anyhow::Result<PathBuf> {
    let Some(home) = homedir::get_my_home()? else {
        anyhow::bail!("No found home dir. I don't know how to fix this.");
    };

    Ok(home.join(RC_FILE))
}
