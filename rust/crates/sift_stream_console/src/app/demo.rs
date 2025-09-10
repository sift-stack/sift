use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use ratatui::DefaultTerminal;
use strip_ansi_escapes::strip_str as strip_ansi;

use super::{state::{FocusedWidget, Status}, App, State};

pub fn run(terminal: DefaultTerminal) {
    let tempdir = std::env::temp_dir().to_string_lossy().to_string();
    let now = SystemTime::now();
    let then = now
        .checked_sub(Duration::from_secs(300))
        .expect("failed to compute start time for demo");

    let now_utc: DateTime<Utc> = now.into();
    let then_utc: DateTime<Utc> = then.into();

    let test_logs = include_str!("../../assets/test_logs.txt")
        .split("\n")
        .map(strip_ansi)
        .collect::<Vec<String>>();

    let app = App::new(State {
        start: now,
        asset_name: "NostromoLV426".into(),
        run_name: "[NostromoLV426].1756587472".into(),
        ingestion_config_id: "6b1b7041-2f0b-4046-9ab8-69afa2edf44a".into(),
        refresh_interval: Duration::from_secs(5),
        status: Status::Running,
        backup_directory: Some(tempdir),
        recovery_counter: 0,
        from: then_utc,
        to: now_utc,
        focused: FocusedWidget::Metrics,
        byte_rates: vec![
            (0.0, 1.0),
            (1.0, 1.59),
            (2.0, 1.95),
            (3.0, 1.95),
            (4.0, 1.59),
            (5.0, 1.0),
            (6.0, 0.41),
            (7.0, 0.05),
            (8.0, 0.05),
            (9.0, 0.41),
        ],
        message_rates: vec![
            (0.0, 1.0),
            (1.0, 1.59),
            (2.0, 1.95),
            (3.0, 1.95),
            (4.0, 1.59),
            (5.0, 1.0),
            (6.0, 0.41),
            (7.0, 0.05),
            (8.0, 0.05),
            (9.0, 0.41),
        ],
        drift: vec![
            (0.0, 12.0),
            (1.0, 1.0),
            (2.0, 50.0),
            (3.0, 27.2),
            (4.0, 21.1),
            (5.0, 11.0),
            (6.0, 8.0),
            (7.0, 3.0),
            (8.0, 2.1),
            (9.0, 3.2),
        ],
        checkpoint_response_times: vec![
            (0.0, 1.0),
            (1.0, 2.0),
            (2.0, 3.0),
            (3.0, 3.5),
            (4.0, 1.1),
            (5.0, 1.0),
            (6.0, 2.0),
            (7.0, 1.7),
            (8.0, 0.1),
            (9.0, 0.5),
        ],
        metric_offset: 0,
        metric_window_width: 10,
        logs: test_logs,
        logs_longest_line_len_in_viewport: 0,
        logs_viewport_width: 0,
        logs_viewport_height: 0,
        logs_entry_offset: 0,
        logs_char_offset: 0,
    });
    app.run(terminal);
}
