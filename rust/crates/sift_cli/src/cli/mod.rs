use clap::{Parser, Subcommand, crate_version};
use clap_complete::Shell;
use parquet::{ChannelMode, ComplexTypesMode};
pub mod hdf5;
pub mod tdms;
pub mod ulog;
use hdf5::Hdf5Schema;
use std::{net::SocketAddr, path::PathBuf};
use tdms::TdmsFallbackMethod;
use ulog::UlogParseErrorPolicy;

pub mod channel;
use channel::DataType;

pub mod agent;
pub mod export;
pub mod parquet;

pub mod time;
use time::TimeFormat;

#[derive(Parser)]
#[command(
    version = crate_version!(),
    about = "Sift command-line interface for importing and exporting time-series data.",
    disable_version_flag = true,
    override_usage = "sift-cli <COMMAND>",
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,

    /// Print the installed CLI version and check for a newer release on GitHub
    #[arg(short = 'V', long)]
    pub version: bool,

    #[arg(long, global = true, hide = true)]
    pub profile: Option<String>,

    #[arg(long, global = true, hide = true)]
    pub disable_tls: bool,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Import time series files into Sift
    #[command(subcommand)]
    Import(ImportCmd),

    /// Export asset/run data from Sift
    #[command(subcommand)]
    Export(ExportCmd),

    /// Ping the Sift API to verify credentials and connectivity
    Ping,

    /// Manage Sift CLI configuration
    #[command(subcommand)]
    Config(ConfigCmd),

    Doc(DocArgs),

    #[command(subcommand)]
    Install(InstallCmd),

    /// Start the Sift MCP server
    #[command(hide = true)]
    Mcp,
}

/// Serve the bundled Sift CLI user documentation over HTTP.
#[derive(clap::Args)]
pub struct DocArgs {
    /// Address the documentation HTTP server binds to.
    #[arg(long, default_value_t = Self::default_addr())]
    pub addr: SocketAddr,
}

/// Install optional Sift tooling such as autocompletions or Agent skills
#[derive(Subcommand)]
pub enum InstallCmd {
    /// Install or print shell completions for sift-cli
    #[command(subcommand)]
    Completions(CompletionsCmd),

    /// Install Sift-specific skills for agentic tooling
    AgentSkills(AgentSkillsArgs),
}

#[derive(clap::Args)]
pub struct AgentSkillsArgs {
    /// The agentic coding assistant to install the skill for.
    pub agent: agent::Agent,

    /// Path to write the skill file to. When omitted, defaults to the
    /// standard skill location for the selected agent.
    #[arg(long)]
    pub output: Option<String>,

    /// Print the skill content to stdout instead of writing it to --output.
    #[arg(long)]
    pub print: bool,
}

#[derive(Subcommand)]
pub enum ExportCmd {
    /// Export data for a run
    Run(ExportRunArgs),

    /// Export data for an asset
    Asset(ExportAssetArgs),
}

#[derive(clap::Args)]
pub struct ExportRunArgs {
    /// The name of the run
    #[arg(short, long, group = "run_identifier")]
    pub name: Option<String>,

    /// The ID of the run
    #[arg(short, long, group = "run_identifier")]
    pub run_id: Option<String>,

    /// The client key of the run
    #[arg(short = 'k', long, group = "run_identifier")]
    pub client_key: Option<String>,

    #[command(flatten)]
    pub common: ExportArgs,
}

#[derive(clap::Args)]
pub struct ExportAssetArgs {
    /// The name of the asset
    pub asset: String,

    #[command(flatten)]
    pub common: ExportArgs,
}

#[derive(clap::Args)]
pub struct ExportArgs {
    /// The file to generate
    #[arg(short, long)]
    pub output: PathBuf,

    /// File format for the output file
    #[arg(short, long)]
    pub format: export::Format,

    /// Regular expression used to filter channels to include in the export
    #[arg(short = 'x', long)]
    pub channel_regex: Option<String>,

    /// Name of channel to include in the export; can be specified multiple times
    #[arg(short, long)]
    pub channel: Vec<String>,

    /// ID of channel to include in the export; can be specified multiple times
    #[arg(long)]
    pub channel_id: Vec<String>,

    /// Regular expression used to filter calculated channels to include in the export
    #[arg(long)]
    pub calculated_channel_regex: Option<String>,

    /// Name of calculated channel to include in the export; can be specified multiple times
    #[arg(long)]
    pub calculated_channel: Vec<String>,

    /// ID of calculated channel to include in the export; can be specified multiple times
    #[arg(long)]
    pub calculated_channel_id: Vec<String>,

