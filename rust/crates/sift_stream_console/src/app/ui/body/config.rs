use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Cell, Row, Table, Widget},
};

use crate::app::{state::{State, Status}, Context};

pub struct Config<'a> {
    ctx: Context<'a>,
}

impl<'a> From<Context<'a>> for Config<'a> {
    fn from(ctx: Context<'a>) -> Self {
        Self { ctx }
    }
}

impl<'a> Widget for Config<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Context { state, .. } = self.ctx;

        let [
            first_col_container,
            second_col_container,
            third_col_container,
        ] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(40),
                Constraint::Percentage(27),
            ])
            .areas(area);

        first_col(state).render(first_col_container, buf);
        second_col(state).render(second_col_container, buf);
        third_col(state).render(third_col_container, buf);
    }
}

fn first_col<'a>(state: &'a State) -> Table<'a> {
    let rows = [
        Row::new([
            Cell::from("Asset").style(Style::new().dim()),
            Cell::from(state.asset_name.as_str()),
        ]),
        Row::new([
            Cell::from("Run").style(Style::new().dim()),
            Cell::from(state.run_name.as_str()),
        ]),
        Row::new([
            Cell::from("Ingestion Config").style(Style::new().dim()),
            Cell::from(state.ingestion_config_id.as_str()),
        ]),
    ];

    let widths = [Constraint::Length(20), Constraint::Length(42)];

    Table::new(rows, widths)
}

fn second_col<'a>(state: &'a State) -> Table<'a> {
    let status_style = match state.status {
        Status::Running => Style::new().green(),
        Status::Retrying => Style::new().yellow(),
        Status::Off => Style::new().red(),
    };

    let backups_path_style = match state.backup_directory {
        Some(_) => Style::new().fg(Color::Yellow),
        None => Style::default(),
    };

    let rows = [
        Row::new([
            Cell::from("Status").style(Style::new().dim()),
            Cell::from(state.status.as_str()).style(status_style),
        ]),
        Row::new([
            Cell::from("Recovery Counter").style(Style::new().dim()),
            Cell::from(format!("{}", state.recovery_counter)),
        ]),
        Row::new([
            Cell::from("Backup Directory").style(Style::new().dim()),
            Cell::from(state.backup_directory.as_deref().unwrap_or("Disabled"))
                .style(backups_path_style),
        ]),
    ];

    let widths = [Constraint::Length(20), Constraint::Length(50)];

    Table::new(rows, widths)
}

fn third_col<'a>(state: &'a State) -> Table<'a> {
    let rows = [
        Row::new([
            Cell::from("From").style(Style::new().dim()),
            Cell::from(format!("{}", state.from)),
        ]),
        Row::new([
            Cell::from("To").style(Style::new().dim()),
            Cell::from(format!("{}", state.to)),
        ]),
        Row::new([
            Cell::from("Refresh Interval").style(Style::new().dim()),
            Cell::from(format!("{}s", state.refresh_interval.as_secs())),
        ]),
    ];

    let widths = [Constraint::Length(20), Constraint::Length(50)];

    Table::new(rows, widths)
}
