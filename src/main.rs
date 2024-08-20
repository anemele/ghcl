use clap::Parser;
use gh2::cli::{Cli, ConfigSubCmd};
use gh2::cmd;
use gh2::config;

fn main() {
    let gh2_config = config::read_config();
    // dbg!(&gh2_config);

    match Cli::parse() {
        Cli::Clone { url, dst, no_owner } => {
            let clone_config = match gh2_config.clone {
                Some(mut c) => {
                    if dst.is_some() {
                        c.destiny = dst;
                    }
                    if let Some(n) = c.no_owner {
                        c.no_owner = Some(no_owner || n);
                    } else {
                        c.no_owner = Some(no_owner)
                    }
                    c
                }
                None => config::CloneConfig {
                    mirror_url: None,
                    destiny: dst,
                    no_owner: Some(no_owner),
                    git_config: None,
                },
            };
            // dbg!(&clone_config);

            cmd::clone(&url, clone_config)
        }

        Cli::Download { url, dst } => {
            let download_config = match gh2_config.download {
                Some(mut c) => {
                    if dst.is_some() {
                        c.destiny = dst;
                    }
                    c
                }
                None => config::DownloadConfig {
                    mirror_urls: None,
                    destiny: dst,
                },
            };
            // dbg!(&download_config);

            cmd::download(&url, download_config)
        }

        Cli::ConfigSubCmd(subcmd) => match subcmd {
            ConfigSubCmd::List => {}
            ConfigSubCmd::Get { key } => {
                dbg!(key);
            }
            ConfigSubCmd::Set { key, value } => {
                dbg!(key, value);
            }
            ConfigSubCmd::Unset { key } => {
                dbg!(key);
            }
        },
    }
}
