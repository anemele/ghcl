use clap::Parser;

#[derive(Parser)]
#[clap(name = "gh2", version, author, about = "gh complement")]
pub enum Cli {
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

    /// config actions
    #[clap(subcommand, name = "config")]
    ConfigSubCmd(ConfigSubCmd),
}

#[derive(Parser)]
pub enum ConfigSubCmd {
    /// list all config
    #[clap(alias = "ls")]
    List,

    /// get key.subkey
    Get { key: String },

    /// set key.subkey value
    Set { key: String, value: String },

    /// unset key.subkey
    Unset { key: String },
}
