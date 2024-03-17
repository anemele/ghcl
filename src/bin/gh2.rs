use clap::Parser;
use gh2::cmd;
use gh2::config;

#[derive(Parser)]
#[clap(
    name = "gh2",
    version,
    author,
    about = "gh complement",
    long_about = None,
)]
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

    /// download releases from GitHub
    #[clap(aliases=["d","dl"])]
    Download {
        url: String,

        /// destiny to download files
        #[arg(short, long)]
        dst: Option<String>,
    },
}

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

        Cli::Download { url: _, dst: _ } => {
            println!("Not Implemented Yet");

            // let download_config = match gh2_config.download {
            //     Some(mut c) => {
            //         if dst.is_some() {
            //             c.destiny = dst;
            //         }
            //         c
            //     }
            //     None => config::DownloadConfig {
            //         mirror_urls: None,
            //         destiny: dst,
            //     },
            // };
            // // dbg!(&download_config);

            // cmd::download(&url, download_config)
        }
    }
}
