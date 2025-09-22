use crate::app::AppState;
use chrono_tz::America::{Los_Angeles, New_York};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use tui_big_text::{BigText, PixelSize};

pub fn render_local_time_widget(f: &mut Frame, rect: Rect, app: &AppState) {
    let local_time_str = app.local_time.format("%H:%M:%S").to_string();
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec![Line::from(local_time_str)])
        .style(Style::default().fg(Color::Blue))
        .build();

    f.render_widget(big_text, rect);
}

pub fn render_world_time_widget(f: &mut Frame, rect: Rect, app: &AppState) {
    let time_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(rect);

    // UTC Time
    let utc_time_str = app.utc_time.format("%H:%M:%S").to_string();
    let utc_time_widget = Paragraph::new(utc_time_str)
        .block(Block::default().title("UTC").borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(utc_time_widget, time_layout[0]);

    // East Coast Time
    let east_coast_time = app.utc_time.with_timezone(&New_York);
    let east_coast_time_str = east_coast_time.format("%H:%M:%S").to_string();
    let east_coast_widget = Paragraph::new(east_coast_time_str)
        .block(
            Block::default()
                .title("East Coast (EST)")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);
    f.render_widget(east_coast_widget, time_layout[1]);

    // West Coast Time
    let west_coast_time = app.utc_time.with_timezone(&Los_Angeles);
    let west_coast_time_str = west_coast_time.format("%H:%M:%S").to_string();
    let west_coast_widget = Paragraph::new(west_coast_time_str)
        .block(
            Block::default()
                .title("West Coast (PST)")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);
    f.render_widget(west_coast_widget, time_layout[2]);
}
