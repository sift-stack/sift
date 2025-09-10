use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};

#[derive(Default)]
pub struct Header;

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_container] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .areas(area);

        let title = Paragraph::new("Sift Stream Console")
            .alignment(Alignment::Center)
            .style(Style::new().fg(Color::Cyan).bold());

        title.render(header_container, buf);
    }
}
