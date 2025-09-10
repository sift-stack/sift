use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

#[derive(Default)]
pub struct Footer;

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let footer = Block::bordered().title("footer");
        footer.render(area, buf);
    }
}