    /// Start time in RFC 3339 format (required for asset exports)
    #[arg(long)]
    pub start: Option<String>,

    /// Stop time in RFC 3339 format (required for asset exports)
    #[arg(long)]
    pub stop: Option<String>,
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
    /// CSV file
    ///
    /// Unless manually specified, all columns are inferred to type string or double.
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import csv <PATH> --asset <ASSET> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import csv data.csv --asset engine"
    )]
    Csv(ImportCsvArgs),

    /// Parquet file
    #[command(subcommand)]
    Parquet(ImportParquetCmd),

    /// TDMS file
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import tdms <PATH> --asset <ASSET> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import tdms data.tdms --asset engine"
    )]
    Tdms(ImportTdmsArgs),

    /// HDF5 file
    ///
    /// Supported channel types: bool, int8/16/32/64, uint8/16/32/64, float32,
    /// float64. Datasets with other types produce a client-side error.
    #[command(subcommand)]
    Hdf5(ImportHdf5Cmd),

    /// PX4 ULog file
    ///
    /// Uses the log's GPS fix, or --relative-start-time when no fix exists.
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import ulog <PATH> --asset <ASSET> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import ulog data.ulg --asset engine"
    )]
    Ulog(ImportUlogArgs),

    /// Backup files from sift_stream
    ///
    /// Run without a subcommand to import; use `ls` to list files without importing.
    #[command(name = "backups")]
    Backup(BackupArgs),
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

    /// Sift web app URL (e.g. https://app.siftstack.com). Optional for standard
    /// Sift hosts; required for custom or on-prem deployments to render Explore
    /// links.
    #[arg(long)]
    pub app_uri: Option<String>,
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
    #[arg(long, default_value_t = 1, hide_short_help = true)]
    pub header_row: usize,

    /// Row number where data starts (1-based)
    #[arg(long, default_value_t = 2, hide_short_help = true)]
    pub first_data_row: usize,

    /// 1-based column indices to override; can appear multiple times
    #[arg(short, long, hide_short_help = true)]
    pub channel_column: Vec<usize>,

    /// Data type for each channel in `--channel-column`. Use `"infer"` to have the program infer
    /// the data type which is useful when wanting to just specify `--unit` and/or `--description`
    #[arg(short, long, hide_short_help = true)]
    pub data_type: Vec<DataType>,

    /// Unit for each channel in `--channel-column` (can be empty)
    #[arg(short, long, hide_short_help = true)]
    pub unit: Vec<String>,

    /// Description for each channel in `--channel-column` (can be empty)
    #[arg(short = 'n', long, hide_short_help = true)]
    pub description: Vec<String>,

    /// Enum configuration pairs `<key,name>` (e.g. `"0,start|1,stop"`) for enum-type channels
    #[arg(short, long, hide_short_help = true)]
    pub enum_config: Vec<String>,

    /// Bit-field configuration triplets `<name,index,length>` (e.g. `"12v,0,4|led,4,4"`)
    #[arg(short, long, hide_short_help = true)]
    pub bit_field_config: Vec<String>,

    /// 1-based index of the time column
    #[arg(short, long, default_value_t = 1, hide_short_help = true)]
    pub time_column: usize,

    /// Time format used in the file
    #[arg(short = 'f', long, default_value_t = TimeFormat::default(), hide_possible_values = true, hide_short_help = true)]
    pub time_format: TimeFormat,

    /// Start time (RFC3339) to use if time format is relative
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,

    /// Wait until the import finishes processing
    #[arg(short, long)]
    pub wait: bool,

    /// Preview the parsed schema without uploading
    #[arg(short, long)]
    pub preview: bool,
}

#[derive(clap::Args)]
pub struct BackupArgs {
    #[command(subcommand)]
    pub cmd: Option<BackupCmd>,

    #[command(flatten)]
    pub import_args: ImportBackupArgs,
}

#[derive(Subcommand)]
pub enum BackupCmd {
    /// List backup files in a directory
    Ls(BackupLsArgs),
}

#[derive(clap::Args)]
pub struct BackupLsArgs {
    /// Path to the directory containing backup files (defaults to your OS data directory if not provided)
    pub path: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct ImportBackupArgs {
    /// Path to the directory containing backup files to import (defaults to your OS data directory if not provided)
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Delete backup files after successful upload
    #[arg(short, long)]
    pub cleanup: bool,
}

#[derive(clap::Args)]
pub struct CommonImportArgs {
    /// Path to the file to import
    pub path: PathBuf,

