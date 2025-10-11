use clap::{Parser, Subcommand, crate_description, crate_version};
use clap_complete::Shell;
use parquet::ComplexTypesMode;
use std::path::PathBuf;

pub mod channel;
use channel::DataType;

pub mod parquet;

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
    /// Manage Sift CLI configuration
    #[command(subcommand)]
    Config(ConfigCmd),

    /// Import time series files into Sift
    #[command(subcommand)]
    Import(ImportCmd),

    /// Manage shell autocompletions
    #[command(subcommand)]
    Completions(CompletionsCmd),
}

#[derive(Subcommand)]
pub enum CompletionsCmd {
    /// Print completions for your shell
    Print(CompletionsPrintArgs),

    /// Attempts to automatically update this CLI's completions file for the current shell
    Update,
}

#[derive(clap::Args)]
pub struct CompletionsPrintArgs {
    /// The shell to print completions for. If empty the program will try to infer the user shell
    /// by reading the "$SHELL" environment variable.
    #[arg(short, long)]
    pub shell: Option<Shell>,
}

#[derive(Subcommand)]
pub enum ImportCmd {
    /// Import a CSV file into Sift. Unless manually specified all columns are inferred to type
    /// string or double.
    Csv(ImportCsvArgs),

    /// Import a Parquet file into Sift.
    #[command(subcommand)]
    Parquet(ImportParquetCmd),
}

#[derive(Subcommand)]
pub enum ConfigCmd {
    /// Display the contents of the current config file
    Show,

    /// Show the path to the current config file
    Where,

    /// Create a new config file (fails if one already exists)
    Create,

    /// Update fields in the existing config file
    Update(ConfigUpdateArgs),
}

#[derive(clap::Args)]
pub struct ConfigUpdateArgs {
    /// Edit or create a profile interactively (ignores other flags)
    #[arg(short, long)]
    pub interactive: bool,

    /// Base gRPC endpoint for Sift
    #[arg(short, long)]
    pub grpc_uri: Option<String>,

    /// Base REST endpoint for Sift
    #[arg(short, long)]
    pub rest_uri: Option<String>,

    /// API key used for authentication
    #[arg(short = 'k', long)]
    pub api_key: Option<String>,
}

#[derive(clap::Args)]
pub struct ImportCsvArgs {
    /// Path to the CSV file to import
    pub path: PathBuf,

    /// Name of the asset this data belongs to
    #[arg(short, long)]
    pub asset: String,

    /// Optional run name to associate with this import
    #[arg(short, long)]
    pub run: Option<String>,

    /// Row number containing column headers (1-based)
    #[arg(long, default_value_t = 1)]
    pub header_row: usize,

    /// Row number where data starts (1-based)
    #[arg(long, default_value_t = 2)]
    pub first_data_row: usize,

    /// 1-based column indices to override; can appear multiple times
    #[arg(short, long)]
    pub channel_column: Vec<usize>,

    /// Data type for each channel in `--channel-column`. Use `"infer"` to have the program infer
    /// the data type which is useful when wanting to just specify `--unit` and/or `--description`
    #[arg(short, long)]
    pub data_type: Vec<DataType>,

    /// Unit for each channel in `--channel-column` (can be empty)
    #[arg(short, long)]
    pub unit: Vec<String>,

    /// Description for each channel in `--channel-column` (can be empty)
    #[arg(short = 'n', long)]
    pub description: Vec<String>,

    /// Enum configuration pairs `<key,name>` (e.g. `"0,start|1,stop"`) for enum-type channels
    #[arg(short, long)]
    pub enum_config: Vec<String>,

    /// Bit-field configuration triplets `<name,index,length>` (e.g. `"12v,0,4|led,4,4"`)
    #[arg(short, long)]
    pub bit_field_config: Vec<String>,

    /// 1-based index of the time column
    #[arg(short, long, default_value_t = 1)]
    pub time_column: usize,

    /// Time format used in the file
    #[arg(short = 'f', long, default_value_t = TimeFormat::default())]
    pub time_format: TimeFormat,

    /// Start time (RFC3339) to use if time format is relative
    #[arg(short = 's')]
    pub relative_start_time: Option<String>,

    /// Wait until the import finishes processing
    #[arg(short, long)]
    pub wait: bool,

    /// Preview the parsed schema without uploading
    #[arg(short, long)]
    pub preview: bool,
}

#[derive(Subcommand)]
pub enum ImportParquetCmd {
    /// A parquet file where every column is exclusive to a single channel except for the time
    /// column
    FlatDataset(FlatDatasetArgs),
}

#[derive(clap::Args)]
pub struct FlatDatasetArgs {
    /// Path to the Parquet file to import
    pub path: PathBuf,

    /// Name of the asset this data belongs to
    #[arg(short, long)]
    pub asset: String,

    /// Optional run name to associate with this import
    #[arg(short, long)]
    pub run: Option<String>,

    /// Paths of data columns to import; can be specified multiple times
    #[arg(short, long)]
    pub channel_path: Vec<String>,

    /// Data type for each channel in `--channel-path`. Use `"infer"` to have the program infer
    /// the data type which is useful when wanting to just specify `--unit` and/or `--description`
    #[arg(short, long)]
    pub data_type: Vec<DataType>,

    /// Unit for each channel in `--channel-path` (can be empty)
    #[arg(short, long)]
    pub unit: Vec<String>,

    /// Description for each channel in `--channel-path` (can be empty)
    #[arg(short = 'n', long)]
    pub description: Vec<String>,

    /// Enum configuration pairs `<key,name>` for enum-type channels
    #[arg(short, long)]
    pub enum_config: Vec<String>,

    /// Bit-field configuration triplets `<index,name,bit_count>` for bit-field channels
    #[arg(short, long)]
    pub bit_field_config: Vec<String>,

    /// Path to the time column
    #[arg(short, long, default_value_t = String::from("timestamp"))]
    pub time_path: String,

    /// Time format used in the file
    #[arg(short = 'f', long, default_value_t = TimeFormat::default())]
    pub time_format: TimeFormat,

    /// Start time (RFC3339) to use if time format is relative
    #[arg(short = 's')]
    pub relative_start_time: Option<String>,

    /// Strategy for handling complex types (maps, lists, structs)
    #[arg(short = 'm', long, default_value_t = ComplexTypesMode::default())]
    pub complex_types_mode: ComplexTypesMode,

    /// Wait until the import finishes processing
    #[arg(short, long)]
    pub wait: bool,

    /// Preview the parsed schema without uploading
    #[arg(short, long)]
    pub preview: bool,
}
