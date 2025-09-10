use crossterm::event::Event;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
    }
};

use crate::app::{state::FocusedWidget, ui::util::focused_border_style, Context};

pub struct Logs<'a> {
    ctx: Context<'a>
}

impl<'a> Logs<'a> {
    pub fn new(ctx: Context<'a>) -> Self {
        Self { ctx }
    }
}

// TODO: Logic to handle resizing.

impl Widget for Logs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Context { state, dispatcher, event_listeners } = self.ctx;

        let [logs_container] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .areas(area);

        let border_style = focused_border_style(state.focused == FocusedWidget::Logs);
        let container = Block::bordered()
            .title("Logs")
            .title_style(Style::new().fg(Color::Magenta))
            .border_style(border_style);

        // Subtract 2 because the border adds two additional rows/columns
        let viewport_height = usize::from(logs_container.height).saturating_sub(2);
        let viewport_width = usize::from(logs_container.width).saturating_sub(2);

        let num_logs = state.logs.len();
        let start = num_logs
            .saturating_sub(1)
            .saturating_sub(viewport_height)
            .min(state.logs_entry_offset);
        let end = (start + viewport_height).min(num_logs);

        let mut longest_line_len = 0;
        let logs_to_display = &state.logs[start..end];

        let logs = logs_to_display
            .iter()
            .map(|l| {
                longest_line_len = longest_line_len.max(l.len());
                Line::from(l.as_str())
            })
            .collect::<Vec<Line<'_>>>();

        event_listeners.add("logs", move |e| match e {
            Event::Resize(columns, rows) => dispatcher.dispatch(move |s| {
                ()
            })
            _ => ()
        });

        dispatcher.dispatch(move |state| {
            state.logs_longest_line_len_in_viewport = longest_line_len;
        });

        if state.logs_viewport_height == 0 {
            dispatcher.dispatch(move |state| {
                state.logs_viewport_height = viewport_height;
                state.logs_viewport_width = viewport_width;
                state.logs_entry_offset = num_logs
                    .saturating_sub(1)
                    .saturating_sub(viewport_height);
            });
            return container.render(logs_container, buf);
        }

        Paragraph::new(logs)
            .scroll((0, state.logs_char_offset as u16))
            .block(
                Block::bordered()
                    .title("Logs")
                    .title_style(Style::new().fg(Color::Magenta))
                    .border_style(border_style)
            )
            .render(logs_container, buf);

        let mut y_scrollbar_state = ScrollbarState::new(num_logs.saturating_sub(viewport_height))
            .position(start);

        // y-scrollbar
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::new().fg(Color::LightYellow))
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(logs_container, buf, &mut y_scrollbar_state);

        let mut x_scrollbar_state = ScrollbarState::new(longest_line_len.saturating_sub(viewport_width))
            .position(state.logs_char_offset);

        // x-scrollbar
        Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("■")
            .thumb_style(Style::new().fg(Color::Green))
            .begin_symbol(Some("←"))
            .end_symbol(Some("→"))
            .render(logs_container, buf, &mut x_scrollbar_state)
    }
}