    /// Name of the asset this data belongs to
    #[arg(short, long)]
    pub asset: String,

    /// Optional run name to associate with this import
    #[arg(short, long)]
    pub run: Option<String>,

    /// The id of an existing run to add this data to. Mutually exclusive with --run
    #[arg(long, conflicts_with = "run")]
    pub run_id: Option<String>,

    /// Wait until the import finishes processing
    #[arg(short, long)]
    pub wait: bool,

    /// Preview the parsed schema without uploading
    #[arg(short, long)]
    pub preview: bool,
}

#[derive(Subcommand)]
pub enum ImportParquetCmd {
    /// Flat dataset (each column is a channel; one time column)
    ///
    /// A parquet file where every column is exclusive to a single channel except for the time
    /// column.
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import parquet flat-dataset <PATH> --asset <ASSET> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import parquet flat-dataset data.parquet --asset engine"
    )]
    FlatDataset(FlatDatasetArgs),

    /// Channel-per-row layout
    ///
    /// One channel for the whole file (single) or a name column identifying the channel per row (multi).
    #[command(name = "cpr", subcommand)]
    ChannelPerRow(ImportParquetCprCmd),
}

#[derive(clap::Args)]
pub struct FlatDatasetArgs {
    #[command(flatten)]
    pub common: CommonImportArgs,

    /// Paths of data columns to import; can be specified multiple times
    #[arg(short, long, hide_short_help = true)]
    pub channel_path: Vec<String>,

    /// Data type for each channel in `--channel-path`. Use `"infer"` to have the program infer
    /// the data type which is useful when wanting to just specify `--unit` and/or `--description`
    #[arg(short, long, hide_short_help = true)]
    pub data_type: Vec<DataType>,

    /// Unit for each channel in `--channel-path` (can be empty)
    #[arg(short, long, hide_short_help = true)]
    pub unit: Vec<String>,

    /// Description for each channel in `--channel-path` (can be empty)
    #[arg(short = 'n', long, hide_short_help = true)]
    pub description: Vec<String>,

    /// Enum configuration pairs `<key,name>` (e.g. `"0,start|1,stop"`) for enum-type channels
    #[arg(short, long, hide_short_help = true)]
    pub enum_config: Vec<String>,

    /// Bit-field configuration triplets `<name,index,length>` (e.g. `"12v,0,4|led,4,4"`) for bit-field channels
    #[arg(short, long, hide_short_help = true)]
    pub bit_field_config: Vec<String>,

    /// Path to the time column. Auto-detected from common names (time, timestamp, timestamps, ts) if omitted
    #[arg(short, long, hide_short_help = true)]
    pub time_path: Option<String>,

    /// Time format used in the file. Inferred from the time column's Arrow type if omitted
    #[arg(short = 'f', long, hide_possible_values = true, hide_short_help = true)]
    pub time_format: Option<TimeFormat>,

    /// Start time (RFC3339) to use if time format is relative
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,

    /// Strategy for handling complex types (maps, lists, structs)
    #[arg(short = 'm', long, default_value_t = ComplexTypesMode::default(), hide_short_help = true)]
    pub complex_types_mode: ComplexTypesMode,
}

#[derive(Subcommand)]
pub enum ImportParquetCprCmd {
    /// One channel for the whole file
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import parquet cpr single <PATH> --asset <ASSET> --data-path <DATA_PATH> --channel-name <CHANNEL_NAME> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import parquet cpr single data.parquet --asset engine \\\n    --data-path value --channel-name temp"
    )]
    Single(ChannelPerRowSingleArgs),

    /// Name column identifies the channel per row
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import parquet cpr multi <PATH> --asset <ASSET> --data-path <DATA_PATH> --name-path <NAME_PATH> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import parquet cpr multi data.parquet --asset engine \\\n    --data-path value --name-path channel"
    )]
    Multi(ChannelPerRowMultiArgs),
}

#[derive(clap::Args)]
pub struct ChannelPerRowCommonArgs {
    #[command(flatten)]
    pub common: CommonImportArgs,

    /// Path to the time column. Auto-detected from common names (time, timestamp, timestamps, ts) if omitted
    #[arg(short, long, hide_short_help = true)]
    pub time_path: Option<String>,

    /// Time format used in the time column. Inferred from the time column's Arrow type if omitted
    #[arg(short = 'f', long, hide_possible_values = true, hide_short_help = true)]
    pub time_format: Option<TimeFormat>,

    /// Start time (RFC3339) to use if time format is relative
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,

