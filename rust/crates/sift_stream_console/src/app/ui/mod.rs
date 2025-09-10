use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

mod header;
use header::Header;

mod body;
use body::Body;

mod footer;

use super::Context;

pub fn render<'a>(frame: &mut Frame, ctx: Context<'a>) {
    let [header_container, body_container] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(6),
            Constraint::Percentage(94),
        ])
        .areas(frame.area());

    frame.render_widget(Header::default(), header_container);
    frame.render_widget(Body::new(ctx), body_container);
}

mod util {
    use ratatui::style::{Color, Style};

    pub fn focused_border_style(selected: bool) -> Style {
        if selected {
            Style::new().fg(Color::Green)
        } else {
            Style::new()
        }
    }
}
