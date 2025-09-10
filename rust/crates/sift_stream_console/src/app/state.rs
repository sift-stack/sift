use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};

pub struct State {
    /// Used for relative offset
    pub start: SystemTime,

    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,

    pub asset_name: String,
    pub run_name: String,
    pub ingestion_config_id: String,
    pub refresh_interval: Duration,
    pub status: Status,
    pub backup_directory: Option<String>,
    pub recovery_counter: usize,

    pub focused: FocusedWidget,

    pub metric_offset: usize,
    pub metric_window_width: usize,
    pub byte_rates: Vec<(f64, f64)>,
    pub message_rates: Vec<(f64, f64)>,
    pub drift: Vec<(f64, f64)>,
    pub checkpoint_response_times: Vec<(f64, f64)>,

    pub logs: Vec<String>,
    pub logs_longest_line_len_in_viewport: usize,
    pub logs_viewport_width: usize,
    pub logs_viewport_height: usize,
    pub logs_entry_offset: usize,
    pub logs_char_offset: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Running,
    Off,
    Retrying,
}

#[derive(Copy, Clone, PartialEq)]
pub enum FocusedWidget {
    Metrics,
    Logs,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Running => "Running",
            Self::Retrying => "Retrying",
            Self::Off => "Off",
        }
    }
}
