use clap::ValueEnum;
use sift_rs::exports::v1::ExportOutputFormat;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Format {
    Csv,
    Sun,
}

impl From<Format> for ExportOutputFormat {
    fn from(val: Format) -> Self {
        match val {
            Format::Csv => ExportOutputFormat::Csv,
            Format::Sun => ExportOutputFormat::Sun,
        }
    }
}
