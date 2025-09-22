use crate::app::AppState;
use crate::disks::render_disk_widgets;
use crate::system::render_system_widgets;
use crate::time::{render_local_time_widget, render_world_time_widget};
use crate::vcs::render_vcs_widgets;
use ratatui::prelude::*;

pub fn ui(f: &mut Frame, app: &AppState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Top row
            Constraint::Percentage(30), // System widgets
            Constraint::Percentage(50), // VCS
        ])
        .split(f.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(main_layout[0]);

    // Time Widgets
    render_local_time_widget(f, top_layout[0], app);
    render_world_time_widget(f, top_layout[1], app);

    // Disk Usage
    render_disk_widgets(f, top_layout[2], app);

    // System Info
    render_system_widgets(f, main_layout[1], app);

    // Version Control
    render_vcs_widgets(f, main_layout[2], app);
}
