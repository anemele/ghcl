use clap::Parser;
use gh2::cmd;
use gh2::config;

#[derive(Parser)]
enum Cli {
    /// wrapper of `git clone`
    #[clap(aliases=["c","cl"])]
    Clone {
        url: String,

        /// destiny to clone a repository
        #[arg(short, long)]
        dst: Option<String>,

        /// whether to create a directory of the owner
        #[arg(long)]
        no_owner: bool,
    },
}

fn main() {
    let config = match config::read_config() {
        Some(c) => c,
        None => config::Config { clone: None },
    };
    // dbg!(&config);

    match Cli::parse() {
        Cli::Clone { url, dst, no_owner } => {
            let clone_config = match config.clone {
                Some(mut c) => {
                    if dst.is_some() {
                        c.destiny = dst;
                    }
                    if c.no_owner.is_none() {
                        c.no_owner = Some(no_owner);
                    };
                    c
                }
                None => config::CloneConfig {
                    mirror_url: None,
                    destiny: dst,
                    no_owner: Some(no_owner),
                },
            };
            // dbg!(&clone_config);

            cmd::clone(&url, clone_config)
        }
    }
}
