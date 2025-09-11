use crate::app::AppState;
use chrono_tz::America::Los_Angeles;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_time_widgets(f: &mut Frame, rect: Rect, app: &AppState) {
    let time_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(rect);

    // Local Time
    let local_time_str = app.local_time.format("%Y-%m-%d\n%H:%M:%S").to_string();
    let local_time_widget = Paragraph::new(local_time_str)
        .block(Block::default().title("Local").borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(local_time_widget, time_layout[0]);

    // UTC Time
    let utc_time_str = app.utc_time.format("%Y-%m-%d\n%H:%M:%S").to_string();
    let utc_time_widget = Paragraph::new(utc_time_str)
        .block(Block::default().title("UTC").borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(utc_time_widget, time_layout[1]);

    // West Coast Time
    let west_coast_time = app.utc_time.with_timezone(&Los_Angeles);
    let west_coast_time_str = west_coast_time.format("%Y-%m-%d\n%H:%M:%S").to_string();
    let west_coast_widget = Paragraph::new(west_coast_time_str)
        .block(
            Block::default()
                .title("West Coast (PST)")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);
    f.render_widget(west_coast_widget, time_layout[2]);
}
