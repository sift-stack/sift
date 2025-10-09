use clap::{Parser, Subcommand, crate_description, crate_version};
use std::path::PathBuf;

pub mod channel;
use channel::DataType;

pub mod time;
use time::TimeFormat;

#[derive(Parser)]
#[command(
    version = crate_version!(),
    about = crate_description!(),
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,

    /// The profile to use
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Disable TLS for non-cloud Sift environments
    #[arg(long, global = true)]
    pub disable_tls: bool,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Interacting with the Sift CLI config
    #[command(subcommand)]
    Config(ConfigCmd),

    /// Import files containing time series to Sift
    #[command(subcommand)]
    Import(ImportCmd),
}

#[derive(Subcommand)]
pub enum ImportCmd {
    /// Import a CSV file into Sift. Unless manually specified all columns are inferred to type
    /// string or double.
    Csv(CsvArgs),
}

#[derive(Subcommand)]
pub enum ConfigCmd {
    /// Print the contents of the config file if it exists
    Show,

    /// Prints the location of the config file
    Where,

    /// Creates a new config file or errors if it already exists
    Create,

    /// Updates the config file
    Update(ConfigUpdateArgs),
}

#[derive(clap::Args)]
pub struct ConfigUpdateArgs {
    /// Create or update a profile interactively, ignoring all other arguments
    #[arg(short, long)]
    pub interactive: bool,

    /// The Sift gRPC base URI
    #[arg(short, long)]
    pub grpc_uri: Option<String>,

    /// The Sift REST base URI
    #[arg(short, long)]
    pub rest_uri: Option<String>,

    /// Your Sift API key
    #[arg(short = 'k', long)]
    pub api_key: Option<String>,
}

#[derive(clap::Args)]
pub struct CsvArgs {
    /// Path to the CSV file
    pub path: PathBuf,

    /// The name of the asset this data is associated with
    #[arg(short, long)]
    pub asset: String,

    /// The name of the wrong to associate this data with
    #[arg(short, long)]
    pub run: Option<String>,

    /// The row containing the channel names and timestamp (1-based indexing)
    #[arg(long, default_value_t = 1)]
    pub header_row: usize,

    /// The first row containing time series data (1-based indexing)
    #[arg(long, default_value_t = 2)]
    pub first_data_row: usize,

    /// Column number to configure an override for (1-based indexing); can be specified multiple times
    #[arg(short, long)]
    pub channel_column: Vec<usize>,

    /// Column-type corresponding to ordered positioning of --channel-column
    #[arg(short, long)]
    pub data_type: Vec<DataType>,

    /// Channel units corresponding to ordered positioning of --channel-column; can be an empty
    /// string
    #[arg(short, long)]
    pub unit: Vec<String>,

    /// Channel description corresponding to ordered positioning of --channel-column; can be an empty
    /// string
    #[arg(short = 'n', long)]
    pub description: Vec<String>,

    /// <name,key> repeated pairs e.g. "0,start,1,stop". Corresponds to the order in
    /// which enum channels appear in --channel-column
    #[arg(short, long)]
    pub enum_config: Vec<String>,

    /// <index,name,bit_count> repeated triplets e.g. "0,12v,4,4,led,4". Corresponds
    /// to the order in which bit-field channels appear in --channel-column
    #[arg(short, long)]
    pub bit_field_config: Vec<String>,

    /// Column number of the time column (1-based indexing)
    #[arg(short, long, default_value_t = 1)]
    pub time_column: usize,

    #[arg(short = 'f', long, default_value_t = TimeFormat::default())]
    pub time_format: TimeFormat,

    /// Start time to use (RFC3339) when time format is relative; ignored otherwise
    #[arg(short = 's')]
    pub relative_start_time: Option<String>,

    /// Wait for the CSV to be fully processed before returning
    #[arg(short, long)]
    pub wait: bool,
}
