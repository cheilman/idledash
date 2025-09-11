use crate::app::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Sparkline},
};

pub fn render_system_widgets(f: &mut Frame, rect: Rect, app: &AppState) {
    let system_block = Block::default().title("System").borders(Borders::ALL);
    let system_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
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
}
