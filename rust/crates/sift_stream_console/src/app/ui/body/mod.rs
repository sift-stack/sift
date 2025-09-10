use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::app::Context;

mod config;
use config::Config;

mod metrics;
use metrics::Metrics;

mod logs;
use logs::Logs;

pub struct Body<'a> {
    ctx: Context<'a>,
}

impl<'a> Body<'a> {
    pub fn new(ctx: Context<'a>) -> Self {
        Self { ctx }
    }
}

impl Widget for Body<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [config_container, metrics_container, logs_container] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(60),
                Constraint::Percentage(30),
            ])
            .areas(area);

        Config::from(self.ctx).render(config_container, buf);
        Metrics::from(self.ctx).render(metrics_container, buf);
        Logs::new(self.ctx).render(logs_container, buf);
    }
}
