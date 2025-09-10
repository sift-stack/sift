use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Padding, Widget},
};

use crate::app::{state::FocusedWidget, ui::util::focused_border_style, Context};

const X_AXIS_PADDING: f64 = 1.0;
const Y_AXIS_PADDING: f64 = 1.0;

pub struct Metrics<'a> {
    ctx: Context<'a>,
}

impl<'a> From<Context<'a>> for Metrics<'a> {
    fn from(ctx: Context<'a>) -> Self {
        Self { ctx }
    }
}

impl Widget for Metrics<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Context { state, .. } = self.ctx;

        let border_style = focused_border_style(matches!(state.focused, FocusedWidget::Metrics));

        Block::bordered()
            .title("Metrics")
            .title_style(Style::new().fg(Color::Cyan))
            .border_style(border_style)
            .render(area, buf);

        let [top_container, bottom_container] = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(2)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(area);

        let [top_left, top_right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(top_container);

        let [bottom_left, bottom_right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_container);

        metric(
            "Byte Rate",
            &state.byte_rates,
            state.metric_offset,
            state.metric_window_width,
            Color::Blue,
            "MiB/s",
        )
        .render(top_left, buf);

        metric(
            "Message Rate",
            &state.message_rates,
            state.metric_offset,
            state.metric_window_width,
            Color::Magenta,
            "messages/s",
        )
        .render(top_right, buf);

        metric(
            "Message Drift",
            &state.drift,
            state.metric_offset,
            state.metric_window_width,
            Color::Yellow,
            "ms",
        )
        .render(bottom_left, buf);

        metric(
            "Checkpoint Response Times",
            &state.checkpoint_response_times,
            state.metric_offset,
            state.metric_window_width,
            Color::Cyan,
            "ms",
        )
        .render(bottom_right, buf);
    }
}

fn metric<'a>(
    chart_name: &'a str,
    time_series: &'a [(f64, f64)],
    metric_offset: usize,
    metric_window_width: usize,
    data_color: Color,
    y_axis_label: &'a str,
) -> Chart<'a> {
    let util::AxisData {
        time_series,
        xbounds,
        xlabels,
        ybounds,
        ylabels,
    } = util::AxisData::new(time_series, metric_offset, metric_window_width);

    let datasets = vec![
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(data_color))
            .data(time_series),
    ];

    let x_axis = Axis::default()
        .title("s".green())
        .style(Style::default().white())
        .bounds(xbounds)
        .labels(xlabels);

    let y_axis = Axis::default()
        .title(y_axis_label.green())
        .style(Style::default().white())
        .bounds(ybounds)
        .labels(ylabels);

    Chart::new(datasets)
        .block(
            Block::new()
                .title(chart_name)
                .padding(Padding::horizontal(5)),
        )
        .x_axis(x_axis)
        .y_axis(y_axis)
}

mod util {
    use super::{X_AXIS_PADDING, Y_AXIS_PADDING};
    use std::usize;

    pub struct AxisData<'a> {
        pub time_series: &'a [(f64, f64)],
        pub xbounds: [f64; 2],
        pub xlabels: [String; 2],
        pub ybounds: [f64; 2],
        pub ylabels: [String; 2],
    }

    impl<'a> AxisData<'a> {
        pub fn new(data: &[(f64, f64)], data_offset: usize, data_width: usize) -> AxisData {
            let time_series = &data[data_offset..data_offset + data_width];

            let mut xl_bound = f64::MAX;
            let mut xr_bound = 0_f64;
            let mut yb_bound = f64::MAX;
            let mut yt_bound = 0_f64;

            for (t, v) in time_series {
                xl_bound = xl_bound.min(*t);
                xr_bound = xr_bound.max(*t);
                yb_bound = yb_bound.min(*v);
                yt_bound = yt_bound.max(*v);
            }

            yt_bound += Y_AXIS_PADDING;
            xr_bound += X_AXIS_PADDING;

            let xbounds = [xl_bound, xr_bound];
            let xlabels = [format!("{xl_bound:.2}"), format!("{xr_bound:.2}")];

            let ybounds = [yb_bound, yt_bound];
            let ylabels = [format!("{yb_bound:.2}"), format!("{yt_bound:.2}")];

            AxisData {
                time_series,
                xbounds,
                xlabels,
                ybounds,
                ylabels,
            }
        }
    }
}
