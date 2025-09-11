use crate::app::AppState;
use crate::disks::render_disk_widgets;
use crate::system::render_system_widgets;
use crate::time::render_time_widgets;
use crate::vcs::render_vcs_widgets;
use ratatui::prelude::*;

pub fn ui(f: &mut Frame, app: &AppState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout[0]);

    let top_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_layout[0]);

    // System Info
    render_system_widgets(f, top_left_layout[0], app);

    // Time Widgets
    render_time_widgets(f, top_left_layout[1], app);

    // Disk Usage
    render_disk_widgets(f, top_layout[1], app);

    // Version Control
    render_vcs_widgets(f, main_layout[1], app);
}
