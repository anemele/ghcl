use clap::Parser;

mod cmd;
mod config;
mod consts;
mod parser;

#[derive(Parser, Debug)]
#[clap(version, author, about = "github clone tool")]
pub struct Args {
    url: String,

    /// destiny to clone a repository
    #[arg(short, long)]
    dst: Option<String>,

    /// whether to create a directory of the owner
    #[arg(long)]
    no_owner: Option<bool>,
}

fn main() -> anyhow::Result<()> {
    let mut ghcl_config = config::read_config();
    // dbg!(&ghcl_config);

    let args = Args::parse();
    // dbg!(&args);

    if let Some(dst) = args.dst {
        ghcl_config.destiny = dst;
    }
    if let Some(no) = args.no_owner {
        ghcl_config.no_owner = ghcl_config.no_owner || no;
    }

    cmd::clone(&args.url, ghcl_config)
}
