use crate::app::AppState;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};

pub fn render_disk_widgets(f: &mut Frame, rect: Rect, app: &AppState) {
    let disk_block = Block::default().title("Disks").borders(Borders::ALL);

    let header_cells = ["Mount", "FS", "Total", "Free"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.disks.iter().map(|disk| {
        let total_space = disk.total_space() as f64 / 1_000_000_000.0;
        let available_space = disk.available_space() as f64 / 1_000_000_000.0;
        let cells = vec![
            Cell::from(disk.mount_point().to_string_lossy()),
            Cell::from(disk.file_system().to_string_lossy()),
            Cell::from(format!("{:.2} GB", total_space)),
            Cell::from(format!("{:.2} GB", available_space)),
        ];
        Row::new(cells).height(1)
    });

    let table = Table::new(rows, [Constraint::Percentage(40), Constraint::Percentage(15), Constraint::Percentage(20), Constraint::Percentage(25)])
        .header(header)
        .block(disk_block);

    f.render_widget(table, rect);
}
