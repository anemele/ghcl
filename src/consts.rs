use std::path::PathBuf;

use homedir::get_my_home;

const RC_FILE: &str = ".gh2rc";
pub fn get_rc_path() -> PathBuf {
    let Ok(home) = get_my_home() else {
        panic!("No found home dir. I don't know how to fix this.");
    };
    let Some(home) = home else {
        panic!("No found home dir. I don't know how to fix this.");
    };

    home.join(RC_FILE)
}
