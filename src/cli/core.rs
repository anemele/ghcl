use clap::Parser;

use crate::cmd;

#[derive(Parser)]
enum Cli {
    /// wrapper of `git clone`
    #[clap(aliases=["c","cl"])]
    Clone {
        #[arg()]
        url: String,
        #[arg(short, long, default_value = ".")]
        dst: String,
    },
}

pub fn run() {
    match Cli::parse() {
        Cli::Clone { url, dst } => cmd::clone(&url, dst),
    }
}
