use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

pub mod color;
pub use color::Color;

pub mod filter;
pub use filter::{Filter, Pagination};

#[derive(Parser, Debug)]
#[command(
    version = clap::crate_version!(),
    about = "A CLI to interact with your Sift environments",
)]
pub struct Clargs {
    #[command(subcommand)]
    pub subcommands: SubCmds,

    /// Use a specific file from your Sift config file.
    #[arg(short, long, global = true)]
    pub profile: Option<String>,

    /// Controls output colorization
    #[arg(long, default_value_t = Color::default(), global = true)]
    pub color: Color,

    /// Disables TLS when connecting to your Sift environment
    #[arg(short, long, global = true)]
    pub disable_tls: bool,
}

#[derive(Subcommand, Debug)]
pub enum SubCmds {
    /// Display and optionally filter one or many resources
    Get {
        #[command(subcommand)]
        subcommand: GetSubCmds,
    },

    /// Manage your Sift config file and profiles
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubCmds,
    },

    /// Print completions for a particular shell to stdout
    Completions {
        /// Shell to produce completions for - if blank then the 'SHELL' environment variable will
        /// be used
        shell: Option<Shell>,
    },
}

#[derive(Subcommand, Debug)]
pub enum GetSubCmds {
    #[clap(alias = "asset")]
    /// Retrieve and filter assets
    Assets {
        #[command(flatten)]
        filter: Filter,

        #[command(flatten)]
        pagination: Pagination,
    },
    #[clap(alias = "channel")]
    /// Retrieve and filter channels
    Channels {
        #[command(flatten)]
        asset: AssetIdentifier,

        #[command(flatten)]
        filter: Filter,

        #[command(flatten)]
        pagination: Pagination,
    }
}

#[derive(Args, Debug)]
#[group(id = "asset-identifier", multiple = false, required = false)]
pub struct AssetIdentifier {
    /// ID of parent asset - only relevant and required when filtering channels by name unless
    /// --asset-name is used.
    #[arg(long)]
    pub asset_id: Option<String>,

    /// Name of parent asset - only required when filtering channels by name unless --asset-id is
    /// used.
    #[arg(long)]
    pub asset_name: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubCmds {
    /// Print the path and contents of your Sift config file
    Show,

    /// Create a named or default profile
    Create {
        /// The name of the profile to create - if blank then a default profile will be made
        name: Option<String>,

        /// The Sift URI of your allotted environment
        #[arg(short, long)]
        uri: String,

        /// Your Sift API key
        #[arg(short, long)]
        apikey: String,

        /// Forcefully create profile and overwrite the existing profile of the same name
        #[arg(short, long)]
        force: bool,
    },
}