    /// Path to the column holding values
    #[arg(long)]
    pub data_path: String,

    /// Strategy for handling complex types (maps, lists, structs)
    #[arg(short = 'm', long, default_value_t = ComplexTypesMode::default(), hide_short_help = true)]
    pub complex_types_mode: ComplexTypesMode,
}

#[derive(clap::Args)]
pub struct ChannelPerRowSingleArgs {
    #[command(flatten)]
    pub common: ChannelPerRowCommonArgs,

    /// Channel name for every row in the file
    #[arg(long)]
    pub channel_name: String,

    /// Data type for the channel. Use `"infer"` to have the program infer the
    /// data type from the parquet schema.
    #[arg(long, hide_short_help = true)]
    pub data_type: Option<DataType>,

    /// Channel units
    #[arg(long, hide_short_help = true)]
    pub unit: Option<String>,

    /// Channel description
    #[arg(short = 'n', long, hide_short_help = true)]
    pub description: Option<String>,
}

#[derive(clap::Args)]
pub struct ChannelPerRowMultiArgs {
    #[command(flatten)]
    pub common: ChannelPerRowCommonArgs,

    /// Path to the column holding channel names
    #[arg(long)]
    pub name_path: String,
}

pub struct ChannelPerRowArgs {
    pub common: CommonImportArgs,
    pub mode: ChannelMode,
    pub time_path: Option<String>,
    pub time_format: Option<TimeFormat>,
    pub relative_start_time: Option<String>,
    pub data_path: String,
    pub channel_name: Option<String>,
    pub data_type: Option<DataType>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub name_path: Option<String>,
    pub complex_types_mode: ComplexTypesMode,
}

impl From<ChannelPerRowSingleArgs> for ChannelPerRowArgs {
    fn from(args: ChannelPerRowSingleArgs) -> Self {
        Self {
            common: args.common.common,
            mode: ChannelMode::Single,
            time_path: args.common.time_path,
            time_format: args.common.time_format,
            relative_start_time: args.common.relative_start_time,
            data_path: args.common.data_path,
            channel_name: Some(args.channel_name),
            data_type: args.data_type,
            unit: args.unit,
            description: args.description,
            name_path: None,
            complex_types_mode: args.common.complex_types_mode,
        }
    }
}

impl From<ChannelPerRowMultiArgs> for ChannelPerRowArgs {
    fn from(args: ChannelPerRowMultiArgs) -> Self {
        Self {
            common: args.common.common,
            mode: ChannelMode::Multi,
            time_path: args.common.time_path,
            time_format: args.common.time_format,
            relative_start_time: args.common.relative_start_time,
            data_path: args.common.data_path,
            channel_name: None,
            data_type: None,
            unit: None,
            description: None,
            name_path: Some(args.name_path),
            complex_types_mode: args.common.complex_types_mode,
        }
    }
}

#[derive(clap::Args)]
pub struct ImportTdmsArgs {
    #[command(flatten)]
    pub common: CommonImportArgs,

    /// Optional override on start time
    #[arg(long, hide_short_help = true)]
    pub start_time_override: Option<String>,

    /// Fallback method for channels with missing timing information
    #[arg(short, long, default_value = "fail-on-error", hide_short_help = true)]
    pub fallback_method: TdmsFallbackMethod,

    /// Time format for the channels not using the TDMS timestamp type
    #[arg(long, hide_possible_values = true, hide_short_help = true)]
    pub time_format: Option<TimeFormat>,

    /// Relative start time for channels using a non standard time channel
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,

    /// Import TDMS file properties to the run as metadata
    #[arg(long, hide_short_help = true)]
    pub import_file_properties: bool,
}

#[derive(Subcommand)]
pub enum ImportHdf5Cmd {
    /// One channel per dataset
    #[command(
        name = "one-d",
        arg_required_else_help = true,
        override_usage = "sift-cli import hdf5 one-d <PATH> --asset <ASSET> --time-format <TIME_FORMAT> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import hdf5 one-d data.h5 --asset engine --time-format absolute-rfc3339"
    )]
    OneD(ImportHdf5OneDArgs),

    /// Channels are columns of a 2-D dataset
    #[command(
        name = "two-d",
        arg_required_else_help = true,
        override_usage = "sift-cli import hdf5 two-d <PATH> --asset <ASSET> --time-format <TIME_FORMAT> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import hdf5 two-d data.h5 --asset engine --time-format absolute-rfc3339"
    )]
    TwoD(ImportHdf5TwoDArgs),

    /// Channels are named fields of a compound dataset
    #[command(
        arg_required_else_help = true,
        override_usage = "sift-cli import hdf5 compound <PATH> --asset <ASSET> --time-format <TIME_FORMAT> [OPTIONS]",
        before_help = "[Reduced help — `--help` shows all options]",
        before_long_help = "",
        after_help = "Example:\n  sift-cli import hdf5 compound data.h5 --asset engine \\\n    --time-format absolute-rfc3339 --time-field ts"
    )]
    Compound(ImportHdf5CompoundArgs),
}

