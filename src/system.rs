use crate::app::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Sparkline},
};

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B/s", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB/s", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB/s", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn render_system_widgets(f: &mut Frame, rect: Rect, app: &AppState) {
    let system_block = Block::default().title("System").borders(Borders::ALL);
    let system_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(system_block.inner(rect));

    f.render_widget(system_block, rect);

    // CPU Usage
    let cpu_sparkline = Sparkline::default()
        .block(Block::default().title("CPU Usage (%)"))
        .data(&app.cpu_history)
        .style(Style::default().fg(Color::Green));
    f.render_widget(cpu_sparkline, system_layout[0]);

    // Memory Usage
    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent( (app.memory_usage * 100.0) as u16);
    f.render_widget(mem_gauge, system_layout[1]);

    // Network Usage
    let network_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(system_layout[2]);

    let up_speed = app.network_up_history.last().unwrap_or(&0);
    let down_speed = app.network_down_history.last().unwrap_or(&0);

    let network_up_sparkline = Sparkline::default()
        .block(Block::default().title(format!("Network Up ({})", format_bytes(*up_speed))))
        .data(&app.network_up_history)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(network_up_sparkline, network_layout[0]);

    let network_down_sparkline = Sparkline::default()
        .block(Block::default().title(format!("Network Down ({})", format_bytes(*down_speed))))
        .data(&app.network_down_history)
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(network_down_sparkline, network_layout[1]);
}