#[derive(clap::Args)]
pub struct ImportHdf5CommonArgs {
    #[command(flatten)]
    pub common: CommonImportArgs,

    /// Time format used in the time dataset/column
    #[arg(long, required = true, hide_possible_values = true)]
    pub time_format: Option<TimeFormat>,

    /// Start time (RFC3339) if the time format is relative
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,
}

#[derive(clap::Args)]
pub struct ImportHdf5OneDArgs {
    #[command(flatten)]
    pub common: ImportHdf5CommonArgs,

    /// Name of the time dataset. Overrides auto-detection (time, timestamp, timestamps, ts)
    #[arg(long)]
    pub time_name: Option<String>,
}

#[derive(clap::Args)]
pub struct ImportHdf5TwoDArgs {
    #[command(flatten)]
    pub common: ImportHdf5CommonArgs,

    /// Index of the time column. Defaults to 0
    #[arg(long)]
    pub time_index: Option<u64>,
}

#[derive(clap::Args)]
pub struct ImportHdf5CompoundArgs {
    #[command(flatten)]
    pub common: ImportHdf5CommonArgs,

    /// Name of the time field. Mutually exclusive with --time-index
    #[arg(long, conflicts_with = "time_index")]
    pub time_field: Option<String>,

    /// Index of the time field. Defaults to 0. Mutually exclusive with --time-field
    #[arg(long)]
    pub time_index: Option<u64>,
}

pub struct ImportHdf5Args {
    pub common: CommonImportArgs,
    pub schema: Hdf5Schema,
    pub time_format: Option<TimeFormat>,
    pub relative_start_time: Option<String>,
    pub time_index: Option<u64>,
    pub time_field: Option<String>,
    pub time_name: Option<String>,
}

impl From<ImportHdf5OneDArgs> for ImportHdf5Args {
    fn from(args: ImportHdf5OneDArgs) -> Self {
        Self {
            common: args.common.common,
            schema: Hdf5Schema::OneD,
            time_format: args.common.time_format,
            relative_start_time: args.common.relative_start_time,
            time_index: None,
            time_field: None,
            time_name: args.time_name,
        }
    }
}

impl From<ImportHdf5TwoDArgs> for ImportHdf5Args {
    fn from(args: ImportHdf5TwoDArgs) -> Self {
        Self {
            common: args.common.common,
            schema: Hdf5Schema::TwoD,
            time_format: args.common.time_format,
            relative_start_time: args.common.relative_start_time,
            time_index: args.time_index,
            time_field: None,
            time_name: None,
        }
    }
}

impl From<ImportHdf5CompoundArgs> for ImportHdf5Args {
    fn from(args: ImportHdf5CompoundArgs) -> Self {
        Self {
            common: args.common.common,
            schema: Hdf5Schema::Compound,
            time_format: args.common.time_format,
            relative_start_time: args.common.relative_start_time,
            time_index: args.time_index,
            time_field: args.time_field,
            time_name: None,
        }
    }
}

#[derive(clap::Args)]
pub struct ImportUlogArgs {
    #[command(flatten)]
    pub common: CommonImportArgs,

    /// Log start time (RFC3339) for boot-relative timestamps. Overrides the
    /// log's GPS fix; required when no fix exists.
    #[arg(short = 's', long, hide_short_help = true)]
    pub relative_start_time: Option<String>,

    /// Info key to import as run metadata (`info.<key>`); repeatable. Requires
    /// --run or --run-id.
    #[arg(long, hide_short_help = true)]
    pub info_key: Vec<String>,

    /// Parameter to import as run metadata (`param.<name>`); repeatable.
    /// Requires --run or --run-id.
    #[arg(long, hide_short_help = true)]
    pub param_key: Vec<String>,

    /// Handling for recoverable parse errors, such as truncated records.
    #[arg(long, default_value = "fail-on-error", hide_short_help = true)]
    pub parse_error_policy: UlogParseErrorPolicy,
}

impl DocArgs {
    fn default_addr() -> SocketAddr {
        "0.0.0.0:3000".parse().unwrap()
    }
}
